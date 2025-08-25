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

use crate::auth::middleware::require_auth;
use crate::tenants::TenantsModule;
use crate::tenants::handler::{
    activate as tenants_activate, create as tenants_create, get as tenants_get,
    list as tenants_list,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use std::sync::Arc;

/// Configures and returns a `Router` instance with routes and middleware for handling tenant-related API endpoints.
///
/// # Arguments
///
/// * `tenants_module` - An `Arc` wrapping the `TenantsModule` instance, which contains the configuration and state required for handling tenant operations.
///
/// # Routes
///
/// The router has a nested route under the `/tenants` path. Inside this path, the following routes are available:
///
/// 1. **POST /tenants/create** - Handled by the `managed_companies_create` function. Used to create a new tenant or company.
///
/// 2. **GET /tenants/get** - Handled by the `managed_companies_get` function. Retrieves the details for a specific tenant or company.
///
/// 3. **GET /tenants/list** - Handled by the `managed_companies_list` function. Fetches a list of all tenants or companies.
///
/// # Middleware
///
/// Applies the following middleware to all `/tenants` routes:
/// * `require_auth`: Ensures that requests are authenticated. This middleware is configured using the state cloned from the `tenants_module` configuration via `from_fn_with_state`.
///
/// # State
///
/// Sets the `tenants_module` as the state of the `/tenants` route, allowing handler functions to access shared state.
///
/// # Returns
///
/// * A `Router` instance configured with the defined routes and middleware.
pub fn routes(tenants_module: Arc<TenantsModule>) -> Router {
    Router::new().nest(
        "/tenants",
        Router::new()
            .route("/create", post(tenants_create))
            .route("/get", get(tenants_get))
            .route("/list", get(tenants_list))
            .route("/activate", post(tenants_activate))
            .layer(from_fn_with_state(
                tenants_module.config.clone(),
                require_auth,
            ))
            .with_state(tenants_module),
    )
}
