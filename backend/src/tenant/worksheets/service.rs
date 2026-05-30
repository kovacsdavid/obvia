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

use crate::common::MailTransporter;
use crate::common::dto::{GeneralError, PaginatorMeta};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::model::SelectOption;
use crate::common::pdf::{PdfGenError, PdfTemplates, gen_pdf_temporary};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::worksheets::WorksheetsModule;
use crate::tenant::worksheets::dto::WorksheetUserInput;
use crate::tenant::worksheets::model::{Worksheet, WorksheetResolved};
use crate::tenant::worksheets::types::worksheet::{WorksheetFilterBy, WorksheetOrderBy};
use axum::body::Bytes;
use axum::http::StatusCode;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum WorksheetsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

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

impl<H> IntoFriendlyError<GeneralError, H> for WorksheetsServiceError
where
    H: MailTransporter + ?Sized,
{
    async fn into_friendly_error(self, module: Arc<H>) -> FriendlyError<GeneralError> {
        match self {
            WorksheetsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: WorksheetsServiceError::Unauthorized.to_string(),
                },
            ),
            e => {
                FriendlyError::internal_with_admin_notify(
                    file!(),
                    GeneralError {
                        message: e.to_string(),
                    },
                    module,
                )
                .await
            }
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
    async fn insert(&self, payload: &WorksheetUserInput) -> WorksheetsServiceResult<Worksheet>;
    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> WorksheetsServiceResult<Vec<SelectOption>>;
    async fn get_resolved(&self, payload: Uuid) -> WorksheetsServiceResult<WorksheetResolved>;
    async fn get(&self, payload: Uuid) -> WorksheetsServiceResult<Worksheet>;
    async fn update(&self, payload: &WorksheetUserInput) -> WorksheetsServiceResult<Worksheet>;
    async fn delete(&self, payload: Uuid) -> WorksheetsServiceResult<()>;
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<WorksheetOrderBy, WorksheetFilterBy>,
    ) -> WorksheetsServiceResult<(PaginatorMeta, Vec<WorksheetResolved>)>;
    async fn print(&self, payload: &[WorksheetResolved]) -> WorksheetsServiceResult<Bytes>;
}

impl<'a, T> WorksheetService for Service<'a, T>
where
    T: WorksheetsModule + ?Sized,
{
    async fn insert(&self, payload: &WorksheetUserInput) -> WorksheetsServiceResult<Worksheet> {
        Ok(self
            .module()
            .worksheets_repo()
            .insert(
                payload.clone(),
                self.claims()?.sub(),
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )
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
                    .customers_repo()
                    .get_select_list_items(active_tenant)
                    .await?
            }
        })
    }
    async fn get_resolved(&self, payload: Uuid) -> WorksheetsServiceResult<WorksheetResolved> {
        Ok(self
            .module()
            .worksheets_repo()
            .get_resolved_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn get(&self, payload: Uuid) -> WorksheetsServiceResult<Worksheet> {
        Ok(self
            .module()
            .worksheets_repo()
            .get_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn update(&self, payload: &WorksheetUserInput) -> WorksheetsServiceResult<Worksheet> {
        Ok(self
            .module()
            .worksheets_repo()
            .update(
                payload.clone(),
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> WorksheetsServiceResult<()> {
        Ok(self
            .module()
            .worksheets_repo()
            .delete_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<WorksheetOrderBy, WorksheetFilterBy>,
    ) -> WorksheetsServiceResult<(PaginatorMeta, Vec<WorksheetResolved>)> {
        Ok(self
            .module()
            .worksheets_repo()
            .get_all_paged(
                get_query,
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )
            .await?)
    }

    async fn print(&self, payload: &[WorksheetResolved]) -> WorksheetsServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::WorksheetView,
            &payload,
        )?))
    }
}
