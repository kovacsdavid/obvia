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
use backend_lib::app::database::{PgPoolManager, PgPoolManagerTrait};
use backend_lib::organizational_units::OrganizationalUnitsModule;
use backend_lib::{
    app::{app_state::AppState, config::AppConfig},
    auth::{self, routes::routes as auth_routes, service::Argon2Hasher},
    organizational_units::routes::routes as organizational_units_routes,
    users::UsersModule,
};
use std::sync::Arc;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn init_subscriber() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_subscriber();
    let config = Arc::new(AppConfig::from_env()?);
    let db_pools = Arc::new(PgPoolManager::new(config.main_database()).await?);
    sqlx::migrate!("../migrations")
        .run(&db_pools.get_main_pool())
        .await?;
    serve(config.clone(), db_pools.clone()).await?;
    Ok(())
}

async fn serve(config: Arc<AppConfig>, db_pools: Arc<PgPoolManager>) -> anyhow::Result<()> {
    let auth_module = auth::AuthModule {
        db_pools: db_pools.clone(),
        password_hasher: Arc::new(Argon2Hasher),
        config: config.clone(),
    };
    let users_module = Arc::new(UsersModule {});
    let organizational_units_module = Arc::new(OrganizationalUnitsModule {
        db_pools: db_pools.clone(),
        config: config.clone(),
    });
    let state = Arc::new(AppState {
        auth_module: Arc::new(auth_module),
        config_module: config.clone(),
        users_module,
        organizational_units_module,
    });

    let app = Router::new()
        .merge(auth_routes(state.clone()))
        .merge(organizational_units_routes(state.clone()))
        .layer(TraceLayer::new_for_http());

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
