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
use crate::tenant::inventory::dto::CreateInventory;
use crate::tenant::inventory::model::{Currency, Inventory, ResolvedInventory};
use crate::tenant::inventory::types::inventory::InventoryOrderBy;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait InventoryRepository: Send + Sync {
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<InventoryOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ResolvedInventory>)>;
    async fn insert(
        &self,
        inventory: CreateInventory,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Inventory, RepositoryError>;
    async fn insert_currency(
        &self,
        currency: &str,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Currency, RepositoryError>;
    async fn get_all_currencies(
        &self,
        active_tenant: Uuid,
    ) -> Result<Vec<Currency>, RepositoryError>;
}

#[async_trait]
impl InventoryRepository for PoolManagerWrapper {
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<InventoryOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ResolvedInventory>)> {
        let total: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM inventory WHERE deleted_at IS NULL")
                .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
                .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY inventory.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

        let sql = format!(
            r#"
            SELECT
                inventory.id as id,
                products.id as product_id,
                products.name as product,
                warehouses.id as warehouse_id,
                warehouses.name as warehouse,
                inventory.quantity as quantity,
                inventory.price as price,
                inventory.cost as cost,
                currencies.id as currency_id,
                currencies.currency as currency,
                users.id as created_by,
                users.last_name || ' ' || users.first_name as created_by_resolved,
                inventory.created_at as created_at,
                inventory.updated_at as updated_at,
                inventory.deleted_at as deleted_at
            FROM inventory
            LEFT JOIN products ON inventory.product_id = products.id
            LEFT JOIN warehouses ON inventory.warehouse_id = warehouses.id
            LEFT JOIN currencies ON inventory.currency_id = currencies.id
            LEFT JOIN users ON inventory.created_by = users.id
            WHERE inventory.deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let inventory = sqlx::query_as::<_, ResolvedInventory>(&sql)
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
            inventory,
        ))
    }
    async fn insert(
        &self,
        inventory: CreateInventory,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Inventory, RepositoryError> {
        let price = match &inventory.price {
            None => None,
            Some(v) => Some(
                v.extract()
                    .get_value()
                    .parse::<f64>()
                    .map_err(|_| RepositoryError::Parse("price".to_string()))?,
            ),
        };
        let cost = match &inventory.cost {
            None => None,
            Some(v) => Some(
                v.extract()
                    .get_value()
                    .parse::<f64>()
                    .map_err(|_| RepositoryError::Parse("cost".to_string()))?,
            ),
        };

        Ok(sqlx::query_as::<_, Inventory>(
            "INSERT INTO inventory (product_id, warehouse_id, quantity, price, cost, currency_id, created_by)\
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
        )
            .bind(inventory.product_id)
            .bind(inventory.warehouse_id)
            .bind(inventory
                .quantity
                .extract()
                .get_value()
                      .trim()
                      .replace(",", ".")
                      .parse::<i32>()
                      .map_err(|_| RepositoryError::Parse("quantity".to_string()))?
            )
            .bind(price)
            .bind(cost)
            .bind(inventory.currency_id.ok_or(RepositoryError::Parse("currency_id".to_string()))?)
            .bind(sub)
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?
        )
    }
    async fn insert_currency(
        &self,
        currency: &str,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Currency, RepositoryError> {
        Ok(sqlx::query_as::<_, Currency>(
            "INSERT INTO currencies(currency, created_by)
             VALUES ($1, $2) RETURNING *",
        )
        .bind(currency.to_string().trim().to_uppercase())
        .bind(sub)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_all_currencies(
        &self,
        active_tenant: Uuid,
    ) -> Result<Vec<Currency>, RepositoryError> {
        Ok(sqlx::query_as::<_, Currency>(
            "SELECT * FROM currencies WHERE deleted_at IS NULL ORDER BY currency",
        )
        .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
}
