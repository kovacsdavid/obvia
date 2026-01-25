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

use std::fmt::{Display, Formatter};

use crate::common::error::FormErrorResponse;
use crate::manager::auth::types::Otp;
use crate::manager::users::model::User;
use crate::{common::types::ValueObject, manager::auth::dto::claims::Claims};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub otp: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct UserPublic {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: String,
    pub profile_picture_url: Option<String>,
    pub is_mfa_enabled: bool,
}

impl From<User> for UserPublic {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            status: value.status,
            profile_picture_url: value.profile_picture_url,
            is_mfa_enabled: value.is_mfa_enabled,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    claims: Claims,
    user: UserPublic,
    token: String,
}

impl LoginResponse {
    pub fn new(claims: Claims, user: UserPublic, token: String) -> Self {
        Self {
            claims,
            user,
            token,
        }
    }
    #[allow(dead_code)]
    pub fn token(&self) -> &String {
        &self.token
    }
}

#[derive(Debug, Deserialize)]
pub struct OtpUserInputHelper {
    pub otp: String,
}

#[derive(Debug, Serialize, Default)]
pub struct OtpUserInputError {
    pub otp: Option<String>,
}

impl OtpUserInputError {
    pub fn is_empty(&self) -> bool {
        self.otp.is_none()
    }
}

impl Display for OtpUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "OtpUserInputError: {}", json),
            Err(e) => write!(f, "OtpUserInputError: {}", e),
        }
    }
}

impl FormErrorResponse for OtpUserInputError {}

impl IntoResponse for OtpUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtpUserInput {
    pub otp: ValueObject<Otp>,
}

impl TryFrom<OtpUserInputHelper> for OtpUserInput {
    type Error = OtpUserInputError;
    fn try_from(value: OtpUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = OtpUserInputError::default();

        let otp = ValueObject::new(Otp(value.otp)).inspect_err(|e| {
            error.otp = Some(e.to_string());
        });

        if error.is_empty() {
            Ok(OtpUserInput {
                otp: otp.map_err(|_| OtpUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
