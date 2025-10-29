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
use crate::tenant::currencies::repository::CurrenciesRepository;
use crate::tenant::services::repository::ServicesRepository;
use crate::tenant::tasks::repository::TasksRepository;
use crate::tenant::taxes::repository::TaxesRepository;
use crate::tenant::worksheets::repository::WorksheetsRepository;
use std::sync::Arc;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub fn init_default_tasks_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> TasksModuleBuilder {
    TasksModuleBuilder::default()
        .config(config)
        .tasks_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .worksheets_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .services_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .taxes_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .currencies_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
}

pub struct TasksModule {
    pub config: Arc<AppConfig>,
    pub tasks_repo: Arc<dyn TasksRepository>,
    pub worksheets_repo: Arc<dyn WorksheetsRepository>,
    pub services_repo: Arc<dyn ServicesRepository>,
    pub taxes_repo: Arc<dyn TaxesRepository>,
    pub currencies_repo: Arc<dyn CurrenciesRepository>,
}

pub struct TasksModuleBuilder {
    pub config: Option<Arc<AppConfig>>,
    pub tasks_repo: Option<Arc<dyn TasksRepository>>,
    pub worksheets_repo: Option<Arc<dyn WorksheetsRepository>>,
    pub services_repo: Option<Arc<dyn ServicesRepository>>,
    pub taxes_repo: Option<Arc<dyn TaxesRepository>>,
    pub currencies_repo: Option<Arc<dyn CurrenciesRepository>>,
}

impl TasksModuleBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            tasks_repo: None,
            worksheets_repo: None,
            services_repo: None,
            taxes_repo: None,
            currencies_repo: None,
        }
    }
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    pub fn tasks_repo(mut self, tasks_repo: Arc<dyn TasksRepository>) -> Self {
        self.tasks_repo = Some(tasks_repo);
        self
    }
    pub fn worksheets_repo(mut self, worksheets_repo: Arc<dyn WorksheetsRepository>) -> Self {
        self.worksheets_repo = Some(worksheets_repo);
        self
    }
    pub fn services_repo(mut self, services_repo: Arc<dyn ServicesRepository>) -> Self {
        self.services_repo = Some(services_repo);
        self
    }
    pub fn taxes_repo(mut self, taxes_repo: Arc<dyn TaxesRepository>) -> Self {
        self.taxes_repo = Some(taxes_repo);
        self
    }
    pub fn currencies_repo(mut self, currencies_repo: Arc<dyn CurrenciesRepository>) -> Self {
        self.currencies_repo = Some(currencies_repo);
        self
    }
    pub fn build(self) -> Result<TasksModule, String> {
        Ok(TasksModule {
            config: self.config.ok_or("config is required".to_string())?,
            tasks_repo: self
                .tasks_repo
                .ok_or("tasks_repo is required".to_string())?,
            worksheets_repo: self
                .worksheets_repo
                .ok_or("worksheets_repo is required".to_string())?,
            services_repo: self
                .services_repo
                .ok_or("worksheets_repo is required".to_string())?,
            taxes_repo: self
                .taxes_repo
                .ok_or("taxes_repo is required".to_string())?,
            currencies_repo: self
                .currencies_repo
                .ok_or("currencies_repo is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for TasksModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for TasksModuleBuilder {
        fn default() -> Self {
            todo!()
        }
    }
}
