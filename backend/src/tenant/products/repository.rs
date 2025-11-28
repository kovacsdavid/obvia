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
use crate::common::types::value_object::ValueObjectable;
use crate::manager::app::database::{PgPoolManager, PoolManager};
use crate::manager::tenants::dto::FilteringParams; // TODO: this is not the right filtering params
use crate::tenant::products::dto::ProductUserInput;
use crate::tenant::products::model::{Product, ProductResolved, UnitOfMeasure};
use crate::tenant::products::types::product::ProductOrderBy;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProductsRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Product>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<ProductResolved>;
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>>;
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<ProductOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ProductResolved>)>;
    async fn insert(
        &self,
        product: ProductUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Product>;
    async fn update(
        &self,
        product: ProductUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Product>;
    async fn insert_unit_of_measure(
        &self,
        unit_of_measure: &str,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<UnitOfMeasure>;
    async fn get_units_of_measure_select_list(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl ProductsRepository for PgPoolManager {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Product> {
        Ok(sqlx::query_as::<_, Product>(
            r#"
            SELECT *
            FROM products
            WHERE products.deleted_at IS NULL
                AND products.id = $1
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
    ) -> RepositoryResult<ProductResolved> {
        Ok(sqlx::query_as::<_, ProductResolved>(
            r#"
            SELECT
                products.id as id,
                products.name as name,
                products.description as description,
                products.unit_of_measure_id as unit_of_measure_id,
                units_of_measure.unit_of_measure as unit_of_measure,
                products.status as status,
                products.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                products.created_at as created_at,
                products.updated_at as updated_at,
                products.deleted_at as deleted_at
            FROM products
            LEFT JOIN units_of_measure ON products.unit_of_measure_id = units_of_measure.id
            LEFT JOIN users ON products.created_by_id = users.id
            WHERE products.deleted_at IS NULL
                AND products.id = $1
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
            "SELECT products.id::VARCHAR as value, products.name as title FROM products WHERE deleted_at IS NULL ORDER BY name",
        )
        .fetch_all(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<ProductOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ProductResolved>)> {
        let total: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM products WHERE deleted_at IS NULL")
                .fetch_one(&self.get_tenant_pool(active_tenant)?)
                .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY products.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

        let sql = format!(
            r#"
            SELECT
                products.id as id,
                products.name as name,
                products.description as description,
                products.unit_of_measure_id as unit_of_measure_id,
                units_of_measure.unit_of_measure as unit_of_measure,
                products.status as status,
                products.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                products.created_at as created_at,
                products.updated_at as updated_at,
                products.deleted_at as deleted_at
            FROM products
            LEFT JOIN units_of_measure ON products.unit_of_measure_id = units_of_measure.id
            LEFT JOIN users ON products.created_by_id = users.id
            WHERE products.deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let products = sqlx::query_as::<_, ProductResolved>(&sql)
            .bind(paginator_params.limit)
            .bind(paginator_params.offset())
            .fetch_all(&self.get_tenant_pool(active_tenant)?)
            .await?;

        Ok((
            PaginatorMeta {
                page: paginator_params.page,
                limit: paginator_params.limit,
                total: total.0,
            },
            products,
        ))
    }
    async fn insert(
        &self,
        product: ProductUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Product, RepositoryError> {
        Ok(sqlx::query_as::<_, Product>(
            "INSERT INTO products (name, description, unit_of_measure_id, status, created_by_id)
                 VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(product.name.extract().get_value())
        .bind(
            product
                .description
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(
            product
                .unit_of_measure_id
                .ok_or(RepositoryError::InvalidInput(
                    "unit_of_measure_id".to_string(),
                ))?,
        )
        .bind(product.status.extract().get_value())
        .bind(sub)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn update(
        &self,
        product: ProductUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Product> {
        let id = product
            .id
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;
        Ok(sqlx::query_as::<_, Product>(
            r#"
            UPDATE products
            SET name = $1,
                description = $2,
                unit_of_measure_id = $3,
                status = $4
            WHERE id = $5
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(product.name.extract().get_value())
        .bind(
            product
                .description
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(
            product
                .unit_of_measure_id
                .ok_or(RepositoryError::InvalidInput(
                    "unit_of_measure_id".to_string(),
                ))?,
        )
        .bind(product.status.extract().get_value())
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn insert_unit_of_measure(
        &self,
        unit_of_measure: &str,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<UnitOfMeasure, RepositoryError> {
        Ok(sqlx::query_as::<_, UnitOfMeasure>(
            "INSERT INTO units_of_measure(unit_of_measure, created_by_id)
             VALUES ($1, $2) RETURNING *",
        )
        .bind(unit_of_measure.to_string().trim())
        .bind(sub)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_units_of_measure_select_list(
        &self,
        active_tenant: Uuid,
    ) -> Result<Vec<SelectOption>, RepositoryError> {
        Ok(sqlx::query_as::<_, SelectOption>(
            "SELECT units_of_measure.id::VARCHAR as value, units_of_measure.unit_of_measure as title FROM units_of_measure WHERE deleted_at IS NULL ORDER BY unit_of_measure",
        )
        .fetch_all(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE products 
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
