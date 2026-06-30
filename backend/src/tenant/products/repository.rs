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
use crate::common::query_parser::ResourceQuery;
use crate::tenant::products::dto::user_input::ProductUserInput;
use crate::tenant::products::model::{Product, ProductResolved, UnitOfMeasure};
use crate::tenant::products::types::product::{ProductFilterBy, ProductOrderBy};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::{AssertSqlSafe, PgPool};
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProductsRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> RepositoryResult<Product>;
    async fn get_resolved_by_id(&self, id: Uuid) -> RepositoryResult<ProductResolved>;
    async fn get_select_list_items(&self) -> RepositoryResult<Vec<SelectOption>>;
    async fn get_paged(
        &self,
        query_params: &ResourceQuery<ProductOrderBy, ProductFilterBy>,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ProductResolved>)>;
    async fn insert(&self, product: &ProductUserInput, sub: Uuid) -> RepositoryResult<Product>;
    async fn update(&self, product: ProductUserInput) -> RepositoryResult<Product>;
    async fn insert_unit_of_measure(
        &self,
        unit_of_measure: &str,
        sub: Uuid,
    ) -> RepositoryResult<UnitOfMeasure>;
    async fn get_units_of_measure_select_list(&self) -> RepositoryResult<Vec<SelectOption>>;
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl ProductsRepository for PgPool {
    async fn get_by_id(&self, id: Uuid) -> RepositoryResult<Product> {
        Ok(sqlx::query_as::<_, Product>(
            r#"
            SELECT *
            FROM products
            WHERE products.deleted_at IS NULL
                AND products.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(self)
        .await?)
    }

    async fn get_resolved_by_id(&self, id: Uuid) -> RepositoryResult<ProductResolved> {
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
        .fetch_one(self)
        .await?)
    }
    async fn get_select_list_items(&self) -> RepositoryResult<Vec<SelectOption>> {
        Ok(sqlx::query_as::<_, SelectOption>(
            "SELECT products.id::VARCHAR as value, products.name as title FROM products WHERE deleted_at IS NULL ORDER BY name",
        )
        .fetch_all(self)
        .await?)
    }
    async fn get_paged(
        &self,
        query_params: &ResourceQuery<ProductOrderBy, ProductFilterBy>,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ProductResolved>)> {
        let total: (i64,) = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                sqlx::query_as(AssertSqlSafe(format!(
                    r#"SELECT COUNT(*) FROM products
                        WHERE deleted_at IS NULL
                            AND ($1::TEXT IS NULL OR products.{filter_by}::TEXT ILIKE '%' || $1 || '%')"#
                )))
                .bind(value_unchecked)
                .fetch_one(self)
                .await?
            }
            (_, _) => {
                sqlx::query_as("SELECT COUNT(*) FROM products WHERE deleted_at IS NULL")
                    .fetch_one(self)
                    .await?
            }
        };

        let order_by_clause = match (
            query_params.ordering().order_by(), // Security: ValueObject
            query_params.ordering().order(),    // Security: enum
        ) {
            (Some(order_by), Some(order)) => format!("ORDER BY products.{order_by} {order}"),
            (_, _) => "".to_string(),
        };

        let limit = i32::try_from(query_params.paging().limit().unwrap_or(25))?;

        let products = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
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
                        AND ($1::TEXT IS NULL OR products.{filter_by}::TEXT ILIKE '%' || $1 || '%')
                    {order_by_clause}
                    LIMIT $2
                    OFFSET $3
                    "#
                );

                sqlx::query_as::<_, ProductResolved>(AssertSqlSafe(sql))
                    .bind(value_unchecked)
                    .bind(limit)
                    .bind(i32::try_from(query_params.paging().offset().unwrap_or(0))?)
                    .fetch_all(self)
                    .await?
            }
            (_, _) => {
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

                sqlx::query_as::<_, ProductResolved>(AssertSqlSafe(sql))
                    .bind(limit)
                    .bind(i32::try_from(query_params.paging().offset().unwrap_or(0))?)
                    .fetch_all(self)
                    .await?
            }
        };

        Ok((
            PaginatorMeta {
                page: query_params.paging().page().unwrap_or(1).try_into()?,
                limit,
                total: total.0,
            },
            products,
        ))
    }
    async fn insert(
        &self,
        input: &ProductUserInput,
        sub: Uuid,
    ) -> Result<Product, RepositoryError> {
        let unit_of_measure_id = match &input.unit_of_measure_id {
            Some(v) => Some(v.as_uuid()?),
            None => None,
        };
        Ok(sqlx::query_as::<_, Product>(
            "INSERT INTO products (name, description, unit_of_measure_id, status, created_by_id)
                 VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(input.name.as_str()?)
        .bind(input.description.as_str())
        .bind(unit_of_measure_id)
        .bind(input.status.as_str()?)
        .bind(sub)
        .fetch_one(self)
        .await?)
    }

    async fn update(&self, input: ProductUserInput) -> RepositoryResult<Product> {
        let id = input
            .id
            .as_uuid()
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;
        let unit_of_measure_id = match &input.unit_of_measure_id {
            Some(v) => Some(v.as_uuid()?),
            None => None,
        };
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
        .bind(input.name.as_str()?)
        .bind(input.description.as_str())
        .bind(unit_of_measure_id)
        .bind(input.status.as_str()?)
        .bind(id)
        .fetch_one(self)
        .await?)
    }

    async fn insert_unit_of_measure(
        &self,
        unit_of_measure: &str,
        sub: Uuid,
    ) -> Result<UnitOfMeasure, RepositoryError> {
        Ok(sqlx::query_as::<_, UnitOfMeasure>(
            "INSERT INTO units_of_measure(unit_of_measure, created_by_id)
             VALUES ($1, $2) RETURNING *",
        )
        .bind(unit_of_measure.to_string().trim())
        .bind(sub)
        .fetch_one(self)
        .await?)
    }

    async fn get_units_of_measure_select_list(&self) -> Result<Vec<SelectOption>, RepositoryError> {
        Ok(sqlx::query_as::<_, SelectOption>(
            "SELECT units_of_measure.id::VARCHAR as value, units_of_measure.unit_of_measure as title FROM units_of_measure WHERE deleted_at IS NULL ORDER BY unit_of_measure",
        )
        .fetch_all(self)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE products 
            SET deleted_at = NOW()
            WHERE id = $1
                AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .execute(self)
        .await?;

        Ok(())
    }
}
