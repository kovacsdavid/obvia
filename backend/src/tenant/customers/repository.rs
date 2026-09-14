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
use crate::tenant::address::dto::user_input::AddressUserInput;
use crate::tenant::address::model::Address;
use crate::tenant::customers::dto::user_input::CustomerUserInput;
use crate::tenant::customers::model::{Customer, CustomerResolved};
use crate::tenant::customers::types::customer::{CustomerFilterBy, CustomerOrderBy};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::{AssertSqlSafe, PgPool};
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CustomersRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> RepositoryResult<Customer>;
    async fn get_resolved_by_id(&self, id: Uuid) -> RepositoryResult<CustomerResolved>;
    async fn get_paged(
        &self,
        query_params: &ResourceQuery<CustomerOrderBy, CustomerFilterBy>,
    ) -> RepositoryResult<(PaginatorMeta, Vec<CustomerResolved>)>;
    async fn get_select_list_items(&self) -> RepositoryResult<Vec<SelectOption>>;
    async fn insert(&self, customer: &CustomerUserInput, sub: Uuid) -> RepositoryResult<Customer>;
    async fn update(&self, customer: &CustomerUserInput) -> RepositoryResult<Customer>;
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl CustomersRepository for PgPool {
    async fn get_by_id(&self, id: Uuid) -> RepositoryResult<Customer> {
        Ok(sqlx::query_as::<_, Customer>(
            r#"
            SELECT *
            FROM customers
            WHERE customers.deleted_at IS NULL
                AND customers.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(self)
        .await?)
    }

    async fn get_resolved_by_id(&self, id: Uuid) -> RepositoryResult<CustomerResolved> {
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
        .fetch_one(self)
        .await?)
    }

    async fn get_paged(
        &self,
        query_params: &ResourceQuery<CustomerOrderBy, CustomerFilterBy>,
    ) -> RepositoryResult<(PaginatorMeta, Vec<CustomerResolved>)> {
        let total: (i64,) = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                // TODO: check pg_trgm
                sqlx::query_as(AssertSqlSafe(format!(
                    r#"SELECT COUNT(*) FROM customers
                           WHERE deleted_at IS NULL
                               AND ($1::TEXT IS NULL OR customers.{filter_by}::TEXT ILIKE '%' || $1 || '%')"#
                )))
                .bind(value_unchecked)
                .fetch_one(self)
                .await?
            }
            (_, _) => {
                sqlx::query_as("SELECT COUNT(*) FROM customers WHERE deleted_at IS NULL")
                    .fetch_one(self)
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
                            AND ($1::TEXT IS NULL OR customers.{filter_by}::TEXT ILIKE '%' || $1 || '%')
                        {order_by_clause}
                        LIMIT $2
                        OFFSET $3
                    "#
                );

                sqlx::query_as::<_, CustomerResolved>(AssertSqlSafe(sql))
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

                sqlx::query_as::<_, CustomerResolved>(AssertSqlSafe(sql))
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
            customers,
        ))
    }

    async fn get_select_list_items(&self) -> RepositoryResult<Vec<SelectOption>> {
        Ok(sqlx::query_as::<_, SelectOption>(
            "SELECT customers.id::VARCHAR as value, customers.name as title FROM customers WHERE deleted_at IS NULL ORDER BY name",
        )
            .fetch_all(self)
            .await?)
    }

    async fn insert(&self, customer: &CustomerUserInput, sub: Uuid) -> RepositoryResult<Customer> {
        let mut tx = self.begin().await?;
        let contact_name = match &customer.contact_name {
            Some(v) => Some(v.as_str()?),
            None => None,
        };
        let mut insert_address = async |address: &AddressUserInput| -> RepositoryResult<Address> {
            Ok(sqlx::query_as::<_, Address>(
                r#"
                    INSERT INTO address (
                        type,
                        country_code, 
                        postal_code,
                        settlement,
                        mailbox,
                        topographic_number,
                        name_of_public_space,
                        type_of_public_space,
                        house_number,
                        building,
                        stairway,
                        floor,
                        door,
                        created_by_id
                    ) VALUES (
                        $1,
                        $2,
                        $3,
                        $4,
                        $5,
                        $6,
                        $7,
                        $8,
                        $9,
                        $10,
                        $11,
                        $12,
                        $13,
                        $14
                    )
                    RETURNING *
                "#,
            )
            .bind(address.address_type.as_str()?)
            .bind(address.country_code.as_str()?)
            .bind(address.postal_code.as_str()?)
            .bind(address.settlement.as_str()?)
            .bind(address.mailbox.as_str())
            .bind(address.topographic_number.as_str())
            .bind(address.name_of_public_space.as_str())
            .bind(address.type_of_public_space.as_str())
            .bind(address.house_number.as_str())
            .bind(address.building.as_str())
            .bind(address.stairway.as_str())
            .bind(address.floor.as_str())
            .bind(address.door.as_str())
            .bind(sub)
            .fetch_one(&mut *tx)
            .await?)
        };

        let billing_address = if let Some(billing_address) = &customer.billing_address {
            insert_address(billing_address).await.ok()
        } else {
            None
        };

        let mailing_address = if let Some(mailing_address) = &customer.mailing_address {
            insert_address(mailing_address).await.ok()
        } else {
            None
        };
        let customer = sqlx::query_as::<_, Customer>(
            r#"
                INSERT INTO customers (
                    name,
                    contact_name,
                    email,
                    phone_number,
                    status,
                    customer_type,
                    created_by_id,
                    billing_address,
                    mailing_address
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7,
                    $8,
                    $9
                ) RETURNING *
            "#,
        )
        .bind(customer.name.as_str()?)
        .bind(contact_name)
        .bind(customer.email.as_str()?)
        .bind(customer.phone_number.as_str())
        .bind(customer.status.as_str()?)
        .bind(customer.customer_type.as_str()?)
        .bind(sub)
        .bind(billing_address.map(|v| v.id))
        .bind(mailing_address.map(|v| v.id))
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(customer)
    }

    async fn update(&self, customer: &CustomerUserInput) -> RepositoryResult<Customer> {
        let contact_name = match &customer.contact_name {
            Some(v) => Some(v.as_str()?),
            None => None,
        };
        let id = customer
            .id
            .as_uuid()
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
        .bind(customer.name.as_str()?)
        .bind(contact_name)
        .bind(customer.email.as_str()?)
        .bind(customer.phone_number.as_str())
        .bind(customer.status.as_str()?)
        .bind(customer.customer_type.as_str()?)
        .bind(id)
        .fetch_one(self)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE customers 
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
