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

use crate::common::dto::PaginatorMeta;
use crate::common::error::RepositoryError;
use crate::common::error::v2::{AppError, AppErrorVisibility};
#[double]
use crate::common::pdf::PdfGenerator;
use crate::common::pdf::{PdfGenError, PdfTemplates};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::warehouses::WarehousesModuleInterface;
use crate::tenant::warehouses::dto::print::WarehouseResolvedPrint;
use crate::tenant::warehouses::dto::user_input::WarehouseUserInput;
use crate::tenant::warehouses::model::{Warehouse, WarehouseResolved};
use crate::tenant::warehouses::types::warehouse::{WarehouseFilterBy, WarehouseOrderBy};
use axum::body::Bytes;
use axum::http::StatusCode;
use mockall_double::double;
use serde_json::json;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum WarehousesServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

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

impl From<WarehousesServiceError> for AppError {
    fn from(value: WarehousesServiceError) -> Self {
        match value {
            WarehousesServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            WarehousesServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            WarehousesServiceError::Repository(RepositoryError::Database(
                sqlx::Error::RowNotFound,
            )) => Self::new(
                Level::DEBUG,
                StatusCode::NOT_FOUND,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": "Nem található"}),
            ),
            _ => Self::new(
                Level::ERROR,
                StatusCode::INTERNAL_SERVER_ERROR,
                file!(),
                AppErrorVisibility::Internal,
                json!({"message": value.to_string()}),
            ),
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
    async fn print(&self, payload: &[WarehouseResolvedPrint]) -> WarehousesServiceResult<Bytes>;
}

impl<'a, T> WarehouseService for Service<'a, T>
where
    T: WarehousesModuleInterface,
{
    async fn insert(&self, payload: &WarehouseUserInput) -> WarehousesServiceResult<Warehouse> {
        Ok(self
            .module()
            .warehouses_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )?
            .insert(payload.clone(), self.claims()?.sub())
            .await?)
    }
    async fn get_resolved(&self, payload: Uuid) -> WarehousesServiceResult<WarehouseResolved> {
        Ok(self
            .module()
            .warehouses_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }
    async fn get(&self, payload: Uuid) -> WarehousesServiceResult<Warehouse> {
        Ok(self
            .module()
            .warehouses_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }

    async fn update(&self, payload: &WarehouseUserInput) -> WarehousesServiceResult<Warehouse> {
        if !payload.id.is_present() {
            return Err(WarehousesServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .warehouses_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )?
            .update(payload.clone())
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> WarehousesServiceResult<()> {
        Ok(self
            .module()
            .warehouses_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<WarehouseOrderBy, WarehouseFilterBy>,
    ) -> WarehousesServiceResult<(PaginatorMeta, Vec<WarehouseResolved>)> {
        Ok(self
            .module()
            .warehouses_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )?
            .get_paged(get_query)
            .await?)
    }
    async fn print(&self, payload: &[WarehouseResolvedPrint]) -> WarehousesServiceResult<Bytes> {
        Ok(Bytes::from(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::WarehouseView,
            payload.to_vec(),
        )?))
    }
}
