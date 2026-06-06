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
use crate::tenant::services::repository::ServicesRepository;
use crate::tenant::tasks::TasksModule;
use crate::tenant::tasks::dto::TaskUserInput;
use crate::tenant::tasks::model::{Task, TaskResolved};
use crate::tenant::tasks::repository::TasksRepository;
use crate::tenant::tasks::types::task::{TaskFilterBy, TaskOrderBy};
use crate::tenant::taxes::repository::TaxesRepository;
use crate::tenant::worksheets::repository::WorksheetsRepository;
use axum::body::Bytes;
use axum::http::StatusCode;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum TasksServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("A lista nem létezik")]
    InvalidSelectList,

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),
}

impl From<ServiceError> for TasksServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => TasksServiceError::Unauthorized,
        }
    }
}

impl IntoFriendlyError for TasksServiceError {
    async fn into_friendly_error<M>(self, module: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            TasksServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: TasksServiceError::Unauthorized.to_string(),
                }
                .to_string(),
            ),
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
        payload: &[TaskResolved],
    ) -> impl Future<Output = TasksServiceResult<Bytes>> + Send;
}

impl<'a, T> TaskService for Service<'a, T>
where
    T: TasksModule,
{
    async fn insert(&self, payload: &TaskUserInput) -> TasksServiceResult<Task> {
        Ok(TasksRepository::insert(
            self.module(),
            payload,
            self.claims()?.sub(),
            self.claims()?
                .active_tenant()
                .ok_or(TasksServiceError::Unauthorized)?,
        )
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
                WorksheetsRepository::get_select_list_items(self.module(), active_tenant).await?
            }
            TasksSelectLists::Services => {
                ServicesRepository::get_select_list_items(self.module(), active_tenant).await?
            }
            TasksSelectLists::Taxes => {
                TaxesRepository::get_select_list_items(self.module(), active_tenant).await?
            }
            TasksSelectLists::Currencies => {
                CurrenciesRepository::get_all_countries_select_list_items(
                    self.module(),
                    active_tenant,
                )
                .await?
            }
        })
    }
    async fn get_resolved(&self, payload: Uuid) -> TasksServiceResult<TaskResolved> {
        Ok(TasksRepository::get_resolved_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(TasksServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn get(&self, payload: Uuid) -> TasksServiceResult<Task> {
        Ok(TasksRepository::get_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(TasksServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn update(&self, payload: &TaskUserInput) -> TasksServiceResult<Task> {
        Ok(TasksRepository::update(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(TasksServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn delete(&self, payload: Uuid) -> TasksServiceResult<()> {
        Ok(TasksRepository::delete_by_id(
            self.module(),
            payload,
            self.claims()?
                .active_tenant()
                .ok_or(TasksServiceError::Unauthorized)?,
        )
        .await?)
    }
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<TaskOrderBy, TaskFilterBy>,
    ) -> TasksServiceResult<(PaginatorMeta, Vec<TaskResolved>)> {
        Ok(TasksRepository::get_all_paged(
            self.module(),
            get_query,
            self.claims()?
                .active_tenant()
                .ok_or(TasksServiceError::Unauthorized)?,
        )
        .await?)
    }

    async fn print(&self, payload: &[TaskResolved]) -> TasksServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::TaskView,
            &payload,
        )?))
    }
}
