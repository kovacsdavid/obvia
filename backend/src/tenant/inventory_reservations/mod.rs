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
use crate::tenant::inventory::repository::InventoryRepository;
use crate::tenant::inventory_reservations::repository::InventoryReservationsRepository;
use crate::tenant::worksheets::repository::WorksheetsRepository;
use lettre::{
    AsyncTransport,
    transport::smtp::{Error, response::Response},
};
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) mod dto;
pub(crate) mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub trait InventoryReservationsModuleInterface: BaseModule {
    fn inventory_reservations_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn InventoryReservationsRepository + Send + Sync>>;
    fn worksheets_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn WorksheetsRepository + Send + Sync>>;
    fn inventory_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn InventoryRepository + Send + Sync>>;
}

impl<P, T> InventoryReservationsModuleInterface for AppState<P, T>
where
    P: PoolManager + Send + Sync + 'static,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync + 'static,
    T::Error: Debug,
{
    fn inventory_reservations_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn InventoryReservationsRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn worksheets_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn WorksheetsRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn inventory_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn InventoryRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::config::AppConfig;
    use crate::common::error::RepositoryResult;
    use crate::common::{BaseModule, ConfigProvider, MailTransporter};
    use lettre::{
        Message,
        transport::smtp::{Error, response::Response},
    };
    use mockall::mock;

    mock!(
        pub InventoryReservationsModule {}
        impl ConfigProvider for InventoryReservationsModule {
            type Cfg = AppConfig;
            fn config(&self) -> &<Self as ConfigProvider>::Cfg;
        }
        impl MailTransporter for InventoryReservationsModule {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl BaseModule for InventoryReservationsModule {}
        impl InventoryReservationsModuleInterface for InventoryReservationsModule {
            fn inventory_reservations_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn InventoryReservationsRepository + Send + Sync>>;
            fn worksheets_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn WorksheetsRepository + Send + Sync>>;
            fn inventory_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn InventoryRepository + Send + Sync>>;
        }
    );
}
