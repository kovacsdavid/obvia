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

use super::{
    AuthModule,
    middleware::AuthenticatedUser,
    service::{try_login, try_register},
};
use crate::auth::dto::register::RegisterRequestHelper;
use crate::{
    auth::dto::{login::LoginRequest, register::RegisterRequest},
    common::error::FriendlyError,
};
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

pub async fn login(
    State(auth_module): State<Arc<AuthModule>>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    match try_login(auth_module.clone(), payload).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn register(
    State(auth_module): State<Arc<AuthModule>>,
    payload: Result<Json<RegisterRequestHelper>, JsonRejection>,
) -> Response {
    match payload {
        Ok(Json(payload)) => match RegisterRequest::try_from(payload) {
            Ok(user_input) => {
                match try_register(
                    auth_module.repo.clone(),
                    auth_module.password_hasher.clone(),
                    user_input,
                )
                .await
                {
                    Ok(resp) => (StatusCode::CREATED, Json(resp)).into_response(),
                    Err(e) => e.into_response(),
                }
            }
            Err(e) => e.into_response(),
        },
        Err(_) => FriendlyError::UserFacing(
            StatusCode::BAD_REQUEST,
            "AUTH/HANDLER/REGISTER".to_string(),
            "Hibás adatszerkezet".to_string(),
        )
        .trace(tracing::Level::DEBUG)
        .into_response(),
    }
}

pub async fn test_protected(AuthenticatedUser(claims): AuthenticatedUser) -> String {
    format!("Hello, {}! You are authenticated.", claims.sub())
}
