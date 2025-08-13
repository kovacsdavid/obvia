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

use crate::app::config::AppConfig;
use crate::app::database::{DatabaseMigrator, PgPoolManagerTrait};
use crate::tenants::repository::TenantsRepository;
use std::sync::Arc;

pub(crate) mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
mod service;
pub(crate) mod types;

pub struct TenantsModule {
    pub pool_manager: Arc<dyn PgPoolManagerTrait>,
    pub config: Arc<AppConfig>,
    pub repo_factory: Box<dyn Fn() -> Box<dyn TenantsRepository + Send + Sync> + Send + Sync>,
    pub migrator_factory: Box<dyn Fn() -> Box<dyn DatabaseMigrator + Send + Sync> + Send + Sync>,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use crate::app::database::{MockDatabaseMigrator, MockPgPoolManagerTrait};
    use crate::tenants::repository::MockTenantsRepository;

    impl Default for TenantsModule {
        fn default() -> Self {
            TenantsModule {
                pool_manager: Arc::new(MockPgPoolManagerTrait::new()),
                config: Arc::new(AppConfig::default()),
                repo_factory: Box::new(|| Box::new(MockTenantsRepository::new())),
                migrator_factory: Box::new(|| Box::new(MockDatabaseMigrator::new())),
            }
        }
    }
}
