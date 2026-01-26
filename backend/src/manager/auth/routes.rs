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

use super::handler::{
    forgotten_password, login, logout, new_password, refresh, register, resend_email_verification,
    verify_email,
};
use crate::manager::auth::AuthModule;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn routes(auth_module: Arc<dyn AuthModule>) -> Router {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(register))
            .route("/login", post(login))
            .route("/verify_email", get(verify_email))
            .route("/resend_email_verification", get(resend_email_verification))
            .route("/forgotten_password", post(forgotten_password))
            .route("/new_password", post(new_password))
            .route("/t/refresh", post(refresh)) // "[t]oken" nest is for cookie path restriction
            .route("/t/logout", post(logout)) // "[t]oken" nest is for cookie path restriction
            .with_state(auth_module),
    )
}
