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

use super::{
    AuthModule,
    dto::{claims::Claims, login::LoginResponse, login::UserPublic},
    repository::AuthRepository,
};
use crate::common::dto::GeneralError;
use crate::common::error::{FriendlyError, RepositoryError};
use crate::common::types::value_object::ValueObjectable;
use crate::manager::auth::dto::{login::LoginRequest, register::RegisterRequest};
use anyhow::Result;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::Error;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("User not found")]
    UserNotFound,

    #[error("User exists")]
    UserExists,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Hash error: {0}")]
    Hash(String),

    #[error("Token generation: {0}")]
    Token(String),
}

impl IntoResponse for AuthServiceError {
    fn into_response(self) -> Response {
        match self {
            AuthServiceError::UserNotFound | AuthServiceError::InvalidPassword => {
                FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    GeneralError {
                        message: "Hibás e-mail cím vagy jelszó".to_string(),
                    },
                )
                .into_response()
            }
            AuthServiceError::UserExists => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::CONFLICT,
                file!(),
                "A megadott e-mail cím már foglalt!".to_string(),
            )
            .into_response(),
            e => FriendlyError::internal(file!(), e.to_string()).into_response(),
        }
    }
}

pub struct AuthService;

impl AuthService {
    /// Attempts to authenticate a user based on the provided credentials. If the login succeeds, a JWT token is
    /// generated and returned along with the user's public information.
    ///
    /// # Arguments
    /// * `repo` - A repository implementing the `AuthRepository` trait, responsible for querying user data.
    /// * `auth_module` - A shared reference to the `AuthModule`, containing the configurations required for authentication.
    /// * `payload` - The `LoginRequest` struct containing the user's email and password provided for login.
    ///
    /// # Returns
    /// * `Ok(OkResponse<LoginResponse>)` - Contains the authenticated user's public information and the generated JWT token
    ///   if login is successful.
    /// * `Err(FriendlyError)` - Returns an error in the following scenarios:
    ///   - If the user does not exist or the email is invalid, an `UNAUTHORIZED` error is returned with a user-facing message.
    ///   - If the provided password does not match the stored password hash, an `UNAUTHORIZED` error is returned.
    ///   - If any internal issue occurs, such as invalid password hashing or issues in generating the JWT token,
    ///     an internal error is returned.
    ///
    /// # Errors
    /// - `FriendlyError::UserFacing` - If the user provides incorrect credentials (email or password).
    /// - `FriendlyError::Internal` - If an internal issue occurs, such as hashing or token encoding errors.
    ///
    /// # Workflow
    /// 1. Retrieves the user data by email using the `AuthRepository`.
    /// 2. Verifies the provided password against the stored password hash using the Argon2 algorithm.
    /// 3. Prepares JWT claims, including user ID, issued-at, expiration, not-before timestamps, issuer, audience,
    ///    and a unique token identifier.
    /// 4. Generates a JWT token using the derived claims and a secret key.
    /// 5. Returns the user's public information and generated token on successful login.
    ///
    /// # Security Notes
    /// - This function ensures that sensitive information such as the authentication secret and password hash
    ///   are not exposed. Errors are surfaced through generic user-facing messages to ensure security.
    /// - Password verification is performed using a secure Argon2 hashing algorithm.
    ///
    /// # Dependencies
    /// - `AuthRepository` for fetching user details.
    /// - `argon2` crate for password hashing and verification.
    /// - `jsonwebtoken` crate for JWT creation.
    /// - `chrono` crate for timestamps and expiration calculations.
    pub async fn try_login(
        auth_module: Arc<AuthModule>,
        payload: LoginRequest,
    ) -> Result<LoginResponse, AuthServiceError> {
        let user = auth_module
            .auth_repo
            .get_user_by_email(&payload.email)
            .await
            .map_err(|_| AuthServiceError::UserNotFound)?;
        let active_user_tenant = auth_module
            .auth_repo
            .get_user_active_tenant(user.id)
            .await?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AuthServiceError::Hash(e.to_string()))?;

        Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .map_err(|_| AuthServiceError::InvalidPassword)?;

        let now = Utc::now().timestamp() as usize;
        let exp = (Utc::now()
            + Duration::minutes(auth_module.config.auth().jwt_expiration_mins() as i64))
        .timestamp() as usize;
        let nbf = now;
        let active_tenant_id = match active_user_tenant {
            None => None,
            Some(user_tenant) => Some(user_tenant.tenant_id),
        };

        let claims = Claims::new(
            user.id,
            exp,
            now,
            nbf,
            auth_module.config.auth().jwt_issuer().to_string(),
            auth_module.config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            active_tenant_id,
        );

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(auth_module.config.auth().jwt_secret().as_bytes()),
        )
        .map_err(|e| AuthServiceError::Token(e.to_string()))?;

        Ok(LoginResponse::new(UserPublic::from(user), token))
    }

    /// Attempts to register a new user in the system.
    ///
    /// This function performs the following tasks:
    /// 1. Hashes the provided password from the registration payload.
    /// 2. Attempts to store the user's information, along with the hashed password, in the provided repository.
    ///
    /// # Arguments
    /// - `repo`: A reference to an object implementing the `AuthRepository` trait for user storage operations. Must be thread-safe (`Send` and `Sync`).
    /// - `password_hasher`: A thread-safe reference-counted pointer to an object implementing the `AuthPasswordHasher` trait for password hashing.
    /// - `payload`: A `RegisterRequest` object containing the user's registration information (e.g., email, password).
    ///
    /// # Returns
    /// - `Ok(OkResponse<SimpleMessageResponse>)`: If the user is successfully registered, returns a success message wrapped in an `OkResponse`.
    /// - `Err(FriendlyError)`: If any error occurs during registration, it produces a `FriendlyError`:
    ///   - If the provided email already exists in the repository, a user-facing error (`StatusCode::CONFLICT`) is returned with the message: "Ez az e-mail cím már foglalt" ("This email address is already taken").
    ///   - For any other errors during password hashing or database operations, an internal error is returned.
    ///
    /// # Errors
    /// - `FriendlyError::UserFacing`: Indicates business logic errors, such as duplicate email addresses.
    /// - `FriendlyError::Internal`: Indicates unexpected system errors during operations (e.g., failed hashing or database issues).
    ///
    /// # Notes
    /// - The email duplication check relies on the database rejecting duplicate entries based on a unique constraint.
    /// - Ensure that the password hashing utility is properly configured and secure.
    pub async fn try_register(
        repo: Arc<dyn AuthRepository + Send + Sync>,
        payload: RegisterRequest,
    ) -> Result<(), AuthServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(payload.password.extract().get_value().as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AuthServiceError::Hash(e.to_string()))?;

        repo.insert_user(&payload, &password_hash)
            .await
            .map_err(|e| {
                if let RepositoryError::Database(sqlxe) = &e
                    && let Error::Database(database_error) = sqlxe
                    && database_error.is_unique_violation()
                {
                    AuthServiceError::UserExists
                } else {
                    e.into()
                }
            })?;

        Ok(())
    }
}
