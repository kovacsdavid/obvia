/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

use std::{fs::File, io::Write, path::Path};

use axum::http::StatusCode;
use serde_json::json;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

use crate::{
    common::{
        CommonBuilderError,
        error::{
            RepositoryError,
            v2::{AppError, AppErrorVisibility},
        },
        pdf::{PdfGenError, PdfGenerator, PdfTemplates},
        query_parser::ResourceQuery,
        service::ServiceError,
    },
    tenant::{
        handover_and_acceptance::HandoverAndAcceptanceModuleInterface, services::model::Service,
    },
};

#[derive(Debug, Error)]
pub enum HandoverAndAcceptanceServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("BuilderError: {0}")]
    BuilderError(#[from] CommonBuilderError),

    #[error("UuidError: {0}")]
    UuidError(#[from] uuid::Error),
}

impl From<ServiceError> for HandoverAndAcceptanceServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => HandoverAndAcceptanceServiceError::Unauthorized,
        }
    }
}

impl From<HandoverAndAcceptanceServiceError> for AppError {
    fn from(value: HandoverAndAcceptanceServiceError) -> Self {
        match value {
            HandoverAndAcceptanceServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            HandoverAndAcceptanceServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            HandoverAndAcceptanceServiceError::Repository(RepositoryError::Database(
                sqlx::Error::RowNotFound,
            )) => Self::new(
                Level::DEBUG,
                StatusCode::NOT_FOUND,
                AppErrorVisibility::UserFacing,
                json!({"message": "Nem található"}),
            ),
            _ => Self::new(
                Level::ERROR,
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorVisibility::Internal,
                json!({"message": value.to_string()}),
            ),
        }
    }
}

type HandoverAndAcceptanceServiceResult<T> = Result<T, HandoverAndAcceptanceServiceError>;

pub trait HandoverAndAcceptanceService {
    fn insert(
        &self,
        payload: &HandoverAndAcceptanceUserInput,
    ) -> impl Future<Output = HandoverAndAcceptanceServiceResult<HandoverAndAcceptance>> + Send;
    fn get_full(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = HandoverAndAcceptanceServiceResult<HandoverAndAcceptanceFull>> + Send;
    fn update(
        &self,
        payload: &HandoverAndAcceptanceUserInput,
    ) -> impl Future<Output = HandoverAndAcceptanceServiceResult<HandoverAndAcceptance>> + Send;
    fn delete(&self, payload: Uuid)
    -> impl Future<Output = HandoverAndAcceptanceServiceResult<()>>;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<HandoverAndAcceptanceOrderBy, HandoverAndAcceptanceFilterBy>,
    ) -> impl Future<
        Output = HandoverAndAcceptanceServiceResult<(
            PaginatorMeta,
            Vec<HandoverAndAcceptanceFull>,
        )>,
    > + Send;
    fn print(
        &self,
        payload: &[HandoverAndAcceptanceFullPrint],
    ) -> impl Future<Output = HandoverAndAcceptanceServiceResult<Vec<u8>>> + Sync;
    fn print_snapshot(
        &self,
        path: &Path,
    ) -> impl Future<Output = HandoverAndAcceptanceServiceResult<()>> + Sync;
}

impl<'a, T> HandoverAndAcceptanceService for Service<'a, T>
where
    T: HandoverAndAcceptanceModuleInterface,
{
    async fn insert(
        &self,
        payload: &HandoverAndAcceptanceUserInput,
    ) -> HandoverAndAcceptanceServiceResult<HandoverAndAcceptance> {
        self.module()
            .handover_and_acceptance_repo(self.active_tenant()?)
            .insert(payload, self.claims()?.sub())
            .await?;
    }

    async fn get_full(
        &self,
        payload: Uuid,
    ) -> HandoverAndAcceptanceServiceResult<HandoverAndAcceptanceFull> {
        Ok(self
            .module()
            .handover_and_acceptance_repo(self.active_tenant()?)?
            .get_full(payload)
            .await?)
    }

    async fn update(
        &self,
        payload: &HandoverAndAcceptanceUserInput,
    ) -> HandoverAndAcceptanceServiceResult<HandoverAndAcceptance> {
        if !payload.id.is_present() {
            return Err(HandoverAndAcceptanceServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .handover_and_acceptance_repo(self.active_tenant()?)?
            .update(payload, self.claims()?.sub())
            .await?)
    }

    async fn delete(&self, payload: Uuid) -> HandoverAndAcceptanceServiceResult<()> {
        Ok(self
            .module()
            .handover_and_acceptance_repo(self.active_tenant()?)?
            .delete_by_id(payload)
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<HandoverAndAcceptanceOrderBy, HandoverAndAcceptanceFilterBy>,
    ) -> HandoverAndAcceptanceServiceResult<(PaginatorMeta, Vec<HandoverAndAcceptanceFull>)> {
        Ok(self
            .module()
            .handover_and_acceptance_repo(self.active_tenant()?)?
            .get_paged(get_query)
            .await?)
    }

    async fn print(
        &self,
        payload: &[HandoverAndAcceptanceFullPrint],
    ) -> HandoverAndAcceptanceServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::HandoverAndAcceptance,
            payload.to_vec(),
        )?)
    }

    async fn print_snapshot(&self, path: &Path) -> HandoverAndAcceptanceServiceResult<()> {
        let handover_and_acceptance_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse()?;
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0".parse()?;
        let handover_and_acceptance_resolved_print =
            test_handover_and_acceptance_full_print_builder()
                .id(handover_and_acceptance_id)
                .created_by_id(created_by_id)
                .build()?;
        let pdf = self
            .print(&[handover_and_acceptance_resolved_print])
            .await?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
