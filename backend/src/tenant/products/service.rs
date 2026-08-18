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
use crate::common::types::UuidVO;
use crate::common::value_object::ValueObjectRequired;
use crate::tenant::products::ProductsModuleInterface;
use crate::tenant::products::dto::print::ProductsResolvedPrint;
use crate::tenant::products::dto::user_input::ProductUserInput;
use crate::tenant::products::model::{Product, ProductResolved};
use crate::tenant::products::types::product::{ProductFilterBy, ProductOrderBy};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use mockall_double::double;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

pub enum ProductsSelectLists {
    UnitsOfMeasure,
}

impl FromStr for ProductsSelectLists {
    type Err = ServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "units_of_measure" => Ok(Self::UnitsOfMeasure),
            _ => Err(Self::Err::InvalidSelectList),
        }
    }
}

pub trait ProductService {
    fn insert(
        &self,
        payload: &mut ProductUserInput,
    ) -> impl Future<Output = ServiceResult<Product>> + Send;
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = ServiceResult<Vec<SelectOption>>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = ServiceResult<ProductResolved>> + Send;
    fn get(&self, payload: Uuid) -> impl Future<Output = ServiceResult<Product>> + Send;
    fn update(
        &self,
        payload: &ProductUserInput,
    ) -> impl Future<Output = ServiceResult<Product>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = ServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<ProductOrderBy, ProductFilterBy>,
    ) -> impl Future<Output = ServiceResult<(PaginatorMeta, Vec<ProductResolved>)>> + Send;
    fn print(
        &self,
        payload: &[ProductsResolvedPrint],
    ) -> impl Future<Output = ServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(&self, path: &Path) -> impl Future<Output = ServiceResult<()>> + Sync;
}

impl<'a, T> ProductService for Service<'a, T>
where
    T: ProductsModuleInterface,
{
    async fn insert(&self, payload: &mut ProductUserInput) -> ServiceResult<Product> {
        if let Some(new_unit_of_measure) = &payload.new_unit_of_measure {
            payload.unit_of_measure_id = self
                .module()
                .products_repo(
                    self.claims()?
                        .active_tenant()
                        .ok_or(ServiceError::Unauthorized)?,
                )?
                .insert_unit_of_measure(new_unit_of_measure.as_str()?, self.claims()?.sub())
                .await?
                .id
                .to_string()
                .parse::<ValueObjectRequired<UuidVO>>()
                .map(Some)
                .map_err(|_| ServiceError::InvalidState)?;
        }
        Ok(self
            .module()
            .products_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .insert(payload, self.claims()?.sub())
            .await?)
    }

    async fn get_select_list_items(&self, select_list: &str) -> ServiceResult<Vec<SelectOption>> {
        match ProductsSelectLists::from_str(select_list)? {
            ProductsSelectLists::UnitsOfMeasure => Ok(self
                .module()
                .products_repo(
                    self.claims()?
                        .active_tenant()
                        .ok_or(ServiceError::Unauthorized)?,
                )?
                .get_units_of_measure_select_list()
                .await?),
        }
    }

    async fn get_resolved(&self, payload: Uuid) -> ServiceResult<ProductResolved> {
        Ok(self
            .module()
            .products_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }

    async fn get(&self, payload: Uuid) -> ServiceResult<Product> {
        Ok(self
            .module()
            .products_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }

    async fn update(&self, payload: &ProductUserInput) -> ServiceResult<Product> {
        if !payload.id.is_present() {
            return Err(ServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .products_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .update(payload.clone())
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> ServiceResult<()> {
        Ok(self
            .module()
            .products_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<ProductOrderBy, ProductFilterBy>,
    ) -> ServiceResult<(PaginatorMeta, Vec<ProductResolved>)> {
        Ok(self
            .module()
            .products_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .get_paged(get_query)
            .await?)
    }

    async fn print(&self, payload: &[ProductsResolvedPrint]) -> ServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::ProductView,
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
        let product_id = "4f321721-37c6-4e91-8e42-6281c36937bc"
            .parse()
            .map_err(|e: uuid::Error| ServiceError::ParseError(e.to_string()))?;
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0"
            .parse()
            .map_err(|e: uuid::Error| ServiceError::ParseError(e.to_string()))?;
        let unit_of_measure_id = "0237354a-21ab-46f4-a4ca-b21cb08561d7"
            .parse()
            .map_err(|e: uuid::Error| ServiceError::ParseError(e.to_string()))?;
        let product_resolved = ProductResolved {
            id: product_id,
            name: "Test product".to_string(),
            description: None,
            unit_of_measure_id,
            unit_of_measure: "cm".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
        };
        let products_resolved_print =
            ProductsResolvedPrint::from_product_resolved(product_resolved, tz);
        let pdf = self.print(&[products_resolved_print]).await?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
