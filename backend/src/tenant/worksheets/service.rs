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
use crate::tenant::worksheets::WorksheetsModuleInterface;
use crate::tenant::worksheets::dto::print::WorksheetResolvedPrint;
use crate::tenant::worksheets::dto::user_input::WorksheetUserInput;
use crate::tenant::worksheets::model::{Worksheet, WorksheetResolved};
use crate::tenant::worksheets::types::worksheet::{WorksheetFilterBy, WorksheetOrderBy};
use axum::body::Bytes;
use axum::http::StatusCode;
use mockall_double::double;
use serde_json::json;
use std::str::FromStr;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum WorksheetsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

    #[error("A lista nem létezik")]
    InvalidSelectList,

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),
}

impl From<ServiceError> for WorksheetsServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => WorksheetsServiceError::Unauthorized,
        }
    }
}

impl From<WorksheetsServiceError> for AppError {
    fn from(value: WorksheetsServiceError) -> Self {
        match value {
            WorksheetsServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            WorksheetsServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            WorksheetsServiceError::Repository(RepositoryError::Database(
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

type WorksheetsServiceResult<T> = Result<T, WorksheetsServiceError>;

pub enum WorksheetsSelectLists {
    Customers,
}

impl FromStr for WorksheetsSelectLists {
    type Err = WorksheetsServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "customers" => Ok(Self::Customers),
            _ => Err(Self::Err::InvalidSelectList),
        }
    }
}

pub trait WorksheetService {
    fn insert(
        &self,
        payload: &WorksheetUserInput,
    ) -> impl Future<Output = WorksheetsServiceResult<Worksheet>> + Send;
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = WorksheetsServiceResult<Vec<SelectOption>>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = WorksheetsServiceResult<WorksheetResolved>> + Send;
    fn get(&self, payload: Uuid)
    -> impl Future<Output = WorksheetsServiceResult<Worksheet>> + Send;
    fn update(
        &self,
        payload: &WorksheetUserInput,
    ) -> impl Future<Output = WorksheetsServiceResult<Worksheet>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = WorksheetsServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<WorksheetOrderBy, WorksheetFilterBy>,
    ) -> impl Future<Output = WorksheetsServiceResult<(PaginatorMeta, Vec<WorksheetResolved>)>> + Send;
    fn print(
        &self,
        payload: &[WorksheetResolvedPrint],
    ) -> impl Future<Output = WorksheetsServiceResult<Bytes>> + Send;
}

impl<'a, T> WorksheetService for Service<'a, T>
where
    T: WorksheetsModuleInterface,
{
    async fn insert(&self, payload: &WorksheetUserInput) -> WorksheetsServiceResult<Worksheet> {
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .insert(payload.clone(), self.claims()?.sub())
            .await?)
    }

    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> WorksheetsServiceResult<Vec<SelectOption>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(WorksheetsServiceError::Unauthorized)?;
        Ok(match WorksheetsSelectLists::from_str(select_list)? {
            WorksheetsSelectLists::Customers => {
                self.module()
                    .customers_repo(active_tenant)?
                    .get_select_list_items()
                    .await?
            }
        })
    }
    async fn get_resolved(&self, payload: Uuid) -> WorksheetsServiceResult<WorksheetResolved> {
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }

    async fn get(&self, payload: Uuid) -> WorksheetsServiceResult<Worksheet> {
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }

    async fn update(&self, payload: &WorksheetUserInput) -> WorksheetsServiceResult<Worksheet> {
        if !payload.id.is_present() {
            return Err(WorksheetsServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .update(payload.clone())
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> WorksheetsServiceResult<()> {
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<WorksheetOrderBy, WorksheetFilterBy>,
    ) -> WorksheetsServiceResult<(PaginatorMeta, Vec<WorksheetResolved>)> {
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .get_paged(get_query)
            .await?)
    }

    async fn print(&self, payload: &[WorksheetResolvedPrint]) -> WorksheetsServiceResult<Bytes> {
        Ok(Bytes::from(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::WorksheetView,
            payload.to_vec(),
        )?))
    }
}
