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
use crate::common::error::RepositoryError;
use crate::manager::auth::dto::claims::Claims;
use crate::manager::common::dto::{OrderingParams, PagedData, PaginatorParams};
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::tags::TagsModule;
use crate::tenant::tags::dto::CreateTag;
use crate::tenant::tags::model::Tag;
use crate::tenant::tags::repository::TagsRepository;
use crate::tenant::tags::types::tag::TagOrderBy;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TagsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Unauthorized")]
    Unauthorized,
}

pub type TagsServiceResult<T> = Result<T, TagsServiceError>;

pub struct TagsService;

impl TagsService {
    pub async fn try_create(
        claims: &Claims,
        payload: &CreateTag,
        tags_module: Arc<TagsModule>,
    ) -> TagsServiceResult<()> {
        tags_module
            .tags_repo
            .insert(
                payload.clone(),
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(TagsServiceError::Unauthorized)?,
            )
            .await?;
        Ok(())
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams<TagOrderBy>,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn TagsRepository>,
    ) -> TagsServiceResult<PagedData<Vec<Tag>>> {
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
