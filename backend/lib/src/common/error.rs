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
use thiserror::Error;

use crate::manager::common::dto::{ErrorBody, ErrorResponse};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::migrate::MigrateError;

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
pub enum FriendlyError {
    #[error("{0}")]
    UserFacing(StatusCode, String, String),
    #[error("Váratlan hiba történt a feldolgozás során!")]
    Internal(String),
}

impl FriendlyError {
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
    /// - `severity`: The `tracing::Level` indicating the severity of the log entry (e.g., `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`).
    ///
    /// # Returns
    /// - `Self`: Returns the current instance of `FriendlyError` unchanged so that further
    ///   method chaining can be performed if necessary.
    ///
    /// # Behavior
    /// - For each severity level, the appropriate log entry is generated using the `tracing::event!` macro.
    ///   The severity level determines the priority of the log entry.
    ///
    /// # Note
    /// - Make sure that the `tracing` subscriber is properly initialized, otherwise the logs
    ///   emitted may not be recorded or displayed.
    pub fn trace(self, severity: tracing::Level) -> Self {
        match &self {
            FriendlyError::UserFacing(_, code, msg) => match severity {
                tracing::Level::ERROR => {
                    tracing::event!(
                        tracing::Level::ERROR,
                        "User-facing error: code={}, message={}",
                        code,
                        msg
                    );
                }
                tracing::Level::WARN => {
                    tracing::event!(
                        tracing::Level::WARN,
                        "User-facing error: code={}, message={}",
                        code,
                        msg
                    );
                }
                tracing::Level::INFO => {
                    tracing::event!(
                        tracing::Level::INFO,
                        "User-facing error: code={}, message={}",
                        code,
                        msg
                    );
                }
                tracing::Level::DEBUG => {
                    tracing::event!(
                        tracing::Level::DEBUG,
                        "User-facing error: code={}, message={}",
                        code,
                        msg
                    );
                }
                tracing::Level::TRACE => {
                    tracing::event!(
                        tracing::Level::TRACE,
                        "User-facing error: code={}, message={}",
                        code,
                        msg
                    );
                }
            },
            FriendlyError::Internal(msg) => match severity {
                tracing::Level::ERROR => {
                    tracing::event!(tracing::Level::ERROR, "Internal error: message={}", msg);
                }
                tracing::Level::WARN => {
                    tracing::event!(tracing::Level::WARN, "Internal error: message={}", msg);
                }
                tracing::Level::INFO => {
                    tracing::event!(tracing::Level::INFO, "Internal error: message={}", msg);
                }
                tracing::Level::DEBUG => {
                    tracing::event!(tracing::Level::DEBUG, "Internal error: message={}", msg);
                }
                tracing::Level::TRACE => {
                    tracing::event!(tracing::Level::TRACE, "Internal error: message={}", msg);
                }
            },
        }
        self
    }
}

impl IntoResponse for FriendlyError {
    /// Converts a `FriendlyError` instance into an HTTP response.
    ///
    /// This method translates an application-level error represented by the
    /// `FriendlyError` enum into an HTTP response that can be sent back to the client.
    /// It supports two types of errors:
    /// - `UserFacing`: Represents errors intended for the client with a specific status code,
    ///   error code, and descriptive message.
    /// - `Internal`: Represents unexpected internal server errors, which are always
    ///   translated into a generic message for the client
    ///
    /// # Variants
    ///
    /// * `FriendlyError::UserFacing`:
    ///     - `status`: The HTTP `StatusCode` to be returned.
    ///     - `code`: Application-specific error code.
    ///     - `message`: A friendly error message intended for the end-user.
    ///
    /// * `FriendlyError::Internal`:
    ///     - Always returns `StatusCode::INTERNAL_SERVER_ERROR`.
    ///     - Uses a default application-specific error code (`"INTERNAL"`).
    ///     - Sends a generic error message to the client: "Váratlan hiba történt a feldolgozás során!".
    ///
    /// # Response Body
    ///
    /// The response body is serialized as a JSON object following the `ErrorResponse` structure,
    /// with the following fields:
    /// - `reference`: Contains the internal error code (a string identifying the error).
    /// - `global`: Contains the error message (may be user-facing or generic, depending on the error variant).
    /// - `fields`: Always `None` in this implementation, reserved for future use to report field-specific issues.
    ///
    /// # Returns
    ///
    /// An `axum::response::Response` object containing the HTTP status code and a JSON payload in
    /// the following structure:
    ///
    /// ```json
    /// {
    ///   "reference": "<error code>",
    ///   "global": "<error message>",
    ///   "fields": null
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - The function ensures that sensitive information about internal server errors is not exposed
    ///   to the client.
    /// - It uses the `ErrorResponse` and `ErrorBody` structures to ensure consistent error formatting.
    /// - If you want to propagate the fields use a specific error struct
    ///   and implement axum `IntoResponse` on it (ex.: crate::tenants::dto::TenantCreateRequestError)
    fn into_response(self) -> Response {
        let msg_for_internal = "Váratlan hiba történt a feldolgozás során!".to_string();
        let (status, code, message) = match self {
            FriendlyError::UserFacing(status, code, msg) => (status, code, msg),
            FriendlyError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL".to_string(),
                msg_for_internal,
            ),
        };
        let body = ErrorResponse::<String>::new(ErrorBody {
            reference: code,
            global: message,
            fields: None,
        });

        (status, Json(body)).into_response()
    }
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

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("ValueObject error: {0}")]
    ValueObject(String),

    #[error("Custom error: {0}")]
    Custom(String),

    #[error("RwLockReadGuard error: {0}")]
    RwLockReadGuard(String),

    #[error("RwLockWriteGuard error: {0}")]
    RwLockWriteGuard(String),
}
