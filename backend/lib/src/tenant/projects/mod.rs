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
use std::sync::Arc;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub fn init_default_projects_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> ProjectsModuleBuilder {
    ProjectsModuleBuilder::default()
        .config(config)
        .projects_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
}

pub struct ProjectsModule {
    pub config: Arc<AppConfig>,
    pub projects_repo: Arc<dyn ProjectsRepository>,
}

pub struct ProjectsModuleBuilder {
    pub config: Option<Arc<AppConfig>>,
    pub projects_repo: Option<Arc<dyn ProjectsRepository>>,
}

impl ProjectsModuleBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            projects_repo: None,
        }
    }
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    pub fn projects_repo(mut self, projects_repo: Arc<dyn ProjectsRepository>) -> Self {
        self.projects_repo = Some(projects_repo);
        self
    }
    pub fn build(self) -> Result<ProjectsModule, String> {
        Ok(ProjectsModule {
            config: self.config.ok_or("config is required".to_string())?,
            projects_repo: self
                .projects_repo
                .ok_or("projects_repo is required".to_string())?,
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
