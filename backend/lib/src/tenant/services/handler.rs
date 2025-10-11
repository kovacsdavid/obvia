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
use crate::common::types::order::Order;
use crate::common::types::value_object::ValueObject;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::services::ServicesModule;
use crate::tenant::services::dto::{ServiceUserInput, ServiceUserInputHelper};
use crate::tenant::services::service::ServicesService;
use crate::tenant::services::types::service::ServiceOrderBy;
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

#[debug_handler]
pub async fn get_resolved(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(services_module): State<Arc<ServicesModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(
            ServicesService::get_resolved_by_id(
                &claims,
                &payload,
                services_module.services_repo.clone(),
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
    State(services_module): State<Arc<ServicesModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(
            ServicesService::get(&claims, &payload, services_module.services_repo.clone())
                .await
                .map_err(|e| e.into_response())?,
        )
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(services_module): State<Arc<ServicesModule>>,
    UserInput(user_input, _): UserInput<ServiceUserInput, ServiceUserInputHelper>,
) -> HandlerResult {
    ServicesService::create(&claims, &user_input, services_module.services_repo.clone())
        .await
        .map_err(|e| e.into_response())?;
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::CREATED)
        .data(SimpleMessageResponse::new(
            "A szolgáltatás létrehozása sikeresen megtörtént",
        ))
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

#[debug_handler]
pub async fn update(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(services_module): State<Arc<ServicesModule>>,
    UserInput(user_input, _): UserInput<ServiceUserInput, ServiceUserInputHelper>,
) -> HandlerResult {
    ServicesService::update(&claims, &user_input, services_module.services_repo.clone())
        .await
        .map_err(|e| e.into_response())?;
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(SimpleMessageResponse::new(
            "A szolgáltatás frissítése sikeresen megtörtént",
        ))
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(services_module): State<Arc<ServicesModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    ServicesService::delete(&claims, &payload, services_module.services_repo.clone())
        .await
        .map_err(|e| e.into_response())?;
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(SimpleMessageResponse::new(
            "A szolgáltatás törlése sikeresen megtörtént",
        ))
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(services_module): State<Arc<ServicesModule>>,
    Query(payload): Query<QueryParam>,
) -> HandlerResult {
    let (meta, data) = ServicesService::get_paged_list(
        &PaginatorParams::try_from(&payload).unwrap_or(PaginatorParams::default()),
        &OrderingParams::try_from(&payload).unwrap_or(OrderingParams {
            order_by: ValueObject::new(ServiceOrderBy("name".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
            order: ValueObject::new(Order("asc".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
        }),
        &FilteringParams::from(&payload),
        &claims,
        services_module.services_repo.clone(),
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
