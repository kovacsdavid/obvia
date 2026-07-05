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

use crate::common::dto::PaginatorMeta;
use crate::common::error::RepositoryError;
use crate::common::error::v2::{AppError, AppErrorVisibility};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::common::types::Empty;
use crate::common::value_object::ValueObjectRequired;
use crate::tenant::activity_feed::ActivityFeedModuleInterface;
use crate::tenant::activity_feed::model::ActivityFeedResolved;
use crate::tenant::activity_feed::types::ResourceType;
use axum::http::StatusCode;
use serde_json::json;
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

impl From<ServiceError> for ActivityFeedServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => ActivityFeedServiceError::Unauthorized,
        }
    }
}

impl From<ActivityFeedServiceError> for AppError {
    fn from(value: ActivityFeedServiceError) -> Self {
        match value {
            ActivityFeedServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            ActivityFeedServiceError::Repository(RepositoryError::Database(
                sqlx::Error::RowNotFound,
            )) => Self::new(
                Level::DEBUG,
                StatusCode::NOT_FOUND,
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

type ActivityFeedServiceResult<T> = Result<T, ActivityFeedServiceError>;

pub trait ActivityFeedService {
    fn get_all_paged(
        &self,
        get_query: &ResourceQuery<Empty, Empty>,
        resource_id: Uuid,
        resource_type: &ValueObjectRequired<ResourceType>,
    ) -> impl Future<Output = ActivityFeedServiceResult<(PaginatorMeta, Vec<ActivityFeedResolved>)>> + Send;
}

impl<'a, T> ActivityFeedService for Service<'a, T>
where
    T: ActivityFeedModuleInterface,
{
    async fn get_all_paged(
        &self,
        get_query: &ResourceQuery<Empty, Empty>,
        resource_id: Uuid,
        resource_type: &ValueObjectRequired<ResourceType>,
    ) -> ActivityFeedServiceResult<(PaginatorMeta, Vec<ActivityFeedResolved>)> {
        Ok(self
            .module()
            .activity_feed_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ActivityFeedServiceError::Unauthorized)?,
            )?
            .get_paged(get_query, resource_id, resource_type)
            .await?)
    }
}
