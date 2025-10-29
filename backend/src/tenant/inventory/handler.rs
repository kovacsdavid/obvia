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
use crate::common::extractors::UserInput;
use crate::common::types::Order;
use crate::common::types::ValueObject;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::inventory::InventoryModule;
use crate::tenant::inventory::dto::{InventoryUserInput, InventoryUserInputHelper};
use crate::tenant::inventory::service::InventoryService;
use crate::tenant::inventory::types::inventory::InventoryOrderBy;
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::sync::Arc;

#[debug_handler]
pub async fn get_resolved(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<InventoryModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(
            InventoryService::get_resolved_by_id(
                &claims,
                &payload,
                inventory_module.inventory_repo.clone(),
            )
            .await
            .map_err(|e| e.into_response())?,
        )
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

#[debug_handler]
pub async fn get(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<InventoryModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(
            InventoryService::get(&claims, &payload, inventory_module.inventory_repo.clone())
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
    State(inventory_module): State<Arc<InventoryModule>>,
    UserInput(user_input, _): UserInput<InventoryUserInput, InventoryUserInputHelper>,
) -> HandlerResult {
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(
            InventoryService::update(
                &claims,
                &user_input,
                inventory_module.inventory_repo.clone(),
            )
            .await
            .map_err(|e| e.into_response())?,
        )
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<InventoryModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    InventoryService::delete(&claims, &payload, inventory_module.inventory_repo.clone())
        .await
        .map_err(|e| e.into_response())?;
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(SimpleMessageResponse::new(
            "A leltár törlése sikeresen megtörtént",
        ))
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<InventoryModule>>,
    UserInput(user_input, _): UserInput<InventoryUserInput, InventoryUserInputHelper>,
) -> HandlerResult {
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::CREATED)
        .data(
            InventoryService::create(&claims, &user_input, inventory_module)
                .await
                .map_err(|e| e.into_response())?,
        )
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<InventoryModule>>,
    Query(payload): Query<QueryParam>,
) -> HandlerResult {
    let (meta, data) = InventoryService::get_paged_list(
        &PaginatorParams::try_from(&payload).unwrap_or(PaginatorParams::default()),
        &OrderingParams::try_from(&payload).unwrap_or(OrderingParams {
            order_by: ValueObject::new(InventoryOrderBy("product_id".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
            order: ValueObject::new(Order("asc".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
        }),
        &FilteringParams::from(&payload),
        &claims,
        inventory_module.inventory_repo.clone(),
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

pub async fn select_list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<InventoryModule>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(
            InventoryService::get_select_list_items(&list_type, &claims, inventory_module)
                .await
                .map_err(|e| e.into_response())?,
        )
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}
