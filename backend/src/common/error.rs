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
use std::fmt::Display;
use thiserror::Error;

use crate::common::{
    MailTransporter,
    dto::{ErrorResponse, FormError, GeneralError},
};
use async_trait::async_trait;
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
use std::sync::Arc;
use tracing::Level;
use tracing::event;

/// An enumeration representing different types of errors that can occur.
/// This enum implements the `Debug`, `Error`, and `Clone` traits for debugging,
/// error handling, and cloning capabilities.
///
/// # Variants
///
/// * `UserFacing(StatusCode, String, String)`:
///   This variant is designed to represent errors that are intended to be displayed
///   to the user. It contains:
///   - `StatusCode`: An HTTP status code indicating the type of error.
///   - `String`: An error identifier or code.
///   - `String`: A human-readable error message.
///
/// * `Internal(String)`:
///   This variant represents internal errors that are not meant to be user-facing.
///   It contains:
///   - `String`: A description of the internal error.
///
/// # Error Message Localization
///
/// For user-facing messages, this enum is designed to provide messages
/// localized for end users' understanding. The `Internal` variant, however,
/// uses a generic Hungarian error message: "Váratlan hiba történt a feldolgozás során!"
#[derive(Debug, Error, Clone)]
pub enum FriendlyError<T>
where
    T: Serialize + Display,
{
    #[error("{0}")]
    UserFacing(StatusCode, String, T),
    #[error("Váratlan hiba történt a feldolgozás során!")]
    Internal(String, T),
}

impl<T> FriendlyError<T>
where
    T: Serialize + Display,
{
    pub fn user_facing(severity: Level, status: StatusCode, loc: &str, body: T) -> Self {
        Self::UserFacing(status, loc.to_string(), body).trace(severity)
    }
    pub fn internal(loc: &str, body: T) -> Self {
        Self::Internal(loc.to_string(), body).trace(Level::ERROR)
    }
    pub async fn internal_with_admin_notify(
        loc: &str,
        body: T,
        mailer: Arc<dyn MailTransporter>,
    ) -> Self {
        let fe = Self::Internal(loc.to_string(), body).trace(Level::ERROR);

        if let Err(e) = fe.notify_admin(mailer).await {
            event!(Level::ERROR, "Could not notify admin: {e}")
        }

        fe
    }

    /// Logs the error information associated with the current `FriendlyError` instance
    /// at the specified severity level using the `tracing` crate.
    ///
    /// Depending on the variant of `FriendlyError`, this method will emit a different
    /// set of log messages:
    ///
    /// - If the error is a `UserFacing` variant, it logs an event with the associated error
    ///   code and message.
    /// - If the error is an `Internal` variant, it logs an event with the internal error message.
    ///
    /// # Parameters
    /// - `severity`: The `Level` indicating the severity of the log entry (e.g., `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`).
    ///
    /// # Returns
    /// - `Self`: Returns the current instance of `FriendlyError` unchanged so that further
    ///   method chaining can be performed if necessary.
    ///
    /// # Behavior
    /// - For each severity level, the appropriate log entry is generated using the `event!` macro.
    ///   The severity level determines the priority of the log entry.
    ///
    /// # Note
    /// - Make sure that the `tracing` subscriber is properly initialized, otherwise the logs
    ///   emitted may not be recorded or displayed.
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
    async fn notify_admin(&self, module: Arc<dyn MailTransporter>) -> Result<(), String> {
        if let FriendlyError::Internal(body, loc) = &self {
            let handlebars = Handlebars::new();
            let email = Message::builder()
                .from(Mailbox::new(Some(module.config().mail().default_from_name().to_owned()), module.config().mail().default_from().parse().map_err(|e: AddressError| e.to_string())?))
                .to(Mailbox::new(None, module.config().mail().default_notification_email().parse().map_err(|e: AddressError| e.to_string())?))
                .subject("Unexpected error")
                .header(ContentType::TEXT_PLAIN)
                .body(handlebars.render_template("Dear Admin!\n\n Check this error!\n Internal error: location={{loc}} message={{body}}", &json!({"body": body, "loc": loc})).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;

            match module.send(email).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("".to_string())
        }
    }
}

impl<T> IntoResponse for FriendlyError<T>
where
    T: Serialize + Display,
{
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
            },
        )
        .into_response()
    }
}

#[async_trait]
pub trait IntoFriendlyError<T>
where
    T: Serialize + Display,
{
    async fn into_friendly_error(self, mailer: Arc<dyn MailTransporter>) -> FriendlyError<T>;
}

/// Represents errors that can occur while interacting with the database.
///
/// This enumeration defines a single variant:
/// - `DatabaseError`: Represents a general database error and includes a descriptive message.
///
/// # Variants
/// - `DatabaseError(String)`:
///     - Contains the error message as a `String`, describing the nature of the database error.
///
/// # Notes
/// - It is compatible with the `thiserror` crate to provide human-readable error messages via the `Display` implementation.
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

    #[error("The selected record is inactive")]
    InactiveRecord,

    #[error("RwLockReadGuard error: {0}")]
    RwLockReadGuard(String),

    #[error("RwLockWriteGuard error: {0}")]
    RwLockWriteGuard(String),

    #[error("Tenant pool not found")]
    TenantPoolNotFound,
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
    pub fn is_inactive_record(&self) -> bool {
        matches!(self, RepositoryError::InactiveRecord)
    }
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, Error, Serialize)]
pub enum BuilderError {
    #[error("{0} is required")]
    MissingRequired(&'static str),
}

#[async_trait]
impl IntoFriendlyError<BuilderError> for BuilderError {
    async fn into_friendly_error(
        self,
        mailer: Arc<dyn MailTransporter>,
    ) -> FriendlyError<BuilderError> {
        FriendlyError::internal_with_admin_notify(file!(), self, mailer).await
    }
}

pub type BuilderResult<T> = Result<T, BuilderError>;
