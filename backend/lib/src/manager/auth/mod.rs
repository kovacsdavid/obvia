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

use std::sync::Arc;

use crate::manager::app::config::AppConfig;
use crate::manager::app::database::PgPoolManagerTrait;
use crate::manager::auth::repository::AuthRepository;
use crate::manager::common::repository::PoolManagerWrapper;

pub(crate) mod dto;
mod handler;
pub(crate) mod middleware;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;

/// Initializes the default authentication module by setting up the necessary configurations,
/// database connection pool, and repository factory for handling authentication functionality.
///
/// # Arguments
///
/// * `pool_manager` - An `Arc` of a type that implements the `PgPoolManagerTrait`. This is used
///   to manage database connection pools for the authentication module.
/// * `config` - An `Arc` containing the shared application configuration (`AppConfig`) to be used
///   within the authentication module.
///
/// # Returns
///
/// Returns an `AuthModuleBuilder` configured with the provided database connection pool,
/// application configuration, and a repository factory to handle authentication repository operations.
pub fn init_default_auth_module(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> AuthModuleBuilder {
    AuthModuleBuilder::default()
        .pool_manager(pool_manager.clone())
        .config(config)
        .auth_repo(Arc::new(PoolManagerWrapper::new(pool_manager.clone())))
}

/// `AuthModule` is a structure that represents the authentication module in the application.
/// It encapsulates the required components for managing authentication operations.
///
/// # Fields
///
/// * `pool_manager` - An `Arc`-wrapped implementation of the `PgPoolManagerTrait` trait, providing the interface
///   for database connection pooling. This is used to manage database interactions securely and efficiently.
///
/// * `password_hasher` - An `Arc`-wrapped implementation of the `AuthPasswordHasher` trait, responsible for
///   hashing and verifying passwords. This ensures that user passwords are stored and processed securely.
///
/// * `config` - An `Arc`-wrapped instance of the `AppConfig` structure, which holds the application's configuration
///   setting necessary for configuring the authentication behavior.
///
/// # Notes
///
/// Using `Arc` for the fields allows safe sharing of these components across multiple threads.
pub struct AuthModule {
    pub pool_manager: Arc<dyn PgPoolManagerTrait>,
    pub config: Arc<AppConfig>,
    pub auth_repo: Arc<dyn AuthRepository + Send + Sync>,
}

/// A builder for constructing an instance of an authentication module with configurable dependencies.
///
/// `AuthModuleBuilder` provides a way to conveniently configure and initialize the dependencies
/// required for the authentication module, such as database connection pool management, application
/// configuration, and a repository factory for authentication-related data access.
///
/// # Fields
///
/// * `pool_manager` - An optional `Arc` containing a trait object that implements `PgPoolManagerTrait`.
///   This is used for managing the PostgreSQL database connection pool. It can handle creating and maintaining
///   connections to the database.
///
/// * `config` - An optional `Arc` containing the `AppConfig`, which stores application-specific configuration
///   values essential for running and configuring the authentication module.
///
/// * `repo_factory` - An optional boxed closure that returns a boxed trait object implementing `AuthRepository`.
///   This trait provides the necessary methods for interacting with authentication-related data storage
///   (e.g., user credentials, tokens, etc.). The closure is designed to create a factory for fetching repository
///   instances and supports `Send + Sync` for concurrent use.
pub struct AuthModuleBuilder {
    pool_manager: Option<Arc<dyn PgPoolManagerTrait>>,
    config: Option<Arc<AppConfig>>,
    auth_repo: Option<Arc<dyn AuthRepository + Send + Sync>>,
}

impl AuthModuleBuilder {
    /// Creates a new instance of the struct with default values.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with the following fields:
    /// - `pool_manager`: Set to `None`.
    /// - `config`: Set to `None`.
    /// - `repo_factory`: Set to `None`.
    pub fn new() -> Self {
        Self {
            pool_manager: None,
            config: None,
            auth_repo: None,
        }
    }
    /// Sets the pool manager for the current instance.
    ///
    /// This method allows you to specify a custom pool manager that conforms to the `PgPoolManagerTrait`.
    /// The provided pool manager will be stored in the current instance for further usage.
    ///
    /// # Arguments
    ///
    /// * `pool_manager` - An `Arc` containing a type that implements the `PgPoolManagerTrait`.
    ///
    /// # Returns
    ///
    /// Returns the updated instance of `Self` with the `pool_manager` set.
    pub fn pool_manager(mut self, pool_manager: Arc<dyn PgPoolManagerTrait>) -> Self {
        self.pool_manager = Some(pool_manager);
        self
    }
    /// Sets the configuration for the current instance.
    ///
    /// This method allows you to provide an `Arc<AppConfig>` object that will be set
    /// as the configuration for the current instance. The configuration is stored
    /// internally and can be used later by the instance for its various operations.
    ///
    /// # Parameters
    /// - `config`: An `Arc<AppConfig>` reference containing the configuration to be set.
    ///
    /// # Returns
    /// Returns the modified instance of `Self` with the configuration applied,
    /// enabling method chaining.
    pub fn config(mut self, config: Arc<AppConfig>) -> Self {
        self.config = Some(config);
        self
    }
    /// Sets the repository factory for the current object.
    ///
    /// This method allows you to specify a factory function that creates a
    /// new instance of a repository implementing the `AuthRepository` trait.
    /// The factory is wrapped in a `Box` to allow for dynamic dispatch.
    /// The `AuthRepository` implementation must be both `Send` and `Sync`
    /// to ensure thread safety.
    ///
    /// # Arguments
    ///
    /// * `repo_factory` - A boxed function that returns a boxed instance of a repository implementing the `AuthRepository` trait. This function must be `Send` and `Sync` compatible for safe execution across threads.
    ///
    /// # Returns
    ///
    /// Returns the modified instance of `Self` with the repository factory set.
    pub fn auth_repo(mut self, auth_repo: Arc<dyn AuthRepository + Send + Sync>) -> Self {
        self.auth_repo = Some(auth_repo);
        self
    }
    /// Builds an `AuthModule` instance using the provided configuration in the builder.
    ///
    /// This function ensures that all required fields (`pool_manager`, `config`, `repo_factory`) are set before
    /// constructing the `AuthModule`. If any of these fields are missing (`None`), the function will return
    /// an `Err` with a descriptive error message.
    ///
    /// # Returns
    /// - `Ok(AuthModule)` if all required fields are present.
    /// - `Err(String)` with a description of the missing field if any required field is `None`.
    ///
    /// # Errors
    /// - Returns an error if `pool_manager` is not set.
    /// - Returns an error if `config` is not set.
    /// - Returns an error if `repo_factory` is not set.
    pub fn build(self) -> Result<AuthModule, String> {
        Ok(AuthModule {
            pool_manager: self
                .pool_manager
                .ok_or("pool_manager is required".to_string())?,
            config: self.config.ok_or("pool_manager is required".to_string())?,
            auth_repo: self
                .auth_repo
                .ok_or("pool_manager is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for AuthModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::manager::app::config::AppConfigBuilder;
    use crate::manager::app::database::MockPgPoolManagerTrait;
    use crate::manager::auth::repository::MockAuthRepository;

    impl Default for AuthModuleBuilder {
        fn default() -> Self {
            AuthModuleBuilder {
                pool_manager: Some(Arc::new(MockPgPoolManagerTrait::new())),
                config: Some(Arc::new(AppConfigBuilder::default().build().unwrap())),
                auth_repo: Some(Arc::new(MockAuthRepository::new())),
            }
        }
    }
}
