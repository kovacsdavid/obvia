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
use crate::common::repository::PoolManagerWrapper;
use crate::common::types::value_object::ValueObjectable;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::tasks::dto::TaskUserInput;
use crate::tenant::tasks::model::{Task, TaskResolved};
use crate::tenant::tasks::types::task::TaskOrderBy;
use async_trait::async_trait;
use chrono::NaiveDate;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TasksRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Task>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<TaskResolved>;
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<TaskOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<TaskResolved>)>;
    async fn insert(
        &self,
        task: TaskUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Task>;
    async fn update(&self, task: TaskUserInput, active_tenant: Uuid) -> RepositoryResult<Task>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl TasksRepository for PoolManagerWrapper {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Task> {
        Ok(sqlx::query_as::<_, Task>(
            r#"
            SELECT *
            FROM tasks
            WHERE tasks.deleted_at IS NULL
                AND tasks.id = $1
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
    ) -> RepositoryResult<TaskResolved> {
        Ok(sqlx::query_as::<_, TaskResolved>(
            r#"
            SELECT
                tasks.id as id,
                tasks.worksheet_id as worksheet_id,
                worksheets.name as worksheet,
                tasks.service_id as service_id,
                services.name as service,
                tasks.currency_code as currency_code,
                tasks.quantity as quantity,
                tasks.price as price,
                tasks.tax_id as tax_id,
                taxes.description as tax,
                tasks.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                tasks.status as status,
                tasks.priority as priority,
                tasks.due_date as due_date,
                tasks.created_at as created_at,
                tasks.updated_at as updated_at,
                tasks.deleted_at as deleted_at,
                tasks.description as description
            FROM tasks
            LEFT JOIN worksheets ON tasks.worksheet_id = worksheets.id
            LEFT JOIN services ON tasks.service_id = services.id
            LEFT JOIN taxes ON tasks.tax_id = taxes.id
            LEFT JOIN users ON tasks.created_by_id = users.id
            WHERE tasks.deleted_at IS NULL
                AND tasks.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<TaskOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<TaskResolved>)> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE deleted_at IS NULL")
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY tasks.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

        let sql = format!(
            r#"
            SELECT
                tasks.id as id,
                tasks.worksheet_id as worksheet_id,
                worksheets.name as worksheet,
                tasks.service_id as service_id,
                services.name as service,
                tasks.currency_code as currency_code,
                tasks.quantity as quantity, 
                tasks.price as price, 
                tasks.tax_id as tax_id,
                taxes.description as tax,
                tasks.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                tasks.status as status,
                tasks.priority as priority,
                tasks.due_date as due_date,
                tasks.created_at as created_at,
                tasks.updated_at as updated_at,
                tasks.deleted_at as deleted_at,
                tasks.description as description
            FROM tasks
            LEFT JOIN worksheets ON tasks.worksheet_id = worksheets.id
            LEFT JOIN services ON tasks.service_id = services.id
            LEFT JOIN taxes ON tasks.tax_id = taxes.id
            LEFT JOIN users ON tasks.created_by_id = users.id
            WHERE tasks.deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let tasks = sqlx::query_as::<_, TaskResolved>(&sql)
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
            tasks,
        ))
    }
    async fn insert(
        &self,
        task: TaskUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Task> {
        let quantity = match &task.quantity {
            None => None,
            Some(v) => Some(
                v.extract()
                    .get_value()
                    .parse::<f64>()
                    .map_err(|_| RepositoryError::InvalidInput("quantity".to_string()))?,
            ),
        };
        let price = match &task.price {
            None => None,
            Some(v) => Some(
                v.extract()
                    .get_value()
                    .parse::<f64>()
                    .map_err(|_| RepositoryError::InvalidInput("price".to_string()))?,
            ),
        };
        let due_date = match task.due_date {
            None => None,
            Some(v) => Some(
                NaiveDate::parse_from_str(v.extract().get_value(), "%Y-%m-%d")
                    .map_err(|e| RepositoryError::InvalidInput(e.to_string()))?,
            ),
        };
        Ok(sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (worksheet_id, service_id, currency_code, quantity, price, tax_id, created_by_id, status, priority, due_date, description)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *"
        )
            .bind(task.worksheet_id)
            .bind(task.service_id)
            .bind(task.currency_code.extract().get_value())
            .bind(quantity)
            .bind(price)
            .bind(task.tax_id)
            .bind(sub)
            .bind(task.status.extract().get_value())
            .bind(task.priority.as_ref()
                .map(|d| d.extract().get_value().as_str()))
            .bind(due_date)
            .bind(task.description.as_ref()
                .map(|d| d.extract().get_value().as_str()))
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?
        )
    }

    async fn update(&self, task: TaskUserInput, active_tenant: Uuid) -> RepositoryResult<Task> {
        let id = task
            .id
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;
        let due_date = match task.due_date {
            None => None,
            Some(v) => Some(
                NaiveDate::parse_from_str(v.extract().get_value(), "%Y-%m-%d %H:%M:%S")
                    .map_err(|e| RepositoryError::InvalidInput(e.to_string()))?,
            ),
        };
        Ok(sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET worksheet_id = $1,
                service_id = $2,
                currency_code = $3,
                quantity = $4,
                price = $5,
                tax_id = $6,
                status = $7,
                priority = $8,
                due_date = $9,
                description = $10
            WHERE id = $11
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(task.worksheet_id)
        .bind(task.service_id)
        .bind(task.currency_code.extract().get_value())
        .bind(
            task.quantity
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(
            task.price
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(task.tax_id)
        .bind(task.status.extract().get_value())
        .bind(
            task.priority
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(due_date)
        .bind(
            task.description
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE tasks
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
