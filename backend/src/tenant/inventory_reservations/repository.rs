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
use crate::common::query_parser::GetQuery;
use crate::manager::app::database::{PgPoolManager, PoolManager};
use crate::tenant::inventory_reservations::dto::InventoryReservationUserInput;
use crate::tenant::inventory_reservations::model::{
    InventoryReservation, InventoryReservationResolved,
};
use crate::tenant::inventory_reservations::types::{
    InventoryReservationFilterBy, InventoryReservationOrderBy,
};
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
        query_params: &GetQuery<InventoryReservationOrderBy, InventoryReservationFilterBy>,
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
impl InventoryReservationsRepository for PgPoolManager {
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
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
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
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_all_paged(
        &self,
        query_params: &GetQuery<InventoryReservationOrderBy, InventoryReservationFilterBy>,
        active_tenant: Uuid,
        inventory_id: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<InventoryReservationResolved>)> {
        let total: (i64,) = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                sqlx::query_as(&format!(
                    r#"
                    SELECT COUNT(*)
                    FROM inventory_reservations
                    WHERE inventory_id = $1
                        AND ($2::TEXT IS NULL OR inventory_reservations.{filter_by}::TEXT ILIKE '%' || $2 || '%')
                    "#,
                ))
                .bind(inventory_id)
                .bind(value_unchecked)
                .fetch_one(&self.get_tenant_pool(active_tenant)?)
                .await?
            }
            (_, _) => {
                sqlx::query_as(
                    r#"
                    SELECT COUNT(*)
                    FROM inventory_reservations
                    WHERE inventory_id = $1
                    "#,
                )
                .bind(inventory_id)
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

        let inventory_reservations = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
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
                        AND ($2::TEXT IS NULL OR inventory_reservations.{filter_by}::TEXT ILIKE '%' || $2 || '%')
                    {order_by_clause}
                    OFFSET $3
                    LIMIT $4
                    "#
                );

                sqlx::query_as::<_, InventoryReservationResolved>(&query)
                    .bind(inventory_id)
                    .bind(value_unchecked)
                    .bind(limit)
                    .bind(i32::try_from(query_params.paging().offset().unwrap_or(0))?)
                    .fetch_all(&self.get_tenant_pool(active_tenant)?)
                    .await?
            }
            (_, _) => {
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
                    {order_by_clause}
                    OFFSET $2
                    LIMIT $3
                    "#
                );

                sqlx::query_as::<_, InventoryReservationResolved>(&query)
                    .bind(inventory_id)
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
            inventory_reservations,
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
                NaiveDate::parse_from_str(v.as_str(), "%Y-%m-%d")
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
        .bind(input.quantity.as_i32()?)
        .bind(input.reference_type.as_ref().map(|d| d.as_str()))
        .bind(input.reference_id)
        .bind(reserved_until)
        .bind(input.status.as_str())
        .bind(sub)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        let _ = sqlx::query(
            r#"
            DELETE FROM inventory_reservations WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.get_tenant_pool(active_tenant)?)
        .await?;
        Ok(())
    }
}
