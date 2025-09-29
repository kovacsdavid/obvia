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
use crate::common::dto::{OrderingParams, PaginatorMeta, PaginatorParams};
use crate::common::error::{FriendlyError, RepositoryError};
use crate::common::types::value_object::ValueObjectable;
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::inventory::InventoryModule;
use crate::tenant::inventory::dto::CreateInventory;
use crate::tenant::inventory::model::{Currency, InventoryResolved};
use crate::tenant::inventory::repository::InventoryRepository;
use crate::tenant::inventory::types::inventory::InventoryOrderBy;
use crate::tenant::products::model::Product;
use crate::tenant::warehouses::model::Warehouse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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
}

impl IntoResponse for InventoryServiceError {
    fn into_response(self) -> Response {
        match self {
            InventoryServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                InventoryServiceError::Unauthorized.to_string(),
            ),
            e => FriendlyError::internal(file!(), e.to_string()),
        }
        .into_response()
    }
}

pub type InventoryServiceResult<T> = Result<T, InventoryServiceError>;

pub struct InventoryService;

impl InventoryService {
    pub async fn create(
        claims: &Claims,
        payload: &CreateInventory,
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
    pub async fn get_all_currencies(
        claims: &Claims,
        inventory_module: Arc<InventoryModule>,
    ) -> InventoryServiceResult<Vec<Currency>> {
        Ok(inventory_module
            .inventory_repo
            .get_all_currencies(
                claims
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_all_products(
        claims: &Claims,
        inventory_module: Arc<InventoryModule>,
    ) -> InventoryServiceResult<Vec<Product>> {
        Ok(inventory_module
            .products_repo
            .get_all(
                claims
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_all_warehouses(
        claims: &Claims,
        inventory_module: Arc<InventoryModule>,
    ) -> InventoryServiceResult<Vec<Warehouse>> {
        Ok(inventory_module
            .warehouses_repo
            .get_all(
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
