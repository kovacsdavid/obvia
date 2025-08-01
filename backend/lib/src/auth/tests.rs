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

use axum::{Json, http::StatusCode};
use chrono::Utc;
use mockall::predicate::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::app::database::MockPgPoolManagerTrait;
use crate::auth::dto::register::RegisterRequestHelper;
use crate::auth::handler::{login_inner, register_inner};
use crate::auth::repository::AuthRepository;
use crate::{
    app::config::AppConfig,
    auth::{
        AuthModule,
        dto::{login::LoginRequest, register::RegisterRequest},
        repository::MockAuthRepository,
        service::{Argon2Hasher, MockAuthPasswordHasher},
    },
    common::error::DatabaseError,
    users::model::User,
};

#[tokio::test]
async fn test_login_success() {
    let auth_module = AuthModule {
        pool_manager: Arc::new(MockPgPoolManagerTrait::new()),
        password_hasher: Arc::new(Argon2Hasher),
        config: Arc::new(AppConfig::default()),
    };
    let request = LoginRequest {
        email: "testuser@example.com".to_string(),
        password: "correctpassword".to_string(),
    };

    let repo_factory = || async {
        let mut repo = MockAuthRepository::new();
        repo.expect_get_user_by_email()
            .with(eq("testuser@example.com"))
            .returning(|_| Ok(User {
                id: Uuid::new_v4(),
                email: "testuser@example.com".to_string(),
                password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                first_name: Some("Test".to_string()),
                last_name: Some("User".to_string()),
                phone: Some("+123456789".to_string()),
                status: "active".to_string(),
                last_login_at: Some(Utc::now()),
                profile_picture_url: None,
                locale: Some("hu-HU".to_string()),
                invited_by: None,
                email_verified_at: Some(Utc::now()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            }));
        Box::new(repo) as Box<dyn AuthRepository + Send + Sync>
    };

    let response = login_inner(Arc::new(auth_module), request, repo_factory).await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_login_success_return_jwt() {
    let auth_module = AuthModule {
        pool_manager: Arc::new(MockPgPoolManagerTrait::new()),
        password_hasher: Arc::new(Argon2Hasher),
        config: Arc::new(AppConfig::default()),
    };
    let request = LoginRequest {
        email: "testuser@example.com".to_string(),
        password: "correctpassword".to_string(),
    };

    let repo_factory = || async {
        let mut repo = MockAuthRepository::new();
        repo.expect_get_user_by_email()
            .with(eq("testuser@example.com"))
            .returning(|_| Ok(User {
                id: Uuid::new_v4(),
                email: "testuser@example.com".to_string(),
                password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                first_name: Some("Test".to_string()),
                last_name: Some("User".to_string()),
                phone: Some("+123456789".to_string()),
                status: "active".to_string(),
                last_login_at: Some(Utc::now()),
                profile_picture_url: None,
                locale: Some("hu-HU".to_string()),
                invited_by: None,
                email_verified_at: Some(Utc::now()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            }));
        Box::new(repo) as Box<dyn AuthRepository + Send + Sync>
    };

    let response = login_inner(Arc::new(auth_module), request, repo_factory).await;

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    let data = body.get("data").unwrap();

    assert!(
        data.get("token").is_some(),
        "Response should contain a token"
    );
    assert!(
        data["token"].as_str().unwrap().len() > 10,
        "Token should be nontrivial"
    );
}

#[tokio::test]
async fn test_login_failure() {
    let auth_module = AuthModule {
        pool_manager: Arc::new(MockPgPoolManagerTrait::new()),
        password_hasher: Arc::new(Argon2Hasher),
        config: Arc::new(AppConfig::default()),
    };
    let request = LoginRequest {
        email: "testuser@example.com".to_string(),
        password: "incorrectpassword".to_string(),
    };

    let repo_factory = || async {
        let mut repo = MockAuthRepository::new();
        repo.expect_get_user_by_email()
            .with(eq("testuser@example.com"))
            .returning(|_| Ok(User {
                id: Uuid::new_v4(),
                email: "testuser@example.com".to_string(),
                password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                first_name: Some("Test".to_string()),
                last_name: Some("User".to_string()),
                phone: Some("+123456789".to_string()),
                status: "active".to_string(),
                last_login_at: Some(Utc::now()),
                profile_picture_url: None,
                locale: Some("hu-HU".to_string()),
                invited_by: None,
                email_verified_at: Some(Utc::now()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            }));
        Box::new(repo) as Box<dyn AuthRepository + Send + Sync>
    };

    let response = login_inner(Arc::new(auth_module), request, repo_factory).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// ===== REGISTER =====

#[tokio::test]
async fn test_register_success() {
    let register_request_helper = RegisterRequestHelper {
        email: "testuser@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        password: "Password1!".to_string(),
        password_confirm: "Password1!".to_string(),
    };
    let mut password_hasher = MockAuthPasswordHasher::new();
    password_hasher
        .expect_hash_password()
        .with(eq("Password1!"))
        .returning(|_| Ok("hashed_password".to_string()));

    let auth_module = Arc::new(AuthModule {
        pool_manager: Arc::new(MockPgPoolManagerTrait::new()),
        password_hasher: Arc::new(password_hasher),
        config: Arc::new(AppConfig::default()),
    });
    let repo_factory = || async {
        let register_request_helper = register_request_helper.clone();
        let register_request = RegisterRequest::try_from(register_request_helper.clone()).unwrap();
        let mut repo = MockAuthRepository::new();
        repo.expect_insert_user()
            .with(eq(register_request.clone()), eq("hashed_password"))
            .returning(|_, _| Ok(()));
        Box::new(repo) as Box<dyn AuthRepository + Send + Sync>
    };

    let response = register_inner(
        auth_module.clone(),
        Ok(Json(register_request_helper.clone())),
        repo_factory,
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_register_user_already_exists() {
    let register_request_helper = RegisterRequestHelper {
        email: "testuser@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        password: "Password1!".to_string(),
        password_confirm: "Password1!".to_string(),
    };

    let mut password_hasher = MockAuthPasswordHasher::new();
    password_hasher
        .expect_hash_password()
        .with(eq("Password1!"))
        .returning(|_| Ok("hashed_password".to_string()));

    let auth_module = Arc::new(AuthModule {
        pool_manager: Arc::new(MockPgPoolManagerTrait::new()),
        password_hasher: Arc::new(password_hasher),
        config: Arc::new(AppConfig::default()),
    });
    let repo_factory = || async {
        let register_request_helper = register_request_helper.clone();
        let register_request = RegisterRequest::try_from(register_request_helper.clone()).unwrap();
        let mut repo = MockAuthRepository::new();
        repo.expect_insert_user()
            .with(eq(register_request.clone()), eq("hashed_password"))
            .returning(|_, _| {
                Err(DatabaseError::DatabaseError(
                    "duplicate key value violates unique constraint".to_string(),
                ))
            });
        Box::new(repo) as Box<dyn AuthRepository + Send + Sync>
    };

    let response = register_inner(
        auth_module.clone(),
        Ok(Json(register_request_helper.clone())),
        repo_factory,
    )
    .await;
    assert_eq!(response.status(), StatusCode::CONFLICT);
}
