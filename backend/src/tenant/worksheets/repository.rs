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

use crate::common::dto::PaginatorMeta;
use crate::common::error::{RepositoryError, RepositoryResult};
use crate::common::model::SelectOption;
use crate::common::query_parser::GetQuery;
use crate::manager::app::database::{PgPoolManager, PoolManager};
use crate::tenant::worksheets::dto::WorksheetUserInput;
use crate::tenant::worksheets::model::{Worksheet, WorksheetResolved};
use crate::tenant::worksheets::types::worksheet::{WorksheetFilterBy, WorksheetOrderBy};
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
        query_params: &GetQuery<WorksheetOrderBy, WorksheetFilterBy>,
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
impl WorksheetsRepository for PgPoolManager {
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
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
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
                                       sum(abs(COALESCE(quantity, 0)) * COALESCE(price, 0)) as net_work_cost,
                                       sum(abs(COALESCE(quantity, 0)) * COALESCE(price, 0) * (CASE
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
                worksheets.customer_id as customer_id,
                customers.name as customer,
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
            LEFT JOIN customers ON worksheets.customer_id = customers.id
            LEFT JOIN projects ON worksheets.project_id = projects.id
            LEFT JOIN users ON worksheets.created_by_id = users.id
            LEFT JOIN material_costs mc ON mc.worksheet_id = worksheets.id
            LEFT JOIN work_costs wc ON wc.worksheet_id = worksheets.id
            WHERE worksheets.deleted_at IS NULL
                AND worksheets.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>> {
        Ok(sqlx::query_as::<_, SelectOption>(
            "SELECT worksheets.id::VARCHAR as value, worksheets.name as title FROM worksheets WHERE deleted_at IS NULL ORDER BY name",
        )
        .fetch_all(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_all_paged(
        &self,
        query_params: &GetQuery<WorksheetOrderBy, WorksheetFilterBy>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<WorksheetResolved>)> {
        let total: (i64,) = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                sqlx::query_as(&format!(
                    r#"SELECT COUNT(*) FROM worksheets
                    WHERE deleted_at IS NULL
                        AND ($1::TEXT IS NULL OR worksheets.{filter_by}::TEXT ILIKE '%' || $1 || '%')"#
                ))
                .bind(value_unchecked)
                .fetch_one(&self.get_tenant_pool(active_tenant)?)
                .await?
            }
            (_, _) => {
                sqlx::query_as("SELECT COUNT(*) FROM worksheets WHERE deleted_at IS NULL")
                    .fetch_one(&self.get_tenant_pool(active_tenant)?)
                    .await?
            }
        };

        let order_by_clause = match (
            query_params.ordering().order_by(), // Security: ValueObject
            query_params.ordering().order(),    // Security: enum
        ) {
            (Some(order_by), Some(order)) => format!("ORDER BY worksheets.{order_by} {order}"),
            (_, _) => "".to_string(),
        };

        let limit = i32::try_from(query_params.paging().limit().unwrap_or(25))?;

        let worksheets = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
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
                                               sum(abs(COALESCE(quantity, 0)) * COALESCE(price, 0)) as net_work_cost,
                                               sum(abs(COALESCE(quantity, 0)) * COALESCE(price, 0) * (CASE
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
                           worksheets.customer_id                     as customer_id,
                           customers.name                             as customer,
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
                    LEFT JOIN customers ON worksheets.customer_id = customers.id
                    LEFT JOIN projects ON worksheets.project_id = projects.id
                    LEFT JOIN users ON worksheets.created_by_id = users.id
                    LEFT JOIN material_costs mc ON mc.worksheet_id = worksheets.id
                    LEFT JOIN work_costs wc ON wc.worksheet_id = worksheets.id
                    WHERE worksheets.deleted_at IS NULL
                        AND ($1::TEXT IS NULL OR worksheets.{filter_by}::TEXT ILIKE '%' || $1 || '%')
                    {order_by_clause}
                    LIMIT $2
                    OFFSET $3
                    "#
                );

                sqlx::query_as::<_, WorksheetResolved>(&sql)
                    .bind(value_unchecked)
                    .bind(limit)
                    .bind(i32::try_from(query_params.paging().offset().unwrap_or(0))?)
                    .fetch_all(&self.get_tenant_pool(active_tenant)?)
                    .await?
            }
            (_, _) => {
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
                                               sum(abs(COALESCE(quantity, 0)) * COALESCE(price, 0)) as net_work_cost,
                                               sum(abs(COALESCE(quantity, 0)) * COALESCE(price, 0) * (CASE
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
                           worksheets.customer_id                     as customer_id,
                           customers.name                             as customer,
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
                    LEFT JOIN customers ON worksheets.customer_id = customers.id
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

                sqlx::query_as::<_, WorksheetResolved>(&sql)
                    .bind(limit)
                    .bind(i32::try_from(query_params.paging().offset().unwrap_or(0))?)
                    .fetch_all(&self.get_tenant_pool(active_tenant)?)
                    .await?
            }
        };

        Ok((
            PaginatorMeta {
                page: query_params.paging().page().unwrap_or(1).try_into()?,
                limit,
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
        let project_id = match &worksheet.project_id {
            Some(v) => Some(v.as_uuid()?),
            None => None,
        };

        Ok(sqlx::query_as::<_, Worksheet>(
            "INSERT INTO worksheets (name, description, customer_id, project_id, created_by_id, status)\
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(worksheet.name.as_str())
        .bind(
            worksheet
                .description
                .as_ref()
                .map(|d| d.as_str()),
        )
        .bind(worksheet.customer_id.as_uuid()?)
        .bind(project_id)
        .bind(sub)
        .bind(worksheet.status.as_str())
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
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
        let project_id = match &worksheet.project_id {
            Some(v) => Some(v.as_uuid()?),
            None => None,
        };
        Ok(sqlx::query_as::<_, Worksheet>(
            r#"
            UPDATE worksheets
            SET name = $1,
                description = $2,
                customer_id = $3,
                project_id = $4,
                status = $5
            WHERE id = $6
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(worksheet.name.as_str())
        .bind(worksheet.description.as_ref().map(|d| d.as_str()))
        .bind(worksheet.customer_id.as_uuid()?)
        .bind(project_id)
        .bind(worksheet.status.as_str())
        .bind(id.as_uuid()?)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
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
        .execute(&self.get_tenant_pool(active_tenant)?)
        .await?;

        Ok(())
    }
}
