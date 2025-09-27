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
use crate::tenant::warehouses::repository::WarehousesRepository;
use std::sync::Arc;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub fn init_default_warehouses_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> WarehousesModuleBuilder {
    WarehousesModuleBuilder::default()
        .config(config)
        .warehouses_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
}

pub struct WarehousesModule {
    pub config: Arc<AppConfig>,
    pub warehouses_repo: Arc<dyn WarehousesRepository>,
}

pub struct WarehousesModuleBuilder {
    pub config: Option<Arc<AppConfig>>,
    pub warehouses_repo: Option<Arc<dyn WarehousesRepository>>,
}

impl WarehousesModuleBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            warehouses_repo: None,
        }
    }
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    pub fn warehouses_repo(mut self, warehouses_repo: Arc<dyn WarehousesRepository>) -> Self {
        self.warehouses_repo = Some(warehouses_repo);
        self
    }
    pub fn build(self) -> Result<WarehousesModule, String> {
        Ok(WarehousesModule {
            config: self.config.ok_or("config is required".to_string())?,
            warehouses_repo: self
                .warehouses_repo
                .ok_or("warehouses_repo is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for WarehousesModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for WarehousesModuleBuilder {
        fn default() -> Self {
            todo!()
        }
    }
}
