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
use crate::common::model::SelectOption;
#[double]
use crate::common::pdf::PdfGenerator;
use crate::common::pdf::{PdfGenError, PdfTemplates};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::inventory_reservations::InventoryReservationsModuleInterface;
use crate::tenant::inventory_reservations::dto::print::InventoryReservationResolvedPrint;
use crate::tenant::inventory_reservations::dto::user_input::InventoryReservationUserInput;
use crate::tenant::inventory_reservations::model::{
    InventoryReservation, InventoryReservationResolved,
};
use crate::tenant::inventory_reservations::types::{
    InventoryReservationFilterBy, InventoryReservationOrderBy,
};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
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
pub enum InventoryReservationsServiceError {
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

impl From<ServiceError> for InventoryReservationsServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => InventoryReservationsServiceError::Unauthorized,
        }
    }
}

impl From<InventoryReservationsServiceError> for AppError {
    fn from(value: InventoryReservationsServiceError) -> Self {
        match value {
            InventoryReservationsServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            InventoryReservationsServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            InventoryReservationsServiceError::Repository(RepositoryError::Database(
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

pub type InventoryReservationsServiceResult<T> = Result<T, InventoryReservationsServiceError>;

pub enum InventoryReservationsSelectLists {
    Worksheets,
    Inventory,
}

impl FromStr for InventoryReservationsSelectLists {
    type Err = InventoryReservationsServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "worksheets" => Ok(Self::Worksheets),
            "inventory" => Ok(Self::Inventory),
            _ => Err(InventoryReservationsServiceError::InvalidSelectList),
        }
    }
}

pub trait InventoryReservationService {
    fn insert(
        &self,
        payload: &InventoryReservationUserInput,
    ) -> impl Future<Output = InventoryReservationsServiceResult<InventoryReservation>> + Send;
    fn update(
        &self,
        payload: &InventoryReservationUserInput,
    ) -> impl Future<Output = InventoryReservationsServiceResult<InventoryReservation>> + Send;
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = InventoryReservationsServiceResult<Vec<SelectOption>>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = InventoryReservationsServiceResult<InventoryReservationResolved>> + Send;
    fn get(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = InventoryReservationsServiceResult<InventoryReservation>> + Send;
    fn delete(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = InventoryReservationsServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryReservationOrderBy, InventoryReservationFilterBy>,
        inventory_id: Uuid,
    ) -> impl Future<
        Output = InventoryReservationsServiceResult<(
            PaginatorMeta,
            Vec<InventoryReservationResolved>,
        )>,
    > + Send;
    fn print(
        &self,
        payload: &[InventoryReservationResolvedPrint],
    ) -> impl Future<Output = InventoryReservationsServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(
        &self,
        path: &Path,
    ) -> impl Future<Output = InventoryReservationsServiceResult<()>> + Sync;
}

impl<'a, T> InventoryReservationService for Service<'a, T>
where
    T: InventoryReservationsModuleInterface,
{
    async fn insert(
        &self,
        payload: &InventoryReservationUserInput,
    ) -> InventoryReservationsServiceResult<InventoryReservation> {
        Ok(self
            .module()
            .inventory_reservations_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )?
            .insert(payload.clone(), self.claims()?.sub())
            .await?)
    }
    async fn update(
        &self,
        payload: &InventoryReservationUserInput,
    ) -> InventoryReservationsServiceResult<InventoryReservation> {
        if !payload.id.is_present() {
            return Err(InventoryReservationsServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .inventory_reservations_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )?
            .update(payload)
            .await?)
    }
    async fn get(&self, payload: Uuid) -> InventoryReservationsServiceResult<InventoryReservation> {
        Ok(self
            .module()
            .inventory_reservations_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }

    async fn get_resolved(
        &self,
        payload: Uuid,
    ) -> InventoryReservationsServiceResult<InventoryReservationResolved> {
        Ok(self
            .module()
            .inventory_reservations_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }

    async fn delete(&self, payload: Uuid) -> InventoryReservationsServiceResult<()> {
        Ok(self
            .module()
            .inventory_reservations_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryReservationOrderBy, InventoryReservationFilterBy>,
        inventory_id: Uuid,
    ) -> InventoryReservationsServiceResult<(PaginatorMeta, Vec<InventoryReservationResolved>)>
    {
        Ok(self
            .module()
            .inventory_reservations_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )?
            .get_paged(get_query, inventory_id)
            .await?)
    }

    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> InventoryReservationsServiceResult<Vec<SelectOption>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(InventoryReservationsServiceError::Unauthorized)?;
        Ok(
            match InventoryReservationsSelectLists::from_str(select_list)? {
                InventoryReservationsSelectLists::Worksheets => {
                    self.module()
                        .worksheets_repo(active_tenant)?
                        .get_select_list_items()
                        .await?
                }
                InventoryReservationsSelectLists::Inventory => {
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
        payload: &[InventoryReservationResolvedPrint],
    ) -> InventoryReservationsServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::InventoryReservationView,
            payload.to_vec(),
        )?)
    }
    async fn print_snapshot(&self, path: &Path) -> InventoryReservationsServiceResult<()> {
        let test_time: DateTime<Utc> =
            "2026-01-02T11:11:11Z"
                .parse()
                .map_err(|e: chrono::ParseError| {
                    InventoryReservationsServiceError::ParseError(e.to_string())
                })?;
        let tz: Tz = "Europe/Budapest"
            .parse()
            .map_err(|e: chrono_tz::ParseError| {
                InventoryReservationsServiceError::ParseError(e.to_string())
            })?;
        let inventory_reservation_id =
            "4f321721-37c6-4e91-8e42-6281c36937bc"
                .parse()
                .map_err(|e: uuid::Error| {
                    InventoryReservationsServiceError::ParseError(e.to_string())
                })?;
        let inventory_id =
            "ac55ca9c-2cd1-4cdf-8b44-ed4df798c750"
                .parse()
                .map_err(|e: uuid::Error| {
                    InventoryReservationsServiceError::ParseError(e.to_string())
                })?;
        let created_by_id =
            "97054cdb-781c-4f40-a489-b43373d75bf0"
                .parse()
                .map_err(|e: uuid::Error| {
                    InventoryReservationsServiceError::ParseError(e.to_string())
                })?;
        let reference_id =
            "fd48ade1-a817-431b-8ada-6faea8c9f9dd"
                .parse()
                .map_err(|e: uuid::Error| {
                    InventoryReservationsServiceError::ParseError(e.to_string())
                })?;
        let inventory_reservation_resolved = InventoryReservationResolved {
            id: inventory_reservation_id,
            inventory_id,
            quantity: "10".parse().unwrap(),
            reference_type: Some("worksheets".to_string()),
            reference_id: Some(reference_id),
            reserved_until: None,
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: test_time,
            updated_at: test_time,
        };
        let inventory_reservation_resolved_print =
            InventoryReservationResolvedPrint::from_inventory_reservation_resolved(
                inventory_reservation_resolved,
                tz,
            );
        let pdf = self.print(&[inventory_reservation_resolved_print]).await?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
