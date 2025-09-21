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
use crate::tenant::products::dto::CreateProduct;
use crate::tenant::products::model::{Currency, Product, UnitOfMeasure};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProductsRepository: Send + Sync {
    async fn insert(
        &self,
        product: CreateProduct,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Product, RepositoryError>;
    async fn insert_unit_of_measure(
        &self,
        unit_of_measure: &str,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<UnitOfMeasure, RepositoryError>;
    async fn get_all_units_of_measure(
        &self,
        active_tenant: Uuid,
    ) -> Result<Vec<UnitOfMeasure>, RepositoryError>;
    async fn insert_currency(
        &self,
        currency: &str,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Currency, RepositoryError>;
    async fn get_all_currencies(
        &self,
        active_tenant: Uuid,
    ) -> Result<Vec<Currency>, RepositoryError>;
}

#[async_trait]
impl ProductsRepository for PoolManagerWrapper {
    async fn insert(
        &self,
        product: CreateProduct,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Product, RepositoryError> {
        let price = match &product.price {
            None => None,
            Some(v) => Some(
                v.extract()
                    .get_value()
                    .parse::<f64>()
                    .map_err(|_| RepositoryError::Parse("price".to_string()))?,
            ),
        };
        let cost = match &product.cost {
            None => None,
            Some(v) => Some(
                v.extract()
                    .get_value()
                    .parse::<f64>()
                    .map_err(|_| RepositoryError::Parse("cost".to_string()))?,
            ),
        };
        Ok(
            sqlx::query_as::<_, Product>(
                "INSERT INTO products (name, description, unit_of_measure_id, price, cost, currency_id, status, created_by)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
            )
                .bind(product.name.extract().get_value())
                .bind(product.description.map(|v| v.extract().get_value().clone()))
                .bind(product.unit_of_measure_id.ok_or(RepositoryError::Parse("unit_of_measure_id".to_string()))?)
                .bind(price)
                .bind(cost)
                .bind(product.currency_id.ok_or(RepositoryError::Parse("currency_id".to_string()))?)
                .bind(product.status.extract().get_value())
                .bind(sub)
                .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
                .await?
        )
    }

    async fn insert_unit_of_measure(
        &self,
        unit_of_measure: &str,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<UnitOfMeasure, RepositoryError> {
        Ok(sqlx::query_as::<_, UnitOfMeasure>(
            "INSERT INTO units_of_measure(unit_of_measure, created_by)
             VALUES ($1, $2) RETURNING *",
        )
        .bind(unit_of_measure.to_string().trim())
        .bind(sub)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_all_units_of_measure(
        &self,
        active_tenant: Uuid,
    ) -> Result<Vec<UnitOfMeasure>, RepositoryError> {
        Ok(sqlx::query_as::<_, UnitOfMeasure>(
            "SELECT * FROM units_of_measure WHERE deleted_at IS NULL ORDER BY unit_of_measure",
        )
        .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn insert_currency(
        &self,
        currency: &str,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Currency, RepositoryError> {
        Ok(sqlx::query_as::<_, Currency>(
            "INSERT INTO currencies(currency, created_by)
             VALUES ($1, $2) RETURNING *",
        )
        .bind(currency.to_string().trim().to_uppercase())
        .bind(sub)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_all_currencies(
        &self,
        active_tenant: Uuid,
    ) -> Result<Vec<Currency>, RepositoryError> {
        Ok(sqlx::query_as::<_, Currency>(
            "SELECT * FROM currencies WHERE deleted_at IS NULL ORDER BY currency",
        )
        .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
}
