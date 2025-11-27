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
use crate::tenant::taxes::TaxesModule;
use crate::tenant::taxes::dto::TaxUserInput;
use crate::tenant::taxes::model::{Tax, TaxResolved};
use crate::tenant::taxes::repository::TaxesRepository;
use crate::tenant::taxes::types::TaxOrderBy;
use async_trait::async_trait;
use axum::http::StatusCode;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum TaxesServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Az adó már létrehozásra került a rendszerben")]
    TaxExists,

    #[error("A lista nem létezik")]
    InvalidSelectList,
}

#[async_trait]
impl IntoFriendlyError<GeneralError> for TaxesServiceError {
    async fn into_friendly_error(
        self,
        module: Arc<dyn MailTransporter>,
    ) -> FriendlyError<GeneralError> {
        match self {
            TaxesServiceError::Unauthorized | TaxesServiceError::TaxExists => {
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

pub enum TaxesSelectLists {
    Countries,
}

impl FromStr for TaxesSelectLists {
    type Err = TaxesServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "countries" => Ok(Self::Countries),
            _ => Err(TaxesServiceError::InvalidSelectList),
        }
    }
}

pub struct TaxesService;

type TaxesServiceResult<T> = Result<T, TaxesServiceError>;

impl TaxesService {
    pub async fn create(
        claims: &Claims,
        payload: &TaxUserInput,
        repo: Arc<dyn TaxesRepository>,
    ) -> TaxesServiceResult<Tax> {
        repo.insert(
            payload.clone(),
            claims.sub(),
            claims
                .active_tenant()
                .ok_or(TaxesServiceError::Unauthorized)?,
        )
        .await
        .map_err(|e| {
            if e.is_unique_violation() {
                TaxesServiceError::TaxExists
            } else {
                e.into()
            }
        })
    }
    pub async fn get_resolved_by_id(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn TaxesRepository>,
    ) -> TaxesServiceResult<TaxResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn TaxesRepository>,
    ) -> TaxesServiceResult<Tax> {
        Ok(repo
            .get_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn update(
        claims: &Claims,
        payload: &TaxUserInput,
        repo: Arc<dyn TaxesRepository>,
    ) -> TaxesServiceResult<Tax> {
        Ok(repo
            .update(
                payload.clone(),
                claims
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn delete(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn TaxesRepository>,
    ) -> TaxesServiceResult<()> {
        Ok(repo
            .delete_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<TaxOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn TaxesRepository>,
    ) -> TaxesServiceResult<(PaginatorMeta, Vec<TaxResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_select_list_items(
        select_list: &str,
        claims: &Claims,
        taxes_module: Arc<dyn TaxesModule>,
    ) -> TaxesServiceResult<Vec<SelectOption>> {
        match TaxesSelectLists::from_str(select_list)? {
            TaxesSelectLists::Countries => Ok(taxes_module
                .address_repo()
                .get_all_countries_select_list_items(
                    claims
                        .active_tenant()
                        .ok_or(TaxesServiceError::Unauthorized)?,
                )
                .await?),
        }
    }
}
