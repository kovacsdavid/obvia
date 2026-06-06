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

use super::UsersModule;
use crate::common::BaseModule;
use crate::common::dto::GeneralError;
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::extractors::ClientContext;
use crate::common::service::{Service, ServiceError};
use crate::common::value_object::ValueObjectError;
use crate::manager::auth::dto::login::OtpUserInput;
use crate::manager::auth::model::{AccountEventStatus, AccountEventType};
use crate::manager::auth::repository::AuthRepository;
use crate::manager::users::repository::UsersRepository;
use axum::http::StatusCode;
use serde_json::json;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UsersServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("MfaToken error: {0}")]
    MfaToken(String),

    #[error("A kétlépcsős azonosításhoz hasznát kód hibás!")]
    InvalidMfaToken,

    #[error("A kétépcsős azonosítás aktiválása korábban már megtörtént!")]
    MfaAlreadyActive,

    #[error("Túl sok próbálkozás történt. Próbáld újra {0} perc múlva!")]
    TooManyAttempts(i64),

    #[error("ValueObjectError: {0}")]
    ValueObjectError(#[from] ValueObjectError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,
}

impl From<ServiceError> for UsersServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => UsersServiceError::Unauthorized,
        }
    }
}

impl IntoFriendlyError for UsersServiceError {
    async fn into_friendly_error<M>(self, module: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            UsersServiceError::InvalidMfaToken
            | UsersServiceError::TooManyAttempts(_)
            | UsersServiceError::MfaAlreadyActive => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
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

pub type UsersServiceResult<T> = Result<T, UsersServiceError>;

pub trait UserService {
    fn otp_enable(
        &self,
        client_context: &ClientContext,
    ) -> impl Future<Output = UsersServiceResult<String>> + Send;
    fn otp_verify(
        &self,
        payload: &OtpUserInput,
        client_context: &ClientContext,
    ) -> impl Future<Output = UsersServiceResult<()>> + Send;
    fn otp_disable(
        &self,
        payload: &OtpUserInput,
        client_context: &ClientContext,
    ) -> impl Future<Output = UsersServiceResult<()>> + Send;
}

impl<'a, T> UserService for Service<'a, T>
where
    T: UsersModule,
{
    async fn otp_enable(&self, client_context: &ClientContext) -> UsersServiceResult<String> {
        let user = match UsersRepository::get_user_by_id(self.module(), self.claims()?.sub()).await
        {
            Ok(v) => v,
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(self.claims()?.sub()),
                    Some(self.claims()?.sub().to_string()),
                    AccountEventType::MfaEnable,
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

        if user.is_mfa_enabled() {
            AuthRepository::insert_account_event_log(
                self.module(),
                Some(self.claims()?.sub()),
                Some(user.email),
                AccountEventType::MfaEnable,
                AccountEventStatus::Error,
                Some(client_context.ip),
                client_context.user_agent.clone(),
                Some(json!({
                    "error": UsersServiceError::MfaAlreadyActive.to_string()
                })),
            )
            .await?;
            return Err(UsersServiceError::MfaAlreadyActive);
        }

        let user = user.init_mfa_secret();

        let new_mfa_secret = match user
            .mfa_secret
            .clone()
            .ok_or_else(|| UsersServiceError::MfaToken("missing secret".to_string()))
        {
            Ok(v) => v,
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(self.claims()?.sub()),
                    Some(user.email),
                    AccountEventType::MfaEnable,
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

        match UsersRepository::update_user(self.module(), user.clone()).await {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(self.claims()?.sub()),
                    Some(user.email),
                    AccountEventType::MfaEnable,
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

        Ok(new_mfa_secret)
    }

    async fn otp_verify(
        &self,
        payload: &OtpUserInput,
        client_context: &ClientContext,
    ) -> UsersServiceResult<()> {
        let mut user =
            match UsersRepository::get_user_by_id(self.module(), self.claims()?.sub()).await {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(self.claims()?.sub()),
                        Some(self.claims()?.sub().to_string()),
                        AccountEventType::MfaEnable,
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

        if user.is_mfa_enabled() {
            AuthRepository::insert_account_event_log(
                self.module(),
                Some(self.claims()?.sub()),
                Some(user.email),
                AccountEventType::MfaEnable,
                AccountEventStatus::Error,
                Some(client_context.ip),
                client_context.user_agent.clone(),
                Some(json!({
                    "error": UsersServiceError::MfaAlreadyActive.to_string()
                })),
            )
            .await?;
            return Err(UsersServiceError::MfaAlreadyActive);
        }

        match user
            .check_mfa_token(payload.otp.as_str()?)
            .map_err(|_| UsersServiceError::InvalidMfaToken)
        {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(self.claims()?.sub()),
                    Some(user.email),
                    AccountEventType::MfaEnable,
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

        user.is_mfa_enabled = true;

        match UsersRepository::update_user(self.module(), user.clone()).await {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(self.claims()?.sub()),
                    Some(user.email),
                    AccountEventType::MfaEnable,
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
            Some(self.claims()?.sub()),
            Some(user.email),
            AccountEventType::MfaEnable,
            AccountEventStatus::Success,
            Some(client_context.ip),
            client_context.user_agent.clone(),
            None,
        )
        .await?;

        Ok(())
    }

    async fn otp_disable(
        &self,
        payload: &OtpUserInput,
        client_context: &ClientContext,
    ) -> UsersServiceResult<()> {
        rate_limit_by_event_type(
            120,
            5,
            self.module(),
            Some(self.claims()?.sub()),
            Some(self.claims()?.sub().to_string()),
            client_context,
            AccountEventType::MfaDisable,
        )
        .await?;
        let mut user =
            match UsersRepository::get_user_by_id(self.module(), self.claims()?.sub()).await {
                Ok(v) => v,
                Err(e) => {
                    AuthRepository::insert_account_event_log(
                        self.module(),
                        Some(self.claims()?.sub()),
                        Some(self.claims()?.sub().to_string()),
                        AccountEventType::MfaDisable,
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

        match user
            .check_mfa_token(payload.otp.as_str()?)
            .map_err(|_| UsersServiceError::InvalidMfaToken)
        {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(self.claims()?.sub()),
                    Some(user.email),
                    AccountEventType::MfaDisable,
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

        user.is_mfa_enabled = false;
        user.mfa_secret = None;

        match UsersRepository::update_user(self.module(), user.clone()).await {
            Ok(_) => (),
            Err(e) => {
                AuthRepository::insert_account_event_log(
                    self.module(),
                    Some(self.claims()?.sub()),
                    Some(user.email),
                    AccountEventType::MfaDisable,
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
            Some(self.claims()?.sub()),
            Some(user.email),
            AccountEventType::MfaDisable,
            AccountEventStatus::Success,
            Some(client_context.ip),
            client_context.user_agent.clone(),
            None,
        )
        .await?;

        Ok(())
    }
}

async fn rate_limit_by_event_type<T>(
    attempt_interval_mins: i64,
    max_attempts: i64,
    users_module: &T,
    user_id: Option<Uuid>,
    identifier: Option<String>,
    client_context: &ClientContext,
    event_type: AccountEventType,
) -> UsersServiceResult<()>
where
    T: UsersModule + ?Sized,
{
    let event_log_entries = match AuthRepository::account_event_log_by_ip_and_event_type_count(
        users_module,
        client_context.ip,
        event_type.clone(),
        attempt_interval_mins,
    )
    .await
    {
        Ok(val) => val,
        Err(e) => {
            AuthRepository::insert_account_event_log(
                users_module,
                user_id,
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
            return Err(UsersServiceError::TooManyAttempts(attempt_interval_mins));
        }
    };

    if event_log_entries >= max_attempts {
        AuthRepository::insert_account_event_log(
            users_module,
            user_id,
            identifier,
            event_type,
            AccountEventStatus::Blocked,
            Some(client_context.ip),
            client_context.user_agent.clone(),
            Some(json!({
                "error":
                    UsersServiceError::TooManyAttempts(attempt_interval_mins).to_string()
            })),
        )
        .await?;
        return Err(UsersServiceError::TooManyAttempts(attempt_interval_mins));
    }
    Ok(())
}
