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

use crate::common::dto::{OrderingParams, PaginatorMeta, PaginatorParams, UuidParam};
use crate::common::error::{FriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::inventory_reservations::InventoryReservationsModule;
use crate::tenant::inventory_reservations::dto::InventoryReservationUserInput;
use crate::tenant::inventory_reservations::model::{
    InventoryReservation, InventoryReservationResolved,
};
use crate::tenant::inventory_reservations::repository::InventoryReservationsRepository;
use crate::tenant::inventory_reservations::types::InventoryReservationOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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
}

impl IntoResponse for InventoryReservationsServiceError {
    fn into_response(self) -> Response {
        match self {
            InventoryReservationsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                crate::common::dto::GeneralError {
                    message: self.to_string(),
                },
            )
            .into_response(),
            e => FriendlyError::internal(file!(), e.to_string()).into_response(),
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

pub struct InventoryReservationsService;

impl InventoryReservationsService {
    pub async fn create(
        claims: &Claims,
        payload: &InventoryReservationUserInput,
        repo: Arc<dyn InventoryReservationsRepository>,
    ) -> InventoryReservationsServiceResult<InventoryReservation> {
        Ok(repo
            .insert(
                payload.clone(),
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn get(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryReservationsRepository>,
    ) -> InventoryReservationsServiceResult<InventoryReservation> {
        Ok(repo
            .get_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn get_resolved(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryReservationsRepository>,
    ) -> InventoryReservationsServiceResult<InventoryReservationResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn delete(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryReservationsRepository>,
    ) -> InventoryReservationsServiceResult<()> {
        Ok(repo
            .delete_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<InventoryReservationOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn InventoryReservationsRepository>,
        inventory_id: Uuid,
    ) -> InventoryReservationsServiceResult<(PaginatorMeta, Vec<InventoryReservationResolved>)>
    {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(InventoryReservationsServiceError::Unauthorized)?,
                inventory_id,
            )
            .await?)
    }
    pub async fn get_select_list_items(
        select_list: &str,
        claims: &Claims,
        inventory_reservations_module: Arc<InventoryReservationsModule>,
    ) -> InventoryReservationsServiceResult<Vec<SelectOption>> {
        let active_tenant = claims
            .active_tenant()
            .ok_or(InventoryReservationsServiceError::Unauthorized)?;
        Ok(
            match InventoryReservationsSelectLists::from_str(select_list)? {
                InventoryReservationsSelectLists::Worksheets => {
                    inventory_reservations_module
                        .worksheets_repo
                        .get_select_list_items(active_tenant)
                        .await?
                }
                InventoryReservationsSelectLists::Inventory => {
                    inventory_reservations_module
                        .inventory_repo
                        .get_select_list_items(active_tenant)
                        .await?
                }
            },
        )
    }
}
