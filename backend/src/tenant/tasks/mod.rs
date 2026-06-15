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

use crate::common::database::PoolManager;
use crate::common::error::RepositoryResult;
use crate::common::{AppState, BaseModule};
use crate::tenant::currencies::repository::CurrenciesRepository;
use crate::tenant::services::repository::ServicesRepository;
use crate::tenant::tasks::repository::TasksRepository;
use crate::tenant::taxes::repository::TaxesRepository;
use crate::tenant::worksheets::repository::WorksheetsRepository;
use lettre::{
    AsyncTransport,
    transport::smtp::{Error, response::Response},
};
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub trait TasksModule: BaseModule {
    fn tasks_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn TasksRepository + Send + Sync>>;
    fn currencies_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn CurrenciesRepository + Send + Sync>>;
    fn taxes_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn TaxesRepository + Send + Sync>>;
    fn services_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn ServicesRepository + Send + Sync>>;
    fn worksheets_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn WorksheetsRepository + Send + Sync>>;
}

impl<P, T> TasksModule for AppState<P, T>
where
    P: PoolManager + Send + Sync + 'static,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync + 'static,
    T::Error: Debug,
{
    fn tasks_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn TasksRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn currencies_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn CurrenciesRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn taxes_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn TaxesRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn services_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn ServicesRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn worksheets_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn WorksheetsRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
}

/*
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::config::AppConfig;
    use async_trait::async_trait;
    use lettre::{
        Message,
        transport::smtp::{Error, response::Response},
    };
    use mockall::mock;

    mock!(
        pub TasksModule {}
        impl ConfigProvider for TasksModule {
            fn config(&self) -> Arc<AppConfig>;
        }
        #[async_trait]
        impl MailTransporter for TasksModule {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl TasksModule for TasksModule {
            fn tasks_repo(&self) -> Arc<dyn TasksRepository>;
            fn worksheets_repo(&self) -> Arc<dyn WorksheetsRepository>;
            fn services_repo(&self) -> Arc<dyn ServicesRepository>;
            fn taxes_repo(&self) -> Arc<dyn TaxesRepository>;
            fn currencies_repo(&self) -> Arc<dyn CurrenciesRepository>;
        }
    );
}
*/
