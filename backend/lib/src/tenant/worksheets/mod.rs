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
use crate::manager::app::config::AppConfig;
use crate::manager::app::database::PgPoolManagerTrait;
use std::sync::Arc;

mod dto;
mod handler;
pub(crate) mod model;
mod routes;
pub(crate) mod types;

pub fn init_default_worksheets_module(
    pool_manager_config: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) {
    todo!()
}

pub struct WorksheetsModule {
    pub config: Arc<AppConfig>,
}

pub struct WorksheetsModuleBuilder {
    pub config: Option<Arc<AppConfig>>,
}

impl WorksheetsModuleBuilder {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn build(self) -> Result<WorksheetsModule, String> {
        Ok(WorksheetsModule {
            config: self.config.ok_or("config is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for WorksheetsModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for WorksheetsModuleBuilder {
        fn default() -> Self {
            todo!()
        }
    }
}
