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

use crate::common::dto::{OrderingParams, PaginatorMeta, PaginatorParams, UuidParam};
use crate::common::error::{FriendlyError, RepositoryError};
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::inventory_movements::dto::InventoryMovementUserInput;
use crate::tenant::inventory_movements::model::{InventoryMovement, InventoryMovementResolved};
use crate::tenant::inventory_movements::repository::InventoryMovementsRepository;
use crate::tenant::inventory_movements::types::InventoryMovementOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum InventoryMovementsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,
}

impl IntoResponse for InventoryMovementsServiceError {
    fn into_response(self) -> Response {
        match self {
            InventoryMovementsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                crate::common::dto::GeneralError {
                    message: self.to_string(),
                },
            )
            .into_response(),
            e => FriendlyError::internal(file!(), e.to_string()).into_response(),
        }
    }
}

pub type InventoryMovementsServiceResult<T> = Result<T, InventoryMovementsServiceError>;

pub struct InventoryMovementsService;

impl InventoryMovementsService {
    pub async fn create(
        claims: &Claims,
        payload: &InventoryMovementUserInput,
        repo: Arc<dyn InventoryMovementsRepository>,
    ) -> InventoryMovementsServiceResult<InventoryMovement> {
        Ok(repo
            .insert(
                payload.clone(),
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn get(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryMovementsRepository>,
    ) -> InventoryMovementsServiceResult<InventoryMovement> {
        Ok(repo
            .get_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn get_resolved(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryMovementsRepository>,
    ) -> InventoryMovementsServiceResult<InventoryMovementResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn delete(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryMovementsRepository>,
    ) -> InventoryMovementsServiceResult<()> {
        Ok(repo
            .delete_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<InventoryMovementOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn InventoryMovementsRepository>,
        inventory_id: Uuid,
    ) -> InventoryMovementsServiceResult<(PaginatorMeta, Vec<InventoryMovementResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
                inventory_id,
            )
            .await?)
    }
}
