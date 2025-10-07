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
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::projects::ProjectsModule;
use crate::tenant::projects::dto::ProjectUserInput;
use crate::tenant::projects::model::{Project, ProjectResolved};
use crate::tenant::projects::repository::ProjectsRepository;
use crate::tenant::projects::types::project::ProjectOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum ProjectsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,
}

impl IntoResponse for ProjectsServiceError {
    fn into_response(self) -> Response {
        match self {
            ProjectsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                ProjectsServiceError::Unauthorized.to_string(),
            ),
            e => FriendlyError::internal(file!(), e.to_string()),
        }
        .into_response()
    }
}

type ProjectsServiceResult<T> = Result<T, ProjectsServiceError>;

pub struct ProjectsService;

impl ProjectsService {
    pub async fn create(
        claims: &Claims,
        payload: &ProjectUserInput,
        projects_module: Arc<ProjectsModule>,
    ) -> ProjectsServiceResult<()> {
        projects_module
            .projects_repo
            .insert(
                payload.clone(),
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(ProjectsServiceError::Unauthorized)?,
            )
            .await?;
        Ok(())
    }
    pub async fn get_resolved_by_id(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn ProjectsRepository>,
    ) -> ProjectsServiceResult<ProjectResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(ProjectsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn ProjectsRepository>,
    ) -> ProjectsServiceResult<Project> {
        Ok(repo
            .get_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(ProjectsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn update(
        claims: &Claims,
        payload: &ProjectUserInput,
        repo: Arc<dyn ProjectsRepository>,
    ) -> ProjectsServiceResult<Project> {
        Ok(repo
            .update(
                payload.clone(),
                claims
                    .active_tenant()
                    .ok_or(ProjectsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn delete(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn ProjectsRepository>,
    ) -> ProjectsServiceResult<()> {
        Ok(repo
            .delete_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(ProjectsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<ProjectOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn ProjectsRepository>,
    ) -> ProjectsServiceResult<(PaginatorMeta, Vec<ProjectResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(ProjectsServiceError::Unauthorized)?,
            )
            .await?)
    }
}
