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
use crate::common::model::SelectOption;
#[double]
use crate::common::pdf::PdfGenerator;
use crate::common::pdf::PdfTemplates;
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError, ServiceResult};
use crate::tenant::inventory::InventoryModuleInterface;
use crate::tenant::inventory::dto::print::InventoryResolvedPrint;
use crate::tenant::inventory::dto::user_input::InventoryUserInput;
use crate::tenant::inventory::model::{Inventory, InventoryResolved};
use crate::tenant::inventory::types::inventory::{InventoryFilterBy, InventoryOrderBy};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use mockall_double::double;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

pub enum InventorySelectLists {
    Products,
    Currencies,
    Warehouses,
    Taxes,
}

impl FromStr for InventorySelectLists {
    type Err = ServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "products" => Ok(Self::Products),
            "currencies" => Ok(Self::Currencies),
            "warehouses" => Ok(Self::Warehouses),
            "taxes" => Ok(Self::Taxes),
            _ => Err(ServiceError::InvalidSelectList),
        }
    }
}

pub trait InventoryService {
    fn insert(
        &self,
        payload: &InventoryUserInput,
    ) -> impl Future<Output = ServiceResult<Inventory>> + Send;
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = ServiceResult<Vec<SelectOption>>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = ServiceResult<InventoryResolved>> + Send;
    fn get(&self, payload: Uuid) -> impl Future<Output = ServiceResult<Inventory>> + Send;
    fn update(
        &self,
        payload: &InventoryUserInput,
    ) -> impl Future<Output = ServiceResult<Inventory>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = ServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryOrderBy, InventoryFilterBy>,
    ) -> impl Future<Output = ServiceResult<(PaginatorMeta, Vec<InventoryResolved>)>> + Send;
    fn print(
        &self,
        payload: &[InventoryResolvedPrint],
    ) -> impl Future<Output = ServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(&self, path: &Path) -> impl Future<Output = ServiceResult<()>> + Sync;
}

impl<'a, T> InventoryService for Service<'a, T>
where
    T: InventoryModuleInterface,
{
    async fn insert(&self, payload: &InventoryUserInput) -> ServiceResult<Inventory> {
        self.module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .insert(payload, self.claims()?.sub())
            .await
            .map_err(|e| {
                if e.is_unique_violation() {
                    ServiceError::Conflict(
                        "A megadott termékhez már létezik raktárkészlet ebben a raktárban!",
                    )
                } else {
                    e.into()
                }
            })
    }

    async fn get_select_list_items(&self, select_list: &str) -> ServiceResult<Vec<SelectOption>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(ServiceError::Unauthorized)?;
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
    async fn get_resolved(&self, payload: Uuid) -> ServiceResult<InventoryResolved> {
        Ok(self
            .module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }
    async fn get(&self, payload: Uuid) -> ServiceResult<Inventory> {
        Ok(self
            .module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }

    async fn update(&self, payload: &InventoryUserInput) -> ServiceResult<Inventory> {
        if !payload.id.is_present() {
            return Err(ServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .update(payload)
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> ServiceResult<()> {
        Ok(self
            .module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<InventoryOrderBy, InventoryFilterBy>,
    ) -> ServiceResult<(PaginatorMeta, Vec<InventoryResolved>)> {
        Ok(self
            .module()
            .inventory_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .get_paged(get_query)
            .await?)
    }

    async fn print(&self, payload: &[InventoryResolvedPrint]) -> ServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::InventoryView,
            payload.to_vec(),
        )?)
    }
    async fn print_snapshot(&self, path: &Path) -> ServiceResult<()> {
        let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z"
            .parse()
            .map_err(|e: chrono::ParseError| ServiceError::ParseError(e.to_string()))?;
        let tz: Tz = "Europe/Budapest"
            .parse()
            .map_err(|e: chrono_tz::ParseError| ServiceError::ParseError(e.to_string()))?;
        let inventory_id = "4f321721-37c6-4e91-8e42-6281c36937bc"
            .parse()
            .map_err(|e: uuid::Error| ServiceError::ParseError(e.to_string()))?;
        let product_id = "0237354a-21ab-46f4-a4ca-b21cb08561d7"
            .parse()
            .map_err(|e: uuid::Error| ServiceError::ParseError(e.to_string()))?;
        let warehouse_id = "521f9728-f59f-435d-8656-69ba4273254c"
            .parse()
            .map_err(|e: uuid::Error| ServiceError::ParseError(e.to_string()))?;
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0"
            .parse()
            .map_err(|e: uuid::Error| ServiceError::ParseError(e.to_string()))?;
        let inventory_resolved = InventoryResolved {
            id: inventory_id,
            product_id,
            product: "Test product".to_string(),
            warehouse_id,
            warehouse: "Test warehouse".to_string(),
            quantity_on_hand: "10".parse().unwrap(),
            quantity_reserved: "20".parse().unwrap(),
            quantity_available: "30".parse().unwrap(),
            minimum_stock: None,
            maximum_stock: None,
            currency_code: "HUF".to_string(),
            currency: "Forint".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
        };
        let inventory_resolved_print =
            InventoryResolvedPrint::from_inventory_resolved(inventory_resolved, tz);
        let pdf = self.print(&[inventory_resolved_print]).await?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
