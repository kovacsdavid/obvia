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
use crate::tenant::warehouses::dto::CreateWarehouse;
use crate::tenant::warehouses::model::Warehouse;
use crate::tenant::warehouses::types::warehouse::WarehouseOrderBy;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WarehousesRepository: Send + Sync {
    async fn get_all(&self, active_tenant: Uuid) -> Result<Vec<Warehouse>, RepositoryError>;
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<WarehouseOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<Warehouse>)>;
    async fn insert(
        &self,
        warehouse: CreateWarehouse,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Warehouse, RepositoryError>;
}

#[async_trait]
impl WarehousesRepository for PoolManagerWrapper {
    async fn get_all(&self, active_tenant: Uuid) -> Result<Vec<Warehouse>, RepositoryError> {
        Ok(sqlx::query_as::<_, Warehouse>(
            "SELECT * FROM warehouses WHERE deleted_at IS NULL ORDER BY name ",
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
    ) -> RepositoryResult<(PaginatorMeta, Vec<Warehouse>)> {
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
            SELECT *
            FROM warehouses
            WHERE deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let warehouses = sqlx::query_as::<_, Warehouse>(&sql)
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
        warehouse: CreateWarehouse,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Warehouse, RepositoryError> {
        Ok(sqlx::query_as::<_, Warehouse>(
            "INSERT INTO warehouses (name, contact_name, contact_phone, status, created_by)\
             VALUES ($1, $2, $3, $4, $5) RETURNING *",
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
}
