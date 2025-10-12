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

use crate::common::dto::{GeneralError, OrderingParams, PaginatorMeta, PaginatorParams, UuidParam};
use crate::common::error::{FriendlyError, RepositoryError};
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::warehouses::WarehousesModule;
use crate::tenant::warehouses::dto::WarehouseUserInput;
use crate::tenant::warehouses::model::{Warehouse, WarehouseResolved};
use crate::tenant::warehouses::repository::WarehousesRepository;
use crate::tenant::warehouses::types::warehouse::WarehouseOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum WarehousesServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,
}

impl IntoResponse for WarehousesServiceError {
    fn into_response(self) -> Response {
        match self {
            WarehousesServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: WarehousesServiceError::Unauthorized.to_string(),
                },
            )
            .into_response(),
            e => FriendlyError::internal(file!(), e.to_string()).into_response(),
        }
    }
}

pub type WarehousesServiceResult<T> = Result<T, WarehousesServiceError>;

pub struct WarehousesService;

impl WarehousesService {
    pub async fn try_create(
        claims: &Claims,
        payload: &WarehouseUserInput,
        warehouses_module: Arc<WarehousesModule>,
    ) -> WarehousesServiceResult<()> {
        warehouses_module
            .warehouses_repo
            .insert(
                payload.clone(),
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )
            .await?;
        Ok(())
    }
    pub async fn get_resolved_by_id(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn WarehousesRepository>,
    ) -> WarehousesServiceResult<WarehouseResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn WarehousesRepository>,
    ) -> WarehousesServiceResult<Warehouse> {
        Ok(repo
            .get_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn update(
        claims: &Claims,
        payload: &WarehouseUserInput,
        repo: Arc<dyn WarehousesRepository>,
    ) -> WarehousesServiceResult<Warehouse> {
        Ok(repo
            .update(
                payload.clone(),
                claims
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn delete(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn WarehousesRepository>,
    ) -> WarehousesServiceResult<()> {
        Ok(repo
            .delete_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<WarehouseOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn WarehousesRepository>,
    ) -> WarehousesServiceResult<(PaginatorMeta, Vec<WarehouseResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )
            .await?)
    }
}
