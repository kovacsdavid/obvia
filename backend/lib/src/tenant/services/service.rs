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

use crate::common::dto::{GeneralError, OrderingParams, PaginatorMeta, PaginatorParams, UuidParam};
use crate::common::error::{FriendlyError, RepositoryError};
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::services::dto::ServiceUserInput;
use crate::tenant::services::model::{Service, ServiceResolved};
use crate::tenant::services::repository::ServicesRepository;
use crate::tenant::services::types::service::ServiceOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum ServicesServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("A megadott névvel már létezik szolgáltatás a rendszerben!")]
    ServiceExists,
}

impl IntoResponse for ServicesServiceError {
    fn into_response(self) -> Response {
        match self {
            ServicesServiceError::Repository(e) => {
                FriendlyError::internal(file!(), e.to_string()).into_response()
            }
            ServicesServiceError::Unauthorized | ServicesServiceError::ServiceExists => {
                FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    GeneralError {
                        message: self.to_string(),
                    },
                )
                    .into_response()
            }
        }
    }
}

pub struct ServicesService;

type ServicesServiceResult<T> = Result<T, ServicesServiceError>;

impl ServicesService {
    pub async fn create(
        claims: &Claims,
        payload: &ServiceUserInput,
        repo: Arc<dyn ServicesRepository>,
    ) -> ServicesServiceResult<()> {
        repo.insert(
            payload.clone(),
            claims.sub(),
            claims
                .active_tenant()
                .ok_or(ServicesServiceError::Unauthorized)?,
        )
            .await
            .map_err(|e| {
                if e.is_unique_violation() {
                    ServicesServiceError::ServiceExists
                } else {
                    e.into()
                }
            })?;
        Ok(())
    }
    pub async fn get_resolved_by_id(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn ServicesRepository>,
    ) -> ServicesServiceResult<ServiceResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn ServicesRepository>,
    ) -> ServicesServiceResult<Service> {
        Ok(repo
            .get_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn update(
        claims: &Claims,
        payload: &ServiceUserInput,
        repo: Arc<dyn ServicesRepository>,
    ) -> ServicesServiceResult<Service> {
        Ok(repo
            .update(
                payload.clone(),
                claims
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn delete(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn ServicesRepository>,
    ) -> ServicesServiceResult<()> {
        Ok(repo
            .delete_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<ServiceOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn ServicesRepository>,
    ) -> ServicesServiceResult<(PaginatorMeta, Vec<ServiceResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )
            .await?)
    }
}
