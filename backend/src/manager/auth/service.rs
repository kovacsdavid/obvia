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
    dto::{claims::Claims, login::UserPublic},
};
use crate::{
    common::extractors::ClientContext,
    manager::auth::{
        dto::{login::LoginRequest, register::RegisterRequest},
        model::{AccountEventStatus, AccountEventType},
    },
};
use crate::{
    common::types::value_object::ValueObjectable,
    manager::{
        auth::{dto::register::ResendEmailValidationRequest, model::EmailVerification},
        users::model::User,
    },
};
use crate::{
    common::{
        MailTransporter,
        error::{FriendlyError, RepositoryError},
    },
    manager::auth::model::ForgottenPassword,
};
use crate::{
    common::{dto::GeneralError, error::IntoFriendlyError},
    manager::auth::dto::register::{ForgottenPasswordRequest, NewPasswordRequest},
};
use anyhow::Result;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use async_trait::async_trait;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use chrono::{Duration, Utc};
use handlebars::Handlebars;
use jsonwebtoken::{EncodingKey, Header, encode};
use lettre::{
    Message,
    address::AddressError,
    message::{Mailbox, header::ContentType},
};
use serde_json::json;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hibás e-mail cím vagy jelszó")]
    UserNotFound,

    #[error("A megadott e-mail cím már foglalt!")]
    UserExists,

    #[error("A rendszer jelenleg zárt béta állapotban van. Látogass vissza később!")]
    UserInactive,

    #[error("Hibás e-mail cím vagy jelszó")]
    InvalidPassword,

    #[error("Hibás e-mail megerősítő hivatkozás")]
    InvalidEmailValidationToken,

    #[error("A megerősítő e-mail újraküldése sikertelen")]
    EmailValidationResend,

    #[error("Hash error: {0}")]
    Hash(String),

    #[error("Token generation: {0}")]
    Token(String),

    #[error("RefreshTokenError: {0}")]
    RefreshTokenError(String),

    #[error("MailTransport error: {0}")]
    MailTransport(String),

    #[error("Nincs jogosultságod az erőforrás használatához")]
    Unauthorized,

    #[error("Túl sok próbálkozás történt. Próbáld újra {0} perc múlva!")]
    TooManyAttempts(i64),
}

#[async_trait]
impl IntoFriendlyError<GeneralError> for AuthServiceError {
    async fn into_friendly_error(
        self,
        module: Arc<dyn MailTransporter>,
    ) -> FriendlyError<GeneralError> {
        match self {
            Self::UserNotFound
            | Self::InvalidPassword
            | Self::UserInactive
            | Self::InvalidEmailValidationToken
            | Self::Unauthorized
            | Self::TooManyAttempts(_) => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: self.to_string(),
                },
            ),
            Self::UserExists => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::CONFLICT,
                file!(),
                GeneralError {
                    message: self.to_string(),
                },
            ),
            e => {
                FriendlyError::internal_with_admin_notify(
                    file!(),
                    GeneralError {
                        message: e.to_string(),
                    },
                    module,
                )
                .await
            }
        }
    }
}

pub struct AuthService;

