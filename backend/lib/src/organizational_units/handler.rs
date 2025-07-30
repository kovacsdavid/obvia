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

use crate::app::database::PgPoolManagerTrait;
use crate::auth::dto::claims::Claims;
use crate::auth::middleware::AuthenticatedUser;
use crate::common::error::FriendlyError;
use crate::common::repository::PoolWrapper;
use crate::organizational_units::OrganizationalUnitsModule;
use crate::organizational_units::dto::{CreateRequest, CreateRequestHelper};
use crate::organizational_units::repository::OrganizationalUnitsRepository;
use crate::organizational_units::service::try_create;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use std::sync::Arc;
use tracing::Level;

pub async fn create_inner<F, Fut>(
    claims: Claims,
    organizational_units_module: Arc<OrganizationalUnitsModule>,
    payload: Result<Json<CreateRequestHelper>, JsonRejection>,
    repo_factory: F,
) -> Response
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = Box<dyn OrganizationalUnitsRepository + Send + Sync>>,
{
    match payload {
        Ok(Json(payload)) => match CreateRequest::try_from(payload) {
            Ok(user_input) => {
                let mut repo = repo_factory().await;
                match try_create(&mut *repo, claims, user_input, organizational_units_module).await
                {
                    Ok(resp) => (StatusCode::CREATED, Json(resp)).into_response(),
                    Err(e) => e.into_response(),
                }
            }
            Err(e) => e.into_response(),
        },
        Err(_) => FriendlyError::UserFacing(
            StatusCode::BAD_REQUEST,
            "ORGANIZATIONAL_UNITS/HANDLER/CREATE".to_string(),
            "Invalid JSON".to_string(),
        )
        .trace(Level::DEBUG)
        .into_response(),
    }
}

pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(organizational_unit_module): State<Arc<OrganizationalUnitsModule>>,
    payload: Result<Json<CreateRequestHelper>, JsonRejection>,
) -> Response {
    create_inner(
        claims,
        organizational_unit_module.clone(),
        payload,
        || async {
            Box::new(PoolWrapper::new(
                organizational_unit_module.db_pools.get_base_tenant_pool(),
            )) as Box<dyn OrganizationalUnitsRepository + Send + Sync>
        },
    )
    .await
}

pub async fn get(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(_organizational_units_module): State<Arc<OrganizationalUnitsModule>>,
) -> Response {
    FriendlyError::UserFacing(
        StatusCode::INTERNAL_SERVER_ERROR,
        "MANAGED_COMPANIES/HANDLER/CREATE".to_string(),
        "Not implemented yet!".to_string(),
    )
    .trace(Level::DEBUG)
    .into_response()
}

pub async fn list(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(_organizational_units_module): State<Arc<OrganizationalUnitsModule>>,
) -> Response {
    FriendlyError::UserFacing(
        StatusCode::INTERNAL_SERVER_ERROR,
        "MANAGED_COMPANIES/HANDLER/CREATE".to_string(),
        "Not implemented yet!".to_string(),
    )
    .trace(Level::DEBUG)
    .into_response()
}
