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
use crate::manager::common::dto::{OrderingParams, PagedData, PaginatorParams};
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::projects::model::Project;
use crate::tenant::worksheets::WorksheetsModule;
use crate::tenant::worksheets::dto::CreateWorksheet;
use crate::tenant::worksheets::model::Worksheet;
use crate::tenant::worksheets::repository::WorksheetsRepository;
use crate::tenant::worksheets::types::worksheet::WorksheetOrderBy;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorksheetsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Unauthorized")]
    Unauthorized,
}

type WorksheetsServiceResult<T> = Result<T, WorksheetsServiceError>;

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
    pub async fn get_all_projects(
        claims: &Claims,
        worksheets_module: Arc<WorksheetsModule>,
    ) -> WorksheetsServiceResult<Vec<Project>> {
        Ok(worksheets_module
            .projects_repo
            .get_all(
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
    ) -> WorksheetsServiceResult<PagedData<Vec<Worksheet>>> {
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
