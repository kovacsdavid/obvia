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
use crate::tenant::inventory_reservations::repository::InventoryReservationsRepository;
use crate::tenant::worksheets::repository::WorksheetsRepository;
use std::sync::Arc;

pub(crate) mod dto;
pub(crate) mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub fn init_default_inventory_reservations_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> InventoryReservationsModuleBuilder {
    InventoryReservationsModuleBuilder::default()
        .config(config)
        .inventory_reservations_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .worksheets_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .inventory_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
}

pub struct InventoryReservationsModule {
    pub config: Arc<AppConfig>,
    pub inventory_reservations_repo: Arc<dyn InventoryReservationsRepository>,
    pub worksheets_repo: Arc<dyn WorksheetsRepository>,
    pub inventory_repo: Arc<dyn InventoryRepository>,
}

pub struct InventoryReservationsModuleBuilder {
    pub config: Option<Arc<AppConfig>>,
    pub inventory_reservations_repo: Option<Arc<dyn InventoryReservationsRepository>>,
    pub worksheets_repo: Option<Arc<dyn WorksheetsRepository>>,
    pub inventory_repo: Option<Arc<dyn InventoryRepository>>,
}

impl InventoryReservationsModuleBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            inventory_reservations_repo: None,
            worksheets_repo: None,
            inventory_repo: None,
        }
    }
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    pub fn inventory_reservations_repo(
        mut self,
        inventory_reservations_repo: Arc<dyn InventoryReservationsRepository>,
    ) -> Self {
        self.inventory_reservations_repo = Some(inventory_reservations_repo);
        self
    }
    pub fn worksheets_repo(mut self, worksheets_repo: Arc<dyn WorksheetsRepository>) -> Self {
        self.worksheets_repo = Some(worksheets_repo);
        self
    }
    pub fn inventory_repo(mut self, inventory_repo: Arc<dyn InventoryRepository>) -> Self {
        self.inventory_repo = Some(inventory_repo);
        self
    }
    pub fn build(self) -> Result<InventoryReservationsModule, String> {
        Ok(InventoryReservationsModule {
            config: self.config.ok_or("config is required".to_string())?,
            inventory_reservations_repo: self
                .inventory_reservations_repo
                .ok_or("inventory_reservations_repo is required".to_string())?,
            worksheets_repo: self
                .worksheets_repo
                .ok_or("worksheets_repo is required".to_string())?,
            inventory_repo: self
                .inventory_repo
                .ok_or("inventory_repo is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for InventoryReservationsModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for InventoryReservationsModuleBuilder {
        fn default() -> Self {
            todo!()
        }
    }
}
