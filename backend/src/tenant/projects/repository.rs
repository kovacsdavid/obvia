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
use crate::common::model::SelectOption;
use crate::common::repository::PoolManagerWrapper;
use crate::common::types::value_object::ValueObjectable;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::projects::dto::ProjectUserInput;
use crate::tenant::projects::model::{Project, ProjectResolved};
use crate::tenant::projects::types::project::ProjectOrderBy;
use async_trait::async_trait;
use chrono::NaiveDate;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProjectsRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Project>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<ProjectResolved>;
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>>;
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<ProjectOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ProjectResolved>)>;
    async fn insert(
        &self,
        project: ProjectUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Project>;
    async fn update(
        &self,
        project: ProjectUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Project>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl ProjectsRepository for PoolManagerWrapper {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Project> {
        Ok(sqlx::query_as::<_, Project>(
            r#"
            SELECT *
            FROM projects
            WHERE projects.deleted_at IS NULL
                AND projects.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<ProjectResolved> {
        Ok(sqlx::query_as::<_, ProjectResolved>(
            r#"
            SELECT
                projects.id as id,
                projects.name as name,
                projects.description as description,
                projects.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                projects.status as status,
                projects.start_date as start_date,
                projects.end_date as end_date,
                projects.created_at as created_at,
                projects.updated_at as updated_at,
                projects.deleted_at as deleted_at
            FROM projects
            LEFT JOIN users ON projects.created_by_id = users.id
            WHERE projects.deleted_at IS NULL
                AND projects.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>> {
        Ok(sqlx::query_as::<_, SelectOption>(
            "SELECT projects.id::VARCHAR as value, projects.name as title FROM projects WHERE deleted_at IS NULL ORDER BY name",
        )
        .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<ProjectOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ProjectResolved>)> {
        let total: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM projects WHERE deleted_at IS NULL")
                .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
                .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY projects.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

        let sql = format!(
            r#"
            SELECT
                projects.id as id,
                projects.name as name,
                projects.description as description,
                projects.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                projects.status as status,
                projects.start_date as start_date,
                projects.end_date as end_date,
                projects.created_at as created_at,
                projects.updated_at as updated_at,
                projects.deleted_at as deleted_at
            FROM projects
            LEFT JOIN users ON projects.created_by_id = users.id
            WHERE projects.deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let projects = sqlx::query_as::<_, ProjectResolved>(&sql)
            .bind(paginator_params.limit)
            .bind(paginator_params.offset())
            .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?;

        Ok((
            PaginatorMeta {
                page: paginator_params.page,
                limit: paginator_params.limit,
                total: total.0,
            },
            projects,
        ))
    }

    async fn insert(
        &self,
        project: ProjectUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Project> {
        let start_date = match project.start_date {
            None => None,
            Some(v) => Some(
                NaiveDate::parse_from_str(v.extract().get_value(), "%Y-%m-%d")
                    .map_err(|e| RepositoryError::InvalidInput(e.to_string()))?,
            ),
        };
        let end_date = match project.end_date {
            None => None,
            Some(v) => Some(
                NaiveDate::parse_from_str(v.extract().get_value(), "%Y-%m-%d")
                    .map_err(|e| RepositoryError::InvalidInput(e.to_string()))?,
            ),
        };

        Ok(sqlx::query_as::<_, Project>(
            "INSERT INTO projects (name, description, created_by_id, status, start_date, end_date)
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(project.name.extract().get_value())
        .bind(
            project
                .description
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(sub)
        .bind(project.status.extract().get_value())
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn update(
        &self,
        project: ProjectUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Project> {
        let id = project
            .id
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;
        let start_date = match project.start_date {
            None => None,
            Some(v) => Some(
                NaiveDate::parse_from_str(v.extract().get_value(), "%Y-%m-%d")
                    .map_err(|e| RepositoryError::InvalidInput(e.to_string()))?,
            ),
        };
        let end_date = match project.end_date {
            None => None,
            Some(v) => Some(
                NaiveDate::parse_from_str(v.extract().get_value(), "%Y-%m-%d")
                    .map_err(|e| RepositoryError::InvalidInput(e.to_string()))?,
            ),
        };
        Ok(sqlx::query_as::<_, Project>(
            r#"
            UPDATE projects
            SET name = $1,
                description = $2,
                status = $3,
                start_date = $4,
                end_date = $5
            WHERE id = $6
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(project.name.extract().get_value())
        .bind(
            project
                .description
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(project.status.extract().get_value())
        .bind(start_date)
        .bind(end_date)
        .bind(id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE projects
            SET deleted_at = NOW()
            WHERE id = $1
                AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .execute(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?;

        Ok(())
    }
}
