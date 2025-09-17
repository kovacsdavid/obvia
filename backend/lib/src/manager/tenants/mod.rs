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
use crate::manager::app::database::{
    ConnectionTester, DatabaseMigrator, PgConnectionTester, PgDatabaseMigrator, PgPoolManagerTrait,
};
use crate::manager::common::repository::PoolManagerWrapper;
use crate::manager::tenants::repository::TenantsRepository;
use std::sync::Arc;

pub(crate) mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
mod service;
pub(crate) mod types;

/// Initializes the default tenants module with the given database pool manager and application configuration.
///
/// This function sets up a `TenantsModuleBuilder` with the necessary components to manage the default tenants in the system,
/// including repository, database migrator, and connection tester factories.
///
/// # Arguments
///
/// * `pool_manager` - An `Arc` of a trait object implementing `PgPoolManagerTrait`, responsible for managing database connection pools.
/// * `config` - An `Arc` of `AppConfig` containing the application configuration.
///
/// # Returns
///
/// A `TenantsModuleBuilder` pre-configured with:
/// - The provided `pool_manager` for managing database connections.
/// - The provided `config` for application-specific settings.
/// - A repository factory that creates instances of `PoolWrapper` for interacting with the default tenant pool.
/// - A migrator factory that generates instances of `PgDatabaseMigrator` for database migrations.
/// - A connection tester factory that generates instances of `PgConnectionTester` for testing database connections.
pub fn init_default_tenants_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> TenantsModuleBuilder {
    TenantsModuleBuilder::default()
        .pool_manager(pool_manager.clone())
        .config(config)
        .tenants_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
        .migrator(Arc::new(PgDatabaseMigrator))
        .connection_tester(Arc::new(PgConnectionTester))
}

pub struct TenantsModule {
    pub pool_manager: Arc<dyn PgPoolManagerTrait>,
    pub config: Arc<AppConfig>,
    pub tenants_repo: Arc<dyn TenantsRepository>,
    pub migrator: Arc<dyn DatabaseMigrator>,
    pub connection_tester: Arc<dyn ConnectionTester>,
}

/// A builder struct for initializing a `TenantsModule`. This struct provides a configurable way to set up the
/// dependencies required by the `TenantsModule`, including database connection management, configuration,
/// repositories, and database migration utilities. It utilizes an optional pattern to allow customization.
pub struct TenantsModuleBuilder {
    pub pool_manager: Option<Arc<dyn PgPoolManagerTrait>>,
    pub config: Option<Arc<AppConfig>>,
    pub tenants_repo: Option<Arc<dyn TenantsRepository>>,
    pub migrator: Option<Arc<dyn DatabaseMigrator>>,
    pub connection_tester: Option<Arc<dyn ConnectionTester>>,
}

