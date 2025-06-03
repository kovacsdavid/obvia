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

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;
use crate::common::error::DatabaseError;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Password hash error: {0}")]
    PasswordHashError(String),
    #[error("JWT error")]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error("Expiration error")]
    ExpirationError,
    #[error("User not found")]
    DatabaseError(#[from] DatabaseError),
    #[error("Invalid credentials")]
    InvalidCredentials,
}




#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Database error")]
    DatabaseError(#[from] DatabaseError),
    #[error("Failed to hash password")]
    HashingFailed,
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        match self {
            RegisterError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
            )
                .into_response(),
            RegisterError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                "User already exists",
            )
                .into_response(),
            RegisterError::HashingFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to hash password",
            )
                .into_response(),
        }
    }
}



