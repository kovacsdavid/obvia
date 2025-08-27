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
use crate::manager::app::database::PgPoolManager;
use crate::manager::tenants::{self, init_default_tenants_module};
use anyhow::{Result, anyhow};
use axum::Router;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub use crate::manager::app::services::init_tenant_pools;
pub use crate::manager::app::services::{migrate_all_tenant_dbs, migrate_main_db};
use crate::manager::auth::{self, init_default_auth_module};

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
pub fn config() -> Result<AppConfig> {
    Ok(AppConfig::from_env()?)
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
pub async fn pg_pool_manager(config: Arc<AppConfig>) -> Result<PgPoolManager> {
    PgPoolManager::new(config.main_database(), config.default_tenant_database()).await
}

/// Initializes the default application with required modules and configurations.
///
/// This asynchronous function sets up the application by performing the following steps:
/// 1. Loads the application configuration and wraps it in an `Arc` for shared ownership.
/// 2. Creates and initializes the PostgreSQL connection pool manager using the loaded configuration.
/// 3. Initializes and builds default modules for authentication and tenant management
///    using dependencies such as the connection pool manager and configuration.
/// 4. Constructs an `axum::Router` to handle API routes, wiring up the routes for the
///    authentication and tenant modules under the `/api` path, and applies an HTTP trace layer.
///
/// # Returns
/// * `Ok(Router)` - Returns the fully initialized `Router` instance to handle API requests.
/// * `Err(anyhow::Error)` - Returns an error if any step in the initialization process fails,
///   such as configuration loading, connection pool setup, or module initialization.
///
/// # Errors
/// - Returns an error if the application configuration cannot be loaded.
/// - Returns an error if the PostgreSQL connection pool manager cannot be initialized.
/// - Returns an error if the authentication or tenant modules fail to initialize.
///
/// # Dependencies
/// - The function depends on the external crates `axum` for the router, `tower-http` for the trace
///   layer, and `anyhow` for error handling.
pub async fn init_default_app() -> Result<Router> {
    let config = Arc::new(config()?);
    let pool_manager = pg_pool_manager(config.clone()).await?;
    let pool_manager = Arc::new(pool_manager);
    let auth_module = init_default_auth_module(pool_manager.clone(), config.clone())
        .build()
        .map_err(|e| anyhow!("{}", e))?;
    let tenants_module = init_default_tenants_module(pool_manager.clone(), config.clone())
        .build()
        .map_err(|e| anyhow!("{}", e))?;
    Ok(Router::new().nest(
        "/api",
        Router::new()
            .merge(auth::routes::routes(Arc::new(auth_module)))
            .merge(tenants::routes::routes(Arc::new(tenants_module)))
            .layer(TraceLayer::new_for_http()),
    ))
}
