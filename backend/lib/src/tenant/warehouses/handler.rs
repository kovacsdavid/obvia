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
use crate::tenant::warehouses::WarehousesModule;
use crate::tenant::warehouses::dto::{CreateWarehouse, CreateWarehouseHelper};
use crate::tenant::warehouses::service::{WarehousesService, WarehousesServiceError};
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, debug_handler};
use std::sync::Arc;
use tracing::Level;

#[debug_handler]
pub async fn get(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(warehouses_module): State<Arc<WarehousesModule>>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(warehouses_module): State<Arc<WarehousesModule>>,
    UserInput(user_input, _): UserInput<CreateWarehouse, CreateWarehouseHelper>,
) -> Result<Response, Response> {
    WarehousesService::try_create(&claims, &user_input, warehouses_module)
        .await
        .map_err(|e| {
            match e {
                WarehousesServiceError::Repository(_) => {
                    FriendlyError::internal(file!(), e.to_string())
                }
                WarehousesServiceError::Unauthorized => FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    "Hozzáférés megtagadva!",
                ),
            }
            .into_response()
        })?;
    Ok((
        StatusCode::CREATED,
        Json(OkResponse::new(SimpleMessageResponse {
            message: String::from("A raktár létrehozása sikeresen megtörtént!"),
        })),
    )
        .into_response())
}

#[debug_handler]
pub async fn update(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(warehouses_module): State<Arc<WarehousesModule>>,
    payload: Result<Json<CreateWarehouse>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(warehouses_module): State<Arc<WarehousesModule>>,
    payload: Result<Json<CreateWarehouse>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(warehouses_module): State<Arc<WarehousesModule>>,
    Query(payload): Query<QueryParam>,
) -> Response {
    todo!()
}
