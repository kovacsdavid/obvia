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
use crate::tenant::products::ProductsModule;
use crate::tenant::products::dto::CreateProduct;
use crate::tenant::products::model::{Product, UnitOfMeasure};
use crate::tenant::products::repository::ProductsRepository;
use crate::tenant::products::types::product::ProductOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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

pub struct ProductsService;

impl ProductsService {
    pub async fn create(
        claims: &Claims,
        payload: &CreateProduct,
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
    pub async fn get_all_units_of_measure(
        claims: &Claims,
        products_module: Arc<ProductsModule>,
    ) -> ProductsServiceResult<Vec<UnitOfMeasure>> {
        Ok(products_module
            .products_repo
            .get_all_units_of_measure(
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
    ) -> ProductsServiceResult<(PaginatorMeta, Vec<Product>)> {
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
