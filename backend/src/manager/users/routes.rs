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

use crate::manager::auth::middleware::require_auth;
use crate::manager::users::handler::{get_claims, otp_disable, otp_enable, otp_verify};
use crate::tenant::users::UsersModule;
use axum::middleware::from_fn_with_state;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn routes(users_module: Arc<dyn UsersModule>) -> Router {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/get_claims", get(get_claims))
            .route("/otp/enable", get(otp_enable))
            .route("/otp/verify", post(otp_verify))
            .route("/otp/disable", post(otp_disable))
            .layer(from_fn_with_state(users_module.config(), require_auth))
            .with_state(users_module),
    )
}
