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
use crate::common::dto::{GeneralError, PaginatorMeta};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::query_parser::GetQuery;
use crate::common::types::{EmptyFilterBy, EmptyOrderBy, ValueObject};
use crate::manager::auth::dto::claims::Claims;
use crate::tenant::activity_feed::model::ActivityFeedResolved;
use crate::tenant::activity_feed::repository::ActivityFeedRepository;
use crate::tenant::activity_feed::types::ResourceType;
use async_trait::async_trait;
use axum::http::StatusCode;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ActivityFeedServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,
}

#[async_trait]
impl IntoFriendlyError<GeneralError> for ActivityFeedServiceError {
    async fn into_friendly_error(
        self,
        module: Arc<dyn MailTransporter>,
    ) -> FriendlyError<GeneralError> {
        match self {
            ActivityFeedServiceError::Unauthorized => FriendlyError::user_facing(
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

pub struct ActivityFeedService;

type ActivityFeedServiceResult<T> = Result<T, ActivityFeedServiceError>;

impl ActivityFeedService {
    pub async fn get_all_paged(
        get_query: &GetQuery<EmptyOrderBy, EmptyFilterBy>,
        claims: &Claims,
        resource_id: Uuid,
        resource_type: &ValueObject<ResourceType>,
        repo: Arc<dyn ActivityFeedRepository>,
    ) -> ActivityFeedServiceResult<(PaginatorMeta, Vec<ActivityFeedResolved>)> {
        Ok(repo
            .get_all_paged(
                get_query,
                resource_id,
                resource_type,
                claims
                    .active_tenant()
                    .ok_or(ActivityFeedServiceError::Unauthorized)?,
            )
            .await?)
    }
}
