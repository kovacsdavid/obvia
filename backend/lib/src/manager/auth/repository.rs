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
use crate::manager::auth::dto::register::RegisterRequest;
use crate::manager::common::repository::PoolManagerWrapper;
use crate::manager::common::types::value_object::ValueObjectable;
use crate::manager::tenants::model::UserTenant;
use crate::manager::users::model::User;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::Error;
use uuid::Uuid;

/// `AuthRepository` is an asynchronous trait that defines the operations for interacting with
/// user authentication-related data in a data store. It is meant to be implemented by any
/// struct that handles such operations. This trait requires implementors to be thread-safe
/// (`Send` and `Sync`) and have a static lifetime.
///
/// # Attributes
/// - `#[cfg_attr(test, automock)]`: This attribute allows the trait to be mockable for use with
///   unit tests when the `test` feature is enabled.
///
/// # Required Methods
///
/// ## `insert_user`
/// Inserts a new user into the data store.
///
/// ### Parameters
/// - `payload` - A reference to a `RegisterRequest` struct containing the user's registration
///   details (e.g., name, email).
/// - `password_hash` - A reference to a string slice containing the hashed password of the user.
///
/// ### Returns
/// - `Ok(())` on successful insertion.
/// - `Err(DatabaseError)` if there was an issue inserting the user, such as database errors.
///
/// ## `get_user_by_email`
/// Retrieves a user from the data store based on their email address.
///
/// ### Parameters
/// - `email` - A string slice representing the email of the user to retrieve.
///
/// ### Returns
/// - `Ok(User)` containing the user data if the user was found.
/// - `Err(DatabaseError)` if there was an issue during retrieval, such as the user not being
///   found or a database error occurred.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuthRepository: Send + Sync {
    /// Inserts a new user into the database.
    ///
    /// # Parameters
    /// - `&self`: A reference to the current instance of the struct implementing this method.
    /// - `payload`: A reference to a `RegisterRequest` object containing the user's registration details such as email, first_name, last_name etc.
    /// - `password_hash`: A reference to a hashed password string for the user.
    ///
    /// # Returns
    /// - `Ok(())`: Returned if the user is successfully inserted into the database.
    /// - `Err(DatabaseError)`: Returned if an error occurs while inserting the user, such as database connection issues or constraints violations.
    ///
    /// # Errors
    /// This function returns a `DatabaseError` if:
    /// - The connection to the database fails.
    /// - The insertion violates a database constraint (e.g., email already exists).
    async fn insert_user(
        &self,
        payload: &RegisterRequest,
        password_hash: &str,
    ) -> Result<(), RepositoryError>;
    /// Asynchronously retrieves a user from the database by their email address.
    ///
    /// # Arguments
    ///
    /// * `email` - A string slice that holds the email address of the user to retrieve.
    ///
    /// # Returns
    ///
    /// * `Ok(User)` - If a user with the specified email is found, returns the corresponding `User` object.
    /// * `Err(DatabaseError)` - If an error occurs during the database operation or if the user is not found.
    ///
    /// # Errors
    ///
    /// This function will return a `DatabaseError` in the following cases:
    /// - The database connection fails.
    /// - The query encounters an error.
    /// - No user is found with the specified email address.
    async fn get_user_by_email(&self, email: &str) -> Result<User, RepositoryError>;

    async fn get_user_active_tenant(
        &self,
        user_id: Uuid,
    ) -> Result<Option<UserTenant>, RepositoryError>;
}

#[async_trait]
impl AuthRepository for PoolManagerWrapper {
    async fn insert_user(
        &self,
        payload: &RegisterRequest,
        password_hash: &str,
    ) -> Result<(), RepositoryError> {
        sqlx::query(
            "INSERT INTO users (
                    id, email, password_hash, first_name, last_name
            ) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(Uuid::new_v4())
        .bind(payload.email.extract().get_value())
        .bind(password_hash)
        .bind(payload.first_name.extract().get_value())
        .bind(payload.last_name.extract().get_value())
        .execute(&self.pool_manager.get_main_pool())
        .await?;
        Ok(())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, RepositoryError> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
                .bind(email)
                .fetch_one(&self.pool_manager.get_main_pool())
                .await?,
        )
    }

    async fn get_user_active_tenant(
        &self,
        user_id: Uuid,
    ) -> Result<Option<UserTenant>, RepositoryError> {
        let user_tenant_result = sqlx::query_as::<_, UserTenant>(
            "SELECT * FROM user_tenants WHERE user_id = $1 AND deleted_at IS NULL ORDER BY last_activated DESC LIMIT 1",
        )
        .bind(user_id)
        .fetch_one(&self.pool_manager.get_main_pool())
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
                .execute(&self.pool_manager.get_main_pool())
                .await?;
        }

        user_tenant_result
    }
}
