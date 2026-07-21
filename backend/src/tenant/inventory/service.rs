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

use crate::common::dto::PaginatorMeta;
use crate::common::error::RepositoryError;
use crate::common::error::v2::{AppError, AppErrorVisibility};
use crate::common::model::SelectOption;
#[double]
use crate::common::pdf::PdfGenerator;
use crate::common::pdf::{PdfGenError, PdfTemplates};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::inventory::InventoryModuleInterface;
use crate::tenant::inventory::dto::print::InventoryResolvedPrint;
use crate::tenant::inventory::dto::user_input::InventoryUserInput;
use crate::tenant::inventory::model::{Inventory, InventoryResolved};
use crate::tenant::inventory::types::inventory::{InventoryFilterBy, InventoryOrderBy};
use axum::http::StatusCode;
use mockall_double::double;
use serde_json::json;
use std::str::FromStr;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum InventoryServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

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

impl From<InventoryServiceError> for AppError {
    fn from(value: InventoryServiceError) -> Self {
        match value {
            InventoryServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            InventoryServiceError::InventoryExists => Self::new(
                Level::DEBUG,
                StatusCode::CONFLICT,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            InventoryServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            InventoryServiceError::Repository(RepositoryError::Database(
                sqlx::Error::RowNotFound,
            )) => Self::new(
                Level::DEBUG,
                StatusCode::NOT_FOUND,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": "Nem található"}),
            ),
            _ => Self::new(
                Level::ERROR,
                StatusCode::INTERNAL_SERVER_ERROR,
                file!(),
                AppErrorVisibility::Internal,
                json!({"message": value.to_string()}),
            ),
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
        payload: &[InventoryResolvedPrint],
    ) -> impl Future<Output = InventoryServiceResult<Vec<u8>>> + Send;
}

impl<'a, T> InventoryService for Service<'a, T>
where
    T: InventoryModuleInterface,
{
    async fn insert(&self, payload: &InventoryUserInput) -> InventoryServiceResult<Inventory> {
        self.module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )?
            .insert(payload, self.claims()?.sub())
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
                    .products_repo(active_tenant)?
                    .get_select_list_items()
                    .await?
            }
            InventorySelectLists::Currencies => {
                self.module()
                    .currencies_repo(active_tenant)?
                    .get_all_countries_select_list_items()
                    .await?
            }
            InventorySelectLists::Warehouses => {
                self.module()
                    .warehouses_repo(active_tenant)?
                    .get_select_list_items()
                    .await?
            }
            InventorySelectLists::Taxes => {
                self.module()
                    .taxes_repo(active_tenant)?
                    .get_select_list_items()
                    .await?
            }
        })
    }
    async fn get_resolved(&self, payload: Uuid) -> InventoryServiceResult<InventoryResolved> {
        Ok(self
            .module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }
    async fn get(&self, payload: Uuid) -> InventoryServiceResult<Inventory> {
        Ok(self
            .module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }

    async fn update(&self, payload: &InventoryUserInput) -> InventoryServiceResult<Inventory> {
        if !payload.id.is_present() {
            return Err(InventoryServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )?
            .update(payload)
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> InventoryServiceResult<()> {
        Ok(self
            .module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryOrderBy, InventoryFilterBy>,
    ) -> InventoryServiceResult<(PaginatorMeta, Vec<InventoryResolved>)> {
        Ok(self
            .module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(InventoryServiceError::Unauthorized)?,
            )?
            .get_paged(get_query)
            .await?)
    }

    async fn print(&self, payload: &[InventoryResolvedPrint]) -> InventoryServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::InventoryView,
            payload.to_vec(),
        )?)
    }
}