type AuthServiceResult<T> = Result<T, AuthServiceError>;

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
        auth_module: Arc<dyn AuthModule>,
        payload: &LoginRequest,
        client_context: &ClientContext,
    ) -> AuthServiceResult<(String, Claims, String, Claims, UserPublic)> {
        Self::rate_limiter(
            60,
            10,
            auth_module.clone(),
            Some(payload.email.clone()),
            client_context,
        )
        .await?;

        let user = match auth_module
            .auth_repo()
            .get_user_by_email(&payload.email)
            .await
        {
            Ok(user) => user,
            Err(e) => {
                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        None,
                        Some(payload.email.clone()),
                        AccountEventType::Login,
                        AccountEventStatus::Failure,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                return Err(AuthServiceError::UserNotFound);
            }
        };

        if !user.is_active() {
            auth_module
                .auth_repo()
                .insert_account_event_log(
                    Some(user.id),
                    Some(payload.email.clone()),
                    AccountEventType::Login,
                    AccountEventStatus::Failure,
                    Some(client_context.ip),
                    client_context.user_agent.clone(),
                    Some(json!({
                        "error": "Inactive user".to_string()
                    })),
                )
                .await?;
            return Err(AuthServiceError::UserInactive);
        }

        let active_user_tenant = auth_module
            .auth_repo()
            .get_user_active_tenant(user.id)
            .await?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AuthServiceError::Hash(e.to_string()))?;

        match Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash) {
            Ok(_) => (),
            Err(e) => {
                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        Some(user.id),
                        Some(payload.email.clone()),
                        AccountEventType::Login,
                        AccountEventStatus::Failure,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                return Err(AuthServiceError::InvalidPassword);
            }
        };

        auth_module
            .auth_repo()
            .update_user_last_login_at(user.id)
            .await?;

        let active_tenant_id = match active_user_tenant {
            None => None,
            Some(user_tenant) => Some(user_tenant.tenant_id),
        };

        let (access_token, access_token_claims) = Self::gen_jwt(
            user.id,
            auth_module.config().auth().jwt_issuer().to_string(),
            format!("{}-api", auth_module.config().auth().jwt_audience()),
            (Utc::now()
                + Duration::minutes(
                    auth_module
                        .config()
                        .auth()
                        .access_token_expiration_mins()
                        .try_into()
                        .map_err(|_| {
                            AuthServiceError::Token(
                                "access_token_expiration_mins can not be converted to i64"
                                    .to_string(),
                            )
                        })?,
                ))
            .timestamp() as usize,
            active_tenant_id,
            auth_module.config().auth().jwt_secret().as_bytes(),
            None,
        )
        .map_err(|e| AuthServiceError::Token(e.to_string()))?;

        let (refresh_token, refresh_token_claims) = Self::gen_jwt(
            user.id,
            auth_module.config().auth().jwt_issuer().to_string(),
            format!("{}-auth", auth_module.config().auth().jwt_audience()),
            (Utc::now()
                + Duration::minutes(
                    auth_module
                        .config()
                        .auth()
                        .refresh_token_expiration_mins()
                        .try_into()
                        .map_err(|_| {
                            AuthServiceError::Token(
                                "refresh_token_expiration_mins can not be converted to i64"
                                    .to_string(),
                            )
                        })?,
                ))
            .timestamp() as usize,
            active_tenant_id,
            auth_module.config().auth().jwt_secret().as_bytes(),
            Some(Uuid::new_v4()),
        )
        .map_err(|e| AuthServiceError::Token(e.to_string()))?;

        auth_module
            .auth_repo()
            .insert_refresh_token(&refresh_token_claims)
            .await?;

        let _ = auth_module
            .auth_repo()
            .insert_account_event_log(
                Some(user.id),
                Some(payload.email.clone()),
                AccountEventType::Login,
                AccountEventStatus::Success,
                Some(client_context.ip),
                client_context.user_agent.clone(),
                None,
            )
            .await?;

        Ok((
            access_token,
            access_token_claims,
            refresh_token,
            refresh_token_claims,
            user.into(),
        ))
    }

    async fn rate_limiter(
        attempt_interval_mins: i64,
        max_attempts: i64,
        auth_module: Arc<dyn AuthModule>,
        identifier: Option<String>,
        client_context: &ClientContext,
    ) -> AuthServiceResult<()> {
        let event_log_entries = match auth_module
            .auth_repo()
            .account_event_log_failures_by_ip(client_context.ip, attempt_interval_mins)
            .await
        {
            Ok(val) => val,
            Err(e) => {
                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        None,
                        identifier,
                        AccountEventType::Login,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                return Err(AuthServiceError::TooManyAttempts(attempt_interval_mins));
            }
        };

        if event_log_entries > max_attempts {
            auth_module
                .auth_repo()
                .insert_account_event_log(
                    None,
                    identifier,
                    AccountEventType::Login,
                    AccountEventStatus::Blocked,
                    Some(client_context.ip),
                    client_context.user_agent.clone(),
                    Some(json!({
                        "error":
                            AuthServiceError::TooManyAttempts(attempt_interval_mins).to_string()
                    })),
                )
                .await?;
            return Err(AuthServiceError::TooManyAttempts(attempt_interval_mins));
        }
        Ok(())
    }

    fn gen_jwt(
        sub: Uuid,
        iss: String,
        aud: String,
        exp: usize,
        active_tenant_id: Option<Uuid>,
        encoding_key: &[u8],
        family_id: Option<Uuid>,
    ) -> AuthServiceResult<(String, Claims)> {
        let now = Utc::now().timestamp() as usize;
        let nbf = now;

        let claims = Claims::new(
            sub,
            exp,
            now,
            nbf,
            iss,
            aud,
            Uuid::new_v4(),
            family_id,
            active_tenant_id,
        );

        Ok((
            encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(encoding_key),
            )
            .map_err(|e| AuthServiceError::Token(e.to_string()))?,
            claims,
        ))
    }

    pub async fn refresh(
        auth_module: Arc<dyn AuthModule>,
        jar: CookieJar,
        client_context: &ClientContext,
    ) -> AuthServiceResult<(String, Claims, String, Claims, UserPublic)> {
        let current_refresh_token = jar
            .get("refresh_token")
            .ok_or_else(|| AuthServiceError::Unauthorized)?
            .value_trimmed()
            .to_string();
        let current_refresh_token_claims = match Claims::from_token(
            &current_refresh_token,
            auth_module.config().auth().jwt_secret().as_bytes(),
            auth_module.config().auth().jwt_issuer(),
            &format!("{}-auth", auth_module.config().auth().jwt_audience()),
        ) {
            Ok(v) => v,
            Err(e) => {
                match Claims::dangerous_from_token_allow_expired(
                    &current_refresh_token,
                    auth_module.config().auth().jwt_secret().as_bytes(),
                    auth_module.config().auth().jwt_issuer(),
                    &format!("{}-auth", auth_module.config().auth().jwt_audience()),
                ) {
                    Ok(dangerous_claims) => {
                        auth_module
                            .auth_repo()
                            .insert_account_event_log(
                                Some(dangerous_claims.sub()),
                                Some(dangerous_claims.sub().to_string()),
                                AccountEventType::Refresh,
                                AccountEventStatus::Blocked,
                                Some(client_context.ip),
                                client_context.user_agent.clone(),
                                Some(json!({
                                    "error": e.to_string()
                                })),
                            )
                            .await?;
                    }
                    Err(_) => {
                        auth_module
                            .auth_repo()
                            .insert_account_event_log(
                                None,
                                None,
                                AccountEventType::Refresh,
                                AccountEventStatus::Blocked,
                                Some(client_context.ip),
                                client_context.user_agent.clone(),
                                Some(json!({
                                    "error": e.to_string()
                                })),
                            )
                            .await?;
                    }
                };
                return Err(AuthServiceError::Unauthorized);
            }
        };

        let user = match auth_module
            .auth_repo()
            .get_user_by_id(current_refresh_token_claims.sub())
            .await
        {
            Ok(v) => v,
            Err(e) => {
                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        Some(current_refresh_token_claims.sub()),
                        Some(current_refresh_token_claims.sub().to_string()),
                        AccountEventType::Refresh,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                return Err(AuthServiceError::Unauthorized);
            }
        };

        if !user.is_active() {
            match auth_module
                .auth_repo()
                .revoke_refresh_tokens_by_user_id(current_refresh_token_claims.sub())
                .await
            {
                Ok(_) => (),
                Err(e) => {
                    auth_module
                        .auth_repo()
                        .insert_account_event_log(
                            Some(user.id),
                            Some(user.id.to_string()),
                            AccountEventType::Refresh,
                            AccountEventStatus::Error,
                            Some(client_context.ip),
                            client_context.user_agent.clone(),
                            Some(json!({
                                "error": e.to_string()
                            })),
                        )
                        .await?;
                    return Err(e.into());
                }
            };

            auth_module
                .auth_repo()
                .insert_account_event_log(
                    Some(user.id),
                    Some(user.id.to_string()),
                    AccountEventType::Refresh,
                    AccountEventStatus::Blocked,
                    Some(client_context.ip),
                    client_context.user_agent.clone(),
                    Some(json!({
                        "error": "Inactive user".to_string()
                    })),
                )
                .await?;
            return Err(AuthServiceError::Unauthorized);
        }

        let family_id = match current_refresh_token_claims.family_id() {
            Some(family_id) => family_id,
            None => {
                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        Some(user.id),
                        Some(user.id.to_string()),
                        AccountEventType::Refresh,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": "missing family_id".to_string()
                        })),
                    )
                    .await?;
                return Err(AuthServiceError::RefreshTokenError(
                    "missing family_id".to_string(),
                ));
            }
        };

        let current_refresh_token_record = match auth_module
            .auth_repo()
            .get_refresh_token(current_refresh_token_claims.jti())
            .await
        {
            Ok(refresh_token_record) => refresh_token_record,
            Err(e) => {
                match auth_module
                    .auth_repo()
                    .revoke_refresh_tokens_by_family_id(family_id)
                    .await
                {
                    Ok(_) => (),
                    Err(e) => {
                        auth_module
                            .auth_repo()
                            .insert_account_event_log(
                                Some(user.id),
                                Some(user.id.to_string()),
                                AccountEventType::Refresh,
                                AccountEventStatus::Error,
                                Some(client_context.ip),
                                client_context.user_agent.clone(),
                                Some(json!({
                                    "error": e.to_string()
                                })),
                            )
                            .await?;
                        return Err(e.into());
                    }
                };

                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        Some(user.id),
                        Some(user.id.to_string()),
                        AccountEventType::Refresh,
                        AccountEventStatus::Blocked,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                return Err(AuthServiceError::Unauthorized);
            }
        };

        let active_user_tenant = match auth_module
            .auth_repo()
            .get_user_active_tenant(user.id)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        Some(user.id),
                        Some(user.id.to_string()),
                        AccountEventType::Refresh,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                return Err(e.into());
            }
        };

        let active_tenant_id = match active_user_tenant {
            None => None,
            Some(user_tenant) => Some(user_tenant.tenant_id),
        };

        let (access_token, access_token_claims) = match Self::gen_jwt(
            user.id,
            auth_module.config().auth().jwt_issuer().to_string(),
            format!("{}-api", auth_module.config().auth().jwt_audience()),
            (Utc::now()
                + Duration::minutes(
                    auth_module
                        .config()
                        .auth()
                        .access_token_expiration_mins()
                        .try_into()
                        .map_err(|_| {
                            AuthServiceError::Token(
                                "access_token_expiration_mins can not be converted to i64"
                                    .to_string(),
                            )
                        })?,
                ))
            .timestamp() as usize,
            active_tenant_id,
            auth_module.config().auth().jwt_secret().as_bytes(),
            None,
        ) {
            Ok(v) => v,
            Err(e) => {
                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        Some(user.id),
                        Some(user.id.to_string()),
                        AccountEventType::Refresh,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                return Err(AuthServiceError::Token(e.to_string()));
            }
        };

        let (new_refresh_token, new_refresh_token_claims) = match Self::gen_jwt(
            user.id,
            auth_module.config().auth().jwt_issuer().to_string(),
            format!("{}-auth", auth_module.config().auth().jwt_audience()),
            current_refresh_token_claims.exp(),
            active_tenant_id,
            auth_module.config().auth().jwt_secret().as_bytes(),
            current_refresh_token_claims.family_id(),
        ) {
            Ok(v) => v,
            Err(e) => {
                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        Some(user.id),
                        Some(user.id.to_string()),
                        AccountEventType::Refresh,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                return Err(AuthServiceError::Token(e.to_string()));
            }
        };

        match auth_module
            .auth_repo()
            .insert_refresh_token(&new_refresh_token_claims)
            .await
        {
            Ok(_) => (),
            Err(e) => {
                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        Some(user.id),
                        Some(user.id.to_string()),
                        AccountEventType::Refresh,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                return Err(e.into());
            }
        };

        match auth_module
            .auth_repo()
            .consume_refresh_token(
                current_refresh_token_claims.jti(),
                new_refresh_token_claims.jti(),
            )
            .await
        {
            Ok(_) => (),
            Err(e) => {
                auth_module
                    .auth_repo()
                    .insert_account_event_log(
                        Some(user.id),
                        Some(user.id.to_string()),
                        AccountEventType::Refresh,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                return Err(e.into());
            }
        };

        let _ = auth_module
            .auth_repo()
            .insert_account_event_log(
                Some(user.id),
                Some(user.id.to_string()),
                AccountEventType::Refresh,
                AccountEventStatus::Success,
                Some(client_context.ip),
                client_context.user_agent.clone(),
                None,
            )
            .await?;

        Ok((
            access_token,
            access_token_claims,
            new_refresh_token,
            new_refresh_token_claims,
            user.into(),
        ))
    }

    pub async fn logout(auth_module: Arc<dyn AuthModule>, jar: CookieJar) -> AuthServiceResult<()> {
        let refresh_token = jar
            .get("refresh_token")
            .ok_or_else(|| AuthServiceError::Unauthorized)?
            .value_trimmed()
            .to_string();
        let refresh_token_claims = Claims::from_token(
            &refresh_token,
            auth_module.config().auth().jwt_secret().as_bytes(),
            auth_module.config().auth().jwt_issuer(),
            &format!("{}-auth", auth_module.config().auth().jwt_audience()),
        )
        .map_err(|_| AuthServiceError::Unauthorized)?;

        let family_id = match refresh_token_claims.family_id() {
            Some(family_id) => family_id,
            None => {
                return Err(AuthServiceError::RefreshTokenError(
                    "missing family_id".to_string(),
                ));
            }
        };
        auth_module
            .auth_repo()
            .revoke_refresh_tokens_by_family_id(family_id)
            .await?;

        Ok(())
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
        auth_module: Arc<dyn AuthModule>,
        payload: RegisterRequest,
    ) -> AuthServiceResult<()> {
        let password_hash =
            Self::generate_password_hash(payload.password.extract().get_value().as_bytes())?;

        let user = auth_module
            .auth_repo()
            .insert_user(&payload, &password_hash)
            .await
            .map_err(|e| {
                if e.is_unique_violation() {
                    AuthServiceError::UserExists
                } else {
                    e.into()
                }
            })?;
        let email_verification = auth_module
            .auth_repo()
            .insert_email_verification(user.id)
            .await?;
        Self::send_email_verification(auth_module, &user, email_verification).await?;
        Ok(())
    }
    fn generate_password_hash(password: &[u8]) -> AuthServiceResult<String> {
        Argon2::default()
            .hash_password(password, &SaltString::generate(&mut OsRng))
            .map(|hash| hash.to_string())
            .map_err(|e| AuthServiceError::Hash(e.to_string()))
    }
    pub async fn verify_email(
        auth_module: Arc<dyn AuthModule>,
        token: &str,
    ) -> AuthServiceResult<()> {
        let parsed_token =
            Uuid::parse_str(token).map_err(|_| AuthServiceError::InvalidEmailValidationToken)?;
        let email_verification = auth_module
            .auth_repo()
            .get_email_verification(parsed_token)
            .await
            .map_err(|_| AuthServiceError::InvalidEmailValidationToken)?;
        let mut user = auth_module
            .auth_repo()
            .get_user_by_id(email_verification.user_id)
            .await?;
        user.status = String::from("pending");
        auth_module.auth_repo().update_user(user).await?;
        auth_module
            .auth_repo()
            .invalidate_email_verification(email_verification.id)
            .await?;
        Ok(())
    }
    pub async fn resend_email_verification(
        auth_module: Arc<dyn AuthModule>,
        payload: ResendEmailValidationRequest,
    ) -> AuthServiceResult<()> {
        let user = auth_module
            .auth_repo()
            .get_user_by_email(payload.email.extract().get_value())
            .await?;
        if user.need_email_verification() {
            let email_verification = auth_module
                .auth_repo()
                .insert_email_verification(user.id)
                .await?;
            Self::send_email_verification(auth_module, &user, email_verification).await?;
            Ok(())
        } else {
            Err(AuthServiceError::EmailValidationResend)
        }
    }
    async fn send_email_verification(
        auth_module: Arc<dyn AuthModule>,
        user: &User,
        email_verification: EmailVerification,
    ) -> AuthServiceResult<()> {
        let handlebars = Handlebars::new();
        let hostname = auth_module.config().server().hostname().to_owned();
        let verification_uuid = email_verification.id;
        let verification_link = format!("https://{hostname}/email_megerosites/{verification_uuid}");
        let email =
            Message::builder()
                .from(Mailbox::new(
                    Some(auth_module.config().mail().default_from_name().to_owned()),
                    auth_module.config().mail().default_from().parse().map_err(
                        |e: AddressError| AuthServiceError::MailTransport(e.to_string()),
                    )?,
                ))
                .to(Mailbox::new(
                    None,
                    user.email.parse().map_err(|e: AddressError| {
                        AuthServiceError::MailTransport(e.to_string())
                    })?,
                ))
                .subject("Kérlek, erősítsd meg az e-mail címedet!")
                .header(ContentType::TEXT_HTML)
                .body(
                    handlebars
                        .render_template(
                            r##"
                <p style="font-weight: bold; margin-bottom: 25px;">
                    Kedves {{last_name}} {{first_name}}!
                </p>
                <p>
                    Kérlek a következő hivatkozásra kattintva erősítsd meg az e-mail címedet!<br>
                    <a href="{{verification_link}}">{{verification_link}}</a>
                </p>
                "##,
                            &json!({
                                "last_name": user.last_name,
                                "first_name": user.first_name,
                                "verification_link": verification_link,
                            }),
                        )
                        .map_err(|e| AuthServiceError::MailTransport(e.to_string()))?,
                )
                .map_err(|e| AuthServiceError::MailTransport(e.to_string()))?;

        match auth_module.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => Err(AuthServiceError::MailTransport(e.to_string())),
        }
    }

    pub async fn forgotten_password(
        auth_module: Arc<dyn AuthModule>,
        payload: ForgottenPasswordRequest,
    ) -> AuthServiceResult<()> {
        let user = auth_module
            .auth_repo()
            .get_user_by_email(payload.email.extract().get_value())
            .await?;
        if user.is_active() {
            let forgotten_password = auth_module
                .auth_repo()
                .insert_forgotten_password(user.id)
                .await?;
            Self::send_forgotten_password_email(auth_module, &user, forgotten_password).await?;
            Ok(())
        } else {
            Err(AuthServiceError::UserInactive)
        }
    }
    pub async fn new_password(
        auth_module: Arc<dyn AuthModule>,
        payload: NewPasswordRequest,
    ) -> AuthServiceResult<()> {
        let forgotten_password = auth_module
            .auth_repo()
            .get_forgotten_password(payload.token)
            .await?;
        let mut user = auth_module
            .auth_repo()
            .get_user_by_id(forgotten_password.user_id)
            .await?;
        if user.is_active() {
            user.password_hash =
                Self::generate_password_hash(payload.password.extract().get_value().as_bytes())?;
            auth_module.auth_repo().update_user(user).await?;
            auth_module
                .auth_repo()
                .invalidate_forgotten_password(forgotten_password.id)
                .await?;
            Ok(())
        } else {
            Err(AuthServiceError::UserInactive)
        }
    }
    async fn send_forgotten_password_email(
        auth_module: Arc<dyn AuthModule>,
        user: &User,
        forgotten_password: ForgottenPassword,
    ) -> AuthServiceResult<()> {
        let handlebars = Handlebars::new();
        let hostname = auth_module.config().server().hostname().to_owned();
        let forgotten_password_uuid = forgotten_password.id;
        let forgotten_password_link =
            format!("https://{hostname}/elfelejtett_jelszo/{forgotten_password_uuid}");
        let email = Message::builder()
            .from(Mailbox::new(
                Some(auth_module.config().mail().default_from_name().to_owned()),
                auth_module
                    .config()
                    .mail()
                    .default_from()
                    .parse()
                    .map_err(|e: AddressError| AuthServiceError::MailTransport(e.to_string()))?,
            ))
            .to(Mailbox::new(
                None,
                user
                    .email
                    .parse()
                    .map_err(|e: AddressError| AuthServiceError::MailTransport(e.to_string()))?,
            ))
            .subject("Elfelejtett jelszó")
            .header(ContentType::TEXT_HTML)
            .body(
                handlebars
                    .render_template(
                        r##"
                <p style="font-weight: bold; margin-bottom: 25px;">
                    Kedves {{last_name}} {{first_name}}!
                </p>
                <p>
                    A következő hivatkozásra kattintva megváltoztathatod a fiókodhoz tartozó jelszavadat.<br>
                    Ha nem te kérted a jelszó emlékeztető e-mail-t, ne használd a hivatkozást 
                    és értesítd a rendszergazdát a következő e-mail címen: {{admin_email}}!<br>
                    <a href="{{forgotten_password_link}}">{{forgotten_password_link}}</a>
                </p>
                "##,
                        &json!({
                            "last_name": user.last_name,
                            "first_name": user.first_name,
                            "forgotten_password_link": forgotten_password_link,
                            "admin_email": auth_module
                            .config()
                            .mail()
                            .default_notification_email()
                        }),
                    )
                    .map_err(|e| AuthServiceError::MailTransport(e.to_string()))?,
            )
            .map_err(|e| AuthServiceError::MailTransport(e.to_string()))?;

        match auth_module.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => Err(AuthServiceError::MailTransport(e.to_string())),
        }
    }
}
