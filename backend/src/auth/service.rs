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

use std::sync::Arc;
use crate::users::dto::{LoginRequest, RegisterRequest};
use super::{dto::{Claims, LoginResponse, LoginUser, RegisterResponse}, error::{LoginError, RegisterError}, repository::AuthRepository, AuthModule};
use anyhow::Result;
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
#[cfg(test)]
use mockall::automock;
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
        argon2.verify_password(password.as_bytes(), &parsed_hash)
            .map(|_| true)
            .map_err(|e| e.to_string())
    }
}

pub async fn try_login(auth_module: Arc<AuthModule>, payload: LoginRequest) -> Result<LoginResponse> {
    let user = auth_module
        .repo
        .get_user_by_email(&payload.email)
        .await?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| LoginError::PasswordHashError(e.to_string()))?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| LoginError::InvalidCredentials)?;

    let now = Utc::now().timestamp() as usize;
    let exp = (
        Utc::now()
        + Duration::minutes(
            auth_module
                .config
                .auth()
                .jwt_expiration_mins() as i64
        )
    ).timestamp() as usize;
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
    .map_err(LoginError::JWTError)?;

    let login_user = LoginUser {
        id: user.id.to_string(),
        email: user.email.clone(),
    };

    Ok(LoginResponse::new(login_user, token))
}

pub async fn try_register(
    repo: Arc<dyn AuthRepository>,
    pasword_hasher: Arc<dyn AuthPasswordHasher>,
    payload: RegisterRequest
) -> Result<RegisterResponse> {
    let password_hash = pasword_hasher
        .hash_password(&payload.password)
        .map_err(|_| RegisterError::HashingFailed)?
        .to_string();

    repo.insert_user(&payload, &password_hash).await.map_err(|e| {
        if e.to_string().contains("duplicate key value violates unique constraint") {
            RegisterError::UserAlreadyExists
        } else {
            RegisterError::DatabaseError(e)
        }
    })?;

    Ok(RegisterResponse {
        message: "A felhasználó sikerese létrehozva".to_string()
    })
}
