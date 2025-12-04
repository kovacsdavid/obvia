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
use crate::tenant::inventory_movements::InventoryMovementsModule;
use crate::tenant::inventory_movements::dto::{
    InventoryMovementUserInput, InventoryMovementUserInputHelper,
};
use crate::tenant::inventory_movements::service::InventoryMovementsService;
use crate::tenant::inventory_movements::types::InventoryMovementOrderBy;
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

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
    Query(payload): Query<QueryParam>,
) -> HandlerResult {
    let inventory_id = payload
        .as_hash_map()
        .and_then(|m| m.get("inventory_id").cloned())
        .and_then(|s| Uuid::parse_str(&s).ok())
        .ok_or_else(|| {
            FriendlyError::internal(
                file!(),
                "Hiányzó vagy hibás raktárkészlet azonosító".to_string(),
            )
            .into_response()
        })?;

    let (meta, data) = match InventoryMovementsService::get_paged_list(
        &PaginatorParams::try_from(&payload).unwrap_or(PaginatorParams::default()),
        &OrderingParams::try_from(&payload).unwrap_or(OrderingParams {
            order_by: ValueObject::new(InventoryMovementOrderBy("movement_date".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
            order: ValueObject::new(Order("desc".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
        }),
        &FilteringParams::from(&payload),
        &claims,
        inventory_movements_module.inventory_movements_repo(),
        inventory_id,
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
