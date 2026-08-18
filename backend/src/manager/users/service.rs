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

use super::UsersModuleInterface;
use crate::common::extractors::ClientContext;
use crate::common::service::{Service, ServiceError};
use crate::manager::auth::dto::login::OtpUserInput;
use crate::manager::auth::model::{AccountEventStatus, AccountEventType};
use serde_json::json;
use uuid::Uuid;

pub type UsersServiceResult<T> = Result<T, ServiceError>;

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
    T: UsersModuleInterface,
{
    async fn otp_enable(&self, client_context: &ClientContext) -> UsersServiceResult<String> {
        let users_repo = self.module().users_repo();
        let auth_repo = self.module().auth_repo();
        let user = match users_repo.get_user_by_id(self.claims()?.sub()).await {
            Ok(v) => v,
            Err(e) => {
                auth_repo
                    .insert_account_event_log(
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
            auth_repo
                .insert_account_event_log(
                    Some(self.claims()?.sub()),
                    Some(user.email),
                    AccountEventType::MfaEnable,
                    AccountEventStatus::Error,
                    Some(client_context.ip),
                    client_context.user_agent.clone(),
                    Some(json!({
                        "error": ServiceError::MfaAlreadyActive.to_string()
                    })),
                )
                .await?;
            return Err(ServiceError::MfaAlreadyActive);
        }

        let user = user.init_mfa_secret();

        let new_mfa_secret = match user
            .mfa_secret
            .clone()
            .ok_or_else(|| ServiceError::MfaToken("missing secret".to_string()))
        {
            Ok(v) => v,
            Err(e) => {
                auth_repo
                    .insert_account_event_log(
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

        match users_repo.update_user(user.clone()).await {
            Ok(_) => (),
            Err(e) => {
                auth_repo
                    .insert_account_event_log(
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
        let users_repo = self.module().users_repo();
        let auth_repo = self.module().auth_repo();
        let mut user = match users_repo.get_user_by_id(self.claims()?.sub()).await {
            Ok(v) => v,
            Err(e) => {
                auth_repo
                    .insert_account_event_log(
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
            auth_repo
                .insert_account_event_log(
                    Some(self.claims()?.sub()),
                    Some(user.email),
                    AccountEventType::MfaEnable,
                    AccountEventStatus::Error,
                    Some(client_context.ip),
                    client_context.user_agent.clone(),
                    Some(json!({
                        "error": ServiceError::MfaAlreadyActive.to_string()
                    })),
                )
                .await?;
            return Err(ServiceError::MfaAlreadyActive);
        }

        match user
            .check_mfa_token(payload.otp.as_str()?)
            .map_err(|_| ServiceError::InvalidMfaToken)
        {
            Ok(_) => (),
            Err(e) => {
                auth_repo
                    .insert_account_event_log(
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

        match users_repo.update_user(user.clone()).await {
            Ok(_) => (),
            Err(e) => {
                auth_repo
                    .insert_account_event_log(
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

        auth_repo
            .insert_account_event_log(
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
        let users_repo = self.module().users_repo();
        let auth_repo = self.module().auth_repo();
        let mut user = match users_repo.get_user_by_id(self.claims()?.sub()).await {
            Ok(v) => v,
            Err(e) => {
                auth_repo
                    .insert_account_event_log(
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
            .map_err(|_| ServiceError::InvalidMfaToken)
        {
            Ok(_) => (),
            Err(e) => {
                auth_repo
                    .insert_account_event_log(
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

        match users_repo.update_user(user.clone()).await {
            Ok(_) => (),
            Err(e) => {
                auth_repo
                    .insert_account_event_log(
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

        auth_repo
            .insert_account_event_log(
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
    T: UsersModuleInterface + ?Sized,
{
    let event_log_entries = match users_module
        .auth_repo()
        .account_event_log_by_ip_and_event_type_count(
            client_context.ip,
            event_type.clone(),
            attempt_interval_mins,
        )
        .await
    {
        Ok(val) => val,
        Err(e) => {
            users_module
                .auth_repo()
                .insert_account_event_log(
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
            return Err(ServiceError::TooManyAttempts(attempt_interval_mins));
        }
    };

    if event_log_entries >= max_attempts {
        users_module
            .auth_repo()
            .insert_account_event_log(
                user_id,
                identifier,
                event_type,
                AccountEventStatus::Blocked,
                Some(client_context.ip),
                client_context.user_agent.clone(),
                Some(json!({
                    "error":
                        ServiceError::TooManyAttempts(attempt_interval_mins).to_string()
                })),
            )
            .await?;
        return Err(ServiceError::TooManyAttempts(attempt_interval_mins));
    }
    Ok(())
}
