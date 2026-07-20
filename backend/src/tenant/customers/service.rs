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
#[double]
use crate::common::pdf::PdfGenerator;
use crate::common::pdf::{PdfGenError, PdfTemplates};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::customers::CustomersModuleInterface;
use crate::tenant::customers::dto::print::CustomerResolvedPrint;
use crate::tenant::customers::dto::user_input::CustomerUserInput;
use crate::tenant::customers::model::{Customer, CustomerResolved};
use crate::tenant::customers::types::customer::{CustomerFilterBy, CustomerOrderBy};
use axum::http::StatusCode;
use mockall_double::double;
use serde_json::json;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CustomersServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

    #[error("A megadot e-mail címmel már létezik vevő a rendszerben!")]
    CustomerExists,

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),
}

impl From<ServiceError> for CustomersServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => CustomersServiceError::Unauthorized,
        }
    }
}

impl From<CustomersServiceError> for AppError {
    fn from(value: CustomersServiceError) -> Self {
        match value {
            CustomersServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            CustomersServiceError::CustomerExists => Self::new(
                Level::DEBUG,
                StatusCode::CONFLICT,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            CustomersServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            CustomersServiceError::Repository(RepositoryError::Database(
                sqlx::Error::RowNotFound,
            )) => Self::new(
                Level::DEBUG,
                StatusCode::NOT_FOUND,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": "Nem található"}),
            ),
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

type CustomersServiceResult<T> = Result<T, CustomersServiceError>;

pub trait CustomerService {
    fn insert(
        &self,
        payload: &CustomerUserInput,
    ) -> impl Future<Output = CustomersServiceResult<Customer>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = CustomersServiceResult<CustomerResolved>> + Send;
    fn get(&self, payload: Uuid) -> impl Future<Output = CustomersServiceResult<Customer>> + Send;
    fn update(
        &self,
        payload: &CustomerUserInput,
    ) -> impl Future<Output = CustomersServiceResult<Customer>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = CustomersServiceResult<()>>;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<CustomerOrderBy, CustomerFilterBy>,
    ) -> impl Future<Output = CustomersServiceResult<(PaginatorMeta, Vec<CustomerResolved>)>> + Send;
    fn print(
        &self,
        payload: &[CustomerResolvedPrint],
    ) -> impl Future<Output = CustomersServiceResult<Vec<u8>>> + Sync;
}

impl<'a, T> CustomerService for Service<'a, T>
where
    T: CustomersModuleInterface,
{
    async fn insert(&self, payload: &CustomerUserInput) -> CustomersServiceResult<Customer> {
        self.module()
            .customers_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )?
            .insert(payload, self.claims()?.sub())
            .await
            .map_err(|e| {
                if e.is_unique_violation() {
                    CustomersServiceError::CustomerExists
                } else {
                    e.into()
                }
            })
    }
    async fn get_resolved(&self, payload: Uuid) -> CustomersServiceResult<CustomerResolved> {
        Ok(self
            .module()
            .customers_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }
    async fn get(&self, payload: Uuid) -> CustomersServiceResult<Customer> {
        Ok(self
            .module()
            .customers_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }
    async fn update(&self, payload: &CustomerUserInput) -> CustomersServiceResult<Customer> {
        if !payload.id.is_present() {
            return Err(CustomersServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .customers_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )?
            .update(payload)
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> CustomersServiceResult<()> {
        Ok(self
            .module()
            .customers_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }
    async fn get_paged(
        &self,
        query: &ResourceQuery<CustomerOrderBy, CustomerFilterBy>,
    ) -> CustomersServiceResult<(PaginatorMeta, Vec<CustomerResolved>)> {
        Ok(self
            .module()
            .customers_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )?
            .get_paged(query)
            .await?)
    }
    async fn print(&self, payload: &[CustomerResolvedPrint]) -> CustomersServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::CustomerView,
            payload.to_vec(),
        )?)
    }
}
