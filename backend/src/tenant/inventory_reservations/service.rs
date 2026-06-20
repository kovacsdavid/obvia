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
use crate::common::model::SelectOption;
use crate::common::pdf::{PdfGenError, PdfTemplates, gen_pdf_temporary};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::inventory_reservations::InventoryReservationsModule;
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

impl IntoFriendlyError for InventoryReservationsServiceError {
    async fn into_friendly_error<M>(self, module: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            InventoryReservationsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: self.to_string(),
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
    T: InventoryReservationsModule,
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
            .get_all_paged(get_query, inventory_id)
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
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::InventoryReservationView,
            &payload,
        )?))
    }
}
