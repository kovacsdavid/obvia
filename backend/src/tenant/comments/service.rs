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

use crate::common::MailTransporter;
use crate::common::dto::GeneralError;
use crate::common::dto::PaginatorMeta;
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::manager::auth::dto::claims::Claims;
use crate::tenant::comments::dto::CommentUserInput;
use crate::tenant::comments::model::CommentsResolved;
use crate::tenant::comments::repository::CommentsRepository;
use async_trait::async_trait;
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

#[async_trait]
impl IntoFriendlyError<GeneralError> for CommentsServiceError {
    async fn into_friendly_error(
        self,
        module: Arc<dyn MailTransporter>,
    ) -> FriendlyError<GeneralError> {
        match self {
            CommentsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: self.to_string(),
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

pub struct CommentsService;

type CommentsServiceResult<T> = Result<T, CommentsServiceError>;

impl CommentsService {
    pub async fn post(
        claims: &Claims,
        payload: &CommentUserInput,
        repo: Arc<dyn CommentsRepository>,
    ) -> CommentsServiceResult<(PaginatorMeta, Vec<CommentsResolved>)> {
        Ok(repo
            .post(
                payload,
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(CommentsServiceError::Unauthorized)?,
            )
            .await?)
    }
}
