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

use crate::common::MailTransporter;
use crate::common::dto::GeneralError;
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::extractors::ClientContext;
use crate::common::types::value_object::ValueObjectable;
use crate::manager::auth::dto::claims::Claims;
use crate::manager::auth::dto::login::OtpUserInput;
use crate::tenant::users::UsersModule;
use async_trait::async_trait;
use axum::http::StatusCode;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum UsersServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("MfaToken error: {0}")]
    MfaToken(String),

    #[error("Invalid MFA token")]
    InvalidMfaToken,
}

#[async_trait]
impl IntoFriendlyError<GeneralError> for UsersServiceError {
    async fn into_friendly_error(
        self,
        module: Arc<dyn MailTransporter>,
    ) -> FriendlyError<GeneralError> {
        match self {
            UsersServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: UsersServiceError::Unauthorized.to_string(),
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

pub type UsersServiceResult<T> = Result<T, UsersServiceError>;

pub struct UsersService;

impl UsersService {
    pub async fn otp_enable(
        users_module: Arc<dyn UsersModule>,
        claims: &Claims,
        client_context: &ClientContext,
    ) -> UsersServiceResult<String> {
        let user = users_module
            .users_repo()
            .get_user_by_id(claims.sub())
            .await?
            .init_mfa_secret();

        let new_mfa_secret = user
            .mfa_secret
            .clone()
            .ok_or_else(|| UsersServiceError::MfaToken("missing secret".to_string()))?;

        let _ = users_module.users_repo().update_user(user).await?;

        Ok(new_mfa_secret)
    }

    pub async fn otp_verify(
        users_module: Arc<dyn UsersModule>,
        claims: &Claims,
        payload: &OtpUserInput,
        client_context: &ClientContext,
    ) -> UsersServiceResult<()> {
        let mut user = users_module
            .users_repo()
            .get_user_by_id(claims.sub())
            .await?;

        user.check_mfa_token(payload.otp.extract().get_value())
            .map_err(|_| UsersServiceError::InvalidMfaToken)?;

        user.is_mfa_enabled = true;

        let _ = users_module.users_repo().update_user(user).await?;

        Ok(())
    }

    pub async fn otp_disable(
        users_module: Arc<dyn UsersModule>,
        claims: &Claims,
        payload: &OtpUserInput,
        client_context: &ClientContext,
    ) -> UsersServiceResult<()> {
        let mut user = users_module
            .users_repo()
            .get_user_by_id(claims.sub())
            .await?;

        user.check_mfa_token(payload.otp.extract().get_value())
            .map_err(|_| UsersServiceError::InvalidMfaToken)?;

        user.is_mfa_enabled = false;
        user.mfa_secret = None;

        let _ = users_module.users_repo().update_user(user).await?;

        Ok(())
    }
}
