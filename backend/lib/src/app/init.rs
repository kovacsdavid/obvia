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
use crate::auth;
use crate::auth::service::Argon2Hasher;
use crate::organizational_units::{self, OrganizationalUnitsModule};
use crate::users::UsersModule;
use anyhow::Result;
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn subscriber() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");
}

pub fn config() -> Result<Arc<AppConfig>> {
    Ok(Arc::new(AppConfig::from_env()?))
}

pub async fn pg_pool_manager(config: Arc<AppConfig>) -> Result<Arc<PgPoolManager>> {
    Ok(Arc::new(
        PgPoolManager::new(config.main_database(), config.default_tenant_database()).await?,
    ))
}

pub async fn migrate_main_db(main_pool: &PgPool) -> Result<()> {
    Ok(sqlx::migrate!("../migrations").run(main_pool).await?)
}

pub async fn app_state(pool_manager: Arc<PgPoolManager>, config: Arc<AppConfig>) -> AppState {
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

pub async fn app(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(auth::routes::routes(app_state.clone()))
        .merge(organizational_units::routes::routes(app_state.clone()))
        .layer(TraceLayer::new_for_http())
}
