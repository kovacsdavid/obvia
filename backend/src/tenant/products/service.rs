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
use crate::common::dto::{GeneralError, PaginatorMeta, UuidParam};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::common::query_parser::GetQuery;
use crate::common::types::{UuidVO, ValueObject};
use crate::manager::auth::dto::claims::Claims;
use crate::tenant::products::ProductsModule;
use crate::tenant::products::dto::ProductUserInput;
use crate::tenant::products::model::{Product, ProductResolved};
use crate::tenant::products::repository::ProductsRepository;
use crate::tenant::products::types::product::{ProductFilterBy, ProductOrderBy};
use async_trait::async_trait;
use axum::http::StatusCode;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

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
}

#[async_trait]
impl IntoFriendlyError<GeneralError> for ProductsServiceError {
    async fn into_friendly_error(
        self,
        module: Arc<dyn MailTransporter>,
    ) -> FriendlyError<GeneralError> {
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

pub struct ProductsService;

impl ProductsService {
    pub async fn create(
        claims: &Claims,
        payload: &ProductUserInput,
        products_module: Arc<dyn ProductsModule>,
    ) -> ProductsServiceResult<Product> {
        let mut product = payload.clone();

        product.unit_of_measure_id = if let Some(units_of_measure_id) = product.unit_of_measure_id {
            Some(units_of_measure_id)
        } else {
            ValueObject::new_optional(UuidVO(
                products_module
                    .products_repo()
                    .insert_unit_of_measure(
                        product
                            .new_unit_of_measure
                            .as_ref()
                            .ok_or(ProductsServiceError::InvalidState)?
                            .as_str(),
                        claims.sub(),
                        claims
                            .active_tenant()
                            .ok_or(ProductsServiceError::Unauthorized)?,
                    )
                    .await?
                    .id
                    .to_string(),
            ))
            .map_err(|_| ProductsServiceError::InvalidState)?
        };

        Ok(products_module
            .products_repo()
            .insert(
                product,
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_select_list_items(
        select_list: &str,
        claims: &Claims,
        products_module: Arc<dyn ProductsModule>,
    ) -> ProductsServiceResult<Vec<SelectOption>> {
        match ProductsSelectLists::from_str(select_list)? {
            ProductsSelectLists::UnitsOfMeasure => Ok(products_module
                .products_repo()
                .get_units_of_measure_select_list(
                    claims
                        .active_tenant()
                        .ok_or(ProductsServiceError::Unauthorized)?,
                )
                .await?),
        }
    }
    pub async fn get_resolved_by_id(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn ProductsRepository>,
    ) -> ProductsServiceResult<ProductResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn ProductsRepository>,
    ) -> ProductsServiceResult<Product> {
        Ok(repo
            .get_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn update(
        claims: &Claims,
        payload: &ProductUserInput,
        repo: Arc<dyn ProductsRepository>,
    ) -> ProductsServiceResult<Product> {
        Ok(repo
            .update(
                payload.clone(),
                claims
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn delete(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn ProductsRepository>,
    ) -> ProductsServiceResult<()> {
        Ok(repo
            .delete_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_paged_list(
        get_query: &GetQuery<ProductOrderBy, ProductFilterBy>,
        claims: &Claims,
        repo: Arc<dyn ProductsRepository>,
    ) -> ProductsServiceResult<(PaginatorMeta, Vec<ProductResolved>)> {
        Ok(repo
            .get_all_paged(
                get_query,
                claims
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }
}
