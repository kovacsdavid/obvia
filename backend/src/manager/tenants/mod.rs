/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::common::AppState;
use crate::common::BaseModule;
use crate::common::database::DatabaseMigrator;
use crate::common::database::PoolManager;
use crate::common::error::RepositoryResult;
use crate::manager::tenants::repository::TenantsRepository;
use crate::manager::users::repository::UsersRepository as ManagerUserRepository;
use crate::tenant::users::repository::UsersRepository as TenantUserRepository;
use lettre::{
    AsyncTransport,
    transport::smtp::{Error, response::Response},
};
use std::sync::Arc;
use uuid::Uuid;

pub(crate) mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
mod service;
pub(crate) mod types;

pub trait TenantsModuleInterface: DatabaseMigrator + PoolManager + BaseModule {
    fn tenants_repo(&self) -> Arc<dyn TenantsRepository + Send + Sync>;
    fn tenant_user_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn TenantUserRepository + Send + Sync>>;
    fn manager_user_repo(&self) -> Arc<dyn ManagerUserRepository + Send + Sync>;
}

impl<P, T> TenantsModuleInterface for AppState<P, T>
where
    P: DatabaseMigrator + PoolManager,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync,
{
    fn tenants_repo(&self) -> Arc<dyn TenantsRepository + Send + Sync> {
        self.get_main_pool()
    }
    fn tenant_user_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn TenantUserRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn manager_user_repo(&self) -> Arc<dyn ManagerUserRepository + Send + Sync> {
        self.get_main_pool()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::{BaseModule, ConfigProvider, MailTransporter};
    use crate::manager::tenants::model::Tenant;
    use crate::{
        common::config::{AppConfig, database_config::BasicDatabaseConfig},
        common::database::{DatabaseMigrator, PoolManager},
        common::error::RepositoryError,
    };
    use lettre::{
        Message,
        transport::smtp::{Error, response::Response},
    };
    use mockall::mock;
    use sqlx::PgPool;
    use uuid::Uuid;

    mock!(
        pub TenantsModule {}
        impl ConfigProvider for TenantsModule {
            type Cfg = AppConfig;
            fn config(&self) -> &<Self as ConfigProvider>::Cfg;
        }
        impl MailTransporter for TenantsModule {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl BaseModule for TenantsModule {}
        impl PoolManager for TenantsModule {
            fn get_main_pool(&self) -> Arc<PgPool>;
            fn get_tenant_pool(&self, tenant_id: Uuid) -> Result<Arc<PgPool>, RepositoryError>;
            fn add_tenant_pool(
                &self,
                tenant_id: Uuid,
                config: &BasicDatabaseConfig,
            ) -> impl Future<Output = Result<Uuid, RepositoryError>> + Send;
            fn delete_tenant_pool(
                &self,
                tenant_id: Uuid,
            ) -> impl Future<Output = Result<(), RepositoryError>> + Send;
        }
        impl DatabaseMigrator for TenantsModule {
            fn migrate_main_db(&self) -> impl Future<Output = RepositoryResult<()>> + Send;
            fn migrate_tenant_db(
                &self,
                tenant_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<()>> + Send;
            fn migrate_all_tenant_dbs(
                &self,
                tenants: &[Tenant],
            ) -> impl Future<Output = RepositoryResult<()>> + Send;
        }
        impl TenantsModuleInterface for TenantsModule {
            fn tenants_repo(&self) -> Arc<dyn TenantsRepository + Send + Sync>;
            fn tenant_user_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn TenantUserRepository + Send + Sync>>;
            fn manager_user_repo(&self) -> Arc<dyn ManagerUserRepository + Send + Sync>;
        }
    );
}
