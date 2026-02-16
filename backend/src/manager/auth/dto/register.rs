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

use crate::common::error::FormErrorResponse;
use crate::common::types::ValueObject;
use crate::common::types::{Email, FirstName, LastName, Password};
use ::serde::Serialize;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct RegisterRequestHelper {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Debug, Serialize, Default)]
pub struct RegisterRequestError {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
    pub password_confirm: Option<String>,
}
impl RegisterRequestError {
    pub fn is_empty(&self) -> bool {
        self.email.is_none()
            && self.first_name.is_none()
            && self.last_name.is_none()
            && self.password.is_none()
            && self.password_confirm.is_none()
    }
}

impl Display for RegisterRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "RegisterRequestError: {}", json),
            Err(e) => write!(f, "RegisterRequestError: {}", e),
        }
    }
}

impl FormErrorResponse for RegisterRequestError {}

impl IntoResponse for RegisterRequestError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct RegisterRequest {
    pub email: ValueObject<Email>,
    pub first_name: ValueObject<FirstName>,
    pub last_name: ValueObject<LastName>,
    pub password: ValueObject<Password>,
}

impl TryFrom<RegisterRequestHelper> for RegisterRequest {
    type Error = RegisterRequestError;
    fn try_from(value: RegisterRequestHelper) -> Result<Self, Self::Error> {
        let mut error = RegisterRequestError::default();

        let email_result = ValueObject::new(Email(value.email)).inspect_err(|e| {
            error.email = Some(e.to_string());
        });
        let first_name_result = ValueObject::new(FirstName(value.first_name)).inspect_err(|e| {
            error.first_name = Some(e.to_string());
        });
        let last_name_result = ValueObject::new(LastName(value.last_name)).inspect_err(|e| {
            error.last_name = Some(e.to_string());
        });
        let password_result = ValueObject::new(Password(value.password)).inspect_err(|e| {
            error.password = Some(e.to_string());
        });

        if let Ok(password) = &password_result
            && password.as_str() != value.password_confirm
        {
            error.password_confirm =
                Some("A jelszó és a jelszó megerősítés mező nem egyezik".to_string());
        }

        if error.is_empty() {
            Ok(RegisterRequest {
                email: email_result.map_err(|_| RegisterRequestError::default())?,
                first_name: first_name_result.map_err(|_| RegisterRequestError::default())?,
                last_name: last_name_result.map_err(|_| RegisterRequestError::default())?,
                password: password_result.map_err(|_| RegisterRequestError::default())?,
            })
        } else {
            Err(error)
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ResendEmailValidationRequestHelper {
    pub email: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ResendEmailValidationError {
    pub email: Option<String>,
}

impl ResendEmailValidationError {
    pub fn is_empty(&self) -> bool {
        self.email.is_none()
    }
}

impl Display for ResendEmailValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "ResendEmailValidationError: {}", json),
            Err(e) => write!(f, "ResendEmailValidationError: {}", e),
        }
    }
}

impl FormErrorResponse for ResendEmailValidationError {}

impl IntoResponse for ResendEmailValidationError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ResendEmailValidationRequest {
    pub email: ValueObject<Email>,
}

impl TryFrom<ResendEmailValidationRequestHelper> for ResendEmailValidationRequest {
    type Error = ResendEmailValidationError;

    fn try_from(value: ResendEmailValidationRequestHelper) -> Result<Self, Self::Error> {
        let mut error = ResendEmailValidationError::default();

        let email_result = ValueObject::new(Email(value.email)).inspect_err(|e| {
            error.email = Some(e.to_string());
        });

        if error.is_empty() {
            Ok(ResendEmailValidationRequest {
                email: email_result.map_err(|_| ResendEmailValidationError::default())?,
            })
        } else {
            Err(error)
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ForgottenPasswordRequestHelper {
    pub email: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ForgottenPasswordRequestError {
    pub email: Option<String>,
}

impl ForgottenPasswordRequestError {
    pub fn is_empty(&self) -> bool {
        self.email.is_none()
    }
}

impl Display for ForgottenPasswordRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "ForgottenPasswordRequestError: {}", json),
            Err(e) => write!(f, "ForgottenPasswordRequestError: {}", e),
        }
    }
}

impl FormErrorResponse for ForgottenPasswordRequestError {}

impl IntoResponse for ForgottenPasswordRequestError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ForgottenPasswordRequest {
    pub email: ValueObject<Email>,
}

impl TryFrom<ForgottenPasswordRequestHelper> for ForgottenPasswordRequest {
    type Error = ForgottenPasswordRequestError;

    fn try_from(value: ForgottenPasswordRequestHelper) -> Result<Self, Self::Error> {
        let mut error = ForgottenPasswordRequestError::default();

        let email_result = ValueObject::new(Email(value.email)).inspect_err(|e| {
            error.email = Some(e.to_string());
        });

        if error.is_empty() {
            Ok(ForgottenPasswordRequest {
                email: email_result.map_err(|_| ForgottenPasswordRequestError::default())?,
            })
        } else {
            Err(error)
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct NewPasswordRequestHelper {
    pub token: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Debug, Serialize, Default)]
pub struct NewPasswordRequestError {
    pub token: Option<String>,
    pub password: Option<String>,
    pub password_confirm: Option<String>,
}

impl NewPasswordRequestError {
    pub fn is_empty(&self) -> bool {
        self.token.is_none() && self.password.is_none() && self.password_confirm.is_none()
    }
}

impl Display for NewPasswordRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "NewPasswordRequestError: {}", json),
            Err(e) => write!(f, "NewPasswordRequestError: {}", e),
        }
    }
}

impl FormErrorResponse for NewPasswordRequestError {}

impl IntoResponse for NewPasswordRequestError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct NewPasswordRequest {
    pub token: Uuid,
    pub password: ValueObject<Password>,
}

impl TryFrom<NewPasswordRequestHelper> for NewPasswordRequest {
    type Error = NewPasswordRequestError;

    fn try_from(value: NewPasswordRequestHelper) -> Result<Self, Self::Error> {
        let mut error = NewPasswordRequestError::default();

        let token_result = Uuid::parse_str(&value.token).inspect_err(|e| {
            error.token = Some("Hibás azonosító".to_string());
        });

        let password_result = ValueObject::new(Password(value.password)).inspect_err(|e| {
            error.password = Some(e.to_string());
        });

        if let Ok(password) = &password_result
            && password.as_str() != value.password_confirm
        {
            error.password_confirm =
                Some("A jelszó és a jelszó megerősítés mező nem egyezik".to_string());
        }

        if error.is_empty() {
            Ok(NewPasswordRequest {
                token: token_result.map_err(|_| NewPasswordRequestError::default())?,
                password: password_result.map_err(|_| NewPasswordRequestError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
