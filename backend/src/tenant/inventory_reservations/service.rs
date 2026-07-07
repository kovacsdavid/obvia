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
use axum::body::Bytes;
use axum::http::StatusCode;
use mockall_double::double;
use serde_json::json;
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
    ) -> impl Future<Output = InventoryReservationsServiceResult<Bytes>> + Send;
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
    ) -> InventoryReservationsServiceResult<Bytes> {
        Ok(Bytes::from(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::InventoryReservationView,
            payload.to_vec(),
        )?))
    }
}
