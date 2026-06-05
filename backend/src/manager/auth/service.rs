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
    common::service::{Service, ServiceError},
    manager::auth::repository::AuthRepository,
};
use crate::{
    common::value_object::ValueObjectError,
    manager::{
        auth::{dto::register::ResendEmailValidationRequest, model::EmailVerification},
        users::model::User,
    },
};
use crate::{
    common::{
        BaseModule,
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
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{Duration, Utc};
use handlebars::Handlebars;
use jsonwebtoken::{EncodingKey, Header, encode};
use lettre::{
    Message,
    address::AddressError,
    message::{Mailbox, header::ContentType},
};
use rand::RngExt;
use serde_json::json;
use std::sync::Arc;
use thiserror::Error;
use time::Duration as TimeDuration;
use tokio::time::{Duration as TokioDuration, sleep};
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

    #[error("RefreshCookieError: {0}")]
    RefreshCookieError(&'static str),

    #[error("MailTransport error: {0}")]
    MailTransport(String),

    #[error("Nincs jogosultságod az erőforrás használatához")]
    Unauthorized,

    #[error("Túl sok próbálkozás történt. Próbáld újra {0} perc múlva!")]
    TooManyAttempts(i64),

    #[error("totp-required")]
    MfaRequired,

    #[error("Hibás kétlépcsős azonosító kód!")]
    MfaInvalid,

    #[error("Hibás elfelejtett jelszó hivatkozás!")]
    InvalidForgottenPasswordToken,

    #[error("Unexpected ValueObjectError: {0}")]
    UnexpectedValueObjectError(#[from] ValueObjectError),
}

impl From<ServiceError> for AuthServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => AuthServiceError::Unauthorized,
        }
    }
}

impl IntoFriendlyError for AuthServiceError {
    async fn into_friendly_error<M>(self, module: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            Self::UserNotFound
            | Self::InvalidPassword
            | Self::UserInactive
            | Self::InvalidEmailValidationToken
            | Self::Unauthorized
            | Self::TooManyAttempts(_)
            | Self::MfaRequired
            | Self::MfaInvalid
            | Self::InvalidForgottenPasswordToken => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: self.to_string(),
                }
                .to_string(),
            ),
            Self::UserExists => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::CONFLICT,
                file!(),
                GeneralError {
                    message: self.to_string(),
                }
                .to_string(),
            ),
            e => {
                FriendlyError::internal_with_admin_notify(
                    file!(),
                    GeneralError {
                        message: e.to_string(),
                    }
                    .to_string(),
                    module,
                )
                .await
            }
        }
    }
}

pub type AuthServiceResult<T> = Result<T, AuthServiceError>;

pub trait AuthService {
    fn try_login(
        &self,
        payload: &LoginRequest,
        client_context: &ClientContext,
    ) -> impl Future<Output = AuthServiceResult<(String, Claims, String, Claims, UserPublic)>> + Send;
    fn refresh(
        &self,
        jar: CookieJar,
        client_context: &ClientContext,
    ) -> impl Future<Output = AuthServiceResult<(String, Claims, String, Claims, UserPublic)>> + Send;
    fn logout(
        &self,
        jar: CookieJar,
        client_context: &ClientContext,
    ) -> impl Future<Output = AuthServiceResult<()>> + Send;
    fn try_register(
        &self,
        payload: &RegisterRequest,
    ) -> impl Future<Output = AuthServiceResult<()>> + Send;
    fn verify_email(&self, token: &str) -> impl Future<Output = AuthServiceResult<()>> + Send;
    fn resend_email_verification(
        &self,
        payload: ResendEmailValidationRequest,
    ) -> impl Future<Output = AuthServiceResult<()>> + Send;
    fn forgotten_password(
        &self,
        payload: ForgottenPasswordRequest,
        client_context: &ClientContext,
    ) -> impl Future<Output = AuthServiceResult<()>> + Send;
    fn new_password(
        &self,
        payload: NewPasswordRequest,
        client_context: &ClientContext,
    ) -> impl Future<Output = AuthServiceResult<()>> + Send;
}

