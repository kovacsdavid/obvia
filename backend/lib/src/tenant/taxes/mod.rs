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
use crate::tenant::taxes::repository::TaxesRepository;
use std::sync::Arc;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub fn init_default_taxes_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> TaxesModuleBuilder {
    TaxesModuleBuilder::default()
        .config(config)
        .taxes_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
}

pub struct TaxesModule {
    pub config: Arc<AppConfig>,
    pub taxes_repo: Arc<dyn TaxesRepository>,
}

pub struct TaxesModuleBuilder {
    pub config: Option<Arc<AppConfig>>,
    pub taxes_repo: Option<Arc<dyn TaxesRepository>>,
}

impl TaxesModuleBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            taxes_repo: None,
        }
    }
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    pub fn taxes_repo(mut self, taxes_repo: Arc<dyn TaxesRepository>) -> Self {
        self.taxes_repo = Some(taxes_repo);
        self
    }
    pub fn build(self) -> Result<TaxesModule, String> {
        Ok(TaxesModule {
            config: self.config.ok_or("config is required".to_string())?,
            taxes_repo: self
                .taxes_repo
                .ok_or("taxes_repo is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for TaxesModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for TaxesModuleBuilder {
        fn default() -> Self {
            todo!()
        }
    }
}
