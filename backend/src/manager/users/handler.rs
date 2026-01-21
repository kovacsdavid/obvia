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

use crate::common::dto::{EmptyType, HandlerResult, SimpleMessageResponse, SuccessResponseBuilder};
use crate::common::error::IntoFriendlyError;
use crate::common::extractors::{ClientContext, UserInput};
use crate::manager::auth::dto::login::{OtpUserInput, OtpUserInputHelper};
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::users::service::UsersService;
use crate::tenant::users::UsersModule;
use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

#[debug_handler]
pub async fn get_claims(
    State(users_module): State<Arc<dyn UsersModule>>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> HandlerResult {
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(claims)
        .build()
    {
        Ok(success) => Ok(success.into_response()),
        Err(e) => Err(e.into_friendly_error(users_module).await.into_response()),
    }
}

#[debug_handler]
pub async fn otp_enable(
    State(users_module): State<Arc<dyn UsersModule>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> HandlerResult {
    let response =
        match UsersService::otp_enable(users_module.clone(), &claims, &client_context).await {
            Ok(v) => v,
            Err(e) => return Err(e.into_friendly_error(users_module).await.into_response()),
        };

    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(response)
        .build()
    {
        Ok(success) => Ok(success.into_response()),
        Err(e) => Err(e.into_friendly_error(users_module).await.into_response()),
    }
}

#[debug_handler]
pub async fn otp_verify(
    State(users_module): State<Arc<dyn UsersModule>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
    UserInput(user_input, _): UserInput<OtpUserInput, OtpUserInputHelper>,
) -> HandlerResult {
    match UsersService::otp_verify(users_module.clone(), &claims, &user_input, &client_context)
        .await
    {
        Ok(v) => v,
        Err(e) => return Err(e.into_friendly_error(users_module).await.into_response()),
    };

    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(SimpleMessageResponse::new(
            "A kétlépcsős azonosítás aktiválása megtörtént!",
        ))
        .build()
    {
        Ok(success) => Ok(success.into_response()),
        Err(e) => Err(e.into_friendly_error(users_module).await.into_response()),
    }
}

#[debug_handler]
pub async fn otp_disable(
    State(users_module): State<Arc<dyn UsersModule>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
    UserInput(user_input, _): UserInput<OtpUserInput, OtpUserInputHelper>,
) -> HandlerResult {
    match UsersService::otp_disable(users_module.clone(), &claims, &user_input, &client_context)
        .await
    {
        Ok(v) => v,
        Err(e) => return Err(e.into_friendly_error(users_module).await.into_response()),
    };

    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(SimpleMessageResponse::new(
            "A kétlépcsős azonosítás kikapcsolása megtörtént!",
        ))
        .build()
    {
        Ok(success) => Ok(success.into_response()),
        Err(e) => Err(e.into_friendly_error(users_module).await.into_response()),
    }
}
