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
use crate::auth::middleware::require_auth;
use crate::tenants::handler::{
    create as managed_companies_create, get as managed_companies_get,
    list as managed_companies_list,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use std::sync::Arc;

/// Sets up application routes for the `/tenants` endpoint.
///
/// This function creates a router and nests routes specific to
/// managing tenants. It ensures that proper middleware
/// (e.g., authentication) is applied and associates the necessary
/// application state to the nested routes.
///
/// # Arguments
///
/// * `state` - An `Arc<AppState>` that holds the application state.
///   This includes shared state modules such as `tenants_module`.
///
/// # Routes
///
/// The following routes are available under the `/tenants` path:
///
/// - `/create`: Handles HTTP POST requests to create a new tenant.
///   This route is handled by the `managed_companies_create` function.
/// - `/get`: Handles HTTP GET requests to retrieve a specific tenant.
///   This route is handled by the `managed_companies_get` function.
/// - `/list`: Handles HTTP GET requests to list all available tenants.
///   This route is handled by the `managed_companies_list` function.
///
/// # Middleware
///
/// The routes are wrapped with the `require_auth` middleware, which is applied
/// via `from_fn_with_state` to ensure proper authentication is required for each
/// request. The middleware function is provided with the cloned application state
/// for its execution.
///
/// # State Association
///
/// - The `tenants_module` section of the application state is associated
///   with the nested router using `.with_state()`.
///
/// # Returns
///
/// A `Router` instance configured with the nested routes for managing tenants.
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new().nest(
        "/tenants",
        Router::new()
            .route("/create", post(managed_companies_create))
            .route("/get", get(managed_companies_get))
            .route("/list", get(managed_companies_list))
            .layer(from_fn_with_state(state.clone(), require_auth))
            .with_state(state.tenants_module.clone()),
    )
}
