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
use crate::tenant::customers::repository::CustomersRespository;
use std::sync::Arc;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub fn init_default_customers_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> CustomersModuleBuilder {
    CustomersModuleBuilder::default()
        .config(config)
        .customers_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
}

pub struct CustomersModule {
    pub config: Arc<AppConfig>,
    pub customers_repo: Arc<dyn CustomersRespository>,
}

pub struct CustomersModuleBuilder {
    pub config: Option<Arc<AppConfig>>,
    pub customers_repo: Option<Arc<dyn CustomersRespository>>,
}

impl CustomersModuleBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            customers_repo: None,
        }
    }
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    pub fn customers_repo(mut self, customers_repo: Arc<dyn CustomersRespository>) -> Self {
        self.customers_repo = Some(customers_repo);
        self
    }
    pub fn build(self) -> Result<CustomersModule, String> {
        Ok(CustomersModule {
            config: self.config.ok_or("config is required".to_string())?,
            customers_repo: self
                .customers_repo
                .ok_or("customers_repo is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for CustomersModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for CustomersModuleBuilder {
        fn default() -> Self {
            todo!()
        }
    }
}
