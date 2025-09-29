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
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::customers::dto::CreateCustomer;
use crate::tenant::customers::model::CustomerResolved;
use crate::tenant::customers::repository::CustomersRespository;
use crate::tenant::customers::types::customer::CustomerOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum CustomersServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,
}

impl IntoResponse for CustomersServiceError {
    fn into_response(self) -> Response {
        match self {
            CustomersServiceError::Repository(e) => FriendlyError::internal(file!(), e.to_string()),
            CustomersServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                CustomersServiceError::Unauthorized.to_string(),
            ),
        }
        .into_response()
    }
}

pub struct CustomersService;

type CustomersServiceResult<T> = Result<T, CustomersServiceError>;

impl CustomersService {
    pub async fn try_create(
        claims: &Claims,
        payload: &CreateCustomer,
        repo: Arc<dyn CustomersRespository>,
    ) -> CustomersServiceResult<()> {
        repo.insert(
            payload.clone(),
            claims.sub(),
            claims
                .active_tenant()
                .ok_or(CustomersServiceError::Unauthorized)?,
        )
        .await?;
        Ok(())
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<CustomerOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn CustomersRespository>,
    ) -> CustomersServiceResult<(PaginatorMeta, Vec<CustomerResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )
            .await?)
    }
}
