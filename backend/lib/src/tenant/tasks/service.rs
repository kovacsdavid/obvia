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
use crate::common::dto::{OrderingParams, PaginatorMeta, PaginatorParams, UuidParam};
use crate::common::error::{FriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::tasks::TasksModule;
use crate::tenant::tasks::dto::CreateTask;
use crate::tenant::tasks::model::TaskResolved;
use crate::tenant::tasks::repository::TasksRepository;
use crate::tenant::tasks::types::task::TaskOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum TasksServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Invalid state")]
    InvalidState,

    #[error("A lista nem létezik")]
    InvalidSelectList,
}

impl IntoResponse for TasksServiceError {
    fn into_response(self) -> Response {
        match self {
            TasksServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                TasksServiceError::Unauthorized.to_string(),
            ),
            e => FriendlyError::internal(file!(), e.to_string()),
        }
        .into_response()
    }
}

type TasksServiceResult<T> = Result<T, TasksServiceError>;

pub enum TasksSelectLists {
    Worksheets,
}

impl FromStr for TasksSelectLists {
    type Err = TasksServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "worksheets" => Ok(Self::Worksheets),
            _ => Err(Self::Err::InvalidSelectList),
        }
    }
}

pub struct TasksService;

impl TasksService {
    pub async fn create(
        claims: &Claims,
        payload: &CreateTask,
        tasks_module: Arc<TasksModule>,
    ) -> TasksServiceResult<()> {
        tasks_module
            .tasks_repo
            .insert(
                payload.clone(),
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(TasksServiceError::Unauthorized)?,
            )
            .await?;
        Ok(())
    }
    pub async fn get_select_list_items(
        select_list: &str,
        claims: &Claims,
        tasks_module: Arc<TasksModule>,
    ) -> TasksServiceResult<Vec<SelectOption>> {
        match TasksSelectLists::from_str(select_list)? {
            TasksSelectLists::Worksheets => Ok(tasks_module
                .worksheets_repo
                .get_select_list_items(
                    claims
                        .active_tenant()
                        .ok_or(TasksServiceError::Unauthorized)?,
                )
                .await?),
        }
    }
    pub async fn get_resolved_by_id(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn TasksRepository>,
    ) -> TasksServiceResult<TaskResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(TasksServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<TaskOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn TasksRepository>,
    ) -> TasksServiceResult<(PaginatorMeta, Vec<TaskResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(TasksServiceError::Unauthorized)?,
            )
            .await?)
    }
}
