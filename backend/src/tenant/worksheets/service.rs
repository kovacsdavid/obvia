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

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
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
    ) -> impl Future<Output = WorksheetsServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(
        &self,
        path: &Path,
    ) -> impl Future<Output = WorksheetsServiceResult<()>> + Sync;
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

    async fn print(&self, payload: &[WorksheetResolvedPrint]) -> WorksheetsServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::WorksheetView,
            payload.to_vec(),
        )?)
    }
    async fn print_snapshot(&self, path: &Path) -> WorksheetsServiceResult<()> {
        let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z"
            .parse()
            .map_err(|e: chrono::ParseError| WorksheetsServiceError::ParseError(e.to_string()))?;
        let tz: Tz = "Europe/Budapest"
            .parse()
            .map_err(|e: chrono_tz::ParseError| {
                WorksheetsServiceError::ParseError(e.to_string())
            })?;
        let worksheet_id = "4f321721-37c6-4e91-8e42-6281c36937bc"
            .parse()
            .map_err(|e: uuid::Error| WorksheetsServiceError::ParseError(e.to_string()))?;
        let customer_id = "fd48ade1-a817-431b-8ada-6faea8c9f9dd"
            .parse()
            .map_err(|e: uuid::Error| WorksheetsServiceError::ParseError(e.to_string()))?;
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0"
            .parse()
            .map_err(|e: uuid::Error| WorksheetsServiceError::ParseError(e.to_string()))?;

        let worksheet_resolved = WorksheetResolved {
            id: worksheet_id,
            name: "Test worksheet".to_string(),
            description: None,
            customer_id,
            customer: "Test customer".to_string(),
            project_id: None,
            project: None,
            created_by_id,
            created_by: "Test user".to_string(),
            status: "active".to_string(),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
            net_material_cost: "10".parse().unwrap(),
            gross_material_cost: "20".parse().unwrap(),
            net_work_cost: "30".parse().unwrap(),
            gross_work_cost: "40".parse().unwrap(),
        };
        let worksheet_resolved_print =
            WorksheetResolvedPrint::from_worksheet_resolved(worksheet_resolved, tz);
        let pdf = self.print(&[worksheet_resolved_print]).await?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
