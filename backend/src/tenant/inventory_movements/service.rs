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
use crate::common::dto::GeneralError;
use crate::common::dto::PaginatorMeta;
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::common::pdf::{PdfGenError, PdfTemplates, gen_pdf_temporary};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::inventory::repository::InventoryRepository;
use crate::tenant::inventory_movements::InventoryMovementsModule;
use crate::tenant::inventory_movements::dto::InventoryMovementUserInput;
use crate::tenant::inventory_movements::model::{InventoryMovement, InventoryMovementResolved};
use crate::tenant::inventory_movements::repository::InventoryMovementsRepository;
use crate::tenant::inventory_movements::types::{
    InventoryMovementFilterBy, InventoryMovementOrderBy,
};
use crate::tenant::taxes::repository::TaxesRepository;
use crate::tenant::worksheets::repository::WorksheetsRepository;
use axum::body::Bytes;
use axum::http::StatusCode;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum InventoryMovementsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("A lista nem létezik")]
    InvalidSelectList,

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),
}

impl From<ServiceError> for InventoryMovementsServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => InventoryMovementsServiceError::Unauthorized,
        }
    }
}

impl IntoFriendlyError for InventoryMovementsServiceError {
    async fn into_friendly_error<M>(self, module: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            InventoryMovementsServiceError::Unauthorized => FriendlyError::user_facing(
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
        payload: &[InventoryMovementResolved],
    ) -> impl Future<Output = InventoryMovementsServiceResult<Bytes>> + Send;
}

impl<'a, T> InventoryMovementService for Service<'a, T>
where
    T: InventoryMovementsModule,
{
    async fn insert(
        &self,
        payload: &InventoryMovementUserInput,
    ) -> InventoryMovementsServiceResult<InventoryMovement> {
        Ok(InventoryMovementsRepository::insert(
            self.module(),
            payload,
            self.claims()?.sub(),
            self.claims()?
                .active_tenant()
                .ok_or(InventoryMovementsServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn get(&self, payload: Uuid) -> InventoryMovementsServiceResult<InventoryMovement> {
        Ok(InventoryMovementsRepository::get_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(InventoryMovementsServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn get_resolved(
        &self,
        payload: Uuid,
    ) -> InventoryMovementsServiceResult<InventoryMovementResolved> {
        Ok(InventoryMovementsRepository::get_resolved_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(InventoryMovementsServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn delete(&self, payload: Uuid) -> InventoryMovementsServiceResult<()> {
        Ok(InventoryMovementsRepository::delete_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(InventoryMovementsServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryMovementOrderBy, InventoryMovementFilterBy>,
        inventory_id: Uuid,
    ) -> InventoryMovementsServiceResult<(PaginatorMeta, Vec<InventoryMovementResolved>)> {
        Ok(InventoryMovementsRepository::get_all_paged(
            self.module(),
            get_query,
            self.claims()?
                .active_tenant()
                .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            inventory_id,
        )
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
                    WorksheetsRepository::get_select_list_items(self.module(), active_tenant)
                        .await?
                }
                InventoryMovementsSelectLists::Taxes => {
                    TaxesRepository::get_select_list_items(self.module(), active_tenant).await?
                }
                InventoryMovementsSelectLists::Inventory => {
                    InventoryRepository::get_select_list_items(self.module(), active_tenant).await?
                }
            },
        )
    }
    async fn print(
        &self,
        payload: &[InventoryMovementResolved],
    ) -> InventoryMovementsServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::InventoryMovementView,
            &payload,
        )?))
    }
}