impl TenantsModuleBuilder {
    /// Creates a new instance of the struct with default values.
    ///
    /// Initializes all fields of the struct to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `Self` with all optional fields uninitialized.
    pub fn new() -> Self {
        Self {
            pool_manager: None,
            config: None,
            tenants_repo: None,
            migrator: None,
            connection_tester: None,
        }
    }
    /// Sets the database pool manager for the current instance.
    ///
    /// This function allows you to specify a custom implementation of `PgPoolManagerTrait`
    /// to manage the database connection pool. The provided pool manager will be stored
    /// in the current instance for later use.
    ///
    /// # Arguments
    ///
    /// * `pool_manager` - An `Arc` pointer to a type implementing the `PgPoolManagerTrait`. This is used to manage database connections.
    ///
    /// # Returns
    ///
    /// Returns the modified instance of `Self` with the provided pool manager set.
    pub fn pool_manager(mut self, pool_manager: Arc<dyn PgPoolManagerTrait>) -> Self {
        self.pool_manager = Some(pool_manager);
        self
    }
    /// Sets the configuration for the application.
    ///
    /// This method allows you to provide a shared application configuration
    /// (`AppConfig`) wrapped in an `Arc`. It sets the configuration for the
    /// current instance and returns the updated instance.
    ///
    /// # Arguments
    ///
    /// * `config` - A shared reference-counted pointer to an `AppConfig` structure.
    ///
    /// # Returns
    ///
    /// * `Self` - The updated instance of the object with the configuration applied.
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    /// Sets the repository factory for the current instance.
    ///
    /// This function allows the user to provide a factory function that produces
    /// a boxed instance implementing the `TenantsRepository` trait. The factory
    /// is expected to be thread-safe (`Send + Sync`). The provided factory will
    /// be stored internally and used to create new repository instances when needed.
    ///
    /// # Parameters
    /// - `repo_factory`: A boxed closure that returns a boxed instance of
    ///   `TenantsRepository`, which must be thread-safe (`Send + Sync`).
    ///
    /// # Returns
    /// - `Self`: Returns the modified instance of `Self` with the provided repository
    ///   factory stored.
    pub fn tenants_repo(mut self, tenants_repo: Arc<dyn TenantsRepository>) -> Self {
        self.tenants_repo = Some(tenants_repo);
        self
    }
    /// Sets a custom migrator factory for the database migration process.
    ///
    /// This method allows you to provide a factory function that creates an instance of a type
    /// implementing the `DatabaseMigrator` trait. The factory is stored internally and will
    /// be used when migrations are executed. The provided factory function needs to be thread-safe
    /// (`Send + Sync`) as it might be used in a multithreaded context.
    ///
    /// # Returns
    ///
    /// Returns `Self`, allowing for method chaining.
    pub fn migrator(mut self, migrator_factory: Arc<dyn DatabaseMigrator>) -> Self {
        self.migrator = Some(migrator_factory);
        self
    }
    /// Sets a custom connection tester for the object.
    ///
    /// This method allows the caller to provide a custom implementation of a connection tester
    /// function. The provided function should return a boxed object implementing the `ConnectionTester`
    /// trait. The function itself must be thread-safe (`Send + Sync`), as well as the returned
    /// connection tester instance.
    ///
    /// # Arguments
    ///
    /// * `connection_tester_factory` - A boxed function that returns a boxed object implementing the
    ///   `ConnectionTester` trait. The function and returned object must both be safe to use across threads.
    ///
    /// # Returns
    ///
    /// Returns the instance of `Self` with the `connection_tester` field updated to the provided
    /// function, allowing for method chaining.
    pub fn connection_tester(mut self, connection_tester: Arc<dyn ConnectionTester>) -> Self {
        self.connection_tester = Some(connection_tester);
        self
    }
    /// Builds and returns an instance of `TenantsModule`.
    ///
    /// This method constructs a `TenantsModule` by consuming the builder.
    /// It ensures that all required fields are present and properly initialized.
    /// If any of the mandatory fields are missing (`pool_manager`, `config`, `repo_factory`,
    /// `migrator_factory`, or `connection_tester`), it will return an error containing a descriptive
    /// message indicating the missing field.
    ///
    /// # Returns
    ///
    /// - `Ok(TenantsModule)` if all required fields are present.
    /// - `Err(String)` if any required field is missing, with a message specifying the missing field(s).
    pub fn build(self) -> Result<TenantsModule, String> {
        Ok(TenantsModule {
            pool_manager: self
                .pool_manager
                .ok_or("pool_manager is required".to_string())?,
            config: self.config.ok_or("config is required".to_string())?,
            tenants_repo: self
                .tenants_repo
                .ok_or("repo_factory is required".to_string())?,
            migrator: self
                .migrator
                .ok_or("migrator_factory is required".to_string())?,
            connection_tester: self
                .connection_tester
                .ok_or("connection_tester is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for TenantsModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::manager::app::config::AppConfigBuilder;

    use crate::manager::app::database::{
        MockConnectionTester, MockDatabaseMigrator, MockPgPoolManagerTrait,
    };
    use crate::manager::tenants::repository::MockTenantsRepository;

    impl Default for TenantsModuleBuilder {
        fn default() -> Self {
            TenantsModuleBuilder {
                pool_manager: Some(Arc::new(MockPgPoolManagerTrait::new())),
                config: Some(Arc::new(AppConfigBuilder::default().build().unwrap())),
                tenants_repo: Some(Arc::new(MockTenantsRepository::new())),
                migrator: Some(Arc::new(MockDatabaseMigrator::new())),
                connection_tester: Some(Arc::new(MockConnectionTester::new())),
            }
        }
    }
}
