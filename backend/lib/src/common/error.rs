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

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::dto::{ErrorBody, ErrorResponse};

#[derive(Debug, Error, Clone)]
pub enum FriendlyError {
    #[error("{0}")]
    UserFacing(StatusCode, String, String),
    #[error("Váratlan hiba történt a feldolgozás során!")]
    Internal(String),
}

impl FriendlyError {
    pub fn trace(self, severity: tracing::Level) -> Self {
        match self.clone() {
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

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    DatabaseError(String),
}
