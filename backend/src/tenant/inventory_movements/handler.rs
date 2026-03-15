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
    EmptyType, HandlerResult, SimpleMessageResponse, SuccessResponseBuilder, UuidParam,
};
use crate::common::error::FriendlyError;
use crate::common::error::IntoFriendlyError;
use crate::common::extractors::UserInput;
use crate::common::query_parser::GetQuery;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::inventory_movements::InventoryMovementsModule;
use crate::tenant::inventory_movements::dto::{
    InventoryMovementUserInput, InventoryMovementUserInputHelper, InventoryMovementsRawQuery,
};
use crate::tenant::inventory_movements::service::InventoryMovementsService;
use crate::tenant::inventory_movements::types::{
    InventoryMovementFilterBy, InventoryMovementOrderBy,
};
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

#[debug_handler]
pub async fn get(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<dyn InventoryMovementsModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let result = match InventoryMovementsService::get(
        &claims,
        &payload,
        inventory_movements_module.inventory_movements_repo(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(e
                .into_friendly_error(inventory_movements_module)
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
            .into_friendly_error(inventory_movements_module)
            .await
            .into_response()),
    }
}

#[debug_handler]
pub async fn get_resolved(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<dyn InventoryMovementsModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let result = match InventoryMovementsService::get_resolved(
        &claims,
        &payload,
        inventory_movements_module.inventory_movements_repo(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(e
                .into_friendly_error(inventory_movements_module)
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
            .into_friendly_error(inventory_movements_module)
            .await
            .into_response()),
    }
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<dyn InventoryMovementsModule>>,
    UserInput(user_input, _): UserInput<
        InventoryMovementUserInput,
        InventoryMovementUserInputHelper,
    >,
) -> HandlerResult {
    let result = match InventoryMovementsService::create(
        &claims,
        &user_input,
        inventory_movements_module.inventory_movements_repo(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(e
                .into_friendly_error(inventory_movements_module)
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
            .into_friendly_error(inventory_movements_module)
            .await
            .into_response()),
    }
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<dyn InventoryMovementsModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    match InventoryMovementsService::delete(
        &claims,
        &payload,
        inventory_movements_module.inventory_movements_repo(),
    )
    .await
    {
        Ok(_) => (),
        Err(e) => {
            return Err(e
                .into_friendly_error(inventory_movements_module)
                .await
                .into_response());
        }
    };

    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(SimpleMessageResponse::new(
            "A készletmozgás törlése sikeresen megtörtént",
        ))
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e
            .into_friendly_error(inventory_movements_module)
            .await
            .into_response()),
    }
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<dyn InventoryMovementsModule>>,
    Query(payload): Query<InventoryMovementsRawQuery>,
) -> HandlerResult {
    let (meta, data) = match InventoryMovementsService::get_paged_list(
        &GetQuery::<InventoryMovementOrderBy, InventoryMovementFilterBy>::from_str(payload.q())
            .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
        &claims,
        inventory_movements_module.inventory_movements_repo(),
        payload.inventory_id(),
    )
    .await
    {
        Ok((m, d)) => (m, d),
        Err(e) => {
            return Err(e
                .into_friendly_error(inventory_movements_module)
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
            .into_friendly_error(inventory_movements_module)
            .await
            .into_response()),
    }
}

pub async fn select_list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<dyn InventoryMovementsModule>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));

    let result = match InventoryMovementsService::get_select_list_items(
        &list_type,
        &claims,
        inventory_movements_module.clone(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(e
                .into_friendly_error(inventory_movements_module)
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
            .into_friendly_error(inventory_movements_module)
            .await
            .into_response()),
    }
}
