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

use super::{
    AuthModule,
    dto::{claims::Claims, login::LoginResponse, login::LoginUser, register::RegisterResponse},
    repository::AuthRepository,
};
use crate::{
    auth::dto::{login::LoginRequest, register::RegisterRequest},
    common::{dto::OkResponse, error::FriendlyError},
};
use anyhow::Result;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
#[cfg(test)]
use mockall::automock;
use std::sync::Arc;
use uuid::Uuid;

#[cfg_attr(test, automock)]
pub trait AuthPasswordHasher: Send + Sync + 'static {
    fn hash_password(&self, password: &str) -> Result<String, String>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String>;
}

pub struct Argon2Hasher;

impl AuthPasswordHasher for Argon2Hasher {
    fn hash_password(&self, password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| e.to_string())?)
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;
        let argon2 = Argon2::default();
        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map(|_| true)
            .map_err(|e| e.to_string())
    }
}

pub async fn try_login(
    auth_module: Arc<AuthModule>,
    payload: LoginRequest,
) -> Result<OkResponse<LoginResponse>, FriendlyError> {
    let user = auth_module
        .repo
        .get_user_by_email(&payload.email)
        .await
        .map_err(|_| {
            FriendlyError::UserFacing(
                StatusCode::UNAUTHORIZED,
                "AUTH/SERVICE/UNAUTHORIZED".to_string(),
                "Hibás e-mail cím vagy jelszó".to_string(),
            )
            .trace(tracing::Level::DEBUG)
        })?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| FriendlyError::Internal(e.to_string()).trace(tracing::Level::ERROR))?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| {
            FriendlyError::UserFacing(
                StatusCode::UNAUTHORIZED,
                "AUTH/SERVICE/UNAUTHORIZED".to_string(),
                "Hibás e-mail cím vagy jelszó".to_string(),
            )
            .trace(tracing::Level::DEBUG)
        })?;

    let now = Utc::now().timestamp() as usize;
    let exp = (Utc::now()
        + Duration::minutes(auth_module.config.auth().jwt_expiration_mins() as i64))
    .timestamp() as usize;
    let nbf = now;

    let claims = Claims::new(
        user.id.to_string(),
        exp,
        now,
        nbf,
        auth_module.config.auth().jwt_issuer().to_string(),
        auth_module.config.auth().jwt_audience().to_string(),
        Uuid::new_v4().to_string(),
    );

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(auth_module.config.auth().jwt_secret().as_bytes()),
    )
    .map_err(|e| FriendlyError::Internal(e.to_string()).trace(tracing::Level::ERROR))?;

    let login_user = LoginUser {
        id: user.id.to_string(),
        email: user.email.clone(),
    };

    Ok(OkResponse::new(LoginResponse::new(login_user, token)))
}

pub async fn try_register(
    repo: Arc<dyn AuthRepository>,
    pasword_hasher: Arc<dyn AuthPasswordHasher>,
    payload: RegisterRequest,
) -> Result<OkResponse<RegisterResponse>, FriendlyError> {
    let password_hash = pasword_hasher
        .hash_password(&payload.password.as_str())
        .map_err(|e| FriendlyError::Internal(e.to_string()).trace(tracing::Level::ERROR))?
        .to_string();

    repo.insert_user(&payload, &password_hash)
        .await
        .map_err(|e| {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                FriendlyError::UserFacing(
                    StatusCode::CONFLICT,
                    "AUTH/SERVICE/ALREADYEXISTS".to_string(),
                    "Ez az e-mail cím már foglalt".to_string(),
                )
                .trace(tracing::Level::DEBUG)
            } else {
                FriendlyError::Internal(e.to_string()).trace(tracing::Level::ERROR)
            }
        })?;

    Ok(OkResponse::new(RegisterResponse {
        message: "A felhasználó sikeresen létrehozva".to_string(),
    }))
}
