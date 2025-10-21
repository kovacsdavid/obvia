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
use crate::tenant::inventory_reservations::dto::InventoryReservationUserInput;
use crate::tenant::inventory_reservations::model::{
    InventoryReservation, InventoryReservationResolved,
};
use crate::tenant::inventory_reservations::types::InventoryReservationOrderBy;
use async_trait::async_trait;
use chrono::NaiveDate;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait InventoryReservationsRepository: Send + Sync {
    async fn get_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<InventoryReservation>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<InventoryReservationResolved>;
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<InventoryReservationOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
        inventory_id: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<InventoryReservationResolved>)>;
    async fn insert(
        &self,
        input: InventoryReservationUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<InventoryReservation>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl InventoryReservationsRepository for PoolManagerWrapper {
    async fn get_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<InventoryReservation> {
        Ok(sqlx::query_as::<_, InventoryReservation>(
            r#"
            SELECT *
            FROM inventory_reservations
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
    ) -> RepositoryResult<InventoryReservationResolved> {
        Ok(sqlx::query_as::<_, InventoryReservationResolved>(
            r#"
            SELECT
                inventory_reservations.id,
                inventory_reservations.inventory_id,
                inventory_reservations.quantity,
                inventory_reservations.reference_type,
                inventory_reservations.reference_id, 
                inventory_reservations.reserved_until,
                inventory_reservations.status,
                inventory_reservations.created_by_id,
                (users.last_name || ' ' || users.first_name) AS created_by,
                inventory_reservations.created_at,
                inventory_reservations.updated_at
            FROM inventory_reservations
            LEFT JOIN users ON inventory_reservations.created_by_id = users.id
            WHERE inventory_reservations.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<InventoryReservationOrderBy>,
        _filtering_params: &FilteringParams,
        active_tenant: Uuid,
        inventory_id: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<InventoryReservationResolved>)> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM inventory_reservations m
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
                inventory_reservations.id,
                inventory_reservations.inventory_id,
                inventory_reservations.quantity,
                inventory_reservations.reference_type,
                inventory_reservations.reference_id,
                inventory_reservations.reserved_until,
                inventory_reservations.status,
                inventory_reservations.created_by_id,
                (users.last_name || ' ' || users.first_name) AS created_by,
                inventory_reservations.created_at,
                inventory_reservations.updated_at
            FROM inventory_reservations
            LEFT JOIN users ON inventory_reservations.created_by_id = users.id
            WHERE inventory_reservations.inventory_id = $1
            ORDER BY {order_by} {order}
            OFFSET $2 LIMIT $3
            "#
        ); // SECURITY: ValueObject

        let items = sqlx::query_as::<_, InventoryReservationResolved>(&query)
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
        input: InventoryReservationUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<InventoryReservation> {
        let reserved_until = match input.reserved_until {
            None => None,
            Some(v) => Some(
                NaiveDate::parse_from_str(v.extract().get_value(), "%Y-%m-%d")
                    .map_err(|e| RepositoryError::InvalidInput(e.to_string()))?,
            ),
        };
        Ok(sqlx::query_as::<_, InventoryReservation>(
            r#"
            INSERT INTO inventory_reservations (
                inventory_id, quantity, reference_type, reference_id, reserved_until,
                status, created_by_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(input.inventory_id)
        .bind(
            input
                .quantity
                .extract()
                .get_value()
                .parse::<i32>()
                .map_err(|_| RepositoryError::InvalidInput("quantity".to_string()))?,
        )
        .bind(
            input
                .reference_type
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(input.reference_id)
        .bind(reserved_until)
        .bind(input.status.extract().get_value())
        .bind(sub)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        let _ = sqlx::query(
            r#"
            DELETE FROM inventory_reservations WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?;
        Ok(())
    }
}
