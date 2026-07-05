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

use crate::common::dto::PaginatorMeta;
use crate::common::error::RepositoryError;
use crate::common::error::v2::{AppError, AppErrorVisibility};
use crate::common::model::SelectOption;
#[double]
use crate::common::pdf::PdfGenerator;
use crate::common::pdf::{PdfGenError, PdfTemplates};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::taxes::TaxesModuleInterface;
use crate::tenant::taxes::dto::print::TaxResolvedPrint;
use crate::tenant::taxes::dto::user_input::TaxUserInput;
use crate::tenant::taxes::model::{Tax, TaxResolved};
use crate::tenant::taxes::types::{TaxFilterBy, TaxOrderBy};
use axum::body::Bytes;
use axum::http::StatusCode;
use mockall_double::double;
use serde_json::json;
use std::str::FromStr;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum TaxesServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

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

impl From<TaxesServiceError> for AppError {
    fn from(value: TaxesServiceError) -> Self {
        match value {
            TaxesServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            TaxesServiceError::TaxExists => Self::new(
                Level::DEBUG,
                StatusCode::CONFLICT,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            TaxesServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            TaxesServiceError::Repository(RepositoryError::Database(sqlx::Error::RowNotFound)) => {
                Self::new(
                    Level::DEBUG,
                    StatusCode::NOT_FOUND,
                    file!(),
                    AppErrorVisibility::UserFacing,
                    json!({"message": value.to_string()}),
                )
            }
            _ => Self::new(
                Level::ERROR,
                StatusCode::INTERNAL_SERVER_ERROR,
                file!(),
                AppErrorVisibility::Internal,
                json!({"message": value.to_string()}),
            ),
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
    fn insert(
        &self,
        payload: &TaxUserInput,
    ) -> impl Future<Output = TaxesServiceResult<Tax>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = TaxesServiceResult<TaxResolved>> + Send;
    fn get(&self, payload: Uuid) -> impl Future<Output = TaxesServiceResult<Tax>> + Send;
    fn update(
        &self,
        payload: &TaxUserInput,
    ) -> impl Future<Output = TaxesServiceResult<Tax>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = TaxesServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<TaxOrderBy, TaxFilterBy>,
    ) -> impl Future<Output = TaxesServiceResult<(PaginatorMeta, Vec<TaxResolved>)>> + Send;

    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = TaxesServiceResult<Vec<SelectOption>>> + Send;
    fn print(
        &self,
        payload: &[TaxResolvedPrint],
    ) -> impl Future<Output = TaxesServiceResult<Bytes>> + Send;
}

impl<'a, T> TaxService for Service<'a, T>
where
    T: TaxesModuleInterface,
{
    async fn insert(&self, payload: &TaxUserInput) -> TaxesServiceResult<Tax> {
        self.module()
            .taxes_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )?
            .insert(payload, self.claims()?.sub())
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
            .taxes_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }

    async fn get(&self, payload: Uuid) -> TaxesServiceResult<Tax> {
        Ok(self
            .module()
            .taxes_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }

    async fn update(&self, payload: &TaxUserInput) -> TaxesServiceResult<Tax> {
        if !payload.id.is_present() {
            return Err(TaxesServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .taxes_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )?
            .update(payload)
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> TaxesServiceResult<()> {
        Ok(self
            .module()
            .taxes_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<TaxOrderBy, TaxFilterBy>,
    ) -> TaxesServiceResult<(PaginatorMeta, Vec<TaxResolved>)> {
        Ok(self
            .module()
            .taxes_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TaxesServiceError::Unauthorized)?,
            )?
            .get_paged(get_query)
            .await?)
    }

    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> TaxesServiceResult<Vec<SelectOption>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(TaxesServiceError::Unauthorized)?;
        match TaxesSelectLists::from_str(select_list)? {
            TaxesSelectLists::Countries => Ok(self
                .module()
                .address_repo(active_tenant)?
                .get_all_countries_select_list_items()
                .await?),
        }
    }

    async fn print(&self, payload: &[TaxResolvedPrint]) -> TaxesServiceResult<Bytes> {
        Ok(Bytes::from(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::TaxView,
            payload.to_vec(),
        )?))
    }
}
