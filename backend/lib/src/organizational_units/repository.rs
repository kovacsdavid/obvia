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
use crate::common::error::DatabaseError;
use crate::common::repository::PoolWrapper;
use crate::common::types::DdlParameter;
use crate::common::types::value_object::ValueObject;
use crate::organizational_units::model::{OrganizationalUnit, UserOrganizationalUnit};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::PgConnection;
use sqlx::error::BoxDynError;
use std::sync::Arc;
use uuid::Uuid;

/// The `OrganizationalUnitsRepository` trait defines the interface for interacting with
/// the organizational units in the storage. It provides methods to query, insert,
/// and retrieve organizational units, supporting asynchronous operations.
///
/// # Methods
///
/// * `get_by_uuid`:
///   - Retrieves an `OrganizationalUnit` by its unique identifier (UUID).
///   - This method may return a `DatabaseError` in case of a failure.
///
///   ### Parameters:
///   - `uuid`: A reference to the UUID of the organizational unit to retrieve.
///
///   ### Returns:
///   - `Result<OrganizationalUnit, DatabaseError>`: A result containing the
///     organizational unit if found, or an error if the operation fails.
///
/// * `insert_and_connect`:
///   - Inserts a new `OrganizationalUnit` and associates it with other entities based
///     on the provided payload and claims.
///   - Performs additional configuration using `app_config`.
///
///   ### Parameters:
///   - `payload`: The `CreateRequest` containing the data needed to insert a new organizational unit.
///   - `claims`: `Claims` associated with the authenticated user or system.
///   - `app_config`: Shared application configuration as an `Arc<AppConfig>`.
///
///   ### Returns:
///   - `Result<OrganizationalUnit, DatabaseError>`: A result containing the newly created
///     organizational unit, or an error if the operation fails.
///
/// * `get_all_by_user_uuid`:
///   - Retrieves all `OrganizationalUnit`s associated with a particular user identified by their UUID.
///   - Marked with `#[allow(dead_code)]` to suppress warnings if not actively used.
///
///   ### Parameters:
///   - `user_uuid`: A reference to the UUID of the user whose organizational units
///     are to be retrieved
///
///   ### Returns:
///   - `Result<Vec<OrganizationalUnit>, DatabaseError>`: A result containing a vector
///     of organizational units associated with the user, or an error if the operation fails.
///
/// * `get_all`:
///   - Retrieves all `OrganizationalUnit`s available in the repository.
///
///   ### Returns:
///   - `Result<Vec<OrganizationalUnit>, DatabaseError>`: A result containing a vector
///     of all organizational units, or an error if the operation fails.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait OrganizationalUnitsRepository: Send + Sync + 'static {
    /// Retrieves an `OrganizationalUnit` by its UUID.
    ///
    /// # Parameters
    /// - `uuid`: A reference to the unique identifier (`Uuid`) of the organizational unit
    ///           to be fetched.
    ///
    /// # Returns
    /// - `Ok(OrganizationalUnit)`: If the organizational unit with the given UUID is found
    ///   in the database.
    /// - `Err(DatabaseError)`: If there is an error during the database operation, or if
    ///   the organizational unit could not be found.
    #[allow(dead_code)]
    async fn get_by_uuid(&self, uuid: Uuid) -> Result<OrganizationalUnit, DatabaseError>;

    /// Sets up a self-hosted instance for a specific organizational unit based on the provided payload.
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
    /// - `Result<OrganizationalUnit, DatabaseError>`:
    ///   - On success, returns an `OrganizationalUnit` representing the created or updated self-hosted instance.
    ///   - On failure, returns a `DatabaseError` if there are issues such as database queries or constraints.
    ///
    /// # Errors
    /// This function returns a `DatabaseError` in one of the following cases:
    /// - If there is an issue executing database operations necessary for setting up the organizational unit.
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
    ) -> Result<OrganizationalUnit, DatabaseError>;

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
    ) -> Result<OrganizationalUnit, DatabaseError>;
    /// Asynchronously retrieves all organizational units associated with a specific user UUID.
    ///
    /// # Parameters
    /// - `user_uuid`: A string slice representing the UUID of the user whose organizational units
    ///   are to be retrieved.
    ///
    /// # Returns
    /// - `Result<Vec<OrganizationalUnit>, DatabaseError>`:
    ///   - On success: A vector containing all organizational units associated with the given user UUID.
    ///   - On failure: A `DatabaseError` indicating the reason for failure.
    ///
    /// # Errors
    /// - Returns a `DatabaseError` if the query fails or if an issue occurs during retrieval.
    #[allow(dead_code)]
    async fn get_all_by_user_uuid(
        &self,
        user_uuid: Uuid,
    ) -> Result<Vec<OrganizationalUnit>, DatabaseError>;
    /// Retrieves all organizational units from the database.
    ///
    /// This asynchronous function fetches and returns a list of all
    /// `OrganizationalUnit` records stored in the database. If an error occurs
    /// during the database query, it returns a `DatabaseError`.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<OrganizationalUnit>)` - A vector containing all organizational unit records.
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
    /// the user is associated to the organizational_unit or not.
    async fn get_all(&self) -> Result<Vec<OrganizationalUnit>, DatabaseError>;
}

