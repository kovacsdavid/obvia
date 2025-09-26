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

use crate::common::error::{RepositoryError, RepositoryResult};
use crate::manager::common::dto::{OrderingParams, PagedData, PaginatorParams};
use crate::manager::common::repository::PoolManagerWrapper;
use crate::manager::common::types::value_object::ValueObjectable;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::tasks::dto::CreateTask;
use crate::tenant::tasks::model::Task;
use crate::tenant::tasks::types::task::TaskOrderBy;
use async_trait::async_trait;
use chrono::NaiveDateTime;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TasksRepository: Send + Sync {
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<TaskOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<PagedData<Vec<Task>>>;
    async fn insert(
        &self,
        task: CreateTask,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Task>;
}

#[async_trait]
impl TasksRepository for PoolManagerWrapper {
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<TaskOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<PagedData<Vec<Task>>> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE deleted_at IS NULL")
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY tasks.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

        let sql = format!(
            r#"
            SELECT *
            FROM tasks
            WHERE deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let tasks = sqlx::query_as::<_, Task>(&sql)
            .bind(paginator_params.limit)
            .bind(paginator_params.offset())
            .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?;

        Ok(PagedData {
            page: paginator_params.page,
            limit: paginator_params.limit,
            total: total.0,
            data: tasks,
        })
    }
    async fn insert(
        &self,
        task: CreateTask,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Task> {
        let due_date = match task.due_date {
            None => None,
            Some(v) => Some(
                NaiveDateTime::parse_from_str(v.extract().get_value(), "%Y-%m-%d %H:%M:%S")
                    .map_err(|e| RepositoryError::Parse(e.to_string()))?,
            ),
        };
        Ok(sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (worksheet_id, title, description, created_by, status, priority, due_date)
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
            )
            .bind(task.worksheet_id)
            .bind(task.title.extract().get_value())
            .bind(task.description.map(|v| v.extract().get_value().clone()))
            .bind(sub)
            .bind(task.status.extract().get_value())
            .bind(task.priority.extract().get_value())
            .bind(due_date)
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?
        )
    }
}
