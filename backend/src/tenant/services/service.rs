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
use crate::common::dto::{GeneralError, OrderingParams, PaginatorMeta, PaginatorParams, UuidParam};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::services::ServicesModule;
use crate::tenant::services::dto::ServiceUserInput;
use crate::tenant::services::model::{Service, ServiceResolved};
use crate::tenant::services::repository::ServicesRepository;
use crate::tenant::services::types::service::ServiceOrderBy;
use async_trait::async_trait;
use axum::http::StatusCode;
use std::str::FromStr;
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

    #[error("A lista nem létezik")]
    InvalidSelectList,
}

#[async_trait]
impl IntoFriendlyError<GeneralError> for ServicesServiceError {
    async fn into_friendly_error(
        self,
        module: Arc<dyn MailTransporter>,
    ) -> FriendlyError<GeneralError> {
        match self {
            ServicesServiceError::Unauthorized | ServicesServiceError::ServiceExists => {
                FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    GeneralError {
                        message: self.to_string(),
                    },
                )
            }
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

pub enum ServicesSelectLists {
    Currencies,
    Taxes,
}

impl FromStr for ServicesSelectLists {
    type Err = ServicesServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "currencies" => Ok(Self::Currencies),
            "taxes" => Ok(Self::Taxes),
            _ => Err(ServicesServiceError::InvalidSelectList),
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
    ) -> ServicesServiceResult<Service> {
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
        })
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
    pub async fn get_select_list_items(
        select_list: &str,
        claims: &Claims,
        services_module: Arc<dyn ServicesModule>,
    ) -> ServicesServiceResult<Vec<SelectOption>> {
        match ServicesSelectLists::from_str(select_list)? {
            ServicesSelectLists::Currencies => Ok(services_module
                .currencies_repo()
                .get_all_countries_select_list_items(
                    claims
                        .active_tenant()
                        .ok_or(ServicesServiceError::Unauthorized)?,
                )
                .await?),
            ServicesSelectLists::Taxes => Ok(services_module
                .taxes_repo()
                .get_select_list_items(
                    claims
                        .active_tenant()
                        .ok_or(ServicesServiceError::Unauthorized)?,
                )
                .await?),
        }
    }
}
