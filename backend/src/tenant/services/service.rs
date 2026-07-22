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
use crate::tenant::services::ServicesModule;
use crate::tenant::services::dto::print::ServicesResolvedPrint;
use crate::tenant::services::dto::user_input::ServiceUserInput;
use crate::tenant::services::model::{Service as ServiceModel, ServiceResolved};
use crate::tenant::services::types::service::{ServiceFilterBy, ServiceOrderBy};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use mockall_double::double;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
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

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

    #[error("A lista nem létezik")]
    InvalidSelectList,

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

impl From<ServiceError> for ServicesServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => ServicesServiceError::Unauthorized,
        }
    }
}

impl From<ServicesServiceError> for AppError {
    fn from(value: ServicesServiceError) -> Self {
        match value {
            ServicesServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            ServicesServiceError::ServiceExists => Self::new(
                Level::DEBUG,
                StatusCode::CONFLICT,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            ServicesServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            ServicesServiceError::Repository(RepositoryError::Database(
                sqlx::Error::RowNotFound,
            )) => Self::new(
                Level::DEBUG,
                StatusCode::NOT_FOUND,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": "Nem található"}),
            ),
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
        payload: &[ServicesResolvedPrint],
    ) -> impl Future<Output = ServicesServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(&self, path: &Path)
    -> impl Future<Output = ServicesServiceResult<()>> + Sync;
}

impl<'a, T> ServiceService for Service<'a, T>
where
    T: ServicesModule,
{
    async fn insert(&self, payload: &ServiceUserInput) -> ServicesServiceResult<ServiceModel> {
        self.module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )?
            .insert(payload, self.claims()?.sub())
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
        Ok(self
            .module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }

    async fn get(&self, payload: Uuid) -> ServicesServiceResult<ServiceModel> {
        Ok(self
            .module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }

    async fn update(&self, payload: &ServiceUserInput) -> ServicesServiceResult<ServiceModel> {
        if !payload.id.is_present() {
            return Err(ServicesServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )?
            .update(payload)
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> ServicesServiceResult<()> {
        Ok(self
            .module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<ServiceOrderBy, ServiceFilterBy>,
    ) -> ServicesServiceResult<(PaginatorMeta, Vec<ServiceResolved>)> {
        Ok(self
            .module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServicesServiceError::Unauthorized)?,
            )?
            .get_paged(get_query)
            .await?)
    }

    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> ServicesServiceResult<Vec<SelectOption>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(ServicesServiceError::Unauthorized)?;
        match ServicesSelectLists::from_str(select_list)? {
            ServicesSelectLists::Currencies => Ok(self
                .module()
                .currencies_repo(active_tenant)?
                .get_all_countries_select_list_items()
                .await?),
            ServicesSelectLists::Taxes => Ok(self
                .module()
                .taxes_repo(active_tenant)?
                .get_select_list_items()
                .await?),
        }
    }

    async fn print(&self, payload: &[ServicesResolvedPrint]) -> ServicesServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::ServiceView,
            payload.to_vec(),
        )?)
    }
    async fn print_snapshot(&self, path: &Path) -> ServicesServiceResult<()> {
        let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z"
            .parse()
            .map_err(|e: chrono::ParseError| ServicesServiceError::ParseError(e.to_string()))?;
        let tz: Tz = "Europe/Budapest"
            .parse()
            .map_err(|e: chrono_tz::ParseError| ServicesServiceError::ParseError(e.to_string()))?;
        let service_id = "4f321721-37c6-4e91-8e42-6281c36937bc"
            .parse()
            .map_err(|e: uuid::Error| ServicesServiceError::ParseError(e.to_string()))?;
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0"
            .parse()
            .map_err(|e: uuid::Error| ServicesServiceError::ParseError(e.to_string()))?;
        let service_resolved = ServiceResolved {
            id: service_id,
            name: "Test Service".to_string(),
            description: Some("Test description".to_string()),
            default_price: None,
            default_tax_id: None,
            default_tax: None,
            currency_code: Some("HUF".to_string()),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
        };
        let service_resolved_print =
            ServicesResolvedPrint::from_service_resolved(service_resolved, tz);
        let pdf = self.print(&[service_resolved_print]).await?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
