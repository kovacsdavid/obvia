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

use crate::common::{ConfigProvider, DefaultAppState};
use crate::manager::app::config::AppConfig;
use crate::manager::app::database::DatabaseMigrator;
use anyhow::Result;
use axum::Router;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init_subscriber() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE) //TODO: make configurable
            .finish(),
    )
    .expect("setting default subscriber failed");
}

pub async fn init_default_app() -> Result<(Arc<AppConfig>, Router)> {
    let app_state = Arc::new(DefaultAppState::new().await?);
    app_state.migrate_main_db().await?;
    app_state.init_tenant_pools().await?;
    app_state.migrate_all_tenant_dbs().await?;
    Ok((
        app_state.config(),
        Router::new().nest(
            "/api",
            Router::new()
                .merge(crate::manager::auth::routes::routes(app_state.clone()))
                .merge(crate::manager::users::routes::routes(app_state.clone()))
                .merge(crate::manager::tenants::routes::routes(app_state.clone()))
                .merge(crate::tenant::customers::routes::routes(app_state.clone()))
                .merge(crate::tenant::inventory::routes::routes(app_state.clone()))
                .merge(crate::tenant::inventory_movements::routes::routes(
                    app_state.clone(),
                ))
                .merge(crate::tenant::inventory_reservations::routes::routes(
                    app_state.clone(),
                ))
                .merge(crate::tenant::products::routes::routes(app_state.clone()))
                //.merge(crate::tenant::projects::routes::routes(app_state.clone()))
                .merge(crate::tenant::services::routes::routes(app_state.clone()))
                .merge(crate::tenant::tags::routes::routes(app_state.clone()))
                .merge(crate::tenant::tasks::routes::routes(app_state.clone()))
                .merge(crate::tenant::taxes::routes::routes(app_state.clone()))
                .merge(crate::tenant::warehouses::routes::routes(app_state.clone()))
                .merge(crate::tenant::worksheets::routes::routes(app_state.clone()))
                .layer(TraceLayer::new_for_http()),
        ),
    ))
}
