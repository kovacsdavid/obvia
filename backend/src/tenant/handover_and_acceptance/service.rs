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
use mockall_double::double;
use serde_json::json;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[double]
use crate::common::pdf::PdfGenerator;
use crate::{
    common::{
        CommonBuilderError,
        dto::PaginatorMeta,
        error::{
            RepositoryError,
            v2::{AppError, AppErrorVisibility},
        },
        pdf::{PdfGenError, PdfTemplates},
        query_parser::ResourceQuery,
        service::{BaseService, BaseServiceError},
    },
    manager::auth::dto::claims::ClaimsError,
    tenant::{
        customers::dto::print::test_customer_full_print_builder,
        handover_and_acceptance::{
            self, ModuleInterface, Print, UserInput,
            model::{HandoverAndAcceptance, HandoverAndAcceptanceFull},
            test_handover_and_acceptance_full_print_builder,
        },
        owner_profile::dto::print::{OwnerProfileFullPrint, test_owner_profile_full_print_builder},
    },
};

#[derive(Debug, Error)]
pub enum ServiceError {
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

    #[error("Claims error: {0}")]
    ClaimsError(#[from] ClaimsError),
}

impl From<BaseServiceError> for ServiceError {
    fn from(value: BaseServiceError) -> Self {
        match value {
            BaseServiceError::Unauthorized => ServiceError::Unauthorized,
        }
    }
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            ServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            ServiceError::Repository(RepositoryError::Database(sqlx::Error::RowNotFound)) => {
                Self::new(
                    Level::DEBUG,
                    StatusCode::NOT_FOUND,
                    AppErrorVisibility::UserFacing,
                    json!({"message": "Nem található"}),
                )
            }
            _ => Self::new(
                Level::ERROR,
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorVisibility::Internal,
                json!({"message": value.to_string()}),
            ),
        }
    }
}

type ServiceResult<T> = Result<T, ServiceError>;

pub trait Service {
    fn insert(
        &self,
        payload: &UserInput,
    ) -> impl Future<Output = ServiceResult<HandoverAndAcceptance>> + Send;
    fn get_full(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = ServiceResult<HandoverAndAcceptanceFull>> + Send;
    fn update(
        &self,
        payload: &UserInput,
    ) -> impl Future<Output = ServiceResult<HandoverAndAcceptance>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = ServiceResult<()>>;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<
            handover_and_acceptance::types::OrderBy,
            handover_and_acceptance::types::FilterBy,
        >,
    ) -> impl Future<Output = ServiceResult<(PaginatorMeta, Vec<HandoverAndAcceptanceFull>)>> + Send;
    fn print(&self, payload: Uuid) -> impl Future<Output = ServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(&self, path: &Path) -> impl Future<Output = ServiceResult<()>> + Send;
}

impl<'a, T> Service for BaseService<'a, T>
where
    T: ModuleInterface,
{
    async fn insert(&self, payload: &UserInput) -> ServiceResult<HandoverAndAcceptance> {
        Ok(self
            .module()
            .handover_and_acceptance_repo(self.active_tenant()?)?
            .insert(payload, self.claims()?.sub())
            .await?)
    }

    async fn get_full(&self, payload: Uuid) -> ServiceResult<HandoverAndAcceptanceFull> {
        Ok(self
            .module()
            .handover_and_acceptance_repo(self.active_tenant()?)?
            .get_full(payload)
            .await?)
    }

    async fn update(&self, payload: &UserInput) -> ServiceResult<HandoverAndAcceptance> {
        if !payload.id.is_present() {
            return Err(ServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .handover_and_acceptance_repo(self.active_tenant()?)?
            .update(payload, self.claims()?.sub())
            .await?)
    }

    async fn delete(&self, payload: Uuid) -> ServiceResult<()> {
        Ok(self
            .module()
            .handover_and_acceptance_repo(self.active_tenant()?)?
            .delete_by_id(payload)
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<
            handover_and_acceptance::types::OrderBy,
            handover_and_acceptance::types::FilterBy,
        >,
    ) -> ServiceResult<(PaginatorMeta, Vec<HandoverAndAcceptanceFull>)> {
        Ok(self
            .module()
            .handover_and_acceptance_repo(self.active_tenant()?)?
            .get_paged(get_query)
            .await?)
    }

    async fn print(&self, payload: Uuid) -> ServiceResult<Vec<u8>> {
        let active_tenant = self.active_tenant()?;
        let tz = self.claims()?.tz()?;

        let owner_profile_print = OwnerProfileFullPrint::new(
            self.module()
                .owner_profile_repo(active_tenant)?
                .get_full()
                .await?,
            tz,
        );

        let print = Print::new(self.get_full(payload).await?, owner_profile_print, tz);
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::HandoverAndAcceptanceView,
            print,
        )?)
    }

    async fn print_snapshot(&self, path: &Path) -> ServiceResult<()> {
        let handover_and_acceptance_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse()?;
        let owner_profile_id = "de6c4563-640e-483c-ad2d-d47b1c9115b7".parse()?;
        let customer_id = "01a1f891-a6e5-733e-b525-41a33aa00cb3".parse().unwrap();
        let owner_profile_full_print = test_owner_profile_full_print_builder()
            .id(owner_profile_id)
            .build()?;
        let handover_and_acceptance_resolved_print =
            test_handover_and_acceptance_full_print_builder(owner_profile_full_print)
                .id(handover_and_acceptance_id)
                .customer(
                    test_customer_full_print_builder()
                        .id(customer_id)
                        .build()
                        .unwrap(),
                )
                .build()?;
        let pdf = PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::HandoverAndAcceptanceView,
            handover_and_acceptance_resolved_print,
        )?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
