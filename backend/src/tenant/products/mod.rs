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
use crate::tenant::products::repository::ProductsRepository;
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

pub trait ProductsModule: BaseModule {
    fn products_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn ProductsRepository + Send + Sync>>;
}

impl<P, T> ProductsModule for AppState<P, T>
where
    P: PoolManager + Send + Sync + 'static,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync + 'static,
    T::Error: Debug,
{
    fn products_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn ProductsRepository + Send + Sync>> {
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
        pub ProductsModule {}
        impl ConfigProvider for ProductsModule {
            fn config(&self) -> Arc<AppConfig>;
        }
        #[async_trait]
        impl MailTransporter for ProductsModule {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl ProductsModule for ProductsModule {
            fn products_repo(&self) -> Arc<dyn ProductsRepository>;
        }
    );
}
*/
