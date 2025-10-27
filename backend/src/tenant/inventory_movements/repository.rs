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
use crate::tenant::inventory_movements::dto::InventoryMovementUserInput;
use crate::tenant::inventory_movements::model::{InventoryMovement, InventoryMovementResolved};
use crate::tenant::inventory_movements::types::InventoryMovementOrderBy;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait InventoryMovementsRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid)
    -> RepositoryResult<InventoryMovement>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<InventoryMovementResolved>;
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<InventoryMovementOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
        inventory_id: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<InventoryMovementResolved>)>;
    async fn insert(
        &self,
        input: InventoryMovementUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<InventoryMovement>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl InventoryMovementsRepository for PoolManagerWrapper {
    async fn get_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<InventoryMovement> {
        Ok(sqlx::query_as::<_, InventoryMovement>(
            r#"
            SELECT *
            FROM inventory_movements
            WHERE id = $1
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
    ) -> RepositoryResult<InventoryMovementResolved> {
        Ok(sqlx::query_as::<_, InventoryMovementResolved>(
            r#"
            SELECT
                inventory_movements.id,
                inventory_movements.inventory_id,
                inventory_movements.movement_type,
                inventory_movements.quantity,
                inventory_movements.reference_type,
                inventory_movements.reference_id,
                inventory_movements.unit_price,
                inventory_movements.total_price,
                inventory_movements.tax_id,
                taxes.description as tax,
                inventory_movements.movement_date,
                inventory_movements.created_by_id,
                (users.last_name || ' ' || users.first_name) AS created_by,
                inventory_movements.created_at
            FROM inventory_movements
            LEFT JOIN taxes ON inventory_movements.tax_id = taxes.id
            LEFT JOIN users ON inventory_movements.created_by_id = users.id
            WHERE inventory_movements.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<InventoryMovementOrderBy>,
        _filtering_params: &FilteringParams,
        active_tenant: Uuid,
        inventory_id: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<InventoryMovementResolved>)> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM inventory_movements m
            WHERE m.inventory_id = $1
            "#,
        )
        .bind(inventory_id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?;

        let order_by = ordering_params.order_by.extract().get_value();
        let order = ordering_params.order.extract().get_value();
        let query = format!(
            r#"
            SELECT
                inventory_movements.id,
                inventory_movements.inventory_id,
                inventory_movements.movement_type,
                inventory_movements.quantity,
                inventory_movements.reference_type,
                inventory_movements.reference_id,
                inventory_movements.unit_price,
                inventory_movements.total_price,
                inventory_movements.tax_id,
                taxes.description as tax,
                inventory_movements.movement_date,
                inventory_movements.created_by_id,
                (users.last_name || ' ' || users.first_name) AS created_by,
                inventory_movements.created_at
            FROM inventory_movements
            LEFT JOIN taxes ON inventory_movements.tax_id = taxes.id
            LEFT JOIN users ON inventory_movements.created_by_id = users.id
            WHERE inventory_movements.inventory_id = $1
            ORDER BY {order_by} {order}
            OFFSET $2 LIMIT $3
            "#
        ); // SECURITY: ValueObject

        let items = sqlx::query_as::<_, InventoryMovementResolved>(&query)
            .bind(inventory_id)
            .bind(paginator_params.offset())
            .bind(paginator_params.limit)
            .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?;

        Ok((
            PaginatorMeta {
                page: paginator_params.page,
                limit: paginator_params.limit,
                total: count.0,
            },
            items,
        ))
    }

    async fn insert(
        &self,
        input: InventoryMovementUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<InventoryMovement> {
        let unit_price = match &input.unit_price {
            None => None,
            Some(v) => Some(
                v.extract()
                    .get_value()
                    .parse::<f64>()
                    .map_err(|_| RepositoryError::InvalidInput("unit_price".to_string()))?,
            ),
        };
        let total_price = match &input.total_price {
            None => None,
            Some(v) => Some(
                v.extract()
                    .get_value()
                    .parse::<f64>()
                    .map_err(|_| RepositoryError::InvalidInput("total_price".to_string()))?,
            ),
        };
        let movement_type = input.movement_type.extract().get_value();
        let quantity = input
            .quantity(movement_type == "out")
            .map_err(|e| RepositoryError::InvalidInput("quantity".to_string()))?;
        Ok(sqlx::query_as::<_, InventoryMovement>(
            r#"
            INSERT INTO inventory_movements (
                inventory_id, movement_type, quantity, reference_type, reference_id, unit_price,
                total_price, tax_id, created_by_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(input.inventory_id)
        .bind(movement_type)
        .bind(quantity)
        .bind(
            input
                .reference_type
                .as_ref()
                .map(|v| v.extract().get_value().as_str()),
        )
        .bind(input.reference_id)
        .bind(unit_price)
        .bind(total_price)
        .bind(input.tax_id)
        .bind(sub)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        let _ = sqlx::query(
            r#"
            DELETE FROM inventory_movements WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?;
        Ok(())
    }
}
