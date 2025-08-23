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
use crate::app::config::{AppConfig, BasicDatabaseConfig, DatabasePoolSizeProvider};
use crate::auth::dto::claims::Claims;
use crate::common::dto::{OrderingParams, PagedResult, PaginatorParams};
use crate::common::error::DatabaseError;
use crate::common::repository::PoolWrapper;
use crate::common::types::DdlParameter;
use crate::common::types::value_object::ValueObject;
use crate::tenants::dto::FilteringParams;
use crate::tenants::model::{Tenant, UserTenant};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::PgConnection;
use sqlx::error::BoxDynError;
use std::sync::Arc;
use uuid::Uuid;

/// The `TenantsRepository` trait defines the interface for interacting with
/// the tenants in the storage. It provides methods to query, insert,
/// and retrieve tenants, supporting asynchronous operations.
///
/// # Methods
///
/// * `get_by_uuid`:
///   - Retrieves an `Tenant` by its unique identifier (UUID).
///   - This method may return a `DatabaseError` in case of a failure.
///
///   ### Parameters:
///   - `uuid`: A reference to the UUID of the tenant to retrieve.
///
///   ### Returns:
///   - `Result<Tenant, DatabaseError>`: A result containing the
///     tenant if found, or an error if the operation fails.
///
/// * `insert_and_connect`:
///   - Inserts a new `Tenant` and associates it with other entities based
///     on the provided payload and claims.
///   - Performs additional configuration using `app_config`.
///
///   ### Parameters:
///   - `payload`: The `CreateRequest` containing the data needed to insert a new tenant.
///   - `claims`: `Claims` associated with the authenticated user or system.
///   - `app_config`: Shared application configuration as an `Arc<AppConfig>`.
///
///   ### Returns:
///   - `Result<Tenant, DatabaseError>`: A result containing the newly created
///     tenant, or an error if the operation fails.
///
/// * `get_all_by_user_uuid`:
///   - Retrieves all `Tenant`s associated with a particular user identified by their UUID.
///   - Marked with `#[allow(dead_code)]` to suppress warnings if not actively used.
///
///   ### Parameters:
///   - `user_uuid`: A reference to the UUID of the user whose tenants
///     are to be retrieved
///
///   ### Returns:
///   - `Result<Vec<Tenant>, DatabaseError>`: A result containing a vector
///     of tenants associated with the user, or an error if the operation fails.
///
/// * `get_all`:
///   - Retrieves all `Tenant`s available in the repository.
///
///   ### Returns:
///   - `Result<Vec<Tenant>, DatabaseError>`: A result containing a vector
///     of all tenants, or an error if the operation fails.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait TenantsRepository: Send + Sync + 'static {
    /// Retrieves an `Tenant` by its UUID.
    ///
    /// # Parameters
    /// - `uuid`: A reference to the unique identifier (`Uuid`) of the tenant
    ///           to be fetched.
    ///
    /// # Returns
    /// - `Ok(Tenant)`: If the tenant with the given UUID is found
    ///   in the database.
    /// - `Err(DatabaseError)`: If there is an error during the database operation, or if
    ///   the tenant could not be found.
    #[allow(dead_code)]
    async fn get_by_uuid(&self, uuid: Uuid) -> Result<Tenant, DatabaseError>;

    /// Sets up a self-hosted instance for a specific tenant based on the provided payload.
    ///
    /// # Parameters
    /// - `&self`: A reference to the current struct instance.
    /// - `payload: CreateRequest` - Contains the necessary data to create and configure the self-hosted instance.
    /// - `claims: Claims` - Represents the authorization and authentication claims of the current user or process,
    ///   used to validate permissions or access rights.
    /// - `app_config: Arc<AppConfig>` - A reference-counted pointer to the application configuration
    ///   which may include global settings or contextual configuration required for the operation.
    ///
    /// # Returns
    /// - `Result<Tenant, DatabaseError>`:
    ///   - On success, returns an `Tenant` representing the created or updated self-hosted instance.
    ///   - On failure, returns a `DatabaseError` if there are issues such as database queries or constraints.
    ///
    /// # Errors
    /// This function returns a `DatabaseError` in one of the following cases:
    /// - If there is an issue executing database operations necessary for setting up the tenant.
    /// - If any constraint or validation fails during the setup process.
    ///
    /// # Notes
    /// - This function is asynchronous and should be awaited.
    /// - Ensure proper validation of the `payload` and `claims` before invoking this function to prevent runtime errors.
    /// - The `app_config` should contain all required configuration values for successfully setting up the instance.
    async fn setup_self_hosted(
        &self,
        name: &str,
        db_config: &BasicDatabaseConfig,
        claims: &Claims,
    ) -> Result<Tenant, DatabaseError>;

    /// Sets up a managed resource based on the provided parameters.
    ///
    /// This asynchronous function is responsible for configuring and initializing
    /// a managed resource using the given payload, user claims, and application configuration.
    ///
    /// # Arguments
    ///
    /// * `payload` - A `CreateRequest` object that contains the information
    ///   necessary for creating or setting up the resource.
    /// * `claims` - A `Claims` object that provides authentication and authorization
    ///   information about the current user or context.
    /// * `app_config` - An `Arc<AppConfig>` reference, which provides access to
    ///   application-level configuration that may be required during setup.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the managed resource setup is successful.
    /// * `Err(DatabaseError)` - If there is an issue interacting with the database
    ///   or other errors occur during the setup process.
    ///
    /// # Errors
    ///
    /// This function returns a `DatabaseError` in cases such as:
    /// - Database connectivity issues.
    /// - Validation errors while processing the `payload`.
    /// - Insufficient permissions based on the provided `claims`.
    ///
    /// # Notes
    ///
    /// This function assumes that the caller has already validated the input `payload`
    /// and that the `claims` contain the necessary permissions for executing the action.
    async fn setup_managed(
        &self,
        uuid: Uuid,
        name: &str,
        db_config: &BasicDatabaseConfig,
        claims: &Claims,
        app_config: Arc<AppConfig>,
    ) -> Result<Tenant, DatabaseError>;
    /// Asynchronously retrieves all tenants associated with a specific user UUID.
    ///
    /// # Parameters
    /// - `user_uuid`: A string slice representing the UUID of the user whose tenants
    ///   are to be retrieved.
    ///
    /// # Returns
    /// - `Result<Vec<Tenant>, DatabaseError>`:
    ///   - On success: A vector containing all tenants associated with the given user UUID.
    ///   - On failure: A `DatabaseError` indicating the reason for failure.
    ///
    /// # Errors
    /// - Returns a `DatabaseError` if the query fails or if an issue occurs during retrieval.
    #[allow(dead_code)]
    async fn get_all_by_user_id(
        &self,
        user_uuid: Uuid,
        paginator_params: PaginatorParams,
        ordering_params: OrderingParams,
        filtering_params: FilteringParams,
    ) -> Result<PagedResult<Vec<Tenant>>, DatabaseError>;
    /// Retrieves all tenants from the database.
    ///
    /// This asynchronous function fetches and returns a list of all
    /// `Tenant` records stored in the database. If an error occurs
    /// during the database query, it returns a `DatabaseError`.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Tenant>)` - A vector containing all tenant records.
    /// * `Err(DatabaseError)` - An error that occurred while querying the database.
    ///
    /// # Errors
    ///
    /// This function will return a `DatabaseError` in the following cases:
    /// - If the connection to the database fails.
    /// - If there's an issue with the underlying query execution.
    ///
    /// # Safety
    ///
    /// This function should not be used in any user facing scenario as it will not check if
    /// the user is associated to the tenant or not.
    async fn get_all(&self) -> Result<Vec<Tenant>, DatabaseError>;
}

