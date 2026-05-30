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

use super::UsersModule;
use super::service::UserService;
use crate::common::dto::{EmptyType, SimpleMessageResponse, SuccessResponseBuilder};
use crate::common::extractors::{ClientContext, UserInput};
use crate::common::handler::{HandlerResult, init_handler};
use crate::manager::auth::dto::login::{OtpUserInput, OtpUserInputHelper};
use crate::manager::auth::middleware::AuthenticatedUser;
use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

#[debug_handler]
pub async fn get_claims(
    State(users_module): State<Arc<dyn UsersModule>>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> HandlerResult {
    let (service, error_mapper) = init_handler(Some(&claims), users_module);
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(service.claims().expect("invalid state"))
                .build(),
        )
        .await?
        .into_response())
}

#[debug_handler]
pub async fn otp_enable(
    State(users_module): State<Arc<dyn UsersModule>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> HandlerResult {
    let (service, error_mapper) = init_handler(Some(&claims), users_module);
    let response = error_mapper
        .or_handler_error(service.otp_enable(&client_context).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(response)
                .build(),
        )
        .await?
        .into_response())
}

#[debug_handler]
pub async fn otp_verify(
    State(users_module): State<Arc<dyn UsersModule>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
    UserInput(user_input, _): UserInput<OtpUserInput, OtpUserInputHelper>,
) -> HandlerResult {
    let (service, error_mapper) = init_handler(Some(&claims), users_module);
    error_mapper
        .or_handler_error(service.otp_verify(&user_input, &client_context).await)
        .await?;

    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A kétlépcsős azonosítás aktiválása megtörtént!",
                ))
                .build(),
        )
        .await?
        .into_response())
}

#[debug_handler]
pub async fn otp_disable(
    State(users_module): State<Arc<dyn UsersModule>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
    UserInput(user_input, _): UserInput<OtpUserInput, OtpUserInputHelper>,
) -> HandlerResult {
    let (service, error_mapper) = init_handler(Some(&claims), users_module);
    error_mapper
        .or_handler_error(service.otp_disable(&user_input, &client_context).await)
        .await?;

    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A kétlépcsős azonosítás kikapcsolása megtörtént!",
                ))
                .build(),
        )
        .await?
        .into_response())
}
