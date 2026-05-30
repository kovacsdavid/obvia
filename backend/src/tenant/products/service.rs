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
use crate::common::types::UuidVO;
use crate::common::value_object::{ValueObjectError, ValueObjectRequired};
use crate::tenant::products::ProductsModule;
use crate::tenant::products::dto::ProductUserInput;
use crate::tenant::products::model::{Product, ProductResolved};
use crate::tenant::products::types::product::{ProductFilterBy, ProductOrderBy};
use axum::body::Bytes;
use axum::http::StatusCode;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ProductsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Invalid state")]
    InvalidState,

    #[error("A lista nem létezik")]
    InvalidSelectList,

    #[error("ValueObjectError: {0}")]
    ValueObjectError(#[from] ValueObjectError),

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),
}

impl From<ServiceError> for ProductsServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => ProductsServiceError::Unauthorized,
        }
    }
}

impl<H> IntoFriendlyError<GeneralError, H> for ProductsServiceError
where
    H: MailTransporter + ?Sized,
{
    async fn into_friendly_error(self, module: Arc<H>) -> FriendlyError<GeneralError> {
        match self {
            ProductsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: ProductsServiceError::Unauthorized.to_string(),
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

type ProductsServiceResult<T> = Result<T, ProductsServiceError>;

pub enum ProductsSelectLists {
    UnitsOfMeasure,
}

impl FromStr for ProductsSelectLists {
    type Err = ProductsServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "units_of_measure" => Ok(Self::UnitsOfMeasure),
            _ => Err(Self::Err::InvalidSelectList),
        }
    }
}

pub trait ProductService {
    async fn insert(&self, payload: &mut ProductUserInput) -> ProductsServiceResult<Product>;
    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> ProductsServiceResult<Vec<SelectOption>>;
    async fn get_resolved(&self, payload: Uuid) -> ProductsServiceResult<ProductResolved>;
    async fn get(&self, payload: Uuid) -> ProductsServiceResult<Product>;
    async fn update(&self, payload: &ProductUserInput) -> ProductsServiceResult<Product>;
    async fn delete(&self, payload: Uuid) -> ProductsServiceResult<()>;
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<ProductOrderBy, ProductFilterBy>,
    ) -> ProductsServiceResult<(PaginatorMeta, Vec<ProductResolved>)>;
    async fn print(&self, payload: &[ProductResolved]) -> ProductsServiceResult<Bytes>;
}

impl<'a, T> ProductService for Service<'a, T>
where
    T: ProductsModule + ?Sized,
{
    async fn insert(&self, payload: &mut ProductUserInput) -> ProductsServiceResult<Product> {
        if let Some(new_unit_of_measure) = &payload.new_unit_of_measure {
            payload.unit_of_measure_id = self
                .module()
                .products_repo()
                .insert_unit_of_measure(
                    new_unit_of_measure.as_str()?,
                    self.claims()?.sub(),
                    self.claims()?
                        .active_tenant()
                        .ok_or(ProductsServiceError::Unauthorized)?,
                )
                .await?
                .id
                .to_string()
                .parse::<ValueObjectRequired<UuidVO>>()
                .map(Some)
                .map_err(|_| ProductsServiceError::InvalidState)?;
        }
        Ok(self
            .module()
            .products_repo()
            .insert(
                payload,
                self.claims()?.sub(),
                self.claims()?
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> ProductsServiceResult<Vec<SelectOption>> {
        match ProductsSelectLists::from_str(select_list)? {
            ProductsSelectLists::UnitsOfMeasure => Ok(self
                .module()
                .products_repo()
                .get_units_of_measure_select_list(
                    self.claims()?
                        .active_tenant()
                        .ok_or(ProductsServiceError::Unauthorized)?,
                )
                .await?),
        }
    }

    async fn get_resolved(&self, payload: Uuid) -> ProductsServiceResult<ProductResolved> {
        Ok(self
            .module()
            .products_repo()
            .get_resolved_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn get(&self, payload: Uuid) -> ProductsServiceResult<Product> {
        Ok(self
            .module()
            .products_repo()
            .get_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn update(&self, payload: &ProductUserInput) -> ProductsServiceResult<Product> {
        Ok(self
            .module()
            .products_repo()
            .update(
                payload.clone(),
                self.claims()?
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> ProductsServiceResult<()> {
        Ok(self
            .module()
            .products_repo()
            .delete_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<ProductOrderBy, ProductFilterBy>,
    ) -> ProductsServiceResult<(PaginatorMeta, Vec<ProductResolved>)> {
        Ok(self
            .module()
            .products_repo()
            .get_all_paged(
                get_query,
                self.claims()?
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn print(&self, payload: &[ProductResolved]) -> ProductsServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::ProductView,
            &payload,
        )?))
    }
}
