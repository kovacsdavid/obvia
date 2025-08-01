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

use crate::app::app_state::AppState;
use crate::app::config::AppConfig;
use crate::app::database::PgPoolManager;
use crate::app::services::{migrate_all_tenant_dbs, migrate_main_db};
use crate::auth;
use crate::auth::service::Argon2Hasher;
use crate::organizational_units::{self, OrganizationalUnitsModule};
use crate::users::UsersModule;
use anyhow::Result;
use axum::Router;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub use crate::app::services::init_tenant_pools;

/// Sets up a global tracing subscriber for the application with a specified logging level.
///
/// This function initializes a tracing subscriber using the `tracing` crate.
/// It configures the subscriber with the maximum log level of `TRACE`, meaning
/// that all log messages at `TRACE` level and above will be captured.
/// The subscriber is then set as the global default.
///
/// If the global subscriber cannot be set due to an error (e.g., trying to set
/// a subscriber when one is already set), the function will panic with a
/// descriptive error message.
///
/// # Panics
/// - If the subscriber is unable to be set as the global default (e.g., when a
///   global subscriber is already set).
///
/// # Dependencies
/// This function requires the `tracing` crate with its `FmtSubscriber`
/// component and the `Level` type.
///
/// # Notes
/// - Call this function early in program initialization, before any
///   tracing events are emitted.
/// - Only one global subscriber can be set. Subsequent calls to this function
///   without resetting the default subscriber will panic.
pub fn subscriber() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE) //TODO: make configurable
            .finish(),
    )
    .expect("setting default subscriber failed");
}

/// Retrieves the application configuration from the environment.
///
/// This function reads the configuration from environment variables and wraps it
/// in a thread-safe `Arc` pointer for shared access throughout the application.
/// The configuration is automatically loaded using the `AppConfig::from_env()`
/// method, which constructs the configuration instance based on the current
/// environment.
///
/// # Returns
///
/// * `Ok(Arc<AppConfig>)` - If the configuration is successfully loaded.
/// * `Err` - If there is an error while initializing the configuration (e.g., missing or invalid configuration file).
pub fn config() -> Result<Arc<AppConfig>> {
    Ok(Arc::new(AppConfig::from_env()?))
}

/// Asynchronously initializes and returns a new `PgPoolManager` instance wrapped in an `Arc`.
///
/// This function creates a `PgPoolManager` using the main database configuration and
/// the default tenant database configuration provided in the `AppConfig`. It handles
/// the asynchronous setup of the pool manager and ensures the returned instance is
/// thread-safe by wrapping it in an `Arc`.
///
/// # Arguments
///
/// * `config` - An `Arc` containing the `AppConfig`. The `AppConfig` holds the necessary
///              database connection information for both the main and default tenant database.
///
/// # Returns
///
/// Returns a `Result` containing an `Arc<PgPoolManager>` on success, or an error if
/// the creation of the `PgPoolManager` fails.
///
/// # Errors
///
/// This function will return an error if the `PgPoolManager::new` method fails to initialize
/// due to invalid configuration or database connection issues.
pub async fn pg_pool_manager(config: Arc<AppConfig>) -> Result<Arc<PgPoolManager>> {
    Ok(Arc::new(
        PgPoolManager::new(config.main_database(), config.default_tenant_database()).await?,
    ))
}

/// Performs database migration for the main database and all tenant databases.
///
/// This function is responsible for orchestrating the migration process for both the main
/// database and all tenant-specific databases.
///
/// # Arguments
///
/// * `pool_manager` - A reference-counted `PgPoolManager` that manages database connection
///   pools, ensuring thread-safe and efficient handling of database operations.
///
/// # Returns
///
/// * `Ok(())` if the migration process for both the main database and tenant databases
///   completes successfully.
/// * `Err` if an error occurs during the migration process for either the main database
///   or any tenant database.
///
/// # Errors
///
/// This function returns an error if:
/// * An issue occurs while migrating the main database.
/// * An issue occurs while migrating any of the tenant databases.
///
/// The returned error can be used to diagnose and address specific issues encountered during
/// the migration process.
pub async fn migrate(pool_manager: Arc<PgPoolManager>) -> Result<()> {
    migrate_main_db(pool_manager.clone()).await?;
    migrate_all_tenant_dbs(pool_manager.clone()).await?;
    Ok(())
}

/// Initializes and constructs the application state.
///
/// This function sets up the state for the application by creating and configuring
/// various modules such as authentication, users, and organizational units. It takes
/// in references to shared resources such as a database connection pool manager and
/// application configuration, and then constructs the `AppState` structure.
///
/// # Parameters
/// - `pool_manager`: An `Arc`-wrapped `PgPoolManager` that provides access to the
///   database connection pool for the application.
/// - `config`: An `Arc`-wrapped `AppConfig` object containing application configuration
///   values such as environment settings, application secrets, or other necessary config
///   options.
///
/// # Returns
/// An instance of `AppState`, which contains references to the initialized modules required
/// for the application's functionality:
/// - `auth_module`: The authentication module, initialized with dependencies for managing
///   user authentication, password hashing, and configuration.
/// - `config_module`: A shared reference to the application configuration.
/// - `users_module`: The users module, currently represented with no additional setup.
/// - `organizational_units_module`: The module for managing organizational units,
///   initialized with dependencies like the database connection pool and configuration.
///
/// # Notes
/// - The use of `Arc` ensures that the state and shared resources can be safely cloned
///   and accessed concurrently across multiple asynchronous tasks.
///
/// # Errors
/// Currently, this function does not return any errors. However, if additional
/// initialization logic is added in the future that could fail (e.g., loading
/// configurations or establishing database connections), error handling may need
/// to be included.
pub fn app_state(pool_manager: Arc<PgPoolManager>, config: Arc<AppConfig>) -> AppState {
    let auth_module = auth::AuthModule {
        pool_manager: pool_manager.clone(),
        password_hasher: Arc::new(Argon2Hasher),
        config: config.clone(),
    };
    let users_module = Arc::new(UsersModule {});
    let organizational_units_module = Arc::new(OrganizationalUnitsModule {
        pool_manager: pool_manager.clone(),
        config: config.clone(),
    });

    AppState {
        auth_module: Arc::new(auth_module),
        config_module: config.clone(),
        users_module,
        organizational_units_module,
    }
}

/// Sets up and returns the main application router.
///
/// This function creates a new `Router` instance, merges route definitions from
/// multiple modules, and adds middleware layers (e.g., tracing) to handle HTTP requests.
///
/// # Arguments
///
/// * `app_state` - An [`Arc<AppState>`](std::sync::Arc), which is the shared application
///   state passed to the routes. It ensures the state can be safely shared and accessed
///   across multiple asynchronous tasks.
///
/// # Returns
///
/// A configured `Router` instance that routes incoming HTTP requests to the appropriate
/// handlers based on the defined routes.
///
/// # Middleware
///
/// - `TraceLayer`: Adds tracing for incoming HTTP requests and outgoing responses.
pub async fn app(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(auth::routes::routes(app_state.clone()))
        .merge(organizational_units::routes::routes(app_state.clone()))
        .layer(TraceLayer::new_for_http())
}
