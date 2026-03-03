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
use crate::common::query_parser::GetQuery;
use crate::common::types::DdlParameter;
use crate::common::types::ValueObject;
use crate::manager::app::config::{AppConfig, BasicDatabaseConfig, DatabasePoolSizeProvider};
use crate::manager::app::database::{PgPoolManager, PoolManager};
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::model::{Tenant, UserTenant};
use crate::manager::tenants::types::{TenantFilterBy, TenantOrderBy};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::Error;
use sqlx::PgConnection;
use std::sync::Arc;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TenantsRepository: Send + Sync {
    #[allow(dead_code)]
    async fn get_by_uuid(&self, uuid: Uuid) -> RepositoryResult<Tenant>;

    async fn setup_self_hosted(
        &self,
        name: &str,
        db_config: &BasicDatabaseConfig,
        claims: &Claims,
    ) -> RepositoryResult<Tenant>;

    async fn setup_managed(
        &self,
        uuid: Uuid,
        name: &str,
        db_config: &BasicDatabaseConfig,
        claims: &Claims,
        app_config: Arc<AppConfig>,
    ) -> RepositoryResult<Tenant>;
    #[allow(dead_code)]
    async fn get_all_by_user_id(
        &self,
        user_uuid: Uuid,
        query_params: &GetQuery<TenantOrderBy, TenantFilterBy>,
    ) -> RepositoryResult<(PaginatorMeta, Vec<Tenant>)>;
    async fn get_all(&self) -> RepositoryResult<Vec<Tenant>>;

    async fn get_user_active_tenant_by_id(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
    ) -> RepositoryResult<Option<UserTenant>>;
}

