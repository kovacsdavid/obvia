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

use crate::common::{ConfigProvider, DefaultAppState, MailTransporter};
use crate::manager::app::database::{ConnectionTester, DatabaseMigrator, PoolManager};
use crate::manager::tenants::repository::TenantsRepository;
use crate::manager::users::repository::UsersRepository as ManagerUserRepository;
use crate::tenant::users::repository::UsersRepository as TenantUserRepository;
use std::sync::Arc;

pub(crate) mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
mod service;
pub(crate) mod types;

pub trait TenantsModule: PoolManager + ConfigProvider + MailTransporter + Send + Sync {
    fn tenants_repo(&self) -> Arc<dyn TenantsRepository>;
    fn tenant_user_repo(&self) -> Arc<dyn TenantUserRepository>;
    fn manager_user_repo(&self) -> Arc<dyn ManagerUserRepository>;
    fn migrator(&self) -> Arc<dyn DatabaseMigrator>;
    fn connection_tester(&self) -> Arc<dyn ConnectionTester>;
}

impl TenantsModule for DefaultAppState {
    fn tenants_repo(&self) -> Arc<dyn TenantsRepository> {
        self.pool_manager.clone()
    }
    fn tenant_user_repo(&self) -> Arc<dyn TenantUserRepository> {
        self.pool_manager.clone()
    }
    fn manager_user_repo(&self) -> Arc<dyn ManagerUserRepository> {
        self.pool_manager.clone()
    }
    fn migrator(&self) -> Arc<dyn DatabaseMigrator> {
        self.migrator.clone()
    }
    fn connection_tester(&self) -> Arc<dyn ConnectionTester> {
        self.connection_tester.clone()
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        common::error::RepositoryError,
        manager::app::config::{AppConfig, BasicDatabaseConfig},
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
            async fn send(&self, message: Message) -> Result<Response, Error>;
        }
        #[async_trait]
        impl PoolManager for TenantsModule {
            fn get_main_pool(&self) -> PgPool;
            fn get_default_tenant_pool(&self) -> PgPool;
            fn get_tenant_pool(&self, tenant_id: Uuid) -> Result<PgPool, RepositoryError>;
            async fn add_tenant_pool(
                &self,
                tenant_id: Uuid,
                config: &BasicDatabaseConfig,
            ) -> Result<Uuid, RepositoryError>;
        }
        impl TenantsModule for TenantsModule {
            fn tenants_repo(&self) -> Arc<dyn TenantsRepository>;
            fn tenant_user_repo(&self) -> Arc<dyn TenantUserRepository>;
            fn manager_user_repo(&self) -> Arc<dyn ManagerUserRepository>;
            fn migrator(&self) -> Arc<dyn DatabaseMigrator>;
            fn connection_tester(&self) -> Arc<dyn ConnectionTester>;
        }
    );
}
