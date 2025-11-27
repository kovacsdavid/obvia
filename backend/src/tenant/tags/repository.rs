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

use crate::common::dto::{OrderingParams, PaginatorMeta, PaginatorParams};
use crate::common::error::{RepositoryError, RepositoryResult};
use crate::common::types::value_object::ValueObjectable;
use crate::manager::app::database::{PgPoolManager, PoolManager};
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::tags::dto::TagUserInput;
use crate::tenant::tags::model::{Tag, TagResolved};
use crate::tenant::tags::types::tag::TagOrderBy;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TagsRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Tag>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<TagResolved>;
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<TagOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<TagResolved>)>;
    async fn insert(
        &self,
        tag: TagUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Tag, RepositoryError>;
    async fn update(&self, tag: TagUserInput, active_tenant: Uuid) -> RepositoryResult<Tag>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl TagsRepository for PgPoolManager {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Tag> {
        Ok(sqlx::query_as::<_, Tag>(
            r#"
            SELECT *
            FROM tags
            WHERE tags.deleted_at IS NULL
                AND tags.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<TagResolved> {
        Ok(sqlx::query_as::<_, TagResolved>(
            r#"
            SELECT
                tags.id as id,
                tags.name as name,
                tags.description as description,
                tags.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                tags.created_at as created_at,
                tags.deleted_at as deleted_at
            FROM tags
            LEFT JOIN users ON tags.created_by_id = users.id
            WHERE tags.deleted_at IS NULL
                AND tags.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<TagOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<TagResolved>)> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tags WHERE deleted_at IS NULL")
            .fetch_one(&self.get_tenant_pool(active_tenant)?)
            .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY tags.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

        let sql = format!(
            r#"
            SELECT
                tags.id as id,
                tags.name as name,
                tags.description as description,
                tags.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                tags.created_at as created_at,
                tags.deleted_at as deleted_at
            FROM tags
            LEFT JOIN users ON tags.created_by_id = users.id
            WHERE tags.deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let tags = sqlx::query_as::<_, TagResolved>(&sql)
            .bind(paginator_params.limit)
            .bind(paginator_params.offset())
            .fetch_all(&self.get_tenant_pool(active_tenant)?)
            .await?;

        Ok((
            PaginatorMeta {
                page: paginator_params.page,
                limit: paginator_params.limit,
                total: total.0,
            },
            tags,
        ))
    }
    async fn insert(
        &self,
        tag: TagUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Tag, RepositoryError> {
        Ok(sqlx::query_as::<_, Tag>(
            "INSERT INTO tags (name, description, created_by_id) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(tag.name.extract().get_value())
        .bind(
            tag.description
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(sub)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn update(&self, tag: TagUserInput, active_tenant: Uuid) -> RepositoryResult<Tag> {
        let id = tag
            .id
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;
        Ok(sqlx::query_as::<_, Tag>(
            r#"
            UPDATE tags
            SET name = $1,
                description = $2
            WHERE id = $3
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(tag.name.extract().get_value())
        .bind(
            tag.description
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE tags
            SET deleted_at = NOW()
            WHERE id = $1
                AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .execute(&self.get_tenant_pool(active_tenant)?)
        .await?;

        Ok(())
    }
}
