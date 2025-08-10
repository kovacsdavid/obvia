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
#![forbid(unsafe_code)]

use anyhow::anyhow;
use axum::Router;
use backend_lib::app::config::AppConfig;
use backend_lib::app::init::{
    app, app_state, config, init_tenant_pools, migrate_all_tenant_dbs, migrate_main_db,
    pg_pool_manager, subscriber,
};
use std::sync::Arc;
use tokio::signal;

/// Initializes the application by setting up the necessary configuration, state, and database.
///
/// This function performs the following tasks in sequence:
/// 1. Initializes the log subscriber to enable logging.
/// 2. Loads the application configuration.
/// 3. Establishes a connection pool manager for PostgreSQL.
/// 4. Creates the shared application state, which includes the database connection pool and configuration.
/// 5. Builds the application router using the shared application state.
/// 6. Initializes connection pools for tenants.
/// 7. Performs database migrations to ensure the schema is up-to-date.
///
/// # Returns
/// A tuple containing:
/// - The shared application configuration wrapped in an `Arc<AppConfig>`.
/// - The configured `Router` for handling requests.
///
/// # Errors
/// This function returns an error wrapped in `anyhow::Result` if:
/// - The configuration file cannot be loaded.
/// - The PostgreSQL connection pool cannot be initialized.
/// - The tenant connection pools cannot be initialized.
/// - The database migrations fail.
async fn init() -> anyhow::Result<(Arc<AppConfig>, Router)> {
    subscriber();
    let config = config()?;
    let pool_manager = pg_pool_manager(config.clone()).await?;
    let app_state =
        Arc::new(app_state(pool_manager.clone(), config.clone()).map_err(|e| anyhow!("{}", e))?);
    let app = app(app_state).await;
    migrate_main_db(pool_manager.clone()).await?;
    init_tenant_pools(pool_manager.clone()).await?;
    migrate_all_tenant_dbs(pool_manager.clone()).await?;
    Ok((config, app))
}

/// The entry point of the asynchronous Tokio application.
///
/// This function is defined as the main function for the application using the `#[tokio::main]`
/// attribute macro, which sets up the Tokio runtime automatically. Using this macro allows
/// the function to run asynchronous code.
///
/// # Returns
///
/// A result of type `anyhow::Result<()>`, which represents either a successful execution
/// (OK with empty tuple) or an error (Err containing the error details).
///
/// # Errors
///
/// If the `serve()` function, which is called within this function, returns an error,
/// this function propagates it upwards.
///
/// The `serve()` function is expected to perform the main operations of the application,
/// such as starting a web server or handling other asynchronous tasks.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    serve().await
}

/// Asynchronously starts and configures the server, then begins serving requests.
///
/// This function initializes the application and its configuration, binds a TCP listener
/// to the specified address and port, and starts serving incoming requests using `axum`.
/// The server is configured to handle graceful shutdown when a shutdown signal is received.
///
/// # Returns
///
/// - `Ok(())` if the server runs successfully and terminates gracefully.
/// - An `anyhow::Error` if an error occurs during initialization, binding, or serving.
///
/// # Errors
/// This function will return an error if:
/// - Initialization (`init`) fails.
/// - Binding the TCP listener to the specified address fails.
///
/// # Graceful Shutdown
/// The server listens for a shutdown signal and terminates gracefully upon receipt.
///
/// # Example
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     serve().await
/// }
/// ```
///
/// This starts the server and listens for incoming API calls on the configured address.
///
/// # Related
/// - `init`: A function to initialize the app and configuration.
/// - `shutdown_signal`: A helper for detecting shutdown signals.
async fn serve() -> anyhow::Result<()> {
    let (config, app) = init().await?;

    let addr = config.server().host().to_string() + ":" + &config.server().port().to_string();
    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, Router::new().nest("/api", app))
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// Asynchronously waits for a shutdown signal.
///
/// This function listens for either a `Ctrl+C` signal (on all platforms) or a terminate signal
/// on Unix systems (e.g., `SIGTERM`). When either of these signals is received, it completes
/// its execution, allowing the application to perform a graceful shutdown.
///
/// ## Platform-specific Behavior
/// - On Unix systems, it listens for `SIGTERM` in addition to `Ctrl+C`.
/// - On non-Unix systems, only `Ctrl+C` is supported.
///
/// ## Errors
/// - If the `Ctrl+C` handler cannot be installed, this function panics with an appropriate error message.
/// - On Unix, if the `SIGTERM` handler cannot be installed, this function also panics.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
            _ = terminate => {},
    }
}
