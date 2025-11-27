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
use crate::tenant::warehouses::repository::WarehousesRepository;
use std::sync::Arc;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub trait WarehousesModule: ConfigProvider + MailTransporter + Send + Sync {
    fn warehouses_repo(&self) -> Arc<dyn WarehousesRepository>;
}

impl WarehousesModule for DefaultAppState {
    fn warehouses_repo(&self) -> Arc<dyn WarehousesRepository> {
        self.pool_manager.clone()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::manager::app::config::AppConfig;
    use async_trait::async_trait;
    use lettre::{
        Message,
        transport::smtp::{Error, response::Response},
    };
    use mockall::mock;

    mock!(
        pub WarehousesModule {}
        impl ConfigProvider for WarehousesModule {
            fn config(&self) -> Arc<AppConfig>;
        }
        #[async_trait]
        impl MailTransporter for WarehousesModule {
            async fn send(&self, message: Message) -> Result<Response, Error>;
        }
        impl WarehousesModule for WarehousesModule {
            fn warehouses_repo(&self) -> Arc<dyn WarehousesRepository>;
        }
    );
}