#[async_trait]
impl TenantsRepository for PoolWrapper {
    async fn get_by_uuid(&self, uuid: Uuid) -> Result<Tenant, DatabaseError> {
        Ok(sqlx::query_as::<_, Tenant>(
            "SELECT * FROM tenants WHERE uuid = $1 AND deleted_at IS NULL",
        )
        .bind(uuid)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?)
    }
    async fn setup_self_hosted(
        &self,
        name: &str,
        db_config: &BasicDatabaseConfig,
        claims: &Claims,
    ) -> Result<Tenant, DatabaseError> {
        let uuid = Uuid::new_v4();
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let tenant = insert_and_connect_with_user(&mut tx, uuid, name, true, db_config, claims)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        Ok(tenant)
    }
    async fn setup_managed(
        &self,
        uuid: Uuid,
        name: &str,
        db_config: &BasicDatabaseConfig,
        claims: &Claims,
        app_config: Arc<AppConfig>,
    ) -> Result<Tenant, DatabaseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;
        let tenant = insert_and_connect_with_user(&mut tx, uuid, name, false, db_config, claims)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;
        create_database_user_for_managed(&mut tx, &tenant, app_config)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;
        tx.commit()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        // NOTE: Postgres is not allow CREATE DATABASE in TX
        let create_db_sql = format!(
            "CREATE DATABASE tenant_{} WITH OWNER = 'tenant_{}'",
            ValueObject::new(DdlParameter(tenant.id.to_string().replace("-", "")))
                .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
            ValueObject::new(DdlParameter(tenant.id.to_string().replace("-", "")))
                .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
        );

