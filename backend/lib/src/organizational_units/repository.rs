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
use crate::app::config::AppConfig;
use crate::auth::dto::claims::Claims;
use crate::common::error::DatabaseError;
use crate::common::repository::PoolWrapper;
use crate::common::services::generate_string_csprng;
use crate::common::types::DdlParameter;
use crate::common::types::tenant::db_password::DbPassword;
use crate::common::types::value_object::{ValueObject, ValueObjectable};
use crate::organizational_units::dto::CreateRequest;
use crate::organizational_units::model::{OrganizationalUnit, UserOrganizationalUnit};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
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
    /// Inserts a new organizational unit into the database and establishes relationships
    /// or connections as needed.
    ///
    /// # Parameters
    /// - `payload`: The `CreateRequest` containing the data necessary to create an
    ///              organizational unit.
    /// - `claims`: An instance of `Claims` containing authentication and authorization
    ///             context for the current operation.
    /// - `app_config`: A thread-safe reference (`Arc`) to the application's configuration
    ///                 settings (`AppConfig`).
    ///
    /// # Returns
    /// - `Ok(OrganizationalUnit)`: The newly created `OrganizationalUnit` instance if the
    ///                             operation is successful.
    /// - `Err(DatabaseError)`: A `DatabaseError` instance if an error occurs during the
    ///                         database operation.
    ///
    /// # Errors
    /// This function returns a `DatabaseError` in the following scenarios:
    /// - The database operation to insert the organizational unit fails.
    /// - Establishing relationships between the new unit and other entities fails.
    async fn insert_and_connect(
        &self,
        payload: CreateRequest,
        claims: Claims,
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
    async fn insert_and_connect(
        &self,
        payload: CreateRequest,
        claims: Claims,
        app_config: Arc<AppConfig>,
    ) -> Result<OrganizationalUnit, DatabaseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let organizational_unit_id = Uuid::new_v4();
        let db_password = match payload.db_password {
            Some(pw) => pw,
            None => ValueObject::new(DbPassword(generate_string_csprng(40)))
                .map_err(DatabaseError::DatabaseError)?,
        };
        let organizational_unit = sqlx::query_as::<_, OrganizationalUnit>(
            "INSERT INTO organizational_units (
            id, name, db_host, db_port, db_name, db_user, db_password, db_max_pool_size
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(organizational_unit_id)
        .bind(payload.name.extract().get_value())
        .bind(
            payload
                .db_host
                .unwrap_or(app_config.default_tenant_database().host.clone())
                .extract()
                .get_value(),
        )
        .bind(
            payload
                .db_port
                .unwrap_or(app_config.default_tenant_database().port.clone())
                .extract()
                .get_value(),
        )
        .bind(
            payload
                .db_name
                .unwrap_or(
                    organizational_unit_id
                        .try_into()
                        .map_err(DatabaseError::DatabaseError)?,
                )
                .extract()
                .get_value(),
        )
        .bind(
            payload
                .db_user
                .unwrap_or(
                    organizational_unit_id
                        .try_into()
                        .map_err(DatabaseError::DatabaseError)?,
                )
                .extract()
                .get_value(),
        )
        .bind(db_password.clone().extract().get_value())
        .bind(app_config.default_tenant_database().pool_size as i32)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let _connect = sqlx::query_as::<_, UserOrganizationalUnit>(
            "INSERT INTO user_organizational_units (
            user_id, organizational_unit_id, role 
            ) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(
            Uuid::parse_str(claims.sub())
                .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
        )
        .bind(organizational_unit_id)
        .bind("owner")
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let create_user_sql = format!(
            "CREATE USER tenant_{} WITH PASSWORD '{}'",
            ValueObject::new(DdlParameter(
                organizational_unit_id.to_string().replace("-", "")
            ))
            .map_err(DatabaseError::DatabaseError)?,
            ValueObject::new(DdlParameter(db_password.to_string()))
                .map_err(DatabaseError::DatabaseError)?
        );

        let _create_user = sqlx::query(&create_user_sql)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let grant_sql = format!(
            "GRANT tenant_{} to {};",
            ValueObject::new(DdlParameter(
                organizational_unit_id.to_string().replace("-", "")
            ))
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
            app_config.default_tenant_database().username // safety: not user input
        );

        let _grant = sqlx::query(&grant_sql)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let create_db_sql = format!(
            "CREATE DATABASE tenant_{} WITH OWNER = 'tenant_{}'",
            ValueObject::new(DdlParameter(
                organizational_unit_id.to_string().replace("-", "")
            ))
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
            ValueObject::new(DdlParameter(
                organizational_unit_id.to_string().replace("-", "")
            ))
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
        );

        let _create_db = sqlx::query(&create_db_sql)
            .bind(organizational_unit_id)
            .bind(organizational_unit_id.to_string())
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
