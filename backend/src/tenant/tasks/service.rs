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
use crate::tenant::tasks::TasksModule;
use crate::tenant::tasks::dto::print::TaskResolvedPrint;
use crate::tenant::tasks::dto::user_input::TaskUserInput;
use crate::tenant::tasks::model::{Task, TaskResolved};
use crate::tenant::tasks::types::task::{TaskFilterBy, TaskOrderBy};
use axum::http::StatusCode;
use chrono::{DateTime, Duration, Utc};
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
pub enum TasksServiceError {
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

impl From<ServiceError> for TasksServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => TasksServiceError::Unauthorized,
        }
    }
}

impl From<TasksServiceError> for AppError {
    fn from(value: TasksServiceError) -> Self {
        match value {
            TasksServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            TasksServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            TasksServiceError::Repository(RepositoryError::Database(sqlx::Error::RowNotFound)) => {
                Self::new(
                    Level::DEBUG,
                    StatusCode::NOT_FOUND,
                    file!(),
                    AppErrorVisibility::UserFacing,
                    json!({"message": "Nem található"}),
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

type TasksServiceResult<T> = Result<T, TasksServiceError>;

pub enum TasksSelectLists {
    Worksheets,
    Services,
    Taxes,
    Currencies,
}

impl FromStr for TasksSelectLists {
    type Err = TasksServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "worksheets" => Ok(Self::Worksheets),
            "services" => Ok(Self::Services),
            "taxes" => Ok(Self::Taxes),
            "currencies" => Ok(Self::Currencies),
            _ => Err(Self::Err::InvalidSelectList),
        }
    }
}

pub trait TaskService {
    fn insert(
        &self,
        payload: &TaskUserInput,
    ) -> impl Future<Output = TasksServiceResult<Task>> + Send;
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = TasksServiceResult<Vec<SelectOption>>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = TasksServiceResult<TaskResolved>> + Send;
    fn get(&self, payload: Uuid) -> impl Future<Output = TasksServiceResult<Task>> + Send;
    fn update(
        &self,
        payload: &TaskUserInput,
    ) -> impl Future<Output = TasksServiceResult<Task>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = TasksServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<TaskOrderBy, TaskFilterBy>,
    ) -> impl Future<Output = TasksServiceResult<(PaginatorMeta, Vec<TaskResolved>)>> + Send;
    fn print(
        &self,
        payload: &[TaskResolvedPrint],
    ) -> impl Future<Output = TasksServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(&self, path: &Path) -> impl Future<Output = TasksServiceResult<()>> + Sync;
}

impl<'a, T> TaskService for Service<'a, T>
where
    T: TasksModule,
{
    async fn insert(&self, payload: &TaskUserInput) -> TasksServiceResult<Task> {
        Ok(self
            .module()
            .tasks_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TasksServiceError::Unauthorized)?,
            )?
            .insert(payload, self.claims()?.sub())
            .await?)
    }
    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> TasksServiceResult<Vec<SelectOption>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(TasksServiceError::Unauthorized)?;
        Ok(match TasksSelectLists::from_str(select_list)? {
            TasksSelectLists::Worksheets => {
                self.module()
                    .worksheets_repo(active_tenant)?
                    .get_select_list_items()
                    .await?
            }
            TasksSelectLists::Services => {
                self.module()
                    .services_repo(active_tenant)?
                    .get_select_list_items()
                    .await?
            }
            TasksSelectLists::Taxes => {
                self.module()
                    .taxes_repo(active_tenant)?
                    .get_select_list_items()
                    .await?
            }
            TasksSelectLists::Currencies => {
                self.module()
                    .currencies_repo(active_tenant)?
                    .get_all_countries_select_list_items()
                    .await?
            }
        })
    }
    async fn get_resolved(&self, payload: Uuid) -> TasksServiceResult<TaskResolved> {
        Ok(self
            .module()
            .tasks_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TasksServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }

    async fn get(&self, payload: Uuid) -> TasksServiceResult<Task> {
        Ok(self
            .module()
            .tasks_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TasksServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }
    async fn update(&self, payload: &TaskUserInput) -> TasksServiceResult<Task> {
        if !payload.id.is_present() {
            return Err(TasksServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .tasks_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TasksServiceError::Unauthorized)?,
            )?
            .update(payload)
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> TasksServiceResult<()> {
        Ok(self
            .module()
            .tasks_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TasksServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<TaskOrderBy, TaskFilterBy>,
    ) -> TasksServiceResult<(PaginatorMeta, Vec<TaskResolved>)> {
        Ok(self
            .module()
            .tasks_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(TasksServiceError::Unauthorized)?,
            )?
            .get_paged(get_query)
            .await?)
    }

    async fn print(&self, payload: &[TaskResolvedPrint]) -> TasksServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::TaskView,
            payload.to_vec(),
        )?)
    }
    async fn print_snapshot(&self, path: &Path) -> TasksServiceResult<()> {
        let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z"
            .parse()
            .map_err(|e: chrono::ParseError| TasksServiceError::ParseError(e.to_string()))?;
        let tz: Tz = "Europe/Budapest"
            .parse()
            .map_err(|e: chrono_tz::ParseError| TasksServiceError::ParseError(e.to_string()))?;
        let task_id = "4f321721-37c6-4e91-8e42-6281c36937bc"
            .parse()
            .map_err(|e: uuid::Error| TasksServiceError::ParseError(e.to_string()))?;
        let worksheet_id = "fd48ade1-a817-431b-8ada-6faea8c9f9dd"
            .parse()
            .map_err(|e: uuid::Error| TasksServiceError::ParseError(e.to_string()))?;
        let tax_id = "86097a0b-3f05-42f4-a98d-fd8a4669f02b"
            .parse()
            .map_err(|e: uuid::Error| TasksServiceError::ParseError(e.to_string()))?;
        let service_id = "ac55ca9c-2cd1-4cdf-8b44-ed4df798c750"
            .parse()
            .map_err(|e: uuid::Error| TasksServiceError::ParseError(e.to_string()))?;
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0"
            .parse()
            .map_err(|e: uuid::Error| TasksServiceError::ParseError(e.to_string()))?;

        let task_resolved = TaskResolved {
            id: task_id,
            worksheet_id,
            worksheet: "Test worksheet".to_string(),
            service_id,
            service: "Test service".to_string(),
            currency_code: "HUF".to_string(),
            quantity: None,
            price: None,
            tax_id,
            tax: "Test tax".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            status: "active".to_string(),
            priority: Some("normal".to_string()),
            due_date: Some(test_time + Duration::weeks(1)),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
            description: None,
        };
        let task_resolved_print = TaskResolvedPrint::from_task_resolved(task_resolved, tz);
        let pdf = self.print(&[task_resolved_print]).await?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
