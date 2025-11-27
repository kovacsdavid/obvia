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
use crate::common::dto::GeneralError;
use crate::common::dto::{OrderingParams, PaginatorMeta, PaginatorParams, UuidParam};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::inventory_movements::InventoryMovementsModule;
use crate::tenant::inventory_movements::dto::InventoryMovementUserInput;
use crate::tenant::inventory_movements::model::{InventoryMovement, InventoryMovementResolved};
use crate::tenant::inventory_movements::repository::InventoryMovementsRepository;
use crate::tenant::inventory_movements::types::InventoryMovementOrderBy;
use async_trait::async_trait;
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
}

#[async_trait]
impl IntoFriendlyError<GeneralError> for InventoryMovementsServiceError {
    async fn into_friendly_error(
        self,
        module: Arc<dyn MailTransporter>,
    ) -> FriendlyError<GeneralError> {
        match self {
            InventoryMovementsServiceError::Unauthorized => FriendlyError::user_facing(
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

pub struct InventoryMovementsService;

impl InventoryMovementsService {
    pub async fn create(
        claims: &Claims,
        payload: &InventoryMovementUserInput,
        repo: Arc<dyn InventoryMovementsRepository>,
    ) -> InventoryMovementsServiceResult<InventoryMovement> {
        Ok(repo
            .insert(
                payload.clone(),
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn get(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryMovementsRepository>,
    ) -> InventoryMovementsServiceResult<InventoryMovement> {
        Ok(repo
            .get_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn get_resolved(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryMovementsRepository>,
    ) -> InventoryMovementsServiceResult<InventoryMovementResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn delete(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryMovementsRepository>,
    ) -> InventoryMovementsServiceResult<()> {
        Ok(repo
            .delete_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
            )
            .await?)
    }

    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<InventoryMovementOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn InventoryMovementsRepository>,
        inventory_id: Uuid,
    ) -> InventoryMovementsServiceResult<(PaginatorMeta, Vec<InventoryMovementResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(InventoryMovementsServiceError::Unauthorized)?,
                inventory_id,
            )
            .await?)
    }
    pub async fn get_select_list_items(
        select_list: &str,
        claims: &Claims,
        inventory_movements_module: Arc<dyn InventoryMovementsModule>,
    ) -> InventoryMovementsServiceResult<Vec<SelectOption>> {
        let active_tenant = claims
            .active_tenant()
            .ok_or(InventoryMovementsServiceError::Unauthorized)?;
        Ok(
            match InventoryMovementsSelectLists::from_str(select_list)? {
                InventoryMovementsSelectLists::Worksheets => {
                    inventory_movements_module
                        .worksheets_repo()
                        .get_select_list_items(active_tenant)
                        .await?
                }
                InventoryMovementsSelectLists::Taxes => {
                    inventory_movements_module
                        .taxes_repo()
                        .get_select_list_items(active_tenant)
                        .await?
                }
                InventoryMovementsSelectLists::Inventory => {
                    inventory_movements_module
                        .inventory_repo()
                        .get_select_list_items(active_tenant)
                        .await?
                }
            },
        )
    }
}
