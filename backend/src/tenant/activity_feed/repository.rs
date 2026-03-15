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
use crate::common::error::RepositoryResult;
use crate::common::query_parser::GetQuery;
use crate::common::types::{EmptyFilterBy, EmptyOrderBy, ValueObject};
use crate::manager::app::database::{PgPoolManager, PoolManager};
use crate::tenant::activity_feed::model::ActivityFeedResolved;
use crate::tenant::activity_feed::types::ResourceType;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ActivityFeedRepository: Send + Sync {
    async fn get_all_paged(
        &self,
        query_params: &GetQuery<EmptyOrderBy, EmptyFilterBy>,
        resource_id: Uuid,
        resource_type: &ValueObject<ResourceType>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ActivityFeedResolved>)>;
}

#[async_trait]
impl ActivityFeedRepository for PgPoolManager {
    async fn get_all_paged(
        &self,
        query_params: &GetQuery<EmptyOrderBy, EmptyFilterBy>,
        resource_id: Uuid,
        resource_type: &ValueObject<ResourceType>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ActivityFeedResolved>)> {
        let total: (i64,) = sqlx::query_as(
            r#"
                SELECT COUNT(*)
                FROM comments
                WHERE deleted_at IS NULL
                    AND commentable_id = $1
                    AND commentable_type = $2
            "#,
        )
        .bind(resource_id)
        .bind(resource_type.as_str())
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?;

        let limit = i32::try_from(query_params.paging().limit().unwrap_or(25))?;

        let sql = r#"
            WITH normalized_feed AS (
                SELECT
                    comments.id,
                    comments.commentable_id AS resource_id,
                    comments.commentable_type AS resource_type,
                    'comment' AS activity_type,
                    comments.comment AS content,
                    comments.created_by_id as created_by_id,
                    users.last_name || ' ' || users.first_name || ' <' || users.email || '>' as created_by,
                    comments.created_at AS created_at,
                    comments.updated_at AS updated_at,
                    comments.deleted_at AS deleted_at
                FROM comments
                LEFT JOIN users ON comments.created_by_id = users.id
                WHERE comments.deleted_at IS NULL
                    AND comments.commentable_id = $1
                    AND comments.commentable_type = $2
            )
            SELECT *
            FROM normalized_feed
            ORDER BY created_at
        "#;

        let activity_feed = sqlx::query_as::<_, ActivityFeedResolved>(sql)
            .bind(resource_id)
            .bind(resource_type.as_str())
            .fetch_all(&self.get_tenant_pool(active_tenant)?)
            .await?;

        Ok((
            PaginatorMeta {
                page: query_params.paging().page().unwrap_or(1).try_into()?,
                limit,
                total: total.0,
            },
            activity_feed,
        ))
    }
}
