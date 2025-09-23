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
use crate::common::error::RepositoryError;
use crate::manager::auth::dto::claims::Claims;
use crate::tenant::tasks::TasksModule;
use crate::tenant::tasks::dto::CreateTask;
use crate::tenant::worksheets::model::Worksheet;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TasksServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid state")]
    InvalidState,
}

type TasksServiceResult<T> = Result<T, TasksServiceError>;

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
    pub async fn get_all_worksheets(
        claims: &Claims,
        tasks_module: Arc<TasksModule>,
    ) -> TasksServiceResult<Vec<Worksheet>> {
        Ok(tasks_module
            .worksheets_repo
            .get_all(
                claims
                    .active_tenant()
                    .ok_or(TasksServiceError::Unauthorized)?,
            )
            .await?)
    }
}
