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
use crate::tenant::taxes::TaxesModule;
use crate::tenant::taxes::dto::TaxUserInput;
use crate::tenant::taxes::model::{Tax, TaxResolved};
use crate::tenant::taxes::types::{TaxFilterBy, TaxOrderBy};
use axum::body::Bytes;
use axum::http::StatusCode;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

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

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),
}

impl From<ServiceError> for TaxesServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => TaxesServiceError::Unauthorized,
        }
    }
}

impl<H> IntoFriendlyError<GeneralError, H> for TaxesServiceError
where
    H: MailTransporter + ?Sized,
{
    async fn into_friendly_error(self, module: Arc<H>) -> FriendlyError<GeneralError> {
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

type TaxesServiceResult<T> = Result<T, TaxesServiceError>;

pub trait TaxService {
    async fn insert(&self, payload: &TaxUserInput) -> TaxesServiceResult<Tax>;
    async fn get_resolved(&self, payload: Uuid) -> TaxesServiceResult<TaxResolved>;
    async fn get(&self, payload: Uuid) -> TaxesServiceResult<Tax>;
    async fn update(&self, payload: &TaxUserInput) -> TaxesServiceResult<Tax>;
    async fn delete(&self, payload: Uuid) -> TaxesServiceResult<()>;
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<TaxOrderBy, TaxFilterBy>,
    ) -> TaxesServiceResult<(PaginatorMeta, Vec<TaxResolved>)>;

    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> TaxesServiceResult<Vec<SelectOption>>;
    async fn print(&self, payload: &[TaxResolved]) -> TaxesServiceResult<Bytes>;
}

impl<'a, T> TaxService for Service<'a, T>
where
    T: TaxesModule + ?Sized,
{
    async fn insert(&self, payload: &TaxUserInput) -> TaxesServiceResult<Tax> {
        self.module()
            .taxes_repo()
            .insert(
                payload,
                self.claims()?.sub(),
                self.claims()?
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

    async fn get_resolved(&self, payload: Uuid) -> TaxesServiceResult<TaxResolved> {
        Ok(self
            .module()
            .taxes_repo()
            .get_resolved_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn get(&self, payload: Uuid) -> TaxesServiceResult<Tax> {
        Ok(self
            .module()
            .taxes_repo()
            .get_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn update(&self, payload: &TaxUserInput) -> TaxesServiceResult<Tax> {
        Ok(self
            .module()
            .taxes_repo()
            .update(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> TaxesServiceResult<()> {
        Ok(self
            .module()
            .taxes_repo()
            .delete_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<TaxOrderBy, TaxFilterBy>,
    ) -> TaxesServiceResult<(PaginatorMeta, Vec<TaxResolved>)> {
        Ok(self
            .module()
            .taxes_repo()
            .get_all_paged(
                get_query,
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> TaxesServiceResult<Vec<SelectOption>> {
        match TaxesSelectLists::from_str(select_list)? {
            TaxesSelectLists::Countries => Ok(self
                .module()
                .address_repo()
                .get_all_countries_select_list_items(
                    self.claims()?
                        .active_tenant()
                        .ok_or(TaxesServiceError::Unauthorized)?,
                )
                .await?),
        }
    }

    async fn print(&self, payload: &[TaxResolved]) -> TaxesServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::TaxView,
            &payload,
        )?))
    }
}
