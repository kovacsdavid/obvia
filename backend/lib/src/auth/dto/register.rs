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

use crate::common::dto::{ErrorBody, ErrorResponse};
use ::serde::Serialize;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

use crate::common::types::{Email, FirstName, LastName, Password};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RegisterRequestHelper {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Debug, Serialize)]
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

impl IntoResponse for RegisterRequestError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                reference: "AUTH/DTO/REGISTER".to_string(),
                global: "Kérjük ellenőrizze a hibás mezőket".to_string(),
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterRequest {
    pub email: Email,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub password: Password,
    pub password_confirm: String,
}

impl TryFrom<RegisterRequestHelper> for RegisterRequest {
    type Error = RegisterRequestError;
    fn try_from(value: RegisterRequestHelper) -> Result<Self, Self::Error> {
        let mut errors = RegisterRequestError {
            email: None,
            first_name: None,
            last_name: None,
            password: None,
            password_confirm: None,
        };

        let email_result = Email::try_from(value.email);
        let first_name_result = FirstName::try_from(value.first_name);
        let last_name_result = LastName::try_from(value.last_name);
        let password_result = Password::try_from(value.password);

        if let Err(e) = &email_result {
            errors.email = Some(e.to_string());
        }
        if let Err(e) = &first_name_result {
            errors.first_name = Some(e.to_string());
        }
        if let Err(e) = &last_name_result {
            errors.last_name = Some(e.to_string());
        }
        if let Err(e) = &password_result {
            errors.password = Some(e.to_string());
        }
        if let Ok(password) = &password_result {
            if password.as_str() != value.password_confirm.clone() {
                errors.password_confirm =
                    Some("A jelszó és a jelszó megerősítés mező nem egyezik".to_string());
            }
        }

        if errors.is_empty() {
            Ok(RegisterRequest {
                email: email_result.unwrap(),
                first_name: first_name_result.unwrap(),
                last_name: last_name_result.unwrap(),
                password: password_result.unwrap(),
                password_confirm: value.password_confirm,
            })
        } else {
            Err(errors)
        }
    }
}
