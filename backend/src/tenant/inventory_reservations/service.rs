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

use crate::common::MailTransporter;
use crate::common::dto::{GeneralError, PaginatorMeta};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::common::pdf::{PdfGenError, PdfTemplates, gen_pdf_temporary};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::inventory_reservations::InventoryReservationsModule;
use crate::tenant::inventory_reservations::dto::InventoryReservationUserInput;
use crate::tenant::inventory_reservations::model::{
    InventoryReservation, InventoryReservationResolved,
};
use crate::tenant::inventory_reservations::types::{
    InventoryReservationFilterBy, InventoryReservationOrderBy,
};
use axum::body::Bytes;
use axum::http::StatusCode;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum InventoryReservationsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

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

impl<H> IntoFriendlyError<GeneralError, H> for InventoryReservationsServiceError
where
    H: MailTransporter + ?Sized,
{
    async fn into_friendly_error(self, module: Arc<H>) -> FriendlyError<GeneralError> {
        match self {
            InventoryReservationsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: self.to_string(),
                },
            ),
            e => {
                FriendlyError::internal_with_admin_notify(
                    file!(),
                    GeneralError {
                        message: e.to_string(),
                    },
                    module,
                )
                .await
            }
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
    async fn insert(
        &self,
        payload: &InventoryReservationUserInput,
    ) -> InventoryReservationsServiceResult<InventoryReservation>;
    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> InventoryReservationsServiceResult<Vec<SelectOption>>;
    async fn get_resolved(
        &self,
        payload: Uuid,
    ) -> InventoryReservationsServiceResult<InventoryReservationResolved>;
    async fn get(&self, payload: Uuid) -> InventoryReservationsServiceResult<InventoryReservation>;
    async fn delete(&self, payload: Uuid) -> InventoryReservationsServiceResult<()>;
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryReservationOrderBy, InventoryReservationFilterBy>,
        inventory_id: Uuid,
    ) -> InventoryReservationsServiceResult<(PaginatorMeta, Vec<InventoryReservationResolved>)>;
    async fn print(
        &self,
        payload: &[InventoryReservationResolved],
    ) -> InventoryReservationsServiceResult<Bytes>;
}

impl<'a, T> InventoryReservationService for Service<'a, T>
where
    T: InventoryReservationsModule + ?Sized,
{
    async fn insert(
        &self,
        payload: &InventoryReservationUserInput,
    ) -> InventoryReservationsServiceResult<InventoryReservation> {
        Ok(self
            .module()
            .inventory_reservations_repo()
            .insert(
                payload.clone(),
                self.claims()?.sub(),
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn get(&self, payload: Uuid) -> InventoryReservationsServiceResult<InventoryReservation> {
        Ok(self
            .module()
            .inventory_reservations_repo()
            .get_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn get_resolved(
        &self,
        payload: Uuid,
    ) -> InventoryReservationsServiceResult<InventoryReservationResolved> {
        Ok(self
            .module()
            .inventory_reservations_repo()
            .get_resolved_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn delete(&self, payload: Uuid) -> InventoryReservationsServiceResult<()> {
        Ok(self
            .module()
            .inventory_reservations_repo()
            .delete_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )
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
            .inventory_reservations_repo()
            .get_all_paged(
                get_query,
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
                inventory_id,
            )
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
                        .worksheets_repo()
                        .get_select_list_items(active_tenant)
                        .await?
                }
                InventoryReservationsSelectLists::Inventory => {
                    self.module()
                        .inventory_repo()
                        .get_select_list_items(active_tenant)
                        .await?
                }
            },
        )
    }

    async fn print(
        &self,
        payload: &[InventoryReservationResolved],
    ) -> InventoryReservationsServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::InventoryReservationView,
            &payload,
        )?))
    }
}
