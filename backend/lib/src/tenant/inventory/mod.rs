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
use crate::tenant::inventory::repository::InventoryRepository;
use crate::tenant::products::repository::ProductsRepository;
use crate::tenant::warehouses::repository::WarehousesRepository;
use std::sync::Arc;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub fn init_default_inventory_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> InventoryModuleBuilder {
    InventoryModuleBuilder::default()
        .config(config)
        .inventory_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .products_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .warehouses_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
}

pub struct InventoryModule {
    pub config: Arc<AppConfig>,
    pub inventory_repo: Arc<dyn InventoryRepository>,
    pub products_repo: Arc<dyn ProductsRepository>,
    pub warehouses_repo: Arc<dyn WarehousesRepository>,
}

pub struct InventoryModuleBuilder {
    pub config: Option<Arc<AppConfig>>,
    pub inventory_repo: Option<Arc<dyn InventoryRepository>>,
    pub products_repo: Option<Arc<dyn ProductsRepository>>,
    pub warehouses_repo: Option<Arc<dyn WarehousesRepository>>,
}

impl InventoryModuleBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            inventory_repo: None,
            products_repo: None,
            warehouses_repo: None,
        }
    }
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    pub fn inventory_repo(mut self, inventory_repo: Arc<dyn InventoryRepository>) -> Self {
        self.inventory_repo = Some(inventory_repo);
        self
    }
    pub fn products_repo(mut self, products_repo: Arc<dyn ProductsRepository>) -> Self {
        self.products_repo = Some(products_repo);
        self
    }
    pub fn warehouses_repo(mut self, warehouses_repo: Arc<dyn WarehousesRepository>) -> Self {
        self.warehouses_repo = Some(warehouses_repo);
        self
    }
    pub fn build(self) -> Result<InventoryModule, String> {
        Ok(InventoryModule {
            config: self.config.ok_or("config is required".to_string())?,
            inventory_repo: self
                .inventory_repo
                .ok_or("inventory_repo  is required".to_string())?,
            products_repo: self
                .products_repo
                .ok_or("products_repo  is required".to_string())?,
            warehouses_repo: self
                .warehouses_repo
                .ok_or("warehouses_repo  is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for InventoryModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for InventoryModuleBuilder {
        fn default() -> Self {
            todo!()
        }
    }
}
