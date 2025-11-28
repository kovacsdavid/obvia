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
    SuccessResponseBuilder, UuidParam,
};
use crate::common::error::FriendlyError;
use crate::common::error::IntoFriendlyError;
use crate::common::extractors::UserInput;
use crate::common::types::Order;
use crate::common::types::ValueObject;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::worksheets::WorksheetsModule;
use crate::tenant::worksheets::dto::{WorksheetUserInput, WorksheetUserInputHelper};
use crate::tenant::worksheets::service::WorksheetsService;
use crate::tenant::worksheets::types::worksheet::WorksheetOrderBy;
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::sync::Arc;

#[debug_handler]
pub async fn get_resolved(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<dyn WorksheetsModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let result = match WorksheetsService::get_resolved_by_id(
        &claims,
        &payload,
        worksheets_module.worksheets_repo(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(e
                .into_friendly_error(worksheets_module)
                .await
                .into_response());
        }
    };
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e
            .into_friendly_error(worksheets_module)
            .await
            .into_response()),
    }
}

#[debug_handler]
pub async fn get(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<dyn WorksheetsModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let result = match WorksheetsService::get(
        &claims,
        &payload,
        worksheets_module.worksheets_repo(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(e
                .into_friendly_error(worksheets_module)
                .await
                .into_response());
        }
    };
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e
            .into_friendly_error(worksheets_module)
            .await
            .into_response()),
    }
}

#[debug_handler]
pub async fn update(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<dyn WorksheetsModule>>,
    UserInput(user_input, _): UserInput<WorksheetUserInput, WorksheetUserInputHelper>,
) -> HandlerResult {
    let result =
        match WorksheetsService::update(&claims, &user_input, worksheets_module.worksheets_repo())
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return Err(e
                    .into_friendly_error(worksheets_module)
                    .await
                    .into_response());
            }
        };
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e
            .into_friendly_error(worksheets_module)
            .await
            .into_response()),
    }
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<dyn WorksheetsModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    match WorksheetsService::delete(&claims, &payload, worksheets_module.worksheets_repo()).await {
        Ok(_) => (),
        Err(e) => {
            return Err(e
                .into_friendly_error(worksheets_module)
                .await
                .into_response());
        }
    };

    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(SimpleMessageResponse::new(
            "A munkalap törlése sikeresen megtörtént",
        ))
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e
            .into_friendly_error(worksheets_module)
            .await
            .into_response()),
    }
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<dyn WorksheetsModule>>,
    UserInput(user_input, _): UserInput<WorksheetUserInput, WorksheetUserInputHelper>,
) -> HandlerResult {
    let result =
        match WorksheetsService::create(&claims, &user_input, worksheets_module.clone()).await {
            Ok(r) => r,
            Err(e) => {
                return Err(e
                    .into_friendly_error(worksheets_module)
                    .await
                    .into_response());
            }
        };
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::CREATED)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e
            .into_friendly_error(worksheets_module)
            .await
            .into_response()),
    }
}

pub async fn select_list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<dyn WorksheetsModule>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));

    let result = match WorksheetsService::get_select_list_items(
        &list_type,
        &claims,
        worksheets_module.clone(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(e
                .into_friendly_error(worksheets_module)
                .await
                .into_response());
        }
    };

    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e
            .into_friendly_error(worksheets_module)
            .await
            .into_response()),
    }
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<dyn WorksheetsModule>>,
    Query(payload): Query<QueryParam>,
) -> HandlerResult {
    let (meta, data) = match WorksheetsService::get_paged_list(
        &PaginatorParams::try_from(&payload).unwrap_or(PaginatorParams::default()),
        &OrderingParams::try_from(&payload).unwrap_or(OrderingParams {
            order_by: ValueObject::new(WorksheetOrderBy("name".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
            order: ValueObject::new(Order("asc".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
        }),
        &FilteringParams::from(&payload),
        &claims,
        worksheets_module.worksheets_repo(),
    )
    .await
    {
        Ok((m, d)) => (m, d),
        Err(e) => {
            return Err(e
                .into_friendly_error(worksheets_module)
                .await
                .into_response());
        }
    };

    match SuccessResponseBuilder::new()
        .status_code(StatusCode::OK)
        .meta(meta)
        .data(data)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e
            .into_friendly_error(worksheets_module)
            .await
            .into_response()),
    }
}
