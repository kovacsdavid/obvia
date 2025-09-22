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
use crate::common::error::FriendlyError;
use crate::common::extractors::UserInput;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::common::dto::{OkResponse, QueryParam, SimpleMessageResponse};
use crate::tenant::products::ProductsModule;
use crate::tenant::products::dto::{CreateProduct, CreateProductHelper};
use crate::tenant::products::service::{ProductsService, ProductsServiceError};
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, debug_handler};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::Level;

#[debug_handler]
pub async fn get(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<ProductsModule>>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<ProductsModule>>,
    UserInput(user_input, _): UserInput<CreateProduct, CreateProductHelper>,
) -> Result<Response, Response> {
    ProductsService::create(&claims, &user_input, products_module)
        .await
        .map_err(|e| {
            match e {
                ProductsServiceError::Unauthorized => FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    "Hozzáférés megtagadva!",
                ),
                _ => FriendlyError::internal(file!(), e.to_string()),
            }
            .into_response()
        })?;
    Ok((
        StatusCode::CREATED,
        Json(OkResponse::new(SimpleMessageResponse {
            message: String::from("A termék létrehozása sikeresen megtörtént"),
        })),
    )
        .into_response())
}

#[debug_handler]
pub async fn update(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<ProductsModule>>,
    payload: Result<Json<CreateProduct>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<ProductsModule>>,
    payload: Result<Json<CreateProduct>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<ProductsModule>>,
    Query(payload): Query<QueryParam>,
) -> Response {
    todo!()
}

pub async fn select_list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<ProductsModule>>,
    Query(payload): Query<HashMap<String, String>>,
) -> Result<Response, Response> {
    let invalid_request = || {
        FriendlyError::user_facing(
            Level::DEBUG,
            StatusCode::BAD_REQUEST,
            file!(),
            "Invalid request",
        )
        .into_response()
    };
    let list_type = payload.get("list").ok_or(invalid_request())?;

    match list_type.as_str() {
        "units_of_measure" => Ok((
            StatusCode::OK,
            Json(OkResponse::new(
                ProductsService::get_all_units_of_measure(&claims, products_module)
                    .await
                    .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
            )),
        )
            .into_response()),
        _ => Err(invalid_request()),
    }
}
