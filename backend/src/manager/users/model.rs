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

use serde::Serialize;
use sqlx::prelude::FromRow;
use thiserror::Error;
use totp_rs::{Algorithm, Secret, TOTP};
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserModelError {
    #[error("GetMfaToken error: {0}")]
    MfaToken(String),

    #[error("Invalid MFA token")]
    InvalidMfaToken,
}

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub status: String,
    pub last_login_at: Option<chrono::DateTime<chrono::Local>>,
    pub profile_picture_url: Option<String>,
    pub locale: Option<String>,
    pub invited_by: Option<Uuid>,
    pub email_verified_at: Option<chrono::DateTime<chrono::Local>>,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
    pub deleted_at: Option<chrono::DateTime<chrono::Local>>,
    pub is_mfa_enabled: bool,
    pub mfa_secret: Option<String>,
}

impl User {
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }
    pub fn need_email_verification(&self) -> bool {
        self.status == "unchecked_email"
    }
    pub fn is_mfa_enabled(&self) -> bool {
        self.is_mfa_enabled
    }
    pub fn init_mfa_secret(mut self) -> Self {
        self.mfa_secret = Some(Secret::default().to_encoded().to_string());
        self
    }
    pub fn get_mfa_token(&self) -> Result<String, UserModelError> {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            Secret::Encoded(
                self.mfa_secret
                    .clone()
                    .ok_or_else(|| UserModelError::MfaToken("missing mfa_secret".to_string()))?,
            )
            .to_bytes()
            .map_err(|e| UserModelError::MfaToken(e.to_string()))?,
            Some("obvia".to_string()),
            self.email.clone(),
        )
        .map_err(|e| UserModelError::MfaToken(e.to_string()))?;
        totp.generate_current()
            .map_err(|e| UserModelError::MfaToken(e.to_string()))
    }
    pub fn check_mfa_token(&self, token_to_test: &str) -> Result<(), UserModelError> {
        match self.get_mfa_token() {
            Ok(current_mfa_token) => {
                if current_mfa_token == token_to_test {
                    Ok(())
                } else {
                    Err(UserModelError::InvalidMfaToken)
                }
            }
            Err(_) => Err(UserModelError::InvalidMfaToken),
        }
    }
}
