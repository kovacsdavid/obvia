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
use crate::tenant::worksheets::WorksheetsModule;
use crate::tenant::worksheets::dto::CreateWorksheet;
use crate::tenant::worksheets::model::WorksheetResolved;
use crate::tenant::worksheets::repository::WorksheetsRepository;
use crate::tenant::worksheets::types::worksheet::WorksheetOrderBy;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum WorksheetsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("A lista nem létezik")]
    InvalidSelectList,
}

impl IntoResponse for WorksheetsServiceError {
    fn into_response(self) -> Response {
        match self {
            WorksheetsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                WorksheetsServiceError::Unauthorized.to_string(),
            ),
            e => FriendlyError::internal(file!(), e.to_string()),
        }
        .into_response()
    }
}

type WorksheetsServiceResult<T> = Result<T, WorksheetsServiceError>;

pub enum WorksheetsSelectLists {
    Projects,
}

impl FromStr for WorksheetsSelectLists {
    type Err = WorksheetsServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "projects" => Ok(Self::Projects),
            _ => Err(Self::Err::InvalidSelectList),
        }
    }
}

pub struct WorksheetsService;

impl WorksheetsService {
    pub async fn create(
        claims: &Claims,
        payload: &CreateWorksheet,
        worksheets_module: Arc<WorksheetsModule>,
    ) -> WorksheetsServiceResult<()> {
        worksheets_module
            .worksheets_repo
            .insert(
                payload.clone(),
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )
            .await?;
        Ok(())
    }
    pub async fn get_select_list_items(
        select_list: &str,
        claims: &Claims,
        worksheets_module: Arc<WorksheetsModule>,
    ) -> WorksheetsServiceResult<Vec<SelectOption>> {
        match WorksheetsSelectLists::from_str(select_list)? {
            WorksheetsSelectLists::Projects => Ok(worksheets_module
                .projects_repo
                .get_select_list_items(
                    claims
                        .active_tenant()
                        .ok_or(WorksheetsServiceError::Unauthorized)?,
                )
                .await?),
        }
    }
    pub async fn get_resolved_by_id(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn WorksheetsRepository>,
    ) -> WorksheetsServiceResult<WorksheetResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<WorksheetOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn WorksheetsRepository>,
    ) -> WorksheetsServiceResult<(PaginatorMeta, Vec<WorksheetResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )
            .await?)
    }
}
