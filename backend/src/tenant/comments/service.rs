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

use crate::common::BaseModule;
use crate::common::dto::GeneralError;
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::service::{Service, ServiceError};
use crate::tenant::comments::CommentsModule;
use crate::tenant::comments::dto::CommentUserInput;
use crate::tenant::comments::model::Comment;
use crate::tenant::comments::repository::CommentsRepository;
use axum::http::StatusCode;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum CommentsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,
}

impl From<ServiceError> for CommentsServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => CommentsServiceError::Unauthorized,
        }
    }
}

impl IntoFriendlyError for CommentsServiceError {
    async fn into_friendly_error<M>(self, module: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            CommentsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
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
                    module,
                )
                .await
            }
        }
    }
}

type CommentsServiceResult<T> = Result<T, CommentsServiceError>;

pub trait CommentService {
    fn post(
        &self,
        payload: &CommentUserInput,
    ) -> impl Future<Output = CommentsServiceResult<Comment>> + Send;
}

impl<'a, T> CommentService for Service<'a, T>
where
    T: CommentsModule,
{
    async fn post(&self, payload: &CommentUserInput) -> CommentsServiceResult<Comment> {
        Ok(CommentsRepository::post(
            self.module(),
            payload,
            self.claims()?.sub(),
            self.claims()?
                .active_tenant()
                .ok_or(CommentsServiceError::Unauthorized)?,
        )
        .await?)
    }
}
