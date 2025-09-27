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

use crate::common::dto::{OrderingParams, PagedData, PaginatorParams};
use crate::common::error::{RepositoryError, RepositoryResult};
use crate::common::repository::PoolManagerWrapper;
use crate::common::types::value_object::ValueObjectable;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::tags::dto::CreateTag;
use crate::tenant::tags::model::Tag;
use crate::tenant::tags::types::tag::TagOrderBy;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TagsRepository: Send + Sync {
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<TagOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<PagedData<Vec<Tag>>>;
    async fn insert(
        &self,
        tag: CreateTag,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Tag, RepositoryError>;
}

#[async_trait]
impl TagsRepository for PoolManagerWrapper {
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<TagOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<PagedData<Vec<Tag>>> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tags WHERE deleted_at IS NULL")
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY tags.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

        let sql = format!(
            r#"
            SELECT *
            FROM tags 
            WHERE deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let tags = sqlx::query_as::<_, Tag>(&sql)
            .bind(paginator_params.limit)
            .bind(paginator_params.offset())
            .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?;

        Ok(PagedData {
            page: paginator_params.page,
            limit: paginator_params.limit,
            total: total.0,
            data: tags,
        })
    }
    async fn insert(
        &self,
        tag: CreateTag,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Tag, RepositoryError> {
        Ok(sqlx::query_as::<_, Tag>(
            "INSERT INTO tags (name, description, created_by) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(tag.name.extract().get_value())
        .bind(tag.description.map(|v| v.extract().get_value().clone()))
        .bind(sub)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
}
