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
use crate::tenant::customers::dto::CreateCustomer;
use crate::tenant::customers::model::Customer;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CustomersRespository: Send + Sync {
    async fn insert(
        &self,
        customer: CreateCustomer,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Customer, RepositoryError>;
}

#[async_trait]
impl CustomersRespository for PoolManagerWrapper {
    async fn insert(
        &self,
        customer: CreateCustomer,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Customer, RepositoryError> {
        Ok(sqlx::query_as::<_, Customer>(
            "INSERT INTO customers (name, contact_name, email, phone_number, status, type, created_by)
                 VALUES ($1, $2, $3,$4, $5, $6, $7) RETURNING *",
        )
        .bind(customer.name.extract().get_value())
        .bind(
            customer
                .contact_name
                .map(|v| v.extract().get_value().clone()),
        )
        .bind(customer.email.extract().get_value())
        .bind(
            customer
                .phone_number
                .map(|v| v.extract().get_value().clone()),
        )
        .bind(customer.status.extract().get_value())
        .bind(customer.customer_type.extract().get_value())
        .bind(sub)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
}
