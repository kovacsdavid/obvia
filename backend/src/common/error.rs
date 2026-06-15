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

use std::{fmt::Display, num::TryFromIntError, sync::Arc};
use thiserror::Error;

use crate::common::{
    BaseModule, ConfigProvider, MailTransporter,
    config::AppConfig,
    dto::{ErrorResponse, FormError, GeneralError},
    value_object::ValueObjectError,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use handlebars::Handlebars;
use lettre::Message;
use lettre::{
    address::AddressError,
    message::{Mailbox, header::ContentType},
};
use serde::Serialize;
use serde_json::json;
use sqlx::Error;
use sqlx::migrate::MigrateError;
use tracing::Level;
use tracing::event;

#[derive(Debug, Error, Clone)]
pub enum FriendlyError {
    #[error("{0}")]
    UserFacing(StatusCode, String, String),
    #[error("Váratlan hiba történt a feldolgozás során!")]
    Internal(String, String),
}

impl FriendlyError {
    pub fn user_facing(severity: Level, status: StatusCode, loc: &str, body: String) -> Self {
        Self::UserFacing(status, loc.to_string(), body).trace(severity)
    }
    pub fn internal(loc: &str, body: String) -> Self {
        Self::Internal(loc.to_string(), body).trace(Level::ERROR)
    }
    pub async fn internal_with_admin_notify<M>(loc: &str, body: String, mailer: Arc<M>) -> Self
    where
        M: MailTransporter + ConfigProvider<Cfg = AppConfig> + Send + Sync,
    {
        let fe = Self::Internal(loc.to_string(), body).trace(Level::ERROR);

        if let Err(e) = fe.notify_admin(mailer).await {
            event!(Level::ERROR, "Could not notify admin: {e}")
        }

        fe
    }

    fn trace(self, severity: Level) -> Self {
        match &self {
            FriendlyError::UserFacing(status, loc, body) => match severity {
                Level::ERROR => {
                    event!(
                        Level::ERROR,
                        "User-facing error: http-status={status} location={loc}, message={body}",
                    );
                }
                Level::WARN => {
                    event!(
                        Level::WARN,
                        "User-facing error: http-status={status} location={loc}, message={body}",
                    );
                }
                Level::INFO => {
                    event!(
                        Level::INFO,
                        "User-facing error: http-status={status} location={loc}, message={body}",
                    );
                }
                Level::DEBUG => {
                    event!(
                        Level::DEBUG,
                        "User-facing error: http-status={status} location={loc}, message={body}",
                    );
                }
                Level::TRACE => {
                    event!(
                        Level::TRACE,
                        "User-facing error: http-status={status} location={loc}, message={body}",
                    );
                }
            },
            FriendlyError::Internal(body, loc) => match severity {
                Level::ERROR => {
                    event!(
                        Level::ERROR,
                        "Internal error: location={loc} message={body}"
                    );
                }
                Level::WARN => {
                    event!(Level::WARN, "Internal error: location={loc} message={body}");
                }
                Level::INFO => {
                    event!(
                        Level::INFO,
                        "Internal error:  location={loc} message={body}"
                    );
                }
                Level::DEBUG => {
                    event!(
                        Level::DEBUG,
                        "Internal error: location={loc} message={body}"
                    );
                }
                Level::TRACE => {
                    event!(
                        Level::TRACE,
                        "Internal error: location={loc} message={body}"
                    );
                }
            },
        }
        self
    }
    async fn notify_admin<M>(&self, mailer: Arc<M>) -> Result<(), String>
    where
        M: MailTransporter + ConfigProvider<Cfg = AppConfig>,
    {
        if let FriendlyError::Internal(loc, body) = &self {
            let config = mailer.config().mail();
            let handlebars = Handlebars::new();
            let email = Message::builder()
                .from(Mailbox::new(
                    Some(config.default_from_name().to_owned()),
                    config
                        .default_from()
                        .parse()
                        .map_err(|e: AddressError| e.to_string())?,
                ))
                .to(Mailbox::new(
                    None,
                    config
                        .default_notification_email()
                        .parse()
                        .map_err(|e: AddressError| e.to_string())?,
                ))
                .subject("Unexpected error")
                .header(ContentType::TEXT_PLAIN)
                .body(
                    handlebars
                        .render_template(
                            r##"
                    Dear Admin!\n\n
                    Check this error!\n
                    Internal error: location={{loc}} message={{body}}
                    "##,
                            &json!({"loc": loc, "body": body.to_string()}),
                        )
                        .map_err(|e| e.to_string())?,
                )
                .map_err(|e| e.to_string())?;

            match mailer.send(email).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("".to_string())
        }
    }
}

impl IntoResponse for FriendlyError {
    fn into_response(self) -> Response {
        match self {
            FriendlyError::UserFacing(status, _, body) => ErrorResponse {
                status_code: status,
                error: body,
            }
            .into_response(),
            FriendlyError::Internal(_, _) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: GeneralError {
                    message: String::from("Váratlan hiba történt a feldolgozás során"),
                },
            }
            .into_response(),
        }
    }
}

pub trait FormErrorResponse: Serialize + Display {
    fn global_message(&self) -> String {
        "Kérjük ellenőrizze a hibás mezőket!".to_string()
    }
    fn status_code(&self) -> StatusCode {
        StatusCode::UNPROCESSABLE_ENTITY
    }
    fn log_level(&self) -> Level {
        Level::DEBUG
    }
    fn get_error_response(&self) -> Response {
        FriendlyError::user_facing(
            self.log_level(),
            self.status_code(),
            file!(),
            FormError {
                message: self.global_message(),
                fields: self,
            }
            .to_string(),
        )
        .into_response()
    }
}

pub trait IntoFriendlyError {
    fn into_friendly_error<M>(self, mailer: Arc<M>) -> impl Future<Output = FriendlyError> + Send
    where
        M: BaseModule;
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Migrate error: {0}")]
    Migrate(#[from] MigrateError),

    #[error("InvalidInput error: {0}")]
    InvalidInput(String),

    #[error("Custom error: {0}")]
    Custom(String),

    #[error("RwLockReadGuard error: {0}")]
    RwLockReadGuard(String),

    #[error("RwLockWriteGuard error: {0}")]
    RwLockWriteGuard(String),

    #[error("Tenant pool not found")]
    TenantPoolNotFound,
}

impl From<ValueObjectError> for RepositoryError {
    fn from(value: ValueObjectError) -> Self {
        match value {
            ValueObjectError::InvalidInput(e) => Self::InvalidInput(e.to_string()),
            _ => Self::Custom(value.to_string()),
        }
    }
}

impl From<TryFromIntError> for RepositoryError {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidInput("could not parse integer".to_string())
    }
}

impl RepositoryError {
    pub fn is_unique_violation(&self) -> bool {
        if let RepositoryError::Database(sqlxe) = self
            && let Error::Database(database_error) = sqlxe
            && database_error.is_unique_violation()
        {
            return true;
        }
        false
    }
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, Error, Serialize)]
pub enum BuilderError {
    #[error("{0} is required")]
    MissingRequired(&'static str),
}

impl IntoFriendlyError for BuilderError {
    async fn into_friendly_error<M>(self, mailer: Arc<M>) -> FriendlyError
    where
        M: MailTransporter + ConfigProvider<Cfg = AppConfig> + Send + Sync,
    {
        FriendlyError::internal_with_admin_notify(file!(), self.to_string(), mailer).await
    }
}

pub type BuilderResult<T> = Result<T, BuilderError>;
