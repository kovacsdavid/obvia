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
use crate::common::dto::{
    EmptyType, HandlerResult, OrderingParams, PaginatorParams, QueryParam, SimpleMessageResponse,
    SuccessResponseBuilder,
};
use crate::common::error::FriendlyError;
use crate::common::extractors::UserInput;
use crate::common::types::order::Order;
use crate::common::types::value_object::ValueObject;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::worksheets::WorksheetsModule;
use crate::tenant::worksheets::dto::{CreateWorksheet, CreateWorksheetHelper};
use crate::tenant::worksheets::service::WorksheetsService;
use crate::tenant::worksheets::types::worksheet::WorksheetOrderBy;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, debug_handler};
use std::collections::HashMap;
use std::sync::Arc;

#[debug_handler]
pub async fn get(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<WorksheetsModule>>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<WorksheetsModule>>,
    UserInput(user_input, _): UserInput<CreateWorksheet, CreateWorksheetHelper>,
) -> HandlerResult {
    WorksheetsService::create(&claims, &user_input, worksheets_module)
        .await
        .map_err(|e| e.into_response())?;
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::CREATED)
        .data(SimpleMessageResponse::new(
            "A munkalap létrehozása sikeresen megtörtént!",
        ))
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

pub async fn select_list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<WorksheetsModule>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(
            WorksheetsService::get_select_list_items(&list_type, &claims, worksheets_module)
                .await
                .map_err(|e| e.into_response())?,
        )
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

#[debug_handler]
pub async fn update(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<WorksheetsModule>>,
    payload: Result<Json<CreateWorksheet>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<WorksheetsModule>>,
    payload: Result<Json<CreateWorksheet>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<WorksheetsModule>>,
    Query(payload): Query<QueryParam>,
) -> HandlerResult {
    let (meta, data) = WorksheetsService::get_paged_list(
        &PaginatorParams::try_from(&payload).unwrap_or(PaginatorParams::default()),
        &OrderingParams::try_from(&payload).unwrap_or(OrderingParams {
            order_by: ValueObject::new(WorksheetOrderBy("name".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
            order: ValueObject::new(Order("asc".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
        }),
        &FilteringParams::from(&payload),
        &claims,
        worksheets_module.worksheets_repo.clone(),
    )
    .await
    .map_err(|e| e.into_response())?;

    Ok(SuccessResponseBuilder::new()
        .status_code(StatusCode::OK)
        .meta(meta)
        .data(data)
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}
