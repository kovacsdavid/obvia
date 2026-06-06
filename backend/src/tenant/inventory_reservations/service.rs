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
use crate::tenant::inventory::repository::InventoryRepository;
use crate::tenant::inventory_reservations::InventoryReservationsModule;
use crate::tenant::inventory_reservations::dto::InventoryReservationUserInput;
use crate::tenant::inventory_reservations::model::{
    InventoryReservation, InventoryReservationResolved,
};
use crate::tenant::inventory_reservations::repository::InventoryReservationsRepository;
use crate::tenant::inventory_reservations::types::{
    InventoryReservationFilterBy, InventoryReservationOrderBy,
};
use crate::tenant::worksheets::repository::WorksheetsRepository;
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
        payload: &[InventoryReservationResolved],
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
        Ok(InventoryReservationsRepository::insert(
            self.module(),
            payload.clone(),
            self.claims()?.sub(),
            self.claims()?
                .active_tenant()
                .ok_or(InventoryReservationsServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn get(&self, payload: Uuid) -> InventoryReservationsServiceResult<InventoryReservation> {
        Ok(InventoryReservationsRepository::get_by_id(
            self.module(),
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
        Ok(InventoryReservationsRepository::get_resolved_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(InventoryReservationsServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn delete(&self, payload: Uuid) -> InventoryReservationsServiceResult<()> {
        Ok(InventoryReservationsRepository::delete_by_id(
            self.module(),
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
        Ok(InventoryReservationsRepository::get_all_paged(
            self.module(),
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
                    WorksheetsRepository::get_select_list_items(self.module(), active_tenant)
                        .await?
                }
                InventoryReservationsSelectLists::Inventory => {
                    InventoryRepository::get_select_list_items(self.module(), active_tenant).await?
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
