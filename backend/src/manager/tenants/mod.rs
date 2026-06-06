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
use crate::manager::tenants::repository::TenantsRepository;
use crate::manager::users::repository::UsersRepository as ManagerUserRepository;
use crate::tenant::users::repository::UsersRepository as TenantUserRepository;
use lettre::{
    AsyncTransport,
    transport::smtp::{Error, response::Response},
};
use std::fmt::Debug;

pub(crate) mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
mod service;
pub(crate) mod types;

pub trait TenantsModule:
    TenantsRepository
    + TenantUserRepository
    + ManagerUserRepository
    + DatabaseMigrator
    + PoolManager
    + BaseModule
{
}

impl<P, T> TenantsModule for AppState<P, T>
where
    P: DatabaseMigrator + PoolManager + Send + Sync + 'static,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync + Send + Sync + 'static,
    T::Error: Debug,
{
}

/*
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        common::config::{AppConfig, database_config::BasicDatabaseConfig},
        common::error::RepositoryError,
    };
    use async_trait::async_trait;
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
            fn config(&self) -> Arc<AppConfig>;
        }
        #[async_trait]
        impl MailTransporter for TenantsModule {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        #[async_trait]
        impl PoolManager for TenantsModule {
            fn get_main_pool(&self) -> PgPool;
            fn get_tenant_pool(&self, tenant_id: Uuid) -> Result<PgPool, RepositoryError>;
            async fn add_tenant_pool(
                &self,
                tenant_id: Uuid,
                config: &BasicDatabaseConfig,
            ) -> Result<Uuid, RepositoryError>;
            async fn delete_tenant_pool(
                &self,
                tenant_id: Uuid,
            ) -> Result<(), RepositoryError>;
        }
        impl TenantsModule for TenantsModule {
            fn tenants_repo(&self) -> Arc<dyn TenantsRepository>;
            fn tenant_user_repo(&self) -> Arc<dyn TenantUserRepository>;
            fn manager_user_repo(&self) -> Arc<dyn ManagerUserRepository>;
            fn migrator(&self) -> Arc<dyn DatabaseMigrator>;
        }
    );
}
*/
