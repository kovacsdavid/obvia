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
use crate::common::dto::{OrderingParams, PaginatorMeta, PaginatorParams, UuidParam};
use crate::common::error::{FriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::common::types::value_object::ValueObjectable;
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::products::ProductsModule;
use crate::tenant::products::dto::ProductUserInput;
use crate::tenant::products::model::{Product, ProductResolved};
use crate::tenant::products::repository::ProductsRepository;
use crate::tenant::products::types::product::ProductOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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

impl IntoResponse for ProductsServiceError {
    fn into_response(self) -> Response {
        match self {
            ProductsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                ProductsServiceError::Unauthorized.to_string(),
            ),
            e => FriendlyError::internal(file!(), e.to_string()),
        }
        .into_response()
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
        products_module: Arc<ProductsModule>,
    ) -> Result<(), ProductsServiceError> {
        let mut product = payload.clone();

        product.unit_of_measure_id = if product.unit_of_measure_id.is_some() {
            product.unit_of_measure_id
        } else {
            Some(
                products_module
                    .products_repo
                    .insert_unit_of_measure(
                        product
                            .new_unit_of_measure
                            .as_ref()
                            .ok_or(ProductsServiceError::InvalidState)?
                            .extract()
                            .get_value()
                            .as_str(),
                        claims.sub(),
                        claims
                            .active_tenant()
                            .ok_or(ProductsServiceError::Unauthorized)?,
                    )
                    .await?
                    .id,
            )
        };

        products_module
            .products_repo
            .insert(
                product,
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?;
        Ok(())
    }
    pub async fn get_select_list_items(
        select_list: &str,
        claims: &Claims,
        products_module: Arc<ProductsModule>,
    ) -> ProductsServiceResult<Vec<SelectOption>> {
        match ProductsSelectLists::from_str(select_list)? {
            ProductsSelectLists::UnitsOfMeasure => Ok(products_module
                .products_repo
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
        paginator: &PaginatorParams,
        ordering: &OrderingParams<ProductOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn ProductsRepository>,
    ) -> ProductsServiceResult<(PaginatorMeta, Vec<ProductResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(ProductsServiceError::Unauthorized)?,
            )
            .await?)
    }
}
