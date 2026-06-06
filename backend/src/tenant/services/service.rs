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

use crate::common::BaseModule;
use crate::common::dto::{GeneralError, PaginatorMeta};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::common::pdf::{PdfGenError, PdfTemplates, gen_pdf_temporary};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::currencies::repository::CurrenciesRepository;
use crate::tenant::services::ServicesModule;
use crate::tenant::services::dto::ServiceUserInput;
use crate::tenant::services::model::{Service as ServiceModel, ServiceResolved};
use crate::tenant::services::repository::ServicesRepository;
use crate::tenant::services::types::service::{ServiceFilterBy, ServiceOrderBy};
use crate::tenant::taxes::repository::TaxesRepository;
use axum::body::Bytes;
use axum::http::StatusCode;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

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

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),
}

impl From<ServiceError> for ServicesServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => ServicesServiceError::Unauthorized,
        }
    }
}

impl IntoFriendlyError for ServicesServiceError {
    async fn into_friendly_error<M>(self, module: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            ServicesServiceError::Unauthorized | ServicesServiceError::ServiceExists => {
                FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    GeneralError {
                        message: self.to_string(),
                    }
                    .to_string(),
                )
            }
            e => {
                FriendlyError::internal_with_admin_notify(
                    file!(),
                    GeneralError {
                        message: e.to_string(),
                    }
                    .to_string(),
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

type ServicesServiceResult<T> = Result<T, ServicesServiceError>;

pub trait ServiceService {
    fn insert(
        &self,
        payload: &ServiceUserInput,
    ) -> impl Future<Output = ServicesServiceResult<ServiceModel>> + Send;
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = ServicesServiceResult<Vec<SelectOption>>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = ServicesServiceResult<ServiceResolved>> + Send;
    fn get(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = ServicesServiceResult<ServiceModel>> + Send;
    fn update(
        &self,
        payload: &ServiceUserInput,
    ) -> impl Future<Output = ServicesServiceResult<ServiceModel>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = ServicesServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<ServiceOrderBy, ServiceFilterBy>,
    ) -> impl Future<Output = ServicesServiceResult<(PaginatorMeta, Vec<ServiceResolved>)>> + Send;
    fn print(
        &self,
        payload: &[ServiceResolved],
    ) -> impl Future<Output = ServicesServiceResult<Bytes>> + Send;
}

impl<'a, T> ServiceService for Service<'a, T>
where
    T: ServicesModule,
{
    async fn insert(&self, payload: &ServiceUserInput) -> ServicesServiceResult<ServiceModel> {
        ServicesRepository::insert(
            self.module(),
            payload,
            self.claims()?.sub(),
            self.claims()?
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

    async fn get_resolved(&self, payload: Uuid) -> ServicesServiceResult<ServiceResolved> {
        Ok(ServicesRepository::get_resolved_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(ServicesServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn get(&self, payload: Uuid) -> ServicesServiceResult<ServiceModel> {
        Ok(ServicesRepository::get_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(ServicesServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn update(&self, payload: &ServiceUserInput) -> ServicesServiceResult<ServiceModel> {
        Ok(ServicesRepository::update(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(ServicesServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn delete(&self, payload: Uuid) -> ServicesServiceResult<()> {
        Ok(ServicesRepository::delete_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(ServicesServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<ServiceOrderBy, ServiceFilterBy>,
    ) -> ServicesServiceResult<(PaginatorMeta, Vec<ServiceResolved>)> {
        Ok(ServicesRepository::get_all_paged(
            self.module(),
            get_query,
            self.claims()?
                .active_tenant()
                .ok_or(ServicesServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> ServicesServiceResult<Vec<SelectOption>> {
        match ServicesSelectLists::from_str(select_list)? {
            ServicesSelectLists::Currencies => {
                Ok(CurrenciesRepository::get_all_countries_select_list_items(
                    self.module(),
                    self.claims()?
                        .active_tenant()
                        .ok_or(ServicesServiceError::Unauthorized)?,
                )
                .await?)
            }
            ServicesSelectLists::Taxes => Ok(TaxesRepository::get_select_list_items(
                self.module(),
                self.claims()?
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )
            .await?),
        }
    }

    async fn print(&self, payload: &[ServiceResolved]) -> ServicesServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::ServiceView,
            &payload,
        )?))
    }
}
