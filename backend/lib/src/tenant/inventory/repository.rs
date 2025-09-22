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

use crate::common::error::RepositoryError;
use crate::manager::common::repository::PoolManagerWrapper;
use crate::manager::common::types::value_object::ValueObjectable;
use crate::tenant::inventory::dto::CreateInventory;
use crate::tenant::inventory::model::{Currency, Inventory};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait InventoryRepository: Send + Sync {
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
