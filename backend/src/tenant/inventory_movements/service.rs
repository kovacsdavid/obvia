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
use crate::common::error::v2::AppError;
use crate::common::error::v2::AppErrorVisibility;
use crate::common::model::SelectOption;
#[double]
use crate::common::pdf::PdfGenerator;
use crate::common::pdf::{PdfGenError, PdfTemplates};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::inventory_movements::InventoryMovementsModuleInterface;
use crate::tenant::inventory_movements::dto::print::InventoryMovementsResolvedPrint;
use crate::tenant::inventory_movements::dto::user_input::InventoryMovementUserInput;
use crate::tenant::inventory_movements::model::{InventoryMovement, InventoryMovementResolved};
use crate::tenant::inventory_movements::types::{
    InventoryMovementFilterBy, InventoryMovementOrderBy,
};
use axum::http::StatusCode;
use chrono::DateTime;
use chrono::Utc;
use chrono_tz::Tz;
use mockall_double::double;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum InventoryMovementsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

    #[error("A lista nem létezik")]
    InvalidSelectList,

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

impl From<ServiceError> for InventoryMovementsServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => InventoryMovementsServiceError::Unauthorized,
        }
    }
}

impl From<InventoryMovementsServiceError> for AppError {
    fn from(value: InventoryMovementsServiceError) -> Self {
        match value {
            InventoryMovementsServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            InventoryMovementsServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            InventoryMovementsServiceError::Repository(RepositoryError::Database(
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

pub type InventoryMovementsServiceResult<T> = Result<T, InventoryMovementsServiceError>;

pub enum InventoryMovementsSelectLists {
    Worksheets,
    Taxes,
    Inventory,
}

impl FromStr for InventoryMovementsSelectLists {
    type Err = InventoryMovementsServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "worksheets" => Ok(Self::Worksheets),
            "taxes" => Ok(Self::Taxes),
            "inventory" => Ok(Self::Inventory),
            _ => Err(InventoryMovementsServiceError::InvalidSelectList),
        }
    }
}

pub trait InventoryMovementService {
    fn insert(
        &self,
        payload: &InventoryMovementUserInput,
    ) -> impl Future<Output = InventoryMovementsServiceResult<InventoryMovement>> + Send;
    fn update(
        &self,
        payload: &InventoryMovementUserInput,
    ) -> impl Future<Output = InventoryMovementsServiceResult<InventoryMovement>> + Send;
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = InventoryMovementsServiceResult<Vec<SelectOption>>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = InventoryMovementsServiceResult<InventoryMovementResolved>> + Send;
    fn get(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = InventoryMovementsServiceResult<InventoryMovement>> + Send;
    fn delete(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = InventoryMovementsServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryMovementOrderBy, InventoryMovementFilterBy>,
        inventory_id: Uuid,
    ) -> impl Future<
        Output = InventoryMovementsServiceResult<(PaginatorMeta, Vec<InventoryMovementResolved>)>,
    > + Send;
    fn print(
        &self,
        payload: &[InventoryMovementsResolvedPrint],
    ) -> impl Future<Output = InventoryMovementsServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(
        &self,
        path: &Path,
    ) -> impl Future<Output = InventoryMovementsServiceResult<()>> + Sync;
}

impl<'a, T> InventoryMovementService for Service<'a, T>
where
    T: InventoryMovementsModuleInterface,
{
    async fn insert(
        &self,
        payload: &InventoryMovementUserInput,
    ) -> InventoryMovementsServiceResult<InventoryMovement> {
        Ok(self
            .module()
            .inventory_movements_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )?
            .insert(payload, self.claims()?.sub())
            .await?)
    }
    async fn get(&self, payload: Uuid) -> InventoryMovementsServiceResult<InventoryMovement> {
        Ok(self
            .module()
            .inventory_movements_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }
    async fn update(
        &self,
        payload: &InventoryMovementUserInput,
    ) -> InventoryMovementsServiceResult<InventoryMovement> {
        if !payload.id.is_present() {
            return Err(InventoryMovementsServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .inventory_movements_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )?
            .update(payload)
            .await?)
    }
    async fn get_resolved(
        &self,
        payload: Uuid,
    ) -> InventoryMovementsServiceResult<InventoryMovementResolved> {
        Ok(self
            .module()
            .inventory_movements_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }

    async fn delete(&self, payload: Uuid) -> InventoryMovementsServiceResult<()> {
        Ok(self
            .module()
            .inventory_movements_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryMovementOrderBy, InventoryMovementFilterBy>,
        inventory_id: Uuid,
    ) -> InventoryMovementsServiceResult<(PaginatorMeta, Vec<InventoryMovementResolved>)> {
        Ok(self
            .module()
            .inventory_movements_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )?
            .get_paged(get_query, inventory_id)
            .await?)
    }
    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> InventoryMovementsServiceResult<Vec<SelectOption>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(InventoryMovementsServiceError::Unauthorized)?;
        Ok(
            match InventoryMovementsSelectLists::from_str(select_list)? {
                InventoryMovementsSelectLists::Worksheets => {
                    self.module()
                        .worksheets_repo(active_tenant)?
                        .get_select_list_items()
                        .await?
                }
                InventoryMovementsSelectLists::Taxes => {
                    self.module()
                        .taxes_repo(active_tenant)?
                        .get_select_list_items()
                        .await?
                }
                InventoryMovementsSelectLists::Inventory => {
                    self.module()
                        .inventory_repo(active_tenant)?
                        .get_select_list_items()
                        .await?
                }
            },
        )
    }

    async fn print(
        &self,
        payload: &[InventoryMovementsResolvedPrint],
    ) -> InventoryMovementsServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::InventoryMovementView,
            payload.to_vec(),
        )?)
    }
    async fn print_snapshot(&self, path: &Path) -> InventoryMovementsServiceResult<()> {
        let test_time: DateTime<Utc> =
            "2026-01-02T11:11:11Z"
                .parse()
                .map_err(|e: chrono::ParseError| {
                    InventoryMovementsServiceError::ParseError(e.to_string())
                })?;
        let tz: Tz = "Europe/Budapest"
            .parse()
            .map_err(|e: chrono_tz::ParseError| {
                InventoryMovementsServiceError::ParseError(e.to_string())
            })?;
        let inventory_movement_id = "4f321721-37c6-4e91-8e42-6281c36937bc"
            .parse()
            .map_err(|e: uuid::Error| InventoryMovementsServiceError::ParseError(e.to_string()))?;
        let inventory_id = "ac55ca9c-2cd1-4cdf-8b44-ed4df798c750"
            .parse()
            .map_err(|e: uuid::Error| InventoryMovementsServiceError::ParseError(e.to_string()))?;
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0"
            .parse()
            .map_err(|e: uuid::Error| InventoryMovementsServiceError::ParseError(e.to_string()))?;
        let reference_id = "fd48ade1-a817-431b-8ada-6faea8c9f9dd"
            .parse()
            .map_err(|e: uuid::Error| InventoryMovementsServiceError::ParseError(e.to_string()))?;
        let tax_id = "86097a0b-3f05-42f4-a98d-fd8a4669f02b"
            .parse()
            .map_err(|e: uuid::Error| InventoryMovementsServiceError::ParseError(e.to_string()))?;
        let inventory_movement_resolved = InventoryMovementResolved {
            id: inventory_movement_id,
            inventory_id,
            movement_type: "in".to_string(),
            quantity: "10".parse().unwrap(),
            reference_type: Some("worksheets".to_string()),
            reference_id: Some(reference_id),
            unit_price: Some("20".parse().unwrap()),
            total_price: Some("30".parse().unwrap()),
            tax_id,
            tax: Some("Test Tax".to_string()),
            movement_date: test_time,
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: test_time,
        };
        let inventory_movement_resolved_print =
            InventoryMovementsResolvedPrint::from_inventory_movements_resolved(
                inventory_movement_resolved,
                tz,
            );
        let pdf = self.print(&[inventory_movement_resolved_print]).await?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
