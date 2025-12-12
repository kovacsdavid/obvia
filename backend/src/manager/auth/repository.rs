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

use crate::common::error::{RepositoryError, RepositoryResult};
use crate::common::types::value_object::ValueObjectable;
use crate::manager::app::database::{PgPoolManager, PoolManager};
use crate::manager::auth::dto::register::RegisterRequest;
use crate::manager::auth::model::{EmailVerification, ForgottenPassword};
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
    ) -> RepositoryResult<User>;
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
    async fn get_user_by_email(&self, email: &str) -> RepositoryResult<User>;

    async fn get_user_by_id(&self, user_id: Uuid) -> RepositoryResult<User>;

    async fn update_user(&self, user: User) -> RepositoryResult<User>;

    async fn update_user_last_login_at(&self, user_id: Uuid) -> RepositoryResult<()>;

    async fn get_user_active_tenant(&self, user_id: Uuid) -> RepositoryResult<Option<UserTenant>>;
    async fn insert_email_verification(&self, user_id: Uuid)
    -> RepositoryResult<EmailVerification>;
    async fn get_email_verification(
        &self,
        email_verification_id: Uuid,
    ) -> RepositoryResult<EmailVerification>;
    async fn invalidate_email_verification(
        &self,
        email_verification_id: Uuid,
    ) -> RepositoryResult<()>;
    async fn insert_forgotten_password(&self, user_id: Uuid)
    -> RepositoryResult<ForgottenPassword>;
    async fn get_forgotten_password(
        &self,
        forgotten_password_id: Uuid,
    ) -> RepositoryResult<ForgottenPassword>;
    async fn invalidate_forgotten_password(
        &self,
        forgotten_password_id: Uuid,
    ) -> RepositoryResult<()>;
}

#[async_trait]
impl AuthRepository for PgPoolManager {
    async fn insert_user(
        &self,
        payload: &RegisterRequest,
        password_hash: &str,
    ) -> RepositoryResult<User> {
        Ok(sqlx::query_as::<_, User>(
            "INSERT INTO users (
                    id, email, password_hash, first_name, last_name, status
            ) VALUES ($1, $2, $3, $4, $5, 'unchecked_email') RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(payload.email.extract().get_value())
        .bind(password_hash)
        .bind(payload.first_name.extract().get_value())
        .bind(payload.last_name.extract().get_value())
        .fetch_one(&self.get_main_pool())
        .await?)
    }

    async fn get_user_by_email(&self, email: &str) -> RepositoryResult<User> {
        Ok(
            sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL",
            )
            .bind(email)
            .fetch_one(&self.get_main_pool())
            .await?,
        )
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> RepositoryResult<User> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL")
                .bind(user_id)
                .fetch_one(&self.get_main_pool())
                .await?,
        )
    }

    async fn update_user(&self, user: User) -> RepositoryResult<User> {
        Ok(sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET email = $1,
                password_hash = $2,
                first_name = $3,
                last_name = $4,
                phone = $5,
                status = $6,
                last_login_at = $7,
                profile_picture_url = $8,
                locale = $9,
                invited_by = $10,
                email_verified_at = $11
            WHERE id = $12
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(user.email)
        .bind(user.password_hash)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.phone)
        .bind(user.status)
        .bind(user.last_login_at)
        .bind(user.profile_picture_url)
        .bind(user.locale)
        .bind(user.invited_by)
        .bind(user.email_verified_at)
        .bind(user.id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }

    async fn update_user_last_login_at(&self, user_id: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET last_login_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.get_main_pool())
        .await?;
        Ok(())
    }

    async fn get_user_active_tenant(&self, user_id: Uuid) -> RepositoryResult<Option<UserTenant>> {
        let user_tenant_result = sqlx::query_as::<_, UserTenant>(
            "SELECT * FROM user_tenants WHERE user_id = $1 AND deleted_at IS NULL ORDER BY last_activated DESC LIMIT 1",
        )
        .bind(user_id)
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
    async fn insert_email_verification(
        &self,
        user_id: Uuid,
    ) -> RepositoryResult<EmailVerification> {
        Ok(sqlx::query_as::<_, EmailVerification>(
            "INSERT INTO email_verifications (
                    user_id, valid_until
            ) VALUES ($1, NOW() + '1 day'::interval) RETURNING *",
        )
        .bind(user_id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn get_email_verification(
        &self,
        email_verification_id: Uuid,
    ) -> RepositoryResult<EmailVerification> {
        Ok(sqlx::query_as::<_, EmailVerification>(
            "SELECT * FROM email_verifications WHERE id = $1 AND valid_until > NOW() AND deleted_at IS NULL",
        )
        .bind(email_verification_id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn invalidate_email_verification(
        &self,
        email_verification_id: Uuid,
    ) -> RepositoryResult<()> {
        let _ = sqlx::query("UPDATE email_verifications SET deleted_at = NOW() WHERE id = $1")
            .bind(email_verification_id)
            .execute(&self.get_main_pool())
            .await?;
        Ok(())
    }
    async fn insert_forgotten_password(
        &self,
        user_id: Uuid,
    ) -> RepositoryResult<ForgottenPassword> {
        Ok(sqlx::query_as::<_, ForgottenPassword>(
            "INSERT INTO forgotten_passwords (
                    user_id, valid_until
            ) VALUES ($1, NOW() + '1 hour'::interval) RETURNING *",
        )
        .bind(user_id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn get_forgotten_password(
        &self,
        forgotten_password_id: Uuid,
    ) -> RepositoryResult<ForgottenPassword> {
        Ok(sqlx::query_as::<_, ForgottenPassword>(
            "SELECT * FROM forgotten_passwords WHERE id = $1 AND valid_until > NOW() AND deleted_at IS NULL",
        )
        .bind(forgotten_password_id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn invalidate_forgotten_password(
        &self,
        forgotten_password_id: Uuid,
    ) -> RepositoryResult<()> {
        let _ = sqlx::query("UPDATE forgotten_passwords SET deleted_at = NOW() WHERE id = $1")
            .bind(forgotten_password_id)
            .execute(&self.get_main_pool())
            .await?;
        Ok(())
    }
}
