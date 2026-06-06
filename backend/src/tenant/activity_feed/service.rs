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
use crate::common::dto::{GeneralError, PaginatorMeta};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::common::types::Empty;
use crate::common::value_object::ValueObjectRequired;
use crate::tenant::activity_feed::ActivityFeedModule;
use crate::tenant::activity_feed::model::ActivityFeedResolved;
use crate::tenant::activity_feed::repository::ActivityFeedRepository;
use crate::tenant::activity_feed::types::ResourceType;
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

impl From<ServiceError> for ActivityFeedServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => ActivityFeedServiceError::Unauthorized,
        }
    }
}

impl IntoFriendlyError for ActivityFeedServiceError {
    async fn into_friendly_error<M>(self, module: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            ActivityFeedServiceError::Unauthorized => FriendlyError::user_facing(
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
    T: ActivityFeedModule,
{
    async fn get_all_paged(
        &self,
        get_query: &ResourceQuery<Empty, Empty>,
        resource_id: Uuid,
        resource_type: &ValueObjectRequired<ResourceType>,
    ) -> ActivityFeedServiceResult<(PaginatorMeta, Vec<ActivityFeedResolved>)> {
        Ok(ActivityFeedRepository::get_all_paged(
            self.module(),
            get_query,
            resource_id,
            resource_type,
            self.claims()?
                .active_tenant()
                .ok_or(ActivityFeedServiceError::Unauthorized)?,
        )
        .await?)
    }
}
