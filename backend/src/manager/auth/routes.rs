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

use super::handler::{get_claims, login, register};
use crate::manager::auth::AuthModule;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

/// Configures and returns a `Router` with authentication-specific routes.
///
/// # Parameters
/// - `auth_module`: An `Arc<AuthModule>` that contains the authentication logic and shared state.
///   This is passed to the routes as state, enabling access to necessary authentication mechanisms.
///
/// # Returns
/// A `Router` instance with the following nested routes under the `/auth` path:
/// - `POST /auth/register`: Calls the `register` handler to handle user registration.
/// - `POST /auth/login`: Calls the `login` handler to handle user login.
///
/// # Dependencies
/// - `axum`: Used for creating the `Router` and defining the HTTP routes.
pub fn routes(auth_module: Arc<AuthModule>) -> Router {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(register))
            .route("/login", post(login))
            .route("/get_claims", get(get_claims))
            .with_state(auth_module),
    )
}
