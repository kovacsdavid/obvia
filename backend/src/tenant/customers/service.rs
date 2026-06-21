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

use std::sync::Arc;

use crate::common::BaseModule;
use crate::common::dto::{GeneralError, PaginatorMeta};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::pdf::{PdfGenError, PdfTemplates, gen_pdf_temporary};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::customers::CustomersModuleInterface;
use crate::tenant::customers::dto::print::CustomerResolvedPrint;
use crate::tenant::customers::dto::user_input::CustomerUserInput;
use crate::tenant::customers::model::{Customer, CustomerResolved};
use crate::tenant::customers::types::customer::{CustomerFilterBy, CustomerOrderBy};
use axum::body::Bytes;
use axum::http::StatusCode;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CustomersServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

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

impl IntoFriendlyError for CustomersServiceError {
    async fn into_friendly_error<M>(self, mailer: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            CustomersServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: self.to_string(),
                }
                .to_string(),
            ),
            CustomersServiceError::CustomerExists => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::CONFLICT,
                file!(),
                GeneralError {
                    message: self.to_string(),
                }
                .to_string(),
            ),
            CustomersServiceError::Repository(RepositoryError::Database(
                sqlx::Error::RowNotFound,
            )) => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::NOT_FOUND,
                file!(),
                GeneralError {
                    message: self.to_string(),
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
                    mailer,
                )
                .await
            }
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
    ) -> impl Future<Output = CustomersServiceResult<Bytes>> + Sync;
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
    async fn print(&self, payload: &[CustomerResolvedPrint]) -> CustomersServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::CustomerView,
            &payload,
        )?))
    }
}
