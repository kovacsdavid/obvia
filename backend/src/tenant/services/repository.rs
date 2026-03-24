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
use crate::tenant::services::dto::ServiceUserInput;
use crate::tenant::services::model::{Service, ServiceResolved};
use crate::tenant::services::types::service::{ServiceFilterBy, ServiceOrderBy};
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
    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>>;
    async fn get_all_paged(
        &self,
        query_params: &GetQuery<ServiceOrderBy, ServiceFilterBy>,
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
impl ServicesRepository for PgPoolManager {
    async fn get_by_id(&self, id: Uuid, active_tenant: Uuid) -> RepositoryResult<Service> {
        let service = sqlx::query_as::<_, Service>(
            r#"SELECT * FROM services WHERE id = $1 AND deleted_at IS NULL"#,
        )
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
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
                taxes.description as default_tax,
                services.currency_code,
                currencies.code as currency,
                services.status,
                services.created_by_id,
                users.last_name || ' ' || users.first_name as created_by,
                services.created_at,
                services.updated_at,
                services.deleted_at
            FROM services
            LEFT JOIN users ON services.created_by_id = users.id
            LEFT JOIN taxes ON services.default_tax_id = taxes.id
            LEFT JOIN currencies ON services.currency_code = currencies.code
            WHERE services.id = $1 AND services.deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
        .await?;
        Ok(service)
    }

    async fn get_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>> {
        Ok(sqlx::query_as::<_, SelectOption>(
            r#"
            SELECT
                services.id::VARCHAR as value,
                services.name as title
                FROM services
                WHERE deleted_at IS NULL
                ORDER BY services.description
                "#,
        )
        .fetch_all(&self.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn get_all_paged(
        &self,
        query_params: &GetQuery<ServiceOrderBy, ServiceFilterBy>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ServiceResolved>)> {
        let total: (i64,) = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                sqlx::query_as(&format!(
                    r#"SELECT COUNT(*) FROM services
                        WHERE deleted_at IS NULL
                            AND ($1::TEXT IS NULL OR services.{filter_by}::TEXT ILIKE '%' || $1 || '%')"#
                ))
                .bind(value_unchecked)
                .fetch_one(&self.get_tenant_pool(active_tenant)?)
                .await?
            }
            (_, _) => {
                sqlx::query_as("SELECT COUNT(*) FROM services WHERE deleted_at IS NULL")
                    .fetch_one(&self.get_tenant_pool(active_tenant)?)
                    .await?
            }
        };

        let order_by_clause = match (
            query_params.ordering().order_by(), // Security: ValueObject
            query_params.ordering().order(),    // Security: enum
        ) {
            (Some(order_by), Some(order)) => format!("ORDER BY services.{order_by} {order}"),
            (_, _) => "".to_string(),
        };

        let limit = i32::try_from(query_params.paging().limit().unwrap_or(25))?;

        let services = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                let sql = format!(
                    r#"
                    SELECT
                        services.id,
                        services.name,
                        services.description,
                        services.default_price,
                        services.default_tax_id,
                        taxes.description as default_tax,
                        services.currency_code,
                        currencies.code as currency,
                        services.status,
                        services.created_by_id,
                        users.last_name || ' ' || users.first_name as created_by,
                        services.created_at,
                        services.updated_at,
                        services.deleted_at
                    FROM services 
                    LEFT JOIN users ON services.created_by_id = users.id
                    LEFT JOIN taxes ON services.default_tax_id = taxes.id
                    LEFT JOIN currencies ON services.currency_code = currencies.code
                    WHERE services.deleted_at IS NULL
                        AND ($1::TEXT IS NULL OR services.{filter_by}::TEXT ILIKE '%' || $1 || '%')
                    {order_by_clause}
                    LIMIT $2
                    OFFSET $3
                    "#
                );

                sqlx::query_as::<_, ServiceResolved>(&sql)
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
                        services.id,
                        services.name,
                        services.description,
                        services.default_price,
                        services.default_tax_id,
                        taxes.description as default_tax,
                        services.currency_code,
                        currencies.code as currency,
                        services.status,
                        services.created_by_id,
                        users.last_name || ' ' || users.first_name as created_by,
                        services.created_at,
                        services.updated_at,
                        services.deleted_at
                    FROM services 
                    LEFT JOIN users ON services.created_by_id = users.id
                    LEFT JOIN taxes ON services.default_tax_id = taxes.id
                    LEFT JOIN currencies ON services.currency_code = currencies.code
                    WHERE services.deleted_at IS NULL
                    {order_by_clause}
                    LIMIT $1
                    OFFSET $2
                    "#
                );

                sqlx::query_as::<_, ServiceResolved>(&sql)
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
            services,
        ))
    }

    async fn insert(
        &self,
        service: ServiceUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<Service> {
        let default_price = match &service.default_price {
            None => None,
            Some(v) => Some(v.as_f64()?),
        };
        let default_tax_id = match &service.default_tax_id {
            Some(v) => Some(v.as_uuid()?),
            None => None,
        };
        Ok(sqlx::query_as::<_, Service>(
            r#"
            INSERT INTO services (name, description, default_price, default_tax_id, currency_code, status, created_by_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
            .bind(service.name.as_str())
            .bind(service.description.as_ref().map(|d| d.as_str()))
            .bind(default_price)
            .bind(default_tax_id)
            .bind(service.currency_code.as_ref().map(|d| d.as_str()))
            .bind(service.status.as_str())
            .bind(sub)
            .fetch_one(&self.get_tenant_pool(active_tenant)?)
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
        let default_price = match &service.default_price {
            None => None,
            Some(v) => Some(v.as_f64()?),
        };
        let default_tax_id = match &service.default_tax_id {
            Some(v) => Some(v.as_uuid()?),
            None => None,
        };
        Ok(sqlx::query_as::<_, Service>(
            r#"
            UPDATE services
            SET name = $1,
                description = $2,
                default_price = $3,
                default_tax_id = $4,
                currency_code = $5,
                status = $6
            WHERE id = $7 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(service.name.as_str())
        .bind(service.description.as_ref().map(|d| d.as_str()))
        .bind(default_price)
        .bind(default_tax_id)
        .bind(service.currency_code.as_ref().map(|d| d.as_str()))
        .bind(service.status.as_str())
        .bind(id.as_uuid()?)
        .fetch_one(&self.get_tenant_pool(active_tenant)?)
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
        .execute(&self.get_tenant_pool(active_tenant)?)
        .await?;

        Ok(())
    }
}
