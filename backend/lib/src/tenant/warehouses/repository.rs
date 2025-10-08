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
use crate::tenant::warehouses::dto::WarehouseUserInput;
use crate::tenant::warehouses::model::{Warehouse, WarehouseResolved};
use crate::tenant::warehouses::types::warehouse::WarehouseOrderBy;
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
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<WarehouseOrderBy>,
        filtering_params: &FilteringParams,
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
impl WarehousesRepository for PoolManagerWrapper {
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
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
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
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>> {
        Ok(sqlx::query_as::<_, SelectOption>(
            "SELECT warehouses.id::VARCHAR as value, warehouses.name as title FROM warehouses WHERE deleted_at IS NULL ORDER BY name",
        )
        .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<WarehouseOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<WarehouseResolved>)> {
        let total: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM warehouses WHERE deleted_at IS NULL")
                .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
                .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY warehouses.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

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

        let warehouses = sqlx::query_as::<_, WarehouseResolved>(&sql)
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
        .bind(warehouse.name.extract().get_value())
        .bind(
            warehouse
                .contact_name
                .map(|v| v.extract().get_value().clone()),
        )
        .bind(
            warehouse
                .contact_phone
                .map(|v| v.extract().get_value().clone()),
        )
        .bind(warehouse.status.extract().get_value())
        .bind(sub)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
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
        .bind(warehouse.name.extract().get_value())
        .bind(
            warehouse
                .contact_name
                .map(|v| v.extract().get_value().clone()),
        )
        .bind(
            warehouse
                .contact_phone
                .map(|v| v.extract().get_value().clone()),
        )
        .bind(warehouse.status.extract().get_value())
        .bind(id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
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
        .execute(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?;

        Ok(())
    }
}
