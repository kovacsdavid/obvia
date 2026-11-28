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

use crate::common::AppStateInterface;

use super::handler;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn routes<A: AppStateInterface>(auth_module: Arc<A>) -> Router {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(handler::register::<A>))
            .route("/login", post(handler::login::<A>))
            .route("/verify_email", get(handler::verify_email::<A>))
            .route(
                "/resend_email_verification",
                post(handler::resend_email_verification::<A>),
            )
            .route(
                "/forgotten_password",
                post(handler::forgotten_password::<A>),
            )
            .route("/new_password", post(handler::new_password::<A>))
            .route("/t/refresh", post(handler::refresh::<A>)) // "[t]oken" nest is for cookie path restriction
            .route("/t/logout", post(handler::logout::<A>)) // "[t]oken" nest is for cookie path restriction
            .with_state(auth_module),
    )
}
