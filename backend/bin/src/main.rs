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

use axum::Router;
use backend_lib::{
    app::{app_state::AppState, config::AppConfig},
    auth::{
        self,
        routes::{auth_routes, test_protected_routes},
        service::Argon2Hasher,
    },
    common::repository::PostgresRepo,
    users::UsersModule,
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
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

fn init_config() -> anyhow::Result<AppConfig> {
    Ok(AppConfig::from_env()?)
}

async fn init_db(config: Arc<AppConfig>) -> anyhow::Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(config.database().pool_size())
        .connect(&config.database().url())
        .await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_subscriber();
    let config = Arc::new(init_config()?);
    serve(config.clone(), init_db(config.clone()).await?).await?;
    Ok(())
}

async fn serve(config: Arc<AppConfig>, db: Pool<Postgres>) -> anyhow::Result<()> {
    let auth_repo = PostgresRepo { db: db.clone() };
    let auth_module = auth::AuthModule {
        repo: Arc::new(auth_repo),
        password_hasher: Arc::new(Argon2Hasher),
        config: config.clone(),
    };
    let users_module = Arc::new(UsersModule {});
    let state = Arc::new(AppState {
        auth_module: Arc::new(auth_module),
        config_module: config.clone(),
        users_module,
    });

    let app = Router::new()
        .merge(auth_routes(state.clone()))
        .merge(test_protected_routes(state.clone()))
        .layer(TraceLayer::new_for_http());

    let addr = config.server().host().to_string() + ":" + &config.server().port().to_string();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
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
