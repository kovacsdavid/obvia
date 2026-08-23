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

use crate::common::error::RepositoryError;
use crate::common::error::v2::{AppError, AppErrorVisibility};
use crate::common::service::{Service, ServiceError};
use crate::tenant::comments::CommentsModuleInterface;
use crate::tenant::comments::dto::CommentUserInput;
use crate::tenant::comments::model::Comment;
use axum::http::StatusCode;
use serde_json::json;
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

impl From<CommentsServiceError> for AppError {
    fn from(value: CommentsServiceError) -> Self {
        match value {
            CommentsServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
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

type CommentsServiceResult<T> = Result<T, CommentsServiceError>;

pub trait CommentService {
    fn post(
        &self,
        payload: &CommentUserInput,
    ) -> impl Future<Output = CommentsServiceResult<Comment>> + Send;
}

impl<'a, T> CommentService for Service<'a, T>
where
    T: CommentsModuleInterface,
{
    async fn post(&self, payload: &CommentUserInput) -> CommentsServiceResult<Comment> {
        Ok(self
            .module()
            .comments_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(CommentsServiceError::Unauthorized)?,
            )?
            .post(payload, self.claims()?.sub())
            .await?)
    }
}
