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

use axum::Router;
use backend_lib::app::config::AppConfig;
use backend_lib::app::init::{
    app, app_state, config, init_tenant_pools, migrate, pg_pool_manager, subscriber,
};
use std::sync::Arc;
use tokio::signal;

async fn init() -> anyhow::Result<(Arc<AppConfig>, Router)> {
    subscriber();
    let config = config()?;
    let pool_manager = pg_pool_manager(config.clone()).await?;
    let app_state = Arc::new(app_state(pool_manager.clone(), config.clone()).await);
    let app = app(app_state).await;
    init_tenant_pools(pool_manager.clone()).await?;
    migrate(pool_manager.clone()).await?;
    Ok((config, app))
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    serve().await?;
    Ok(())
}

async fn serve() -> anyhow::Result<()> {
    let (config, app) = init().await?;

    let addr = config.server().host().to_string() + ":" + &config.server().port().to_string();
    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, Router::new().nest("/api", app))
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

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
