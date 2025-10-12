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
use crate::common::dto::{GeneralError, OrderingParams, PaginatorMeta, PaginatorParams, UuidParam};
use crate::common::error::{FriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::common::types::value_object::ValueObjectable;
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::inventory::InventoryModule;
use crate::tenant::inventory::dto::InventoryUserInput;
use crate::tenant::inventory::model::{Inventory, InventoryResolved};
use crate::tenant::inventory::repository::InventoryRepository;
use crate::tenant::inventory::types::inventory::InventoryOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum InventoryServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Invalid state")]
    InvalidState,

    #[error("A lista nem létezik")]
    InvalidSelectList,
}

impl IntoResponse for InventoryServiceError {
    fn into_response(self) -> Response {
        match self {
            InventoryServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: InventoryServiceError::Unauthorized.to_string(),
                },
            )
            .into_response(),
            e => FriendlyError::internal(file!(), e.to_string()).into_response(),
        }
    }
}

pub type InventoryServiceResult<T> = Result<T, InventoryServiceError>;

pub enum InventorySelectLists {
    Products,
    Currencies,
    Warehouses,
}

impl FromStr for InventorySelectLists {
    type Err = InventoryServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "products" => Ok(Self::Products),
            "currencies" => Ok(Self::Currencies),
            "warehouses" => Ok(Self::Warehouses),
            _ => Err(InventoryServiceError::InvalidSelectList),
        }
    }
}

pub struct InventoryService;

impl InventoryService {
    pub async fn create(
        claims: &Claims,
        payload: &InventoryUserInput,
        inventory_module: Arc<InventoryModule>,
    ) -> InventoryServiceResult<()> {
        let mut inventory = payload.clone();
        inventory.currency_id = if inventory.currency_id.is_some() {
            inventory.currency_id
        } else {
            Some(
                inventory_module
                    .inventory_repo
                    .insert_currency(
                        inventory
                            .new_currency
                            .as_ref()
                            .ok_or(InventoryServiceError::InvalidState)?
                            .extract()
                            .get_value()
                            .as_str(),
                        claims.sub(),
                        claims
                            .active_tenant()
                            .ok_or(InventoryServiceError::Unauthorized)?,
                    )
                    .await?
                    .id,
            )
        };
        inventory_module
            .inventory_repo
            .insert(
                inventory,
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?;
        Ok(())
    }
    pub async fn get_select_list_items(
        select_list: &str,
        claims: &Claims,
        inventory_module: Arc<InventoryModule>,
    ) -> InventoryServiceResult<Vec<SelectOption>> {
        match InventorySelectLists::from_str(select_list)? {
            InventorySelectLists::Products => Ok(inventory_module
                .products_repo
                .get_select_list_items(
                    claims
                        .active_tenant()
                        .ok_or(InventoryServiceError::Unauthorized)?,
                )
                .await?),
            InventorySelectLists::Currencies => Ok(inventory_module
                .inventory_repo
                .get_select_list_items(
                    claims
                        .active_tenant()
                        .ok_or(InventoryServiceError::Unauthorized)?,
                )
                .await?),
            InventorySelectLists::Warehouses => Ok(inventory_module
                .warehouses_repo
                .get_select_list_items(
                    claims
                        .active_tenant()
                        .ok_or(InventoryServiceError::Unauthorized)?,
                )
                .await?),
        }
    }
    pub async fn get_resolved_by_id(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryRepository>,
    ) -> InventoryServiceResult<InventoryResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryRepository>,
    ) -> InventoryServiceResult<Inventory> {
        Ok(repo
            .get_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn update(
        claims: &Claims,
        payload: &InventoryUserInput,
        repo: Arc<dyn InventoryRepository>,
    ) -> InventoryServiceResult<Inventory> {
        Ok(repo
            .update(
                payload.clone(),
                claims
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn delete(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn InventoryRepository>,
    ) -> InventoryServiceResult<()> {
        Ok(repo
            .delete_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<InventoryOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn InventoryRepository>,
    ) -> InventoryServiceResult<(PaginatorMeta, Vec<InventoryResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }
}
