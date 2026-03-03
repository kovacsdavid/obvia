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
use crate::tenant::warehouses::dto::WarehouseUserInput;
use crate::tenant::warehouses::model::{Warehouse, WarehouseResolved};
use crate::tenant::warehouses::types::warehouse::{WarehouseFilterBy, WarehouseOrderBy};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WarehousesRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Warehouse>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<WarehouseResolved>;
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>>;
    async fn get_all_paged(
        &self,
        query_params: &GetQuery<WarehouseOrderBy, WarehouseFilterBy>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<WarehouseResolved>)>;
    async fn insert(
        &self,
        warehouse: WarehouseUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Warehouse, RepositoryError>;
    async fn update(
        &self,
        warehouse: WarehouseUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Warehouse>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl WarehousesRepository for PgPoolManager {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Warehouse> {
        Ok(sqlx::query_as::<_, Warehouse>(
            r#"
            SELECT *
            FROM warehouses
            WHERE warehouses.deleted_at IS NULL
                AND warehouses.id = $1
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
    ) -> RepositoryResult<WarehouseResolved> {
        Ok(sqlx::query_as::<_, WarehouseResolved>(
            r#"
            SELECT
                warehouses.id as id,
                warehouses.name as name,
                warehouses.contact_name as contact_name,
                warehouses.contact_phone as contact_phone,
                warehouses.status as status,
                warehouses.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                warehouses.created_at as created_at,
                warehouses.updated_at as updated_at,
                warehouses.deleted_at as deleted_at
            FROM warehouses
            LEFT JOIN users ON warehouses.created_by_id = users.id
            WHERE warehouses.deleted_at IS NULL
                AND warehouses.id = $1
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
            "SELECT warehouses.id::VARCHAR as value, warehouses.name as title FROM warehouses WHERE deleted_at IS NULL ORDER BY name",
        )
        .fetch_all(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_all_paged(
        &self,
        query_params: &GetQuery<WarehouseOrderBy, WarehouseFilterBy>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<WarehouseResolved>)> {
        let total: (i64,) = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                sqlx::query_as(&format!(
                    r#"SELECT COUNT(*) FROM warehouses
                        WHERE deleted_at IS NULL
                            AND ($1::TEXT IS NULL OR warehouses.{filter_by}::TEXT ILIKE '%' || $1 || '%')"#
                ))
                .bind(value_unchecked)
                .fetch_one(&self.get_tenant_pool(active_tenant)?)
                .await?
            }
            (_, _) => {
                sqlx::query_as("SELECT COUNT(*) FROM warehouses WHERE deleted_at IS NULL")
                    .fetch_one(&self.get_tenant_pool(active_tenant)?)
                    .await?
            }
        };

        let order_by_clause = match (
            query_params.ordering().order_by(), // Security: ValueObject
            query_params.ordering().order(),    // Security: enum
        ) {
            (Some(order_by), Some(order)) => format!("ORDER BY customers.{order_by} {order}"),
            (_, _) => "".to_string(),
        };

        let limit = i32::try_from(query_params.paging().limit().unwrap_or(25))?;

        let warehouses = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                let sql = format!(
                    r#"
                    SELECT
                        warehouses.id as id,
                        warehouses.name as name,
                        warehouses.contact_name as contact_name,
                        warehouses.contact_phone as contact_phone,
                        warehouses.status as status,
                        warehouses.created_by_id as created_by_id,
                        users.last_name || ' ' || users.first_name as created_by,
                        warehouses.created_at as created_at,
                        warehouses.updated_at as updated_at,
                        warehouses.deleted_at as deleted_at
                    FROM warehouses
                    LEFT JOIN users ON warehouses.created_by_id = users.id
                    WHERE warehouses.deleted_at IS NULL
                        AND ($1::TEXT IS NULL OR warehouses.{filter_by}::TEXT ILIKE '%' || $1 || '%')
                    {order_by_clause}
                    LIMIT $2
                    OFFSET $3
                    "#
                );

                sqlx::query_as::<_, WarehouseResolved>(&sql)
                    .bind(value_unchecked)
                    .bind(limit)
                    .bind(i32::try_from(query_params.paging().offset().unwrap_or(0))?)
                    .fetch_all(&self.get_tenant_pool(active_tenant)?)
                    .await?
            }
            (_, _) => {
                let sql = format!(
                    r#"
                    SELECT
                        warehouses.id as id,
                        warehouses.name as name,
                        warehouses.contact_name as contact_name,
                        warehouses.contact_phone as contact_phone,
                        warehouses.status as status,
                        warehouses.created_by_id as created_by_id,
                        users.last_name || ' ' || users.first_name as created_by,
                        warehouses.created_at as created_at,
                        warehouses.updated_at as updated_at,
                        warehouses.deleted_at as deleted_at
                    FROM warehouses
                    LEFT JOIN users ON warehouses.created_by_id = users.id
                    WHERE warehouses.deleted_at IS NULL
                    {order_by_clause}
                    LIMIT $1
                    OFFSET $2
                    "#
                );

                sqlx::query_as::<_, WarehouseResolved>(&sql)
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
            warehouses,
        ))
    }
    async fn insert(
        &self,
        warehouse: WarehouseUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Warehouse, RepositoryError> {
        Ok(sqlx::query_as::<_, Warehouse>(
            r#"
            INSERT INTO warehouses (name, contact_name, contact_phone, status, created_by_id)
            VALUES ($1, $2, $3, $4, $5) RETURNING *
             "#,
        )
        .bind(warehouse.name.as_str())
        .bind(warehouse.contact_name.as_ref().map(|d| d.as_str()))
        .bind(warehouse.contact_phone.as_ref().map(|d| d.as_str()))
        .bind(warehouse.status.as_str())
        .bind(sub)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn update(
        &self,
        warehouse: WarehouseUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Warehouse> {
        let id = warehouse
            .id
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;
        Ok(sqlx::query_as::<_, Warehouse>(
            r#"
            UPDATE warehouses
            SET name = $1,
                contact_name = $2,
                contact_phone = $3,
                status = $4
            WHERE id = $5
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(warehouse.name.as_str())
        .bind(warehouse.contact_name.as_ref().map(|d| d.as_str()))
        .bind(warehouse.contact_phone.as_ref().map(|d| d.as_str()))
        .bind(warehouse.status.as_str())
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE warehouses
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
