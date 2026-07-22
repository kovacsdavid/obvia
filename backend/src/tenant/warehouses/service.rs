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
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use mockall_double::double;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
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

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
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
    fn insert(
        &self,
        payload: &WarehouseUserInput,
    ) -> impl Future<Output = WarehousesServiceResult<Warehouse>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = WarehousesServiceResult<WarehouseResolved>> + Send;
    fn get(&self, payload: Uuid)
    -> impl Future<Output = WarehousesServiceResult<Warehouse>> + Send;
    fn update(
        &self,
        payload: &WarehouseUserInput,
    ) -> impl Future<Output = WarehousesServiceResult<Warehouse>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = WarehousesServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<WarehouseOrderBy, WarehouseFilterBy>,
    ) -> impl Future<Output = WarehousesServiceResult<(PaginatorMeta, Vec<WarehouseResolved>)>> + Send;
    fn print(
        &self,
        payload: &[WarehouseResolvedPrint],
    ) -> impl Future<Output = WarehousesServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(
        &self,
        path: &Path,
    ) -> impl Future<Output = WarehousesServiceResult<()>> + Sync;
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
    async fn print(&self, payload: &[WarehouseResolvedPrint]) -> WarehousesServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::WarehouseView,
            payload.to_vec(),
        )?)
    }
    async fn print_snapshot(&self, path: &Path) -> WarehousesServiceResult<()> {
        let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z"
            .parse()
            .map_err(|e: chrono::ParseError| WarehousesServiceError::ParseError(e.to_string()))?;
        let tz: Tz = "Europe/Budapest"
            .parse()
            .map_err(|e: chrono_tz::ParseError| {
                WarehousesServiceError::ParseError(e.to_string())
            })?;
        let warehouse_id = "4f321721-37c6-4e91-8e42-6281c36937bc"
            .parse()
            .map_err(|e: uuid::Error| WarehousesServiceError::ParseError(e.to_string()))?;
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0"
            .parse()
            .map_err(|e: uuid::Error| WarehousesServiceError::ParseError(e.to_string()))?;
        let warehouse_resolved = WarehouseResolved {
            id: warehouse_id,
            name: "Test Warehouse".to_string(),
            contact_name: Some("Test Contact".to_string()),
            contact_phone: Some("+36301234567".to_string()),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
        };
        let warehouse_resolved_print =
            WarehouseResolvedPrint::from_warehouse_resolved(warehouse_resolved, tz);
        let pdf = self.print(&[warehouse_resolved_print]).await?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
