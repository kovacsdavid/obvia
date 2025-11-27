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
use crate::common::dto::{GeneralError, OrderingParams, PaginatorMeta, PaginatorParams, UuidParam};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::tags::TagsModule;
use crate::tenant::tags::dto::TagUserInput;
use crate::tenant::tags::model::{Tag, TagResolved};
use crate::tenant::tags::repository::TagsRepository;
use crate::tenant::tags::types::tag::TagOrderBy;
use async_trait::async_trait;
use axum::http::StatusCode;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum TagsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,
}

#[async_trait]
impl IntoFriendlyError<GeneralError> for TagsServiceError {
    async fn into_friendly_error(
        self,
        module: Arc<dyn MailTransporter>,
    ) -> FriendlyError<GeneralError> {
        match self {
            TagsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: TagsServiceError::Unauthorized.to_string(),
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

pub type TagsServiceResult<T> = Result<T, TagsServiceError>;

pub struct TagsService;

impl TagsService {
    pub async fn try_create(
        claims: &Claims,
        payload: &TagUserInput,
        tags_module: Arc<dyn TagsModule>,
    ) -> TagsServiceResult<Tag> {
        Ok(tags_module
            .tags_repo()
            .insert(
                payload.clone(),
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(TagsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_resolved_by_id(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn TagsRepository>,
    ) -> TagsServiceResult<TagResolved> {
        Ok(repo
            .get_resolved_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(TagsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn TagsRepository>,
    ) -> TagsServiceResult<Tag> {
        Ok(repo
            .get_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(TagsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn update(
        claims: &Claims,
        payload: &TagUserInput,
        repo: Arc<dyn TagsRepository>,
    ) -> TagsServiceResult<Tag> {
        Ok(repo
            .update(
                payload.clone(),
                claims
                    .active_tenant()
                    .ok_or(TagsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn delete(
        claims: &Claims,
        payload: &UuidParam,
        repo: Arc<dyn TagsRepository>,
    ) -> TagsServiceResult<()> {
        Ok(repo
            .delete_by_id(
                payload.uuid,
                claims
                    .active_tenant()
                    .ok_or(TagsServiceError::Unauthorized)?,
            )
            .await?)
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<TagOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn TagsRepository>,
    ) -> TagsServiceResult<(PaginatorMeta, Vec<TagResolved>)> {
        Ok(repo
            .get_all_paged(
                paginator,
                ordering,
                filtering,
                claims
                    .active_tenant()
                    .ok_or(TagsServiceError::Unauthorized)?,
            )
            .await?)
    }
}
