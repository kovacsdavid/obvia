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
use crate::tenant::inventory::InventoryModule;
use crate::tenant::inventory::dto::InventoryUserInput;
use crate::tenant::inventory::model::{Inventory, InventoryResolved};
use crate::tenant::inventory::types::inventory::{InventoryFilterBy, InventoryOrderBy};
use axum::body::Bytes;
use axum::http::StatusCode;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum InventoryServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("A lista nem létezik")]
    InvalidSelectList,

    #[error("A megadott termékhez már létezik raktárkészlet ebben a raktárban!")]
    InventoryExists,

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),
}

impl From<ServiceError> for InventoryServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => InventoryServiceError::Unauthorized,
        }
    }
}

impl<H> IntoFriendlyError<GeneralError, H> for InventoryServiceError
where
    H: MailTransporter + ?Sized,
{
    async fn into_friendly_error(self, module: Arc<H>) -> FriendlyError<GeneralError> {
        match self {
            InventoryServiceError::Unauthorized | InventoryServiceError::InventoryExists => {
                FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    GeneralError {
                        message: self.to_string(),
                    },
                )
            }
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

pub type InventoryServiceResult<T> = Result<T, InventoryServiceError>;

pub enum InventorySelectLists {
    Products,
    Currencies,
    Warehouses,
    Taxes,
}

impl FromStr for InventorySelectLists {
    type Err = InventoryServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "products" => Ok(Self::Products),
            "currencies" => Ok(Self::Currencies),
            "warehouses" => Ok(Self::Warehouses),
            "taxes" => Ok(Self::Taxes),
            _ => Err(InventoryServiceError::InvalidSelectList),
        }
    }
}

pub trait InventoryService {
    async fn insert(&self, payload: &InventoryUserInput) -> InventoryServiceResult<Inventory>;
    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> InventoryServiceResult<Vec<SelectOption>>;
    async fn get_resolved(&self, payload: Uuid) -> InventoryServiceResult<InventoryResolved>;
    async fn get(&self, payload: Uuid) -> InventoryServiceResult<Inventory>;
    async fn update(&self, payload: &InventoryUserInput) -> InventoryServiceResult<Inventory>;
    async fn delete(&self, payload: Uuid) -> InventoryServiceResult<()>;
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryOrderBy, InventoryFilterBy>,
    ) -> InventoryServiceResult<(PaginatorMeta, Vec<InventoryResolved>)>;
    async fn print(&self, payload: &[InventoryResolved]) -> InventoryServiceResult<Bytes>;
}

impl<'a, T> InventoryService for Service<'a, T>
where
    T: InventoryModule + ?Sized,
{
    async fn insert(&self, payload: &InventoryUserInput) -> InventoryServiceResult<Inventory> {
        self.module()
            .inventory_repo()
            .insert(
                payload.clone(),
                self.claims()?.sub(),
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await
            .map_err(|e| {
                if e.is_unique_violation() {
                    InventoryServiceError::InventoryExists
                } else {
                    e.into()
                }
            })
    }

    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> InventoryServiceResult<Vec<SelectOption>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(InventoryServiceError::Unauthorized)?;
        Ok(match InventorySelectLists::from_str(select_list)? {
            InventorySelectLists::Products => {
                self.module()
                    .products_repo()
                    .get_select_list_items(active_tenant)
                    .await?
            }
            InventorySelectLists::Currencies => {
                self.module()
                    .currencies_repo()
                    .get_all_countries_select_list_items(active_tenant)
                    .await?
            }
            InventorySelectLists::Warehouses => {
                self.module()
                    .warehouses_repo()
                    .get_select_list_items(active_tenant)
                    .await?
            }
            InventorySelectLists::Taxes => {
                self.module()
                    .taxes_repo()
                    .get_select_list_items(active_tenant)
                    .await?
            }
        })
    }
    async fn get_resolved(&self, payload: Uuid) -> InventoryServiceResult<InventoryResolved> {
        Ok(self
            .module()
            .inventory_repo()
            .get_resolved_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn get(&self, payload: Uuid) -> InventoryServiceResult<Inventory> {
        Ok(self
            .module()
            .inventory_repo()
            .get_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn update(&self, payload: &InventoryUserInput) -> InventoryServiceResult<Inventory> {
        Ok(self
            .module()
            .inventory_repo()
            .update(
                payload.clone(),
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> InventoryServiceResult<()> {
        Ok(self
            .module()
            .inventory_repo()
            .delete_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryOrderBy, InventoryFilterBy>,
    ) -> InventoryServiceResult<(PaginatorMeta, Vec<InventoryResolved>)> {
        Ok(self
            .module()
            .inventory_repo()
            .get_all_paged(
                get_query,
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn print(&self, payload: &[InventoryResolved]) -> InventoryServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::InventoryView,
            &payload,
        )?))
    }
}
