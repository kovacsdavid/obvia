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
use crate::tenant::customers::dto::CustomerUserInput;
use crate::tenant::customers::model::{Customer, CustomerResolved};
use crate::tenant::customers::types::customer::{CustomerFilterBy, CustomerOrderBy};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CustomersRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Customer>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<CustomerResolved>;
    async fn get_all_paged(
        &self,
        query_params: &GetQuery<CustomerOrderBy, CustomerFilterBy>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<CustomerResolved>)>;
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>>;
    async fn insert(
        &self,
        customer: CustomerUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Customer>;
    async fn update(
        &self,
        customer: CustomerUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Customer>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl CustomersRepository for PgPoolManager {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Customer> {
        Ok(sqlx::query_as::<_, Customer>(
            r#"
            SELECT * 
            FROM customers
            WHERE customers.deleted_at IS NULL
                AND customers.id = $1
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
    ) -> RepositoryResult<CustomerResolved> {
        Ok(sqlx::query_as::<_, CustomerResolved>(
            r#"
            SELECT
                customers.id as id,
                customers.name as name,
                customers.contact_name as contact_name,
                customers.email as email,
                customers.phone_number as phone_number,
                customers.status as status,
                customers.customer_type as customer_type,
                customers.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                customers.created_at as created_at,
                customers.updated_at as updated_at,
                customers.deleted_at as deleted_at
            FROM customers
            LEFT JOIN users ON customers.created_by_id = users.id
            WHERE customers.deleted_at IS NULL
                AND customers.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_all_paged(
        &self,
        query_params: &GetQuery<CustomerOrderBy, CustomerFilterBy>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<CustomerResolved>)> {
        let total: (i64,) = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                sqlx::query_as(&format!(
                    r#"SELECT COUNT(*) FROM customers
                           WHERE deleted_at IS NULL
                               AND $1::TEXT IS NULL OR customers.{filter_by}::TEXT ILIKE $1"#
                ))
                .bind(value_unchecked)
                .fetch_one(&self.get_tenant_pool(active_tenant)?)
                .await?
            }
            (_, _) => {
                sqlx::query_as("SELECT COUNT(*) FROM customers WHERE deleted_at IS NULL")
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

        let customers = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                let sql = format!(
                    r#"
                        SELECT
                            customers.id as id,
                            customers.name as name,
                            customers.contact_name as contact_name,
                            customers.email as email,
                            customers.phone_number as phone_number,
                            customers.status as status,
                            customers.customer_type as customer_type,
                            customers.created_by_id as created_by_id,
                            users.last_name || ' ' || users.first_name as created_by,
                            customers.created_at as created_at,
                            customers.updated_at as updated_at,
                            customers.deleted_at as deleted_at
                        FROM customers
                        LEFT JOIN users ON customers.created_by_id = users.id
                        WHERE customers.deleted_at IS NULL
                            AND $1::TEXT IS NULL OR customers.{filter_by}::TEXT ILIKE $1
                        {order_by_clause}
                        LIMIT $2
                        OFFSET $3
                    "#
                );

                sqlx::query_as::<_, CustomerResolved>(&sql)
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
                            customers.id as id,
                            customers.name as name,
                            customers.contact_name as contact_name,
                            customers.email as email,
                            customers.phone_number as phone_number,
                            customers.status as status,
                            customers.customer_type as customer_type,
                            customers.created_by_id as created_by_id,
                            users.last_name || ' ' || users.first_name as created_by,
                            customers.created_at as created_at,
                            customers.updated_at as updated_at,
                            customers.deleted_at as deleted_at
                        FROM customers
                        LEFT JOIN users ON customers.created_by_id = users.id
                        WHERE customers.deleted_at IS NULL
                        {order_by_clause}
                        LIMIT $1
                        OFFSET $2
                        "#
                );

                sqlx::query_as::<_, CustomerResolved>(&sql)
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
            customers,
        ))
    }

    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>> {
        Ok(sqlx::query_as::<_, SelectOption>(
            "SELECT customers.id::VARCHAR as value, customers.name as title FROM customers WHERE deleted_at IS NULL ORDER BY name",
        )
            .fetch_all(&self.get_tenant_pool(active_tenant)?)
            .await?)
    }

    async fn insert(
        &self,
        customer: CustomerUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Customer> {
        Ok(sqlx::query_as::<_, Customer>(
            "INSERT INTO customers (name, contact_name, email, phone_number, status, customer_type, created_by_id)
                 VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        )
        .bind(customer.name.as_str())
        .bind(
            customer
                .contact_name
                .as_ref()
                .map(|d| d.as_str()),
        )
        .bind(customer.email.as_str())
        .bind(
            customer
                .phone_number
                .as_ref()
                .map(|d| d.as_str()),
        )
        .bind(customer.status.as_str())
        .bind(customer.customer_type.as_str())
        .bind(sub)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn update(
        &self,
        customer: CustomerUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Customer> {
        let id = customer
            .id
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;
        Ok(sqlx::query_as::<_, Customer>(
            r#"
            UPDATE customers 
            SET name = $1,
                contact_name = $2,
                email = $3,
                phone_number = $4,
                status = $5,
                customer_type = $6
            WHERE id = $7
                AND deleted_at IS NULL 
            RETURNING *
            "#,
        )
        .bind(customer.name.as_str())
        .bind(customer.contact_name.as_ref().map(|d| d.as_str()))
        .bind(customer.email.as_str())
        .bind(customer.phone_number.as_ref().map(|d| d.as_str()))
        .bind(customer.status.as_str())
        .bind(customer.customer_type.as_str())
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE customers 
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
