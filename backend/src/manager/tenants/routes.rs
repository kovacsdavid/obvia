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

use super::TenantsModule;
use super::handler;
use crate::manager::auth::middleware::require_auth;
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn routes(tenants_module: Arc<dyn TenantsModule>) -> Router {
    Router::new().nest(
        "/tenants",
        Router::new()
            .route("/create", post(handler::create))
            .route("/get", get(handler::get))
            .route("/list", get(handler::list))
            .route("/activate", post(handler::activate))
            .layer(from_fn_with_state(tenants_module.config(), require_auth))
            .with_state(tenants_module),
    )
}
