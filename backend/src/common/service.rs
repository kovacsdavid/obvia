/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

use std::sync::Arc;

use axum::http::StatusCode;
use serde_json::json;
use thiserror::Error;
use tracing::Level;

use crate::common::{
    error::{
        RepositoryError,
        v2::{AppError, AppErrorVisibility},
    },
    pdf::PdfGenError,
    value_object::ValueObjectError,
};
use crate::manager::{
    auth::dto::claims::{Claims, ClaimsError},
    users::model::UserModelError,
};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Nincs jogosultságod az erőforrás használatához")]
    AuthUnauthorized,

    #[error("{0}")]
    Conflict(&'static str),

    #[error("Invalid state")]
    InvalidState,

    #[error("A lista nem létezik")]
    InvalidSelectList,

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

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

    #[error("Túl sok próbálkozás történt. Próbáld újra {0} perc múlva!")]
    TooManyAttempts(i64),

    #[error("totp-required")]
    MfaRequired,

    #[error("Hibás kétlépcsős azonosító kód!")]
    MfaInvalid,

    #[error("MfaToken error: {0}")]
    MfaToken(String),

    #[error("A kétlépcsős azonosításhoz hasznát kód hibás!")]
    InvalidMfaToken,

    #[error("A kétépcsős azonosítás aktiválása korábban már megtörtént!")]
    MfaAlreadyActive,

    #[error("Hibás elfelejtett jelszó hivatkozás!")]
    InvalidForgottenPasswordToken,

    #[error("Config error: {0}")]
    Config(String),

    #[error("rng error")]
    RngError,

    #[error("ValueObjectError: {0}")]
    ValueObjectError(#[from] ValueObjectError),

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("UserModelError: {0}")]
    UserModelError(#[from] UserModelError),

    #[error("ClaimsError: {0}")]
    ClaimsError(#[from] ClaimsError),
}

type ServiceResult<T> = Result<T, ServiceError>;

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized
            | ServiceError::AuthUnauthorized
            | ServiceError::UserNotFound
            | ServiceError::UserInactive
            | ServiceError::InvalidPassword
            | ServiceError::InvalidEmailValidationToken
            | ServiceError::EmailValidationResend
            | ServiceError::TooManyAttempts(_)
            | ServiceError::MfaRequired
            | ServiceError::MfaInvalid
            | ServiceError::InvalidMfaToken
            | ServiceError::MfaAlreadyActive
            | ServiceError::InvalidForgottenPasswordToken => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            ServiceError::Conflict(_) | ServiceError::UserExists => Self::new(
                Level::DEBUG,
                StatusCode::CONFLICT,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            ServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            ServiceError::Repository(RepositoryError::Database(sqlx::Error::RowNotFound)) => {
                Self::new(
                    Level::DEBUG,
                    StatusCode::NOT_FOUND,
                    file!(),
                    AppErrorVisibility::UserFacing,
                    json!({"message": "Nem található"}),
                )
            }
            _ => Self::new(
                Level::ERROR,
                StatusCode::INTERNAL_SERVER_ERROR,
                file!(),
                AppErrorVisibility::Internal,
                json!({"message": value.to_string()}),
            ),
        }
    }
}

pub struct Service<'a, T>
where
    T: Send + Sync,
{
    claims: Option<&'a Claims>,
    module: Arc<T>,
}

impl<'a, T> Service<'a, T>
where
    T: Send + Sync,
{
    pub fn new(claims: Option<&'a Claims>, module: Arc<T>) -> Self {
        Service { claims, module }
    }
    pub fn claims(&self) -> ServiceResult<&Claims> {
        self.claims.ok_or(ServiceError::Unauthorized)
    }
    pub fn module(&self) -> &T {
        &self.module
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceError;
    use crate::common::error::{RepositoryError, v2::AppError};
    use axum::{http::StatusCode, response::IntoResponse};

    fn assert_status(error: ServiceError, expected: StatusCode) {
        let response = AppError::from(error).into_response();
        assert_eq!(response.status(), expected);
    }

    #[test]
    fn unauthorized_maps_to_401() {
        assert_status(ServiceError::Unauthorized, StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn conflict_maps_to_409() {
        assert_status(
            ServiceError::Conflict("already exists"),
            StatusCode::CONFLICT,
        );
    }

    #[test]
    fn unprocessable_maps_to_422() {
        assert_status(
            ServiceError::UnprocessableEntry("invalid payload"),
            StatusCode::UNPROCESSABLE_ENTITY,
        );
    }

    #[test]
    fn row_not_found_maps_to_404() {
        assert_status(
            ServiceError::Repository(RepositoryError::Database(sqlx::Error::RowNotFound)),
            StatusCode::NOT_FOUND,
        );
    }

    #[test]
    fn auth_errors_map_to_401() {
        assert_status(ServiceError::AuthUnauthorized, StatusCode::UNAUTHORIZED);
        assert_status(ServiceError::UserNotFound, StatusCode::UNAUTHORIZED);
        assert_status(ServiceError::TooManyAttempts(1), StatusCode::UNAUTHORIZED);
        assert_status(ServiceError::MfaAlreadyActive, StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn user_exists_maps_to_409() {
        assert_status(ServiceError::UserExists, StatusCode::CONFLICT);
    }

    #[test]
    fn unmapped_errors_remain_internal() {
        assert_status(
            ServiceError::InvalidSelectList,
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    }
}