#[async_trait]
impl OrganizationalUnitsRepository for PoolWrapper {
    async fn get_by_uuid(&self, uuid: Uuid) -> Result<OrganizationalUnit, DatabaseError> {
        Ok(sqlx::query_as::<_, OrganizationalUnit>(
            "SELECT * FROM organizational_units WHERE uuid = $1 AND deleted_at IS NULL",
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
    ) -> Result<OrganizationalUnit, DatabaseError> {
        let uuid = Uuid::new_v4();
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let organizational_unit =
            insert_and_connect_with_user(&mut tx, uuid, name, db_config, claims)
                .await
                .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        Ok(organizational_unit)
    }
    async fn setup_managed(
        &self,
        uuid: Uuid,
        name: &str,
        db_config: &BasicDatabaseConfig,
        claims: &Claims,
        app_config: Arc<AppConfig>,
    ) -> Result<OrganizationalUnit, DatabaseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;
        let organizational_unit =
            insert_and_connect_with_user(&mut tx, uuid, name, db_config, claims)
                .await
                .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;
        create_database_user_for_managed(&mut tx, &organizational_unit, app_config)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;
        tx.commit()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        // NOTE: Postgres is not allow CREATE DATABASE in TX
        let create_db_sql = format!(
            "CREATE DATABASE tenant_{} WITH OWNER = 'tenant_{}'",
            ValueObject::new(DdlParameter(
                organizational_unit.id.to_string().replace("-", "")
            ))
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
            ValueObject::new(DdlParameter(
                organizational_unit.id.to_string().replace("-", "")
            ))
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
        );

        let _create_db = sqlx::query(&create_db_sql)
            .bind(organizational_unit.id)
            .bind(organizational_unit.id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;
        Ok(organizational_unit)
    }

    async fn get_all_by_user_uuid(
        &self,
        user_uuid: Uuid,
    ) -> Result<Vec<OrganizationalUnit>, DatabaseError> {
        sqlx::query_as::<_, OrganizationalUnit>(
            "SELECT * FROM organizational_units WHERE user_uuid = $1 AND deleted_at IS NULL",
        )
        .bind(user_uuid)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))
    }

    async fn get_all(&self) -> Result<Vec<OrganizationalUnit>, DatabaseError> {
        sqlx::query_as::<_, OrganizationalUnit>(
            "SELECT * FROM organizational_units WHERE deleted_at IS NULL",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))
    }
}

async fn insert_and_connect_with_user(
    conn: &mut PgConnection,
    uuid: Uuid,
    name: &str,
    db_config: &BasicDatabaseConfig,
    claims: &Claims,
) -> Result<OrganizationalUnit, BoxDynError> {
    let organizational_unit = sqlx::query_as::<_, OrganizationalUnit>(
        "INSERT INTO organizational_units (
            id, name, db_host, db_port, db_name, db_user, db_password, db_max_pool_size, db_ssl_mode
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
    )
    .bind(uuid)
    .bind(name)
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

    let _connect = sqlx::query_as::<_, UserOrganizationalUnit>(
        "INSERT INTO user_organizational_units (
            user_id, organizational_unit_id, role
            ) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(Uuid::parse_str(claims.sub()).map_err(|e| DatabaseError::DatabaseError(e.to_string()))?)
    .bind(uuid)
    .bind("owner")
    .fetch_one(&mut *conn)
    .await
    .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

    Ok(organizational_unit)
}

async fn create_database_user_for_managed(
    conn: &mut PgConnection,
    organizational_unit: &OrganizationalUnit,
    app_config: Arc<AppConfig>,
) -> Result<(), BoxDynError> {
    let create_user_sql = format!(
        "CREATE USER tenant_{} WITH PASSWORD '{}'",
        ValueObject::new(DdlParameter(
            organizational_unit.id.to_string().replace("-", "")
        ))
        .map_err(DatabaseError::DatabaseError)?,
        ValueObject::new(DdlParameter(organizational_unit.db_password.to_string()))
            .map_err(DatabaseError::DatabaseError)?
    );

    let _create_user = sqlx::query(&create_user_sql)
        .execute(&mut *conn)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

    let grant_sql = format!(
        "GRANT tenant_{} to {};",
        ValueObject::new(DdlParameter(
            organizational_unit.id.to_string().replace("-", "")
        ))
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
        app_config.default_tenant_database().username // safety: not user input
    );

    let _grant = sqlx::query(&grant_sql)
        .execute(&mut *conn)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

    Ok(())
}
