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
use tracing::error;
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Json};
use crate::users::dto::{LoginRequest, RegisterRequest};
use super::{middleware::AuthenticatedUser, service::{try_login, try_register}, AuthModule};

// ===== LOGIN =====
pub async fn login(
    State(auth_module): State<Arc<AuthModule>>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    match try_login(auth_module.clone(), payload).await {
        Ok(resp) => (StatusCode::OK, axum::Json(resp)).into_response(),
        Err(e) => {
            // TODO: improve this so you can notify users if internal server error happens
            error!("Login error: {:?}", e);
            (StatusCode::UNAUTHORIZED, "Hibás e-mail cím vagy jelszó").into_response()
        }
    }
}

// ===== REGISTER =====
pub async fn register(
    State(auth_module): State<Arc<AuthModule>>,
    Json(payload): Json<RegisterRequest>,
) -> Response {
    match try_register(
        auth_module.repo.clone(),
        auth_module.password_hasher.clone(),
        payload
    ).await {
        Ok(resp) => (StatusCode::CREATED, axum::Json(resp)).into_response(),
        Err(e) => {
            // TODO: improve this so you can notify users if internal server error happens
            error!("Register error: {:?}", e);
            (StatusCode::CONFLICT, "Ez az e-mail cím már foglalt").into_response()
        }
    }
}

pub async fn test_protected(AuthenticatedUser(claims): AuthenticatedUser) -> String {
    format!("Hello, {}! You are authenticated.", claims.sub())
}
