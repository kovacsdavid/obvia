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

use crate::common::AppState;
use crate::common::config::AppConfig;
use crate::common::database::{DatabaseMigrator, PgPoolManager, PoolManager};
use crate::manager::tenants::repository::TenantsRepository;
use anyhow::Result;
use axum::Router;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use tower_http::trace::TraceLayer;
use tracing_subscriber::FmtSubscriber;

pub fn init_subscriber(config: &AppConfig) {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(config.server().log_level())
            .finish(),
    )
    .expect("setting default subscriber failed");
}

pub async fn init_default_app_state(
    config: AppConfig,
) -> Result<AppState<PgPoolManager, AsyncSmtpTransport<Tokio1Executor>>> {
    let pg_pool_manager = PgPoolManager::new(config.main_database()).await?;
    let smtp_transport =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(config.mail().smtp_host())?
            .credentials(Credentials::new(
                config.mail().smtp_user().to_owned(),
                config.mail().smtp_passwd().to_owned(),
            ))
            .build();
    let app_state = AppState::new(config, pg_pool_manager, smtp_transport).await?;

    let tenants = TenantsRepository::get_all(&*app_state.pool_manager().get_main_pool()).await?;
    app_state.pool_manager().migrate_main_db().await?;
    app_state.pool_manager().init_tenant_pools(&tenants).await?;
    app_state
        .pool_manager()
        .migrate_all_tenant_dbs(&tenants)
        .await?;

    Ok(app_state)
}

pub async fn init_default_app(
    app_state: AppState<PgPoolManager, AsyncSmtpTransport<Tokio1Executor>>,
) -> Result<Router> {
    let app_state = Arc::new(app_state);
    Ok(Router::new().nest(
        "/api",
        Router::new()
            .merge(crate::manager::auth::routes::routes(app_state.clone()))
            .merge(crate::manager::users::routes::routes(app_state.clone()))
            .merge(crate::manager::tenants::routes::routes(app_state.clone()))
            .merge(crate::tenant::activity_feed::routes::routes(
                app_state.clone(),
            ))
            .merge(crate::tenant::comments::routes::routes(app_state.clone()))
            .merge(crate::tenant::customers::routes::routes(app_state.clone()))
            .merge(crate::tenant::inventory::routes::routes(app_state.clone()))
            .merge(crate::tenant::inventory_movements::routes::routes(
                app_state.clone(),
            ))
            .merge(crate::tenant::inventory_reservations::routes::routes(
                app_state.clone(),
            ))
            .merge(crate::tenant::products::routes::routes(app_state.clone()))
            .merge(crate::tenant::services::routes::routes(app_state.clone()))
            .merge(crate::tenant::tasks::routes::routes(app_state.clone()))
            .merge(crate::tenant::taxes::routes::routes(app_state.clone()))
            .merge(crate::tenant::warehouses::routes::routes(app_state.clone()))
            .merge(crate::tenant::worksheets::routes::routes(app_state.clone()))
            .layer(TraceLayer::new_for_http()),
    ))
}
