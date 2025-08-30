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
pub(crate) mod routes;
pub(crate) mod types;

pub fn init_default_projects_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> ProjectsModuleBuilder {
    ProjectsModuleBuilder::default()
        .pool_manager(pool_manager)
        .config(config)
}

pub struct ProjectsModule {
    pub pool_manager: Arc<dyn PgPoolManagerTrait>,
    pub config: Arc<AppConfig>,
}

pub struct ProjectsModuleBuilder {
    pub pool_manager: Option<Arc<dyn PgPoolManagerTrait>>,
    pub config: Option<Arc<AppConfig>>,
}

impl ProjectsModuleBuilder {
    pub fn new() -> Self {
        Self {
            pool_manager: None,
            config: None,
        }
    }
    pub fn pool_manager(mut self, pool_manager: Arc<dyn PgPoolManagerTrait>) -> Self {
        self.pool_manager = Some(pool_manager);
        self
    }
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    pub fn build(self) -> Result<ProjectsModule, String> {
        Ok(ProjectsModule {
            pool_manager: self
                .pool_manager
                .ok_or("pool_manager is required".to_string())?,
            config: self.config.ok_or("config is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for ProjectsModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for ProjectsModuleBuilder {
        fn default() -> Self {
            todo!()
        }
    }
}