impl<'a, T> AuthService for Service<'a, T>
where
    T: AuthModule,
{
    async fn try_login(
        &self,
        payload: &LoginRequest,
        client_context: &ClientContext,
    ) -> AuthServiceResult<(String, Claims, String, Claims, UserPublic)> {
        rate_limit_by_event_status(
            60,
            10,
            self.module(),
            Some(payload.email.clone()),
            client_context,
            AccountEventStatus::Failure,
            AccountEventType::Login,
        )
        .await?;

        let user = match AuthRepository::get_user_by_email(self.module(), &payload.email).await {
            Ok(user) => user,
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
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
            AuthRepository::insert_account_event_log(
                self.module(),
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

        let parsed_hash = match PasswordHash::new(&user.password_hash) {
            Ok(v) => v,
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(payload.email.clone()),
                    AccountEventType::Login,
                    AccountEventStatus::Error,
                    Some(client_context.ip),
                    client_context.user_agent.clone(),
                    Some(json!({
                        "error": e.to_string()
                    })),
                )
                .await?;
                return Err(AuthServiceError::Hash(e.to_string()));
            }
        };

        match Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash) {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
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

        if user.is_mfa_enabled() {
            match &payload.otp {
                Some(v) => {
                    match user.check_mfa_token(v) {
                        Ok(_) => (),
                        Err(e) => {
                            AuthRepository::insert_account_event_log(
                                self.module(),
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
                            return Err(AuthServiceError::MfaInvalid);
                        }
                    };
                }
                None => return Err(AuthServiceError::MfaRequired),
            }
        }

        let active_user_tenant =
            match AuthRepository::get_user_active_tenant(self.module(), user.id).await {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(user.id),
                        Some(payload.email.clone()),
                        AccountEventType::Login,
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

        let (access_token, access_token_claims) = match gen_jwt(
            user.id,
            self.module().config().auth().jwt_issuer().to_string(),
            format!("{}-api", self.module().config().auth().jwt_audience()),
            match gen_exp(self.module().config().auth().access_token_expiration_mins()) {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(user.id),
                        Some(payload.email.clone()),
                        AccountEventType::Login,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                    return Err(e);
                }
            },
            active_tenant_id,
            self.module().config().auth().jwt_secret().as_bytes(),
            None,
        ) {
            Ok(v) => v,
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(payload.email.clone()),
                    AccountEventType::Login,
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

        let (refresh_token, refresh_token_claims) = match gen_jwt(
            user.id,
            self.module().config().auth().jwt_issuer().to_string(),
            format!("{}-auth", self.module().config().auth().jwt_audience()),
            match gen_exp(
                self.module()
                    .config()
                    .auth()
                    .refresh_token_expiration_mins(),
            ) {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(user.id),
                        Some(payload.email.clone()),
                        AccountEventType::Login,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                    return Err(e);
                }
            },
            active_tenant_id,
            self.module().config().auth().jwt_secret().as_bytes(),
            Some(Uuid::new_v4()),
        ) {
            Ok(v) => v,
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(payload.email.clone()),
                    AccountEventType::Login,
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

        match AuthRepository::insert_refresh_token(self.module(), &refresh_token_claims).await {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(payload.email.clone()),
                    AccountEventType::Login,
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

        match AuthRepository::update_user_last_login_at(self.module(), user.id).await {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(payload.email.clone()),
                    AccountEventType::Login,
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

        let _ = AuthRepository::insert_account_event_log(
            self.module(),
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

    async fn refresh(
        &self,
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
            self.module().config().auth().jwt_secret().as_bytes(),
            self.module().config().auth().jwt_issuer(),
            &format!("{}-auth", self.module().config().auth().jwt_audience()),
        ) {
            Ok(v) => v,
            Err(e) => {
                match Claims::dangerous_from_token_allow_expired(
                    &current_refresh_token,
                    self.module().config().auth().jwt_secret().as_bytes(),
                    self.module().config().auth().jwt_issuer(),
                    &format!("{}-auth", self.module().config().auth().jwt_audience()),
                ) {
                    Ok(dangerous_claims) => {
                        AuthRepository::insert_account_event_log(
                            self.module(),
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
                        AuthRepository::insert_account_event_log(
                            self.module(),
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

        let user =
            match AuthRepository::get_user_by_id(self.module(), current_refresh_token_claims.sub())
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
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
            match AuthRepository::revoke_refresh_tokens_by_user_id(
                self.module(),
                current_refresh_token_claims.sub(),
            )
            .await
            {
                Ok(_) => (),
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(user.id),
                        Some(user.email),
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

            AuthRepository::insert_account_event_log(
                self.module(),
                Some(user.id),
                Some(user.email),
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
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(user.email),
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

        let _ = match AuthRepository::get_refresh_token(
            self.module(),
            current_refresh_token_claims.jti(),
        )
        .await
        {
            Ok(refresh_token_record) => refresh_token_record,
            Err(e) => {
                match AuthRepository::revoke_refresh_tokens_by_family_id(self.module(), family_id)
                    .await
                {
                    Ok(_) => (),
                    Err(e) => {
                        AuthRepository::insert_account_event_log(
                            self.module(),
                            Some(user.id),
                            Some(user.email),
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

                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(user.email),
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

        let active_user_tenant =
            match AuthRepository::get_user_active_tenant(self.module(), user.id).await {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(user.id),
                        Some(user.email),
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

        let (access_token, access_token_claims) = match gen_jwt(
            user.id,
            self.module().config().auth().jwt_issuer().to_string(),
            format!("{}-api", self.module().config().auth().jwt_audience()),
            match gen_exp(self.module().config().auth().access_token_expiration_mins()) {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(user.id),
                        Some(user.email),
                        AccountEventType::Refresh,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                    return Err(e);
                }
            },
            active_tenant_id,
            self.module().config().auth().jwt_secret().as_bytes(),
            None,
        ) {
            Ok(v) => v,
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(user.email),
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

        let (new_refresh_token, new_refresh_token_claims) = match gen_jwt(
            user.id,
            self.module().config().auth().jwt_issuer().to_string(),
            format!("{}-auth", self.module().config().auth().jwt_audience()),
            current_refresh_token_claims.exp(),
            active_tenant_id,
            self.module().config().auth().jwt_secret().as_bytes(),
            current_refresh_token_claims.family_id(),
        ) {
            Ok(v) => v,
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(user.email),
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

        match AuthRepository::insert_refresh_token(self.module(), &new_refresh_token_claims).await {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(user.email),
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

        match AuthRepository::consume_refresh_token(
            self.module(),
            current_refresh_token_claims.jti(),
            new_refresh_token_claims.jti(),
        )
        .await
        {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(user.id),
                    Some(user.email),
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

        let _ = AuthRepository::insert_account_event_log(
            self.module(),
            Some(user.id),
            Some(user.email.clone()),
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

    async fn logout(
        &self,
        jar: CookieJar,
        client_context: &ClientContext,
    ) -> AuthServiceResult<()> {
        let refresh_token = jar
            .get("refresh_token")
            .ok_or_else(|| AuthServiceError::Unauthorized)?
            .value_trimmed()
            .to_string();
        let dangerous_refresh_claims = match Claims::dangerous_from_token_allow_expired(
            &refresh_token,
            self.module().config().auth().jwt_secret().as_bytes(),
            self.module().config().auth().jwt_issuer(),
            &format!("{}-auth", self.module().config().auth().jwt_audience()),
        ) {
            Ok(v) => v,
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    None,
                    None,
                    AccountEventType::Logout,
                    AccountEventStatus::Failure,
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

        let family_id = match dangerous_refresh_claims.family_id() {
            Some(family_id) => family_id,
            None => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(dangerous_refresh_claims.sub()),
                    Some(dangerous_refresh_claims.sub().to_string()),
                    AccountEventType::Logout,
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
        match AuthRepository::revoke_refresh_tokens_by_family_id(self.module(), family_id).await {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(dangerous_refresh_claims.sub()),
                    Some(dangerous_refresh_claims.sub().to_string()),
                    AccountEventType::Logout,
                    AccountEventStatus::Error,
                    Some(client_context.ip),
                    client_context.user_agent.clone(),
                    Some(json!({
                        "error": "missing family_id".to_string()
                    })),
                )
                .await?;
                return Err(e.into());
            }
        };

        let _ = AuthRepository::insert_account_event_log(
            self.module(),
            Some(dangerous_refresh_claims.sub()),
            Some(dangerous_refresh_claims.sub().to_string()),
            AccountEventType::Logout,
            AccountEventStatus::Success,
            Some(client_context.ip),
            client_context.user_agent.clone(),
            None,
        )
        .await?;

        Ok(())
    }

    async fn try_register(&self, payload: &RegisterRequest) -> AuthServiceResult<()> {
        let password_hash = generate_password_hash(payload.password.as_str()?.as_bytes())?;

        let user = AuthRepository::insert_user(self.module(), payload, &password_hash)
            .await
            .map_err(|e| {
                if e.is_unique_violation() {
                    AuthServiceError::UserExists
                } else {
                    e.into()
                }
            })?;
        let email_verification =
            AuthRepository::insert_email_verification(self.module(), user.id).await?;
        send_email_verification(self.module(), &user, email_verification).await?;
        Ok(())
    }
    async fn verify_email(&self, token: &str) -> AuthServiceResult<()> {
        let parsed_token =
            Uuid::parse_str(token).map_err(|_| AuthServiceError::InvalidEmailValidationToken)?;
        let email_verification =
            AuthRepository::get_email_verification(self.module(), parsed_token)
                .await
                .map_err(|_| AuthServiceError::InvalidEmailValidationToken)?;
        let mut user =
            AuthRepository::get_user_by_id(self.module(), email_verification.user_id).await?;
        user.status = String::from("pending");
        AuthRepository::update_user(self.module(), user).await?;
        AuthRepository::invalidate_email_verification(self.module(), email_verification.id).await?;
        Ok(())
    }
    async fn resend_email_verification(
        &self,
        payload: ResendEmailValidationRequest,
    ) -> AuthServiceResult<()> {
        let user =
            AuthRepository::get_user_by_email(self.module(), payload.email.as_str()?).await?;
        if user.need_email_verification() {
            let email_verification =
                AuthRepository::insert_email_verification(self.module(), user.id).await?;
            send_email_verification(self.module(), &user, email_verification).await?;
            Ok(())
        } else {
            Err(AuthServiceError::EmailValidationResend)
        }
    }

    async fn forgotten_password(
        &self,
        payload: ForgottenPasswordRequest,
        client_context: &ClientContext,
    ) -> AuthServiceResult<()> {
        rate_limit_by_event_type(
            120,
            5,
            self.module(),
            Some(payload.email.to_string()),
            client_context,
            AccountEventType::PasswordResetRequest,
        )
        .await?;
        rate_limit_by_event_status(
            60,
            10,
            self.module(),
            Some(payload.email.to_string()),
            client_context,
            AccountEventStatus::Failure,
            AccountEventType::PasswordResetRequest,
        )
        .await?;
        let user =
            match AuthRepository::get_user_by_email(self.module(), payload.email.as_str()?).await {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        None,
                        Some(payload.email.to_string()),
                        AccountEventType::PasswordResetRequest,
                        AccountEventStatus::Failure,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                    let sleep_secs = rand::rng().random_range(1000..=6000);
                    sleep(TokioDuration::from_millis(sleep_secs)).await;
                    return Ok(());
                }
            };
        if user.is_active() {
            let forgotten_password =
                AuthRepository::insert_forgotten_password(self.module(), user.id).await?;
            send_forgotten_password_email(self.module(), &user, forgotten_password).await?;
            AuthRepository::insert_account_event_log(
                self.module(),
                Some(user.id),
                Some(payload.email.to_string()),
                AccountEventType::PasswordResetRequest,
                AccountEventStatus::Success,
                Some(client_context.ip),
                client_context.user_agent.clone(),
                None,
            )
            .await?;
            let sleep_secs = rand::rng().random_range(1000..=4000);
            sleep(TokioDuration::from_millis(sleep_secs)).await;
            Ok(())
        } else {
            AuthRepository::insert_account_event_log(
                self.module(),
                Some(user.id),
                Some(payload.email.to_string()),
                AccountEventType::PasswordResetRequest,
                AccountEventStatus::Failure,
                Some(client_context.ip),
                client_context.user_agent.clone(),
                Some(json!({
                    "error": "Inactive user".to_string()
                })),
            )
            .await?;
            let sleep_secs = rand::rng().random_range(1000..=6000);
            sleep(TokioDuration::from_millis(sleep_secs)).await;
            Ok(())
        }
    }
    async fn new_password(
        &self,
        payload: NewPasswordRequest,
        client_context: &ClientContext,
    ) -> AuthServiceResult<()> {
        rate_limit_by_event_type(
            120,
            5,
            self.module(),
            Some(payload.token.to_string()),
            client_context,
            AccountEventType::PasswordChange,
        )
        .await?;
        rate_limit_by_event_status(
            60,
            10,
            self.module(),
            Some(payload.token.to_string()),
            client_context,
            AccountEventStatus::Failure,
            AccountEventType::PasswordChange,
        )
        .await?;

        let forgotten_password =
            match AuthRepository::get_forgotten_password(self.module(), payload.token.as_uuid()?)
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        None,
                        Some(payload.token.to_string()),
                        AccountEventType::PasswordChange,
                        AccountEventStatus::Failure,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                    return Err(AuthServiceError::InvalidForgottenPasswordToken);
                }
            };

        let mut user =
            match AuthRepository::get_user_by_id(self.module(), forgotten_password.user_id).await {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(forgotten_password.user_id),
                        Some(payload.token.to_string()),
                        AccountEventType::PasswordChange,
                        AccountEventStatus::Failure,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                    return Err(AuthServiceError::InvalidForgottenPasswordToken);
                }
            };

        if user.is_active() {
            user.password_hash = match generate_password_hash(payload.password.as_str()?.as_bytes())
            {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(user.id),
                        Some(user.email),
                        AccountEventType::PasswordChange,
                        AccountEventStatus::Error,
                        Some(client_context.ip),
                        client_context.user_agent.clone(),
                        Some(json!({
                            "error": e.to_string()
                        })),
                    )
                    .await?;
                    return Err(e);
                }
            };

            match AuthRepository::update_user(self.module(), user.clone()).await {
                Ok(_) => (),
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(user.id),
                        Some(user.email),
                        AccountEventType::PasswordChange,
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
            match AuthRepository::invalidate_forgotten_password(
                self.module(),
                forgotten_password.id,
            )
            .await
            {
                Ok(_) => (),
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(user.id),
                        Some(user.email),
                        AccountEventType::PasswordChange,
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
            AuthRepository::insert_account_event_log(
                self.module(),
                Some(user.id),
                Some(user.email),
                AccountEventType::PasswordChange,
                AccountEventStatus::Success,
                Some(client_context.ip),
                client_context.user_agent.clone(),
                None,
            )
            .await?;
            Ok(())
        } else {
            AuthRepository::insert_account_event_log(
                self.module(),
                Some(user.id),
                Some(user.email),
                AccountEventType::PasswordChange,
                AccountEventStatus::Blocked,
                Some(client_context.ip),
                client_context.user_agent.clone(),
                Some(json!({
                    "error": "Inactive user".to_string()
                })),
            )
            .await?;
            Err(AuthServiceError::UserInactive)
        }
    }
}

pub fn gen_refresh_cookie(
    refresh_token: String,
    secure_cookie: bool,
    refresh_token_expiration_mins: u64,
) -> AuthServiceResult<Cookie<'static>> {
    let max_age: i64 = refresh_token_expiration_mins.try_into().map_err(|_| {
        AuthServiceError::RefreshCookieError(
            "refresh_token_expiration_mins could not be converted to i64",
        )
    })?;
    Ok(Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .secure(secure_cookie)
        .same_site(SameSite::Strict)
        .path("/api/auth/t")
        .max_age(TimeDuration::minutes(max_age))
        .build())
}

async fn rate_limit_by_event_status<T>(
    attempt_interval_mins: i64,
    max_attempts: i64,
    auth_module: &T,
    identifier: Option<String>,
    client_context: &ClientContext,
    event_status: AccountEventStatus,
    event_type: AccountEventType,
) -> AuthServiceResult<()>
where
    T: AuthModule,
{
    let event_log_entries = match AuthRepository::account_event_log_ip_and_event_status_count(
        auth_module,
        client_context.ip,
        event_status,
        attempt_interval_mins,
    )
    .await
    {
        Ok(val) => val,
        Err(e) => {
            AuthRepository::insert_account_event_log(
                auth_module,
                None,
                identifier,
                event_type,
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

    if event_log_entries >= max_attempts {
        AuthRepository::insert_account_event_log(
            auth_module,
            None,
            identifier,
            event_type,
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

async fn rate_limit_by_event_type<T>(
    attempt_interval_mins: i64,
    max_attempts: i64,
    auth_module: &T,
    identifier: Option<String>,
    client_context: &ClientContext,
    event_type: AccountEventType,
) -> AuthServiceResult<()>
where
    T: AuthModule,
{
    let event_log_entries = match AuthRepository::account_event_log_by_ip_and_event_type_count(
        auth_module,
        client_context.ip,
        event_type.clone(),
        attempt_interval_mins,
    )
    .await
    {
        Ok(val) => val,
        Err(e) => {
            AuthRepository::insert_account_event_log(
                auth_module,
                None,
                identifier,
                event_type,
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

    if event_log_entries >= max_attempts {
        AuthRepository::insert_account_event_log(
            auth_module,
            None,
            identifier,
            event_type,
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

fn gen_exp(expiration_mins: u64) -> AuthServiceResult<usize> {
    (Utc::now()
        + Duration::minutes(expiration_mins.try_into().map_err(|_| {
            AuthServiceError::Token(
                "refresh_token_expiration_mins can not be converted to i64".to_string(),
            )
        })?))
    .timestamp()
    .try_into()
    .map_err(|_| AuthServiceError::Token("exp can not be converted to usize".to_string()))
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

fn generate_password_hash(password: &[u8]) -> AuthServiceResult<String> {
    Argon2::default()
        .hash_password(password, &SaltString::generate(&mut OsRng))
        .map(|hash| hash.to_string())
        .map_err(|e| AuthServiceError::Hash(e.to_string()))
}

async fn send_email_verification<T>(
    auth_module: &T,
    user: &User,
    email_verification: EmailVerification,
) -> AuthServiceResult<()>
where
    T: AuthModule,
{
    let handlebars = Handlebars::new();
    let hostname = auth_module.config().server().public_base_url().to_owned();
    let verification_uuid = email_verification.id;
    let verification_link = format!("https://{hostname}/email_megerosites/{verification_uuid}");
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
            user.email
                .parse()
                .map_err(|e: AddressError| AuthServiceError::MailTransport(e.to_string()))?,
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

async fn send_forgotten_password_email<T>(
    auth_module: &T,
    user: &User,
    forgotten_password: ForgottenPassword,
) -> AuthServiceResult<()>
where
    T: AuthModule,
{
    let handlebars = Handlebars::new();
    let hostname = auth_module.config().server().public_base_url().to_owned();
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