#[async_trait]
impl TenantsRepository for PgPoolManager {
    async fn get_by_uuid(&self, uuid: Uuid) -> RepositoryResult<Tenant> {
        Ok(sqlx::query_as::<_, Tenant>(
            "SELECT * FROM tenants WHERE uuid = $1 AND deleted_at IS NULL",
        )
        .bind(uuid)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn setup_self_hosted(
        &self,
        name: &str,
        db_config: &BasicDatabaseConfig,
        claims: &Claims,
    ) -> RepositoryResult<Tenant> {
        let uuid = Uuid::new_v4();
        let mut tx = self.get_main_pool().begin().await?;

        let tenant =
            insert_and_connect_with_user(&mut tx, uuid, name, true, db_config, claims).await?;

        tx.commit().await?;

        Ok(tenant)
    }
    async fn setup_managed(
        &self,
        uuid: Uuid,
        name: &str,
        db_config: &BasicDatabaseConfig,
        claims: &Claims,
        app_config: Arc<AppConfig>,
    ) -> RepositoryResult<Tenant> {
        let mut tx = self.get_main_pool().begin().await?;
        let tenant =
            insert_and_connect_with_user(&mut tx, uuid, name, false, db_config, claims).await?;

        let mut default_tenant_pool = self.get_default_tenant_pool().acquire().await?;
        create_database_user_for_managed(&mut default_tenant_pool, &tenant, app_config).await?;
        tx.commit().await?;

        // NOTE: Postgres is not allow CREATE DATABASE in TX
        let create_db_sql = format!(
            "CREATE DATABASE tenant_{} WITH OWNER = 'tenant_{}'",
            ValueObject::new(DdlParameter(tenant.id.to_string().replace("-", "")))?,
            ValueObject::new(DdlParameter(tenant.id.to_string().replace("-", "")))?,
        );

        let _create_db = sqlx::query(&create_db_sql)
            .bind(tenant.id)
            .bind(tenant.id.to_string())
            .execute(&self.get_default_tenant_pool())
            .await?;
        Ok(tenant)
    }

    async fn get_all_by_user_id(
        &self,
        user_uuid: Uuid,
        query_params: &GetQuery<TenantOrderBy, TenantFilterBy>,
    ) -> RepositoryResult<(PaginatorMeta, Vec<Tenant>)> {
        let total: (i64,) = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                sqlx::query_as(&format!(
                    r#"SELECT COUNT(*) FROM tenants
                LEFT JOIN user_tenants ON tenants.id = user_tenants.tenant_id
                WHERE user_tenants.user_id = $1
                    AND tenants.deleted_at IS NULL
                    AND user_tenants.deleted_at IS NULL
                    AND ($2::TEXT IS NULL OR tenants.{filter_by}::TEXT ILIKE '%' || $2 || '%')"#,
                ))
                .bind(user_uuid)
                .bind(value_unchecked)
                .fetch_one(&self.get_main_pool())
                .await?
            }
            (_, _) => {
                sqlx::query_as(
                    r#"SELECT COUNT(*) FROM tenants
                        LEFT JOIN user_tenants ON tenants.id = user_tenants.tenant_id
                        WHERE user_tenants.user_id = $1
                            AND tenants.deleted_at IS NULL
                            AND user_tenants.deleted_at IS NULL"#,
                )
                .bind(user_uuid)
                .fetch_one(&self.get_main_pool())
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

        let tenants = match (
            query_params.filtering().filter_by(), // Security: ValueObject
            query_params.filtering().value_unchecked(), // Security: bind
        ) {
            (Some(filter_by), Some(value_unchecked)) => {
                let sql = format!(
                    r#"
                    SELECT tenants.*
                        FROM tenants
                        LEFT JOIN user_tenants
                            ON tenants.id = user_tenants.tenant_id
                        WHERE user_tenants.user_id = $1
                            AND tenants.deleted_at IS NULL
                            AND user_tenants.deleted_at IS NULL
                            AND ($2::TEXT IS NULL OR tenants.{filter_by} ILIKE '%' || $2 || '%')
                        {order_by_clause}
                        LIMIT $3
                        OFFSET $4
                        "#
                );

                sqlx::query_as::<_, Tenant>(&sql)
                    .bind(user_uuid)
                    .bind(value_unchecked)
                    .bind(limit)
                    .bind(i32::try_from(query_params.paging().offset().unwrap_or(0))?)
                    .fetch_all(&self.get_main_pool())
                    .await?
            }
            (_, _) => {
                let sql = format!(
                    r#"
                    SELECT tenants.*
                        FROM tenants
                        LEFT JOIN user_tenants
                            ON tenants.id = user_tenants.tenant_id
                        WHERE user_tenants.user_id = $1
                            AND tenants.deleted_at IS NULL
                            AND user_tenants.deleted_at IS NULL
                        {order_by_clause}
                        LIMIT $2
                        OFFSET $3
                        "#
                );

                sqlx::query_as::<_, Tenant>(&sql)
                    .bind(user_uuid)
                    .bind(limit)
                    .bind(i32::try_from(query_params.paging().offset().unwrap_or(0))?)
                    .fetch_all(&self.get_main_pool())
                    .await?
            }
        };

        Ok((
            PaginatorMeta {
                page: query_params.paging().page().unwrap_or(1).try_into()?,
                limit,
                total: total.0,
            },
            tenants,
        ))
    }

    async fn get_all(&self) -> RepositoryResult<Vec<Tenant>> {
        Ok(
            sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE deleted_at IS NULL")
                .fetch_all(&self.get_main_pool())
                .await?,
        )
    }

    async fn get_user_active_tenant_by_id(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
    ) -> RepositoryResult<Option<UserTenant>> {
        let user_tenant_result = sqlx::query_as::<_, UserTenant>(
            "SELECT * FROM user_tenants WHERE user_id = $1 AND tenant_id = $2 AND deleted_at IS NULL LIMIT 1",
        )
            .bind(user_id)
            .bind(tenant_id)
            .fetch_one(&self.get_main_pool())
            .await;
        let user_tenant_result = match user_tenant_result {
            Ok(user_tenant) => Ok(Some(user_tenant)),
            Err(e) => match e {
                Error::RowNotFound => Ok(None),
                _ => Err(RepositoryError::Database(e)),
            },
        };
        if let Ok(user_tenant_option) = &user_tenant_result
            && let Some(user_tenant) = user_tenant_option
        {
            let _ = sqlx::query("UPDATE user_tenants SET last_activated = NOW() WHERE id = $1 AND deleted_at IS NULL")
                .bind(user_tenant.id)
                .execute(&self.get_main_pool())
                .await?;
        }

        user_tenant_result
    }
}

async fn insert_and_connect_with_user(
    conn: &mut PgConnection,
    uuid: Uuid,
    name: &str,
    is_self_hosted: bool,
    db_config: &BasicDatabaseConfig,
    claims: &Claims,
) -> RepositoryResult<Tenant> {
    let tenant = sqlx::query_as::<_, Tenant>(
        "INSERT INTO tenants (
            id, name, is_self_hosted, db_host, db_port, db_name, db_user, db_password, db_max_pool_size, db_ssl_mode, created_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *",
    )
    .bind(uuid)
    .bind(name)
    .bind(is_self_hosted)
    .bind(&db_config.host)
    .bind(i32::from(db_config.port))
    .bind(&db_config.database)
    .bind(&db_config.username)
    .bind(&db_config.password)
    .bind(
        i32::try_from(db_config.max_pool_size())
            .map_err(|e| RepositoryError::InvalidInput(e.to_string()))?,
    )
    .bind(&db_config.ssl_mode)
    .bind(claims.sub())
    .fetch_one(&mut *conn)
    .await?;

    let _connect = sqlx::query_as::<_, UserTenant>(
        "INSERT INTO user_tenants (
            user_id, tenant_id, role
            ) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(claims.sub())
    .bind(uuid)
    .bind("owner")
    .fetch_one(&mut *conn)
    .await?;

    Ok(tenant)
}

async fn create_database_user_for_managed(
    conn: &mut PgConnection,
    tenant: &Tenant,
    app_config: Arc<AppConfig>,
) -> RepositoryResult<()> {
    let create_user_sql = format!(
        "CREATE USER tenant_{} WITH PASSWORD '{}'",
        ValueObject::new(DdlParameter(tenant.id.to_string().replace("-", "")))?,
        ValueObject::new(DdlParameter(tenant.db_password.to_string()))?
    );

    let _create_user = sqlx::query(&create_user_sql).execute(&mut *conn).await?;

    let grant_sql = format!(
        "GRANT tenant_{} to {};",
        ValueObject::new(DdlParameter(tenant.id.to_string().replace("-", "")))?,
        app_config.default_tenant_database().username // safety: not user input
    );

    let _grant = sqlx::query(&grant_sql).execute(&mut *conn).await?;

    Ok(())
}
