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

use crate::common::BaseModule;
use crate::common::dto::{GeneralError, PaginatorMeta};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::pdf::{PdfGenError, PdfTemplates, gen_pdf_temporary};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::warehouses::WarehousesModule;
use crate::tenant::warehouses::dto::WarehouseUserInput;
use crate::tenant::warehouses::model::{Warehouse, WarehouseResolved};
use crate::tenant::warehouses::repository::WarehousesRepository;
use crate::tenant::warehouses::types::warehouse::{WarehouseFilterBy, WarehouseOrderBy};
use axum::body::Bytes;
use axum::http::StatusCode;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum WarehousesServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),
}

impl From<ServiceError> for WarehousesServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => WarehousesServiceError::Unauthorized,
        }
    }
}

impl IntoFriendlyError for WarehousesServiceError {
    async fn into_friendly_error<M>(self, module: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            WarehousesServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: WarehousesServiceError::Unauthorized.to_string(),
                }
                .to_string(),
            ),
            e => {
                FriendlyError::internal_with_admin_notify(
                    file!(),
                    GeneralError {
                        message: e.to_string(),
                    }
                    .to_string(),
                    module,
                )
                .await
            }
        }
    }
}

pub type WarehousesServiceResult<T> = Result<T, WarehousesServiceError>;

pub trait WarehouseService {
    async fn insert(&self, payload: &WarehouseUserInput) -> WarehousesServiceResult<Warehouse>;
    async fn get_resolved(&self, payload: Uuid) -> WarehousesServiceResult<WarehouseResolved>;
    async fn get(&self, payload: Uuid) -> WarehousesServiceResult<Warehouse>;
    async fn update(&self, payload: &WarehouseUserInput) -> WarehousesServiceResult<Warehouse>;
    async fn delete(&self, payload: Uuid) -> WarehousesServiceResult<()>;
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<WarehouseOrderBy, WarehouseFilterBy>,
    ) -> WarehousesServiceResult<(PaginatorMeta, Vec<WarehouseResolved>)>;
    async fn print(&self, payload: &[WarehouseResolved]) -> WarehousesServiceResult<Bytes>;
}

impl<'a, T> WarehouseService for Service<'a, T>
where
    T: WarehousesModule,
{
    async fn insert(&self, payload: &WarehouseUserInput) -> WarehousesServiceResult<Warehouse> {
        Ok(WarehousesRepository::insert(
            self.module(),
            payload.clone(),
            self.claims()?.sub(),
            self.claims()?
                .active_tenant()
                .ok_or(WarehousesServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn get_resolved(&self, payload: Uuid) -> WarehousesServiceResult<WarehouseResolved> {
        Ok(WarehousesRepository::get_resolved_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(WarehousesServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn get(&self, payload: Uuid) -> WarehousesServiceResult<Warehouse> {
        Ok(WarehousesRepository::get_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(WarehousesServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn update(&self, payload: &WarehouseUserInput) -> WarehousesServiceResult<Warehouse> {
        Ok(WarehousesRepository::update(
            self.module(),
            payload.clone(),
            self.claims()?
                .active_tenant()
                .ok_or(WarehousesServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn delete(&self, payload: Uuid) -> WarehousesServiceResult<()> {
        Ok(WarehousesRepository::delete_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(WarehousesServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<WarehouseOrderBy, WarehouseFilterBy>,
    ) -> WarehousesServiceResult<(PaginatorMeta, Vec<WarehouseResolved>)> {
        Ok(WarehousesRepository::get_all_paged(
            self.module(),
            get_query,
            self.claims()?
                .active_tenant()
                .ok_or(WarehousesServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn print(&self, payload: &[WarehouseResolved]) -> WarehousesServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::WarehouseView,
            &payload,
        )?))
    }
}
