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

use crate::common::CommonBuilderError;
use crate::common::error::RepositoryError;
use crate::common::error::v2::{AppError, AppErrorVisibility};
use crate::common::service::{Service, ServiceError};
use crate::tenant::owner_profile::OwnerProfileModuleInterface;
use crate::tenant::owner_profile::dto::user_input::OwnerProfileUserInput;
use crate::tenant::owner_profile::model::{OwnerProfile, OwnerProfileFull};
use axum::http::StatusCode;
use serde_json::json;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum OwnerProfileServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("BuilderError: {0}")]
    BuilderError(#[from] CommonBuilderError),

    #[error("UuidError: {0}")]
    UuidError(#[from] uuid::Error),
}

impl From<ServiceError> for OwnerProfileServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => OwnerProfileServiceError::Unauthorized,
        }
    }
}

impl From<OwnerProfileServiceError> for AppError {
    fn from(value: OwnerProfileServiceError) -> Self {
        match value {
            OwnerProfileServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            OwnerProfileServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            OwnerProfileServiceError::Repository(RepositoryError::Database(
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

type OwnerProfileServiceResult<T> = Result<T, OwnerProfileServiceError>;

pub trait OwnerProfileService {
    fn get_full(&self) -> impl Future<Output = OwnerProfileServiceResult<OwnerProfileFull>> + Send;
    fn update(
        &self,
        payload: &OwnerProfileUserInput,
    ) -> impl Future<Output = OwnerProfileServiceResult<OwnerProfile>> + Send;
}

impl<'a, T> OwnerProfileService for Service<'a, T>
where
    T: OwnerProfileModuleInterface,
{
    async fn get_full(&self) -> OwnerProfileServiceResult<OwnerProfileFull> {
        Ok(self
            .module()
            .owner_profile_repo(self.active_tenant()?)?
            .get_full()
            .await?)
    }
    async fn update(
        &self,
        payload: &OwnerProfileUserInput,
    ) -> OwnerProfileServiceResult<OwnerProfile> {
        if !payload.id.is_present() {
            return Err(OwnerProfileServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .owner_profile_repo(self.active_tenant()?)?
            .update(payload, self.claims()?.sub())
            .await?)
    }
}
