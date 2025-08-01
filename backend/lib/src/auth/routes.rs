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

use super::handler::{login, register};
use crate::app::app_state::AppState;
use axum::{Router, routing::post};
use std::sync::Arc;

/// Configures the application routes for the authentication module.
///
/// This function sets up the routing structure for authentication-related
/// endpoints using the `Router` from the `axum` library. It nests authentication
/// routes under the "/auth" path and associates handlers with specific HTTP methods.
///
/// # Arguments
///
/// * `state` - An `Arc<AppState>` instance that holds shared application state,
///   including the `auth_module` for authentication-specific handling.
///
/// # Routes
///
/// - `POST /auth/register`: Handled by the `register` function for user registration.
/// - `POST /auth/login`: Handled by the `login` function for user login.
///
/// # Returns
///
/// A `Router` instance with the configured "/auth" routes and the provided application state.
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(register))
            .route("/login", post(login))
            .with_state(state.auth_module.clone()),
    )
}
