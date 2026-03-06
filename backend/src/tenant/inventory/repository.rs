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
use crate::tenant::inventory::dto::InventoryUserInput;
use crate::tenant::inventory::model::{Inventory, InventoryResolved};
use crate::tenant::inventory::types::inventory::{InventoryFilterBy, InventoryOrderBy};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait InventoryRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Inventory>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<InventoryResolved>;
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>>;
    async fn get_all_paged(
        &self,
        query_params: &GetQuery<InventoryOrderBy, InventoryFilterBy>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<InventoryResolved>)>;
    async fn insert(
        &self,
        inventory: InventoryUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Inventory>;
    async fn update(
        &self,
        inventory: InventoryUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Inventory>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl InventoryRepository for PgPoolManager {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Inventory> {
        Ok(sqlx::query_as::<_, Inventory>(
            r#"
            SELECT *
            FROM inventory
            WHERE inventory.deleted_at IS NULL
                AND inventory.id = $1
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
    ) -> RepositoryResult<InventoryResolved> {
        Ok(sqlx::query_as::<_, InventoryResolved>(
            r#"
            SELECT
                inventory.id as id,
                inventory.product_id as product_id,
                products.name as product,
                inventory.warehouse_id as warehouse_id,
                warehouses.name as warehouse,
                inventory.quantity_on_hand as quantity_on_hand,
                inventory.quantity_reserved as quantity_reserved,
                inventory.quantity_available as quantity_available,
                inventory.minimum_stock as minimum_stock,
                inventory.maximum_stock as maximum_stock,
                inventory.currency_code as currency_code,
                currencies.code as currency,
                inventory.status as status,
                inventory.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                inventory.created_at as created_at,
                inventory.updated_at as updated_at,
                inventory.deleted_at as deleted_at
            FROM inventory
            LEFT JOIN products ON inventory.product_id = products.id
            LEFT JOIN warehouses ON inventory.warehouse_id = warehouses.id
            LEFT JOIN currencies ON inventory.currency_code = currencies.code
            LEFT JOIN users ON inventory.created_by_id = users.id
            WHERE inventory.deleted_at IS NULL
                AND inventory.id = $1
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
            r#"
            SELECT
                inventory.id::VARCHAR as value,
                products.name || ' (Raktáron: ' || inventory.quantity_on_hand || ' - Lefoglalt: ' || inventory.quantity_reserved || ' - Elérhető: ' || inventory.quantity_available || ')' as title
                FROM inventory
                LEFT JOIN products ON inventory.product_id = products.id
                WHERE inventory.deleted_at IS NULL
                ORDER BY products.name
                "#,
        )
        .fetch_all(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_all_paged(
        &self,
        query_params: &GetQuery<InventoryOrderBy, InventoryFilterBy>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<InventoryResolved>)> {
        let total: (i64,) = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                let filter_by = match filter_by {
                    "product" => "products.name",
                    _ => return Err(RepositoryError::InvalidInput("filter_by".to_string())),
                };
                sqlx::query_as(&format!(
                    r#"SELECT COUNT(*) FROM inventory
                        LEFT JOIN products ON inventory.product_id = products.id
                        WHERE inventory.deleted_at IS NULL
                            AND ($1::TEXT IS NULL OR {filter_by}::TEXT ILIKE '%' || $1 || '%')
                    "#
                ))
                .bind(value_unchecked)
                .fetch_one(&self.get_tenant_pool(active_tenant)?)
                .await?
            }
            (_, _) => {
                sqlx::query_as("SELECT COUNT(*) FROM inventory WHERE deleted_at IS NULL")
                    .fetch_one(&self.get_tenant_pool(active_tenant)?)
                    .await?
            }
        };

        let order_by_clause = match (
            query_params.ordering().order_by(), // Security: ValueObject
            query_params.ordering().order(),    // Security: enum
        ) {
            (Some(order_by), Some(order)) => format!("ORDER BY inventory.{order_by} {order}"),
            (_, _) => "".to_string(),
        };

        let limit = i32::try_from(query_params.paging().limit().unwrap_or(25))?;

        let inventory = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                let filter_by = match filter_by {
                    "product" => "products.name",
                    _ => return Err(RepositoryError::InvalidInput("filter_by".to_string())),
                };
                let sql = format!(
                    r#"
                    SELECT
                        inventory.id as id,
                        inventory.product_id as product_id,
                        products.name as product,
                        inventory.warehouse_id as warehouse_id,
                        warehouses.name as warehouse,
                        inventory.quantity_on_hand as quantity_on_hand,
                        inventory.quantity_reserved as quantity_reserved,
                        inventory.quantity_available as quantity_available,
                        inventory.minimum_stock as minimum_stock,
                        inventory.maximum_stock as maximum_stock,
                        inventory.currency_code as currency_code,
                        currencies.code as currency,
                        inventory.status as status,
                        inventory.created_by_id as created_by_id,
                        users.last_name || ' ' || users.first_name as created_by,
                        inventory.created_at as created_at,
                        inventory.updated_at as updated_at,
                        inventory.deleted_at as deleted_at
                    FROM inventory
                    LEFT JOIN products ON inventory.product_id = products.id
                    LEFT JOIN warehouses ON inventory.warehouse_id = warehouses.id
                    LEFT JOIN currencies ON inventory.currency_code = currencies.code
                    LEFT JOIN users ON inventory.created_by_id = users.id
                    WHERE inventory.deleted_at IS NULL
                        AND ($1::TEXT IS NULL OR {filter_by}::TEXT ILIKE '%' || $1 || '%')
                    {order_by_clause}
                    LIMIT $2
                    OFFSET $3
                    "#
                );

                sqlx::query_as::<_, InventoryResolved>(&sql)
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
                        inventory.id as id,
                        inventory.product_id as product_id,
                        products.name as product,
                        inventory.warehouse_id as warehouse_id,
                        warehouses.name as warehouse,
                        inventory.quantity_on_hand as quantity_on_hand,
                        inventory.quantity_reserved as quantity_reserved,
                        inventory.quantity_available as quantity_available,
                        inventory.minimum_stock as minimum_stock,
                        inventory.maximum_stock as maximum_stock,
                        inventory.currency_code as currency_code,
                        currencies.code as currency,
                        inventory.status as status,
                        inventory.created_by_id as created_by_id,
                        users.last_name || ' ' || users.first_name as created_by,
                        inventory.created_at as created_at,
                        inventory.updated_at as updated_at,
                        inventory.deleted_at as deleted_at
                    FROM inventory
                    LEFT JOIN products ON inventory.product_id = products.id
                    LEFT JOIN warehouses ON inventory.warehouse_id = warehouses.id
                    LEFT JOIN currencies ON inventory.currency_code = currencies.code
                    LEFT JOIN users ON inventory.created_by_id = users.id
                    WHERE inventory.deleted_at IS NULL
                    {order_by_clause}
                    LIMIT $1
                    OFFSET $2
                    "#
                );

                sqlx::query_as::<_, InventoryResolved>(&sql)
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
            inventory,
        ))
    }
    async fn insert(
        &self,
        inventory: InventoryUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Inventory> {
        let minimum_stock = match &inventory.minimum_stock {
            None => None,
            Some(v) => Some(v.as_i32()?),
        };
        let maximum_stock = match &inventory.maximum_stock {
            None => None,
            Some(v) => Some(v.as_i32()?),
        };

        Ok(sqlx::query_as::<_, Inventory>(
            "INSERT INTO inventory (product_id, warehouse_id, minimum_stock, maximum_stock, currency_code, status, created_by_id)\
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
        )
            .bind(inventory.product_id)
            .bind(inventory.warehouse_id)
            .bind(minimum_stock)
            .bind(maximum_stock)
            .bind(inventory.currency_code.as_str())
            .bind(inventory.status.as_str())
            .bind(sub)
            .fetch_one(&self.get_tenant_pool(active_tenant)?)
            .await?
        )
    }

    async fn update(
        &self,
        inventory: InventoryUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Inventory> {
        let id = inventory
            .id
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;

        // Convert optional ValueObjects for minimum_stock and maximum_stock to Option<i32>
        let minimum_stock = match &inventory.minimum_stock {
            None => None,
            Some(v) => Some(v.as_i32()?),
        };
        let maximum_stock = match &inventory.maximum_stock {
            None => None,
            Some(v) => Some(v.as_i32()?),
        };

        Ok(sqlx::query_as::<_, Inventory>(
            r#"
            UPDATE inventory
            SET product_id = $1,
                warehouse_id = $2,
                minimum_stock = $3,
                maximum_stock = $4,
                currency_code = $5,
                status = $6
            WHERE id = $7
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(inventory.product_id)
        .bind(inventory.warehouse_id)
        .bind(minimum_stock)
        .bind(maximum_stock)
        .bind(inventory.currency_code.as_str())
        .bind(inventory.status.as_str())
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE inventory
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
