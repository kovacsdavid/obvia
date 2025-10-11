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
use crate::tenant::services::dto::ServiceUserInput;
use crate::tenant::services::model::{Service, ServiceResolved};
use crate::tenant::services::types::service::ServiceOrderBy;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ServicesRepository: Send + Sync + 'static {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Service>;
    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<ServiceResolved>;
    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<ServiceOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ServiceResolved>)>;
    async fn insert(
        &self,
        service: ServiceUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Service>;
    async fn update(
        &self,
        service: ServiceUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Service>;
    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl ServicesRepository for PoolManagerWrapper {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Service> {
        let service = sqlx::query_as::<_, Service>(
            r#"SELECT * FROM services WHERE id = $1 AND deleted_at IS NULL"#,
        )
            .bind(id)
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?;
        Ok(service)
    }

    async fn get_resolved_by_id(
        &self,
        id: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<ServiceResolved> {
        let service = sqlx::query_as::<_, ServiceResolved>(
            r#"
            SELECT 
                services.id,
                services.name,
                services.description,
                services.default_price,
                services.default_tax_id,
                tax.description as default_tax,
                services.currency_id,
                currencies.currency as currency,
                services.status,
                services.created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                services.created_at,
                services.updated_at,
                services.deleted_at
            FROM services
            LEFT JOIN users ON services.created_by_id = users.id
            LEFT JOIN tax ON services.default_tax_id = tax.id
            LEFT JOIN currencies ON services.currency_id = currencies.id
            WHERE services.id = $1 AND services.deleted_at IS NULL
            "#,
        )
            .bind(id)
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?;
        Ok(service)
    }

    async fn get_all_paged(
        &self,
        paginator_params: &PaginatorParams,
        ordering_params: &OrderingParams<ServiceOrderBy>,
        filtering_params: &FilteringParams,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ServiceResolved>)> {
        let total: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM services WHERE deleted_at IS NULL")
                .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
                .await?;

        let order_by_clause = match ordering_params.order_by.extract().get_value().as_str() {
            "" => "".to_string(),
            order_by => format!("ORDER BY services.{order_by} {}", ordering_params.order),
        }; // SECURITY: ValueObject

        let sql = format!(
            r#"
            SELECT
                services.id,
                services.name,
                services.description,
                services.default_price,
                services.default_tax_id,
                tax.description as default_tax,
                services.currency_id,
                currencies.currency as currency,
                services.status,
                services.created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                services.created_at,
                services.updated_at,
                services.deleted_at
            FROM services 
            LEFT JOIN users ON services.created_by_id = users.id
            LEFT JOIN tax ON services.default_tax_id = tax.id
            LEFT JOIN currencies ON services.currency_id = currencies.id
            WHERE services.deleted_at IS NULL
            {order_by_clause}
            LIMIT $1
            OFFSET $2
            "#
        );

        let services = sqlx::query_as::<_, ServiceResolved>(&sql)
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
            services,
        ))
    }

    async fn insert(
        &self,
        service: ServiceUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Service> {
        Ok(sqlx::query_as::<_, Service>(
            r#"
            INSERT INTO services (name, description, default_price, default_tax_id, currency_id, status, created_by_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
            .bind(service.name.extract().get_value())
            .bind(service.description.as_ref().map(|d| d.extract().get_value().as_str()))
            .bind(service.default_price.as_ref().map(|d| d.extract().get_value().as_str()))
            .bind(service.default_tax_id)
            .bind(service.currency_id)
            .bind(service.status.extract().get_value())
            .bind(sub)
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?)
    }

    async fn update(
        &self,
        service: ServiceUserInput,
        active_tenant: Uuid,
    ) -> RepositoryResult<Service> {
        let id = service
            .id
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;
        Ok(sqlx::query_as::<_, Service>(
            r#"
            UPDATE services
            SET name = $1,
                description = $2,
                default_price = $3,
                default_tax_id = $4,
                currency_id = $5,
                status = $6
            WHERE id = $7 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
            .bind(service.name.extract().get_value())
            .bind(
                service
                    .description
                    .as_ref()
                    .map(|d| d.extract().get_value().as_str()),
            )
            .bind(
                service
                    .default_price
                    .as_ref()
                    .map(|d| d.extract().get_value().as_str()),
            )
            .bind(service.default_tax_id)
            .bind(service.currency_id)
            .bind(service.status.extract().get_value())
            .bind(id)
            .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?)
    }

    async fn delete_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE services
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
