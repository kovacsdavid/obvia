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
use crate::tenant::taxes::dto::TaxUserInput;
use crate::tenant::taxes::model::{Tax, TaxResolved};
use crate::tenant::taxes::types::TaxOrderBy;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TaxesRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Tax>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<TaxResolved>;
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>>;
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<TaxOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<TaxResolved>)>;
    async fn insert(
        &self,
        tax: TaxUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Tax>;
    async fn update(&self, tax: TaxUserInput, active_tenant: Uuid) -> RepositoryResult<Tax>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl TaxesRepository for PoolManagerWrapper {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Tax> {
        Ok(sqlx::query_as::<_, Tax>(
            r#"
            SELECT * 
            FROM taxes
            WHERE taxes.deleted_at IS NULL
                AND taxes.id = $1
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
    ) -> RepositoryResult<TaxResolved> {
        Ok(sqlx::query_as::<_, TaxResolved>(
            r#"
            SELECT
                taxes.id as id,
                taxes.rate as rate,
                taxes.description as description,
                taxes.country_code as country_code,
                countries.name as country,
                taxes.tax_category as tax_category,
                taxes.is_rate_applicable as is_rate_applicable,
                taxes.legal_text as legal_text,
                taxes.reporting_code as reporting_code,
                taxes.is_default as is_default,
                taxes.status as status,
                taxes.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                taxes.created_at as created_at,
                taxes.updated_at as updated_at,
                taxes.deleted_at as deleted_at
            FROM taxes
            LEFT JOIN users ON taxes.created_by_id = users.id
            LEFT JOIN countries ON taxes.country_code = countries.code
            WHERE taxes.deleted_at IS NULL
                AND taxes.id = $1
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
            r#"
            SELECT
                taxes.id::VARCHAR as value,
                taxes.description as title
                FROM taxes
                WHERE deleted_at IS NULL
                ORDER BY taxes.description
                "#,
        )
        .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<TaxOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<TaxResolved>)> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM taxes WHERE deleted_at IS NULL")
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY taxes.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

        let sql = format!(
            r#"
            SELECT
                taxes.id as id,
                taxes.rate as rate,
                taxes.description as description,
                taxes.country_code as country_code,
                countries.name as country,
                taxes.tax_category as tax_category,
                taxes.is_rate_applicable as is_rate_applicable,
                taxes.legal_text as legal_text,
                taxes.reporting_code as reporting_code,
                taxes.is_default as is_default,
                taxes.status as status,
                taxes.created_by_id as created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                taxes.created_at as created_at,
                taxes.updated_at as updated_at,
                taxes.deleted_at as deleted_at
            FROM taxes
            LEFT JOIN users ON taxes.created_by_id = users.id
            LEFT JOIN countries ON taxes.country_code = countries.code
            WHERE taxes.deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let taxes = sqlx::query_as::<_, TaxResolved>(&sql)
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
            taxes,
        ))
    }
    async fn insert(
        &self,
        tax: TaxUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Tax> {
        let rate = match &tax.rate {
            None => None,
            Some(v) => Some(
                v.extract()
                    .get_value()
                    .parse::<f64>()
                    .map_err(|_| RepositoryError::InvalidInput("rate".to_string()))?,
            ),
        };
        Ok(sqlx::query_as::<_, Tax>(
            "INSERT INTO taxes (
                rate,
                description,
                country_code,
                tax_category,
                is_rate_applicable,
                legal_text,
                reporting_code,
                is_default,
                status,
                created_by_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *",
        )
        .bind(rate)
        .bind(tax.description.extract().get_value())
        .bind(tax.country_code.extract().get_value())
        .bind(tax.tax_category.extract().get_value())
        .bind(tax.is_rate_applicable)
        .bind(
            tax.legal_text
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(
            tax.reporting_code
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(tax.is_default)
        .bind(tax.status.extract().get_value())
        .bind(sub)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn update(&self, tax: TaxUserInput, active_tenant: Uuid) -> RepositoryResult<Tax> {
        let id = tax
            .id
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;
        let rate = match &tax.rate {
            None => None,
            Some(v) => Some(
                v.extract()
                    .get_value()
                    .parse::<f64>()
                    .map_err(|_| RepositoryError::InvalidInput("rate".to_string()))?,
            ),
        };
        Ok(sqlx::query_as::<_, Tax>(
            r#"
            UPDATE taxes
            SET rate = $1,
                description = $2,
                country_code = $3,
                tax_category = $4,
                is_rate_applicable = $5,
                legal_text = $6,
                reporting_code = $7,
                is_default = $8,
                status = $9
            WHERE id = $10
                AND deleted_at IS NULL 
            RETURNING *
            "#,
        )
        .bind(rate)
        .bind(tax.description.extract().get_value())
        .bind(tax.country_code.extract().get_value())
        .bind(tax.tax_category.extract().get_value())
        .bind(tax.is_rate_applicable)
        .bind(
            tax.legal_text
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(
            tax.reporting_code
                .as_ref()
                .map(|d| d.extract().get_value().as_str()),
        )
        .bind(tax.is_default)
        .bind(tax.status.extract().get_value())
        .bind(id)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE taxes
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
