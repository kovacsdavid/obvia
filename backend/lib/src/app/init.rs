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

use crate::app::app_state::{AppState, AppStateBuilder};
use crate::app::config::AppConfig;
use crate::app::database::{
    DatabaseMigrator, PgDatabaseMigrator, PgPoolManager, PgPoolManagerTrait,
};
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
pub use crate::app::services::{migrate_all_tenant_dbs, migrate_main_db};
use crate::common::repository::PoolWrapper;
use crate::organizational_units::repository::OrganizationalUnitsRepository;

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
/// * `config` - An `Arc` containing the `AppConfig`. The `AppConfig` holds the necessary database connection information for both the main and default tenant database.
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

/// Constructs an instance of `AppState`, which acts as the central state and dependency manager for the application.
///
/// This function is primarily responsible for setting up the various modules and dependencies required by the application,
/// such as authentication, organizational units, and user management modules. It uses the provided database pool manager
/// and application configuration to configure each module.
///
/// # Arguments
///
/// * `pool_manager` - An `Arc` reference to an object implementing the `PgPoolManagerTrait` trait. This is used to manage
///   database connection pools for the application.
/// * `config` - An `Arc` reference to an `AppConfig` object, which provides configuration values for the application.
///
/// # Returns
///
/// Returns a `Result<AppState, String>`:
/// - `Ok(AppState)` on successful initialization, containing the built application state with all modules set up.
/// - `Err(String)` if an error occurs during the initialization process, returning a descriptive error message.
/// # Errors
///
/// If any error occurs during the setup process, such as failing to initialize a module or resolve dependencies, the function
/// will return an error message wrapped in a `String`.
///
/// # Note
///
/// The function makes heavy use of closures to define factories for repositories and migrators, ensuring that the necessary
/// instances are created dynamically at runtime.
pub fn app_state(
    pool_manager: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) -> Result<AppState, String> {
    let pool_manager_clone = pool_manager.clone();
    AppStateBuilder::new()
        .users_module(Arc::new(UsersModule {}))
        .config_module(config.clone())
        .organizational_units_module(Arc::new(OrganizationalUnitsModule {
            pool_manager: pool_manager.clone(),
            config: config.clone(),
            repo_factory: Box::new(
                move || -> Box<dyn OrganizationalUnitsRepository + Send + Sync> {
                    Box::new(PoolWrapper::new(
                        pool_manager_clone.get_default_tenant_pool(),
                    ))
                },
            ),
            migrator_factory: Box::new(|| -> Box<dyn DatabaseMigrator + Send + Sync> {
                Box::new(PgDatabaseMigrator)
            }),
        }))
        .auth_module(Arc::new(auth::AuthModule {
            pool_manager: pool_manager.clone(),
            password_hasher: Arc::new(Argon2Hasher),
            config: config.clone(),
        }))
        .build()
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
