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
use crate::tenant::currencies::repository::CurrenciesRepository;
use crate::tenant::services::repository::ServicesRepository;
use crate::tenant::taxes::repository::TaxesRepository;
use std::sync::Arc;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub trait ServicesModule: ConfigProvider + MailTransporter + Send + Sync {
    fn services_repo(&self) -> Arc<dyn ServicesRepository>;
    fn currencies_repo(&self) -> Arc<dyn CurrenciesRepository>;
    fn taxes_repo(&self) -> Arc<dyn TaxesRepository>;
}

impl ServicesModule for DefaultAppState {
    fn services_repo(&self) -> Arc<dyn ServicesRepository> {
        self.pool_manager.clone()
    }
    fn currencies_repo(&self) -> Arc<dyn CurrenciesRepository> {
        self.pool_manager.clone()
    }
    fn taxes_repo(&self) -> Arc<dyn TaxesRepository> {
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
        pub ServicesModule {}
        impl ConfigProvider for ServicesModule {
            fn config(&self) -> Arc<AppConfig>;
        }
        #[async_trait]
        impl MailTransporter for ServicesModule {
            async fn send(&self, message: Message) -> Result<Response, Error>;
        }
        impl ServicesModule for ServicesModule {
            fn services_repo(&self) -> Arc<dyn ServicesRepository>;
            fn currencies_repo(&self) -> Arc<dyn CurrenciesRepository>;
            fn taxes_repo(&self) -> Arc<dyn TaxesRepository>;
        }
    );
}
