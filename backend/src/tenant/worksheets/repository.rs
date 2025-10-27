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
use crate::tenant::worksheets::dto::WorksheetUserInput;
use crate::tenant::worksheets::model::{Worksheet, WorksheetResolved};
use crate::tenant::worksheets::types::worksheet::WorksheetOrderBy;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WorksheetsRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Worksheet>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<WorksheetResolved>;
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>>;
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<WorksheetOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<WorksheetResolved>)>;
    async fn insert(
        &self,
        worksheet: WorksheetUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Worksheet>;
    async fn update(
        &self,
        worksheet: WorksheetUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Worksheet>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl WorksheetsRepository for PoolManagerWrapper {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Worksheet> {
        Ok(sqlx::query_as::<_, Worksheet>(
            r#"
            SELECT *
            FROM worksheets
            WHERE worksheets.deleted_at IS NULL
                AND worksheets.id = $1
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
    ) -> RepositoryResult<WorksheetResolved> {
        Ok(sqlx::query_as::<_, WorksheetResolved>(
            r#"
            WITH material_costs AS (SELECT reference_id                                                                         as worksheet_id,
                                           sum(abs(inventory_movements.quantity) *
                                               COALESCE(inventory_movements.unit_price, 0))                                     as net_material_cost,
                                           sum((abs(inventory_movements.quantity) * COALESCE(inventory_movements.unit_price, 0)) *
                                               (CASE
                                                    WHEN taxes.is_rate_applicable THEN ((taxes.rate / 100) + 1)
                                                    ELSE 1
                                                   END))                                                                        as gross_material_cost
                                    FROM inventory_movements
                                             LEFT JOIN taxes ON inventory_movements.tax_id = taxes.id
                                    WHERE reference_type = 'worksheets'
                                      AND movement_type = 'out'
                                    GROUP BY reference_id),
                 work_costs AS (SELECT worksheet_id,
                                       sum(COALESCE(price, 0)) as net_work_cost,
                                       sum(COALESCE(price, 0) * (CASE
                                                                     WHEN taxes.is_rate_applicable THEN ((taxes.rate / 100) + 1)
                                                                     ELSE 1
                                           END))               as gross_work_cost
                                FROM tasks
                                         LEFT JOIN taxes ON tasks.tax_id = taxes.id
                                WHERE tasks.deleted_at IS NULL
                                GROUP BY worksheet_id)
            SELECT
                worksheets.id as id,
                worksheets.name as name,
                worksheets.description as description,
                worksheets.project_id as project_id,
                projects.name as project,
                worksheets.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                worksheets.status as status,
                worksheets.created_at as created_at,
                worksheets.updated_at as updated_at,
                worksheets.deleted_at as deleted_at,
                COALESCE(mc.net_material_cost, 0)          as net_material_cost,
                COALESCE(mc.gross_material_cost, 0)        as gross_material_cost,
                COALESCE(wc.net_work_cost, 0)              as net_work_cost,
                COALESCE(wc.gross_work_cost, 0)            as gross_work_cost
            FROM worksheets
            LEFT JOIN projects ON worksheets.project_id = projects.id
            LEFT JOIN users ON worksheets.created_by_id = users.id
            LEFT JOIN material_costs mc ON mc.worksheet_id = worksheets.id
            LEFT JOIN work_costs wc ON wc.worksheet_id = worksheets.id
            WHERE worksheets.deleted_at IS NULL
                AND worksheets.id = $1
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
            "SELECT worksheets.id::VARCHAR as value, worksheets.name as title FROM worksheets WHERE deleted_at IS NULL ORDER BY name",
        )
        .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<WorksheetOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<WorksheetResolved>)> {
        let total: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM worksheets WHERE deleted_at IS NULL")
                .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
                .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY worksheets.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

        let sql = format!(
            r#"
            WITH material_costs AS (SELECT reference_id                                                                         as worksheet_id,
                                           sum(abs(inventory_movements.quantity) *
                                               COALESCE(inventory_movements.unit_price, 0))                                     as net_material_cost,
                                           sum((abs(inventory_movements.quantity) * COALESCE(inventory_movements.unit_price, 0)) *
                                               (CASE
                                                    WHEN taxes.is_rate_applicable THEN ((taxes.rate / 100) + 1)
                                                    ELSE 1
                                                   END))                                                                        as gross_material_cost
                                    FROM inventory_movements
                                             LEFT JOIN taxes ON inventory_movements.tax_id = taxes.id
                                    WHERE reference_type = 'worksheets'
                                      AND movement_type = 'out'
                                    GROUP BY reference_id),
                 work_costs AS (SELECT worksheet_id,
                                       sum(COALESCE(price, 0)) as net_work_cost,
                                       sum(COALESCE(price, 0) * (CASE
                                                                     WHEN taxes.is_rate_applicable THEN ((taxes.rate / 100) + 1)
                                                                     ELSE 1
                                           END))               as gross_work_cost
                                FROM tasks
                                         LEFT JOIN taxes ON tasks.tax_id = taxes.id
                                WHERE tasks.deleted_at IS NULL
                                GROUP BY worksheet_id)
            SELECT worksheets.id                              as id,
                   worksheets.name                            as name,
                   worksheets.description                     as description,
                   worksheets.project_id                      as project_id,
                   projects.name                              as project,
                   worksheets.created_by_id                   as created_by_id,
                   users.last_name || ' ' || users.first_name as created_by,
                   worksheets.status                          as status,
                   worksheets.created_at                      as created_at,
                   worksheets.updated_at                      as updated_at,
                   worksheets.deleted_at                      as deleted_at,
                   COALESCE(mc.net_material_cost, 0)          as net_material_cost,
                   COALESCE(mc.gross_material_cost, 0)        as gross_material_cost,
                   COALESCE(wc.net_work_cost, 0)              as net_work_cost,
                   COALESCE(wc.gross_work_cost, 0)            as gross_work_cost
            FROM worksheets
            LEFT JOIN projects ON worksheets.project_id = projects.id
            LEFT JOIN users ON worksheets.created_by_id = users.id
            LEFT JOIN material_costs mc ON mc.worksheet_id = worksheets.id
            LEFT JOIN work_costs wc ON wc.worksheet_id = worksheets.id
            WHERE worksheets.deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let worksheets = sqlx::query_as::<_, WorksheetResolved>(&sql)
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
            worksheets,
        ))
    }
    async fn insert(
        &self,
        worksheet: WorksheetUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Worksheet, RepositoryError> {
        Ok(sqlx::query_as::<_, Worksheet>(
            "INSERT INTO worksheets (name, description, project_id, created_by_id, status)\
             VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(worksheet.name.extract().get_value())
        .bind(
            worksheet
                .description
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(worksheet.project_id)
        .bind(sub)
        .bind(worksheet.status.extract().get_value())
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn update(
        &self,
        worksheet: WorksheetUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Worksheet> {
        let id = worksheet
            .id
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;
        Ok(sqlx::query_as::<_, Worksheet>(
            r#"
            UPDATE worksheets
            SET name = $1,
                description = $2,
                project_id = $3,
                status = $4
            WHERE id = $5
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(worksheet.name.extract().get_value())
        .bind(
            worksheet
                .description
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(worksheet.project_id)
        .bind(worksheet.status.extract().get_value())
        .bind(id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE worksheets
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
