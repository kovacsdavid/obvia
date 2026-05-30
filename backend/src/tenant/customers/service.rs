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
use crate::common::pdf::{PdfGenError, PdfTemplates, gen_pdf_temporary};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::tenant::customers::CustomersModule;
use crate::tenant::customers::dto::CustomerUserInput;
use crate::tenant::customers::model::{Customer, CustomerResolved};
use crate::tenant::customers::types::customer::{CustomerFilterBy, CustomerOrderBy};
use axum::body::Bytes;
use axum::http::StatusCode;
use std::sync::Arc;
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

impl<H> IntoFriendlyError<GeneralError, H> for CustomersServiceError
where
    H: MailTransporter + ?Sized,
{
    async fn into_friendly_error(self, module: Arc<H>) -> FriendlyError<GeneralError> {
        match self {
            CustomersServiceError::Unauthorized | CustomersServiceError::CustomerExists => {
                FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    GeneralError {
                        message: self.to_string(),
                    },
                )
            }
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

type CustomersServiceResult<T> = Result<T, CustomersServiceError>;

pub trait CustomerService {
    async fn insert(&self, payload: &CustomerUserInput) -> CustomersServiceResult<Customer>;
    async fn get_resolved(&self, payload: Uuid) -> CustomersServiceResult<CustomerResolved>;
    async fn get(&self, payload: Uuid) -> CustomersServiceResult<Customer>;
    async fn update(&self, payload: &CustomerUserInput) -> CustomersServiceResult<Customer>;
    async fn delete(&self, payload: Uuid) -> CustomersServiceResult<()>;
    async fn get_paged(
        &self,
        get_query: &ResourceQuery<CustomerOrderBy, CustomerFilterBy>,
    ) -> CustomersServiceResult<(PaginatorMeta, Vec<CustomerResolved>)>;
    async fn print(&self, payload: &[CustomerResolved]) -> CustomersServiceResult<Bytes>;
}

impl<'a, T> CustomerService for Service<'a, T>
where
    T: CustomersModule + ?Sized,
{
    async fn insert(&self, payload: &CustomerUserInput) -> CustomersServiceResult<Customer> {
        self.module()
            .customers_repo()
            .insert(
                payload,
                self.claims()?.sub(),
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )
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
            .customers_repo()
            .get_resolved_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn get(&self, payload: Uuid) -> CustomersServiceResult<Customer> {
        Ok(self
            .module()
            .customers_repo()
            .get_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn update(&self, payload: &CustomerUserInput) -> CustomersServiceResult<Customer> {
        Ok(self
            .module()
            .customers_repo()
            .update(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> CustomersServiceResult<()> {
        Ok(self
            .module()
            .customers_repo()
            .delete_by_id(
                payload,
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn get_paged(
        &self,
        query: &ResourceQuery<CustomerOrderBy, CustomerFilterBy>,
    ) -> CustomersServiceResult<(PaginatorMeta, Vec<CustomerResolved>)> {
        Ok(self
            .module()
            .customers_repo()
            .get_paged(
                query,
                self.claims()?
                    .active_tenant()
                    .ok_or(CustomersServiceError::Unauthorized)?,
            )
            .await?)
    }
    async fn print(&self, payload: &[CustomerResolved]) -> CustomersServiceResult<Bytes> {
        Ok(Bytes::from(gen_pdf_temporary(
            &PdfTemplates::CustomerView,
            &payload,
        )?))
    }
}
