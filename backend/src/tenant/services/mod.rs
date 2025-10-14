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
use crate::common::repository::PoolManagerWrapper;
use crate::manager::app::config::AppConfig;
use crate::manager::app::database::PgPoolManagerTrait;
use crate::tenant::currencies::repository::CurrencyRepository;
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

pub fn init_default_services_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> ServicesModuleBuilder {
    ServicesModuleBuilder::default()
        .config(config)
        .services_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .currencies_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .taxes_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
}

pub struct ServicesModule {
    pub config: Arc<AppConfig>,
    pub services_repo: Arc<dyn ServicesRepository>,
    pub currencies_repo: Arc<dyn CurrencyRepository>,
    pub taxes_repo: Arc<dyn TaxesRepository>,
}

pub struct ServicesModuleBuilder {
    pub config: Option<Arc<AppConfig>>,
    pub services_repo: Option<Arc<dyn ServicesRepository>>,
    pub currencies_repo: Option<Arc<dyn CurrencyRepository>>,
    pub taxes_repo: Option<Arc<dyn TaxesRepository>>,
}

impl ServicesModuleBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            services_repo: None,
            currencies_repo: None,
            taxes_repo: None,
        }
    }
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    pub fn services_repo(mut self, services_repo: Arc<dyn ServicesRepository>) -> Self {
        self.services_repo = Some(services_repo);
        self
    }
    pub fn currencies_repo(mut self, currencies_repo: Arc<dyn CurrencyRepository>) -> Self {
        self.currencies_repo = Some(currencies_repo);
        self
    }
    pub fn taxes_repo(mut self, taxes: Arc<dyn TaxesRepository>) -> Self {
        self.taxes_repo = Some(taxes);
        self
    }
    pub fn build(self) -> Result<ServicesModule, String> {
        Ok(ServicesModule {
            config: self.config.ok_or("config is required".to_string())?,
            services_repo: self
                .services_repo
                .ok_or("services_repo is required".to_string())?,
            currencies_repo: self
                .currencies_repo
                .ok_or("currencies_repo is required".to_string())?,
            taxes_repo: self
                .taxes_repo
                .ok_or("taxes_repo is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for ServicesModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for ServicesModuleBuilder {
        fn default() -> Self {
            todo!()
        }
    }
}
