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
use crate::tenant::customers::repository::CustomersRepository;
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

pub trait WorksheetsModule: BaseModule {
    fn worksheets_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn WorksheetsRepository + Send + Sync>>;
    fn customers_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn CustomersRepository + Send + Sync>>;
}

impl<P, T> WorksheetsModule for AppState<P, T>
where
    P: PoolManager + Send + Sync,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync,
    T::Error: Debug,
{
    fn worksheets_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn WorksheetsRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn customers_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn CustomersRepository + Send + Sync>> {
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
        pub WorksheetsModule {}
        impl ConfigProvider for WorksheetsModule {
            fn config(&self) -> Arc<AppConfig>;
        }
        #[async_trait]
        impl MailTransporter for WorksheetsModule {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl WorksheetsModule for WorksheetsModule {
            fn worksheets_repo(&self) -> Arc<dyn WorksheetsRepository>;
            fn customers_repo(&self) -> Arc<dyn CustomersRepository>;
        }
    );
}
*/
