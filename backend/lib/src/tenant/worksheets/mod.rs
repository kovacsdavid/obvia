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
use crate::manager::common::repository::PoolManagerWrapper;
use crate::tenant::projects::repository::ProjectsRepository;
use crate::tenant::worksheets::repository::WorksheetsRepository;
use std::sync::Arc;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub fn init_default_worksheets_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> WorksheetsModuleBuilder {
    WorksheetsModuleBuilder::default()
        .config(config)
        .worksheets_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .projects_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
}

pub struct WorksheetsModule {
    pub config: Arc<AppConfig>,
    pub worksheets_repo: Arc<dyn WorksheetsRepository>,
    pub projects_repo: Arc<dyn ProjectsRepository>,
}

pub struct WorksheetsModuleBuilder {
    pub config: Option<Arc<AppConfig>>,
    pub worksheets_repo: Option<Arc<dyn WorksheetsRepository>>,
    pub projects_repo: Option<Arc<dyn ProjectsRepository>>,
}

impl WorksheetsModuleBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            worksheets_repo: None,
            projects_repo: None,
        }
    }
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    pub fn worksheets_repo(mut self, worksheets_repo: Arc<dyn WorksheetsRepository>) -> Self {
        self.worksheets_repo = Some(worksheets_repo);
        self
    }
    pub fn projects_repo(mut self, projects_repo: Arc<dyn ProjectsRepository>) -> Self {
        self.projects_repo = Some(projects_repo);
        self
    }
    pub fn build(self) -> Result<WorksheetsModule, String> {
        Ok(WorksheetsModule {
            config: self.config.ok_or("config is required".to_string())?,
            worksheets_repo: self
                .worksheets_repo
                .ok_or("worksheets_repo is required".to_string())?,
            projects_repo: self
                .projects_repo
                .ok_or("projects_repo is required".to_string())?,
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
