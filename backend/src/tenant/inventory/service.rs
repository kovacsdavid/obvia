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
use crate::tenant::currencies::repository::CurrenciesRepository;
use crate::tenant::inventory::InventoryModule;
use crate::tenant::inventory::dto::InventoryUserInput;
use crate::tenant::inventory::model::{Inventory, InventoryResolved};
use crate::tenant::inventory::repository::InventoryRepository;
use crate::tenant::inventory::types::inventory::{InventoryFilterBy, InventoryOrderBy};
use crate::tenant::products::repository::ProductsRepository;
use crate::tenant::taxes::repository::TaxesRepository;
use crate::tenant::warehouses::repository::WarehousesRepository;
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

impl IntoFriendlyError for InventoryServiceError {
    async fn into_friendly_error<M>(self, module: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            InventoryServiceError::Unauthorized | InventoryServiceError::InventoryExists => {
                FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    GeneralError {
                        message: self.to_string(),
                    }
                    .to_string(),
                )
            }
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
    fn insert(
        &self,
        payload: &InventoryUserInput,
    ) -> impl Future<Output = InventoryServiceResult<Inventory>> + Send;
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = InventoryServiceResult<Vec<SelectOption>>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = InventoryServiceResult<InventoryResolved>> + Send;
    fn get(&self, payload: Uuid) -> impl Future<Output = InventoryServiceResult<Inventory>> + Send;
    fn update(
        &self,
        payload: &InventoryUserInput,
    ) -> impl Future<Output = InventoryServiceResult<Inventory>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = InventoryServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryOrderBy, InventoryFilterBy>,
    ) -> impl Future<Output = InventoryServiceResult<(PaginatorMeta, Vec<InventoryResolved>)>> + Send;
    fn print(
        &self,
        payload: &[InventoryResolved],
    ) -> impl Future<Output = InventoryServiceResult<Bytes>> + Send;
}

impl<'a, T> InventoryService for Service<'a, T>
where
    T: InventoryModule,
{
    async fn insert(&self, payload: &InventoryUserInput) -> InventoryServiceResult<Inventory> {
        InventoryRepository::insert(
            self.module(),
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
                ProductsRepository::get_select_list_items(self.module(), active_tenant).await?
            }
            InventorySelectLists::Currencies => {
                CurrenciesRepository::get_all_countries_select_list_items(
                    self.module(),
                    active_tenant,
                )
                .await?
            }
            InventorySelectLists::Warehouses => {
                WarehousesRepository::get_select_list_items(self.module(), active_tenant).await?
            }
            InventorySelectLists::Taxes => {
                TaxesRepository::get_select_list_items(self.module(), active_tenant).await?
            }
        })
    }
    async fn get_resolved(&self, payload: Uuid) -> InventoryServiceResult<InventoryResolved> {
        Ok(InventoryRepository::get_resolved_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(InventoryServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn get(&self, payload: Uuid) -> InventoryServiceResult<Inventory> {
        Ok(InventoryRepository::get_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(InventoryServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn update(&self, payload: &InventoryUserInput) -> InventoryServiceResult<Inventory> {
        Ok(InventoryRepository::update(
            self.module(),
            payload.clone(),
            self.claims()?
                .active_tenant()
                .ok_or(InventoryServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn delete(&self, payload: Uuid) -> InventoryServiceResult<()> {
        Ok(InventoryRepository::delete_by_id(
            self.module(),
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
        Ok(InventoryRepository::get_all_paged(
            self.module(),
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