        let _create_db = sqlx::query(&create_db_sql)
            .bind(tenant.id)
            .bind(tenant.id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;
        Ok(tenant)
    }

    async fn get_all_by_user_id(
        &self,
        user_uuid: Uuid,
        paginator_params: PaginatorParams,
        ordering_params: OrderingParams,
        filtering_params: FilteringParams,
    ) -> Result<PagedResult<Vec<Tenant>>, DatabaseError> {
        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tenants
                LEFT JOIN user_tenants ON tenants.id = user_tenants.tenant_id
                WHERE user_tenants.user_id = $1
                    AND tenants.deleted_at IS NULL
                    AND user_tenants.deleted_at IS NULL
                    AND $2::TEXT IS NULL OR tenants.name ILIKE $2",
        )
        .bind(user_uuid)
        .bind(filtering_params.name.clone())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let order_by_clause = match ordering_params.order_by.as_str() {
            "name" => format!("ORDER BY tenants.name {}", ordering_params.order),
            "created_at" => format!("ORDER BY tenants.created_at {}", ordering_params.order),
            "updated_at" => format!("ORDER BY tenants.updated_at {}", ordering_params.order),
            _ => "".to_string(),
        };

        let sql = format!(
            r#"
            SELECT tenants.*
                FROM tenants
                LEFT JOIN user_tenants
                    ON tenants.id = user_tenants.tenant_id
                WHERE user_tenants.user_id = $1
                    AND tenants.deleted_at IS NULL
                    AND user_tenants.deleted_at IS NULL
                    AND $2::TEXT IS NULL OR tenants.name ILIKE $2
                {}
                LIMIT $3
                OFFSET $4
                "#,
            order_by_clause
        );

        let tenants = sqlx::query_as::<_, Tenant>(&sql)
            .bind(user_uuid)
            .bind(filtering_params.name)
            .bind(paginator_params.limit)
            .bind(paginator_params.offset())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        Ok(PagedResult {
            page: paginator_params.page,
            limit: paginator_params.limit,
            total: total.0,
            data: tenants,
        })
    }

    async fn get_all(&self) -> Result<Vec<Tenant>, DatabaseError> {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE deleted_at IS NULL")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))
    }
}

async fn insert_and_connect_with_user(
    conn: &mut PgConnection,
    uuid: Uuid,
    name: &str,
    is_self_hosted: bool,
    db_config: &BasicDatabaseConfig,
    claims: &Claims,
) -> Result<Tenant, BoxDynError> {
    let tenant = sqlx::query_as::<_, Tenant>(
        "INSERT INTO tenants (
            id, name, is_self_hosted, db_host, db_port, db_name, db_user, db_password, db_max_pool_size, db_ssl_mode
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *",
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
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
    )
    .bind(&db_config.ssl_mode)
    .fetch_one(&mut *conn)
    .await
    .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

    let _connect = sqlx::query_as::<_, UserTenant>(
        "INSERT INTO user_tenants (
            user_id, tenant_id, role
            ) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(claims.sub())
    .bind(uuid)
    .bind("owner")
    .fetch_one(&mut *conn)
    .await
    .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

    Ok(tenant)
}

async fn create_database_user_for_managed(
    conn: &mut PgConnection,
    tenant: &Tenant,
    app_config: Arc<AppConfig>,
) -> Result<(), BoxDynError> {
    let create_user_sql = format!(
        "CREATE USER tenant_{} WITH PASSWORD '{}'",
        ValueObject::new(DdlParameter(tenant.id.to_string().replace("-", "")))
            .map_err(DatabaseError::DatabaseError)?,
        ValueObject::new(DdlParameter(tenant.db_password.to_string()))
            .map_err(DatabaseError::DatabaseError)?
    );

    let _create_user = sqlx::query(&create_user_sql)
        .execute(&mut *conn)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

    let grant_sql = format!(
        "GRANT tenant_{} to {};",
        ValueObject::new(DdlParameter(tenant.id.to_string().replace("-", "")))
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
        app_config.default_tenant_database().username // safety: not user input
    );

    let _grant = sqlx::query(&grant_sql)
        .execute(&mut *conn)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

    Ok(())
}
