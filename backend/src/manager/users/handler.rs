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

use super::UsersModuleInterface;
use super::service::UserService;
use crate::common::dto::{EmptyType, SimpleMessageResponse, SuccessResponseBuilder};
use crate::common::extractors::{ClientContext, UserInput};
use crate::common::handler::{ErrorMapper, ErrorMapperInterface, HandlerResult};
use crate::common::service::Service;
use crate::manager::auth::dto::login::{OtpUserInput, OtpUserInputHelper};
use crate::manager::auth::middleware::AuthenticatedUser;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

pub async fn get_claims<M: UsersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(users_module): State<Arc<M>>,
) -> HandlerResult {
    let error_mapper = ErrorMapper::new(users_module);
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(claims)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn otp_enable<M: UsersModuleInterface>(
    State(users_module): State<Arc<M>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> HandlerResult {
    let service = Service::new(Some(&claims), users_module.clone());
    let error_mapper = ErrorMapper::new(users_module);
    let response = error_mapper
        .or_handler_error(service.otp_enable(&client_context).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(response)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn otp_verify<M: UsersModuleInterface>(
    State(users_module): State<Arc<M>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
    UserInput(user_input, _): UserInput<OtpUserInput, OtpUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), users_module.clone());
    let error_mapper = ErrorMapper::new(users_module);
    error_mapper
        .or_handler_error(service.otp_verify(&user_input, &client_context).await)
        .await?;

    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A kétlépcsős azonosítás aktiválása megtörtént!",
                ))
                .build(),
        )
        .await?
        .into_response())
}

pub async fn otp_disable<M: UsersModuleInterface>(
    State(users_module): State<Arc<M>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
    UserInput(user_input, _): UserInput<OtpUserInput, OtpUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), users_module.clone());
    let error_mapper = ErrorMapper::new(users_module);
    error_mapper
        .or_handler_error(service.otp_disable(&user_input, &client_context).await)
        .await?;

    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A kétlépcsős azonosítás kikapcsolása megtörtént!",
                ))
                .build(),
        )
        .await?
        .into_response())
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;
    use crate::common::config::tests::AppConfigBuilder;
    use crate::common::handler::tests::{generate_expired_jwt, generate_valid_jwt};
    use crate::manager::auth::model::{AccountEventLogEntry, AccountEventStatus, AccountEventType};
    use crate::manager::auth::repository::MockAuthRepository;
    use crate::manager::users::{
        self, model::User, repository::MockUsersRepository, tests::MockUsersModule,
    };
    use axum::body::Body;
    use axum::{Router, http::Request};
    use chrono::Utc;
    use mockall::predicate::eq;
    use serde_json::json;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_otp_enable_success() {
        let sub = Uuid::new_v4();
        let active_tenant_id = Uuid::new_v4();

        let mut repo = MockUsersRepository::new();
        repo.expect_get_user_by_id()
            .times(1)
            .with(eq(sub))
            .returning(|sub| {
                Ok(User {
                    id: sub,
                    email: "testuser@example.com".to_string(),
                    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    phone: Some("+123456789".to_string()),
                    status: "active".to_string(),
                    last_login_at: Some(Utc::now()),
                    profile_picture_url: None,
                    locale: "hu-HU".to_string(),
                    invited_by: None,
                    email_verified_at: Some(Utc::now()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    is_mfa_enabled: false,
                    mfa_secret: None,
                    timezone: "Europe/Budapest".to_string(),
                })
            });
        repo.expect_update_user()
            .times(1)
            .withf(|user| {
                if let Some(mfa_secret) = &user.mfa_secret
                    && mfa_secret.len() == 32
                {
                    true
                } else {
                    false
                }
            })
            .returning(Ok);

        let mut app_state = MockUsersModule::new();
        let users_repo = Arc::new(repo);
        let auth_repo = Arc::new(MockAuthRepository::new());
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_auth_repo()
            .times(1)
            .returning(move || auth_repo.clone());
        app_state
            .expect_users_repo()
            .times(1)
            .returning(move || users_repo.clone());
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(sub), Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/users/otp/enable")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_otp_enable_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();
        let mut app_state = MockUsersModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt(Some(active_tenant_id))),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/users/otp/enable")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn test_otp_enable_unauthorized_missing() {
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/users/otp/enable")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(MockUsersModule::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_otp_verify_success() {
        let sub = Uuid::new_v4();
        let user_email = "testuser@example.com".to_string();
        let active_tenant_id = Uuid::new_v4();
        let mut users_repo = MockUsersRepository::new();

        let user = User {
                    id: sub,
                    email: user_email.clone(),
                    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    phone: Some("+123456789".to_string()),
                    status: "active".to_string(),
                    last_login_at: Some(Utc::now()),
                    profile_picture_url: None,
                    locale: "hu-HU".to_string(),
                    invited_by: None,
                    email_verified_at: Some(Utc::now()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    is_mfa_enabled: false,
                    mfa_secret: None,
                    timezone: "Europe/Budapest".to_string(),
                };

        assert!(!user.is_mfa_enabled());
        let user = user.init_mfa_secret();

        users_repo
            .expect_get_user_by_id()
            .times(1)
            .with(eq(sub))
            .returning({
                let user_clone = user.clone();
                move |_| Ok(user_clone.clone())
            });
        users_repo
            .expect_update_user()
            .times(1)
            .withf(|user| {
                if let Some(mfa_secret) = &user.mfa_secret
                    && mfa_secret.len() == 32
                    && user.is_mfa_enabled()
                {
                    true
                } else {
                    false
                }
            })
            .returning(Ok);

        let mut auth_repo = MockAuthRepository::new();
        auth_repo
            .expect_insert_account_event_log()
            .times(1)
            .with(
                eq(Some(sub)),
                eq(Some(user_email)),
                eq(AccountEventType::MfaEnable),
                eq(AccountEventStatus::Success),
                eq(Some("127.0.0.1".parse().unwrap())),
                eq(None),
                eq(None),
            )
            .returning(
                |user_id, identifier, event_type, status, ip_address, user_agent, metadata| {
                    let ip_address = Some(ipnetwork::IpNetwork::from(ip_address.unwrap()));
                    Ok(AccountEventLogEntry {
                        id: Uuid::new_v4(),
                        user_id,
                        identifier,
                        event_type,
                        status,
                        ip_address,
                        user_agent,
                        metadata,
                        created_at: Utc::now(),
                    })
                },
            );

        let mut app_state = MockUsersModule::new();
        let users_repo = Arc::new(users_repo);
        let auth_repo = Arc::new(auth_repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_auth_repo()
            .times(1)
            .returning(move || auth_repo.clone());
        app_state
            .expect_users_repo()
            .times(1)
            .returning(move || users_repo.clone());
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let payload = serde_json::to_string(&OtpUserInputHelper {
            otp: user.get_mfa_token().unwrap(),
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(sub), Some(active_tenant_id))
                ),
            )
            .method("POST")
            .uri("/api/users/otp/verify")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_otp_verify_invalid_user_input() {
        let sub = Uuid::new_v4();
        let user_email = "testuser@example.com".to_string();
        let active_tenant_id = Uuid::new_v4();
        let mut users_repo = MockUsersRepository::new();

        let user = User {
                    id: sub,
                    email: user_email.clone(),
                    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    phone: Some("+123456789".to_string()),
                    status: "active".to_string(),
                    last_login_at: Some(Utc::now()),
                    profile_picture_url: None,
                    locale: "hu-HU".to_string(),
                    invited_by: None,
                    email_verified_at: Some(Utc::now()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    is_mfa_enabled: false,
                    mfa_secret: None,
                    timezone: "Europe/Budapest".to_string(),
                };

        assert!(!user.is_mfa_enabled());
        let user = user.init_mfa_secret();

        users_repo
            .expect_get_user_by_id()
            .times(1)
            .with(eq(sub))
            .returning({
                let user_clone = user.clone();
                move |_| Ok(user_clone.clone())
            });
        users_repo.expect_update_user().times(0);

        let mut auth_repo = MockAuthRepository::new();
        auth_repo
            .expect_insert_account_event_log()
            .times(1)
            .with(
                eq(Some(sub)),
                eq(Some(user_email)),
                eq(AccountEventType::MfaEnable),
                eq(AccountEventStatus::Error),
                eq(Some("127.0.0.1".parse().unwrap())),
                eq(None),
                eq(Some(json!({
                    "error": "A kétlépcsős azonosításhoz hasznát kód hibás!".to_string()
                }))),
            )
            .returning(
                |user_id, identifier, event_type, status, ip_address, user_agent, metadata| {
                    let ip_address = Some(ipnetwork::IpNetwork::from(ip_address.unwrap()));
                    Ok(AccountEventLogEntry {
                        id: Uuid::new_v4(),
                        user_id,
                        identifier,
                        event_type,
                        status,
                        ip_address,
                        user_agent,
                        metadata,
                        created_at: Utc::now(),
                    })
                },
            );

        let mut app_state = MockUsersModule::new();
        let users_repo = Arc::new(users_repo);
        let auth_repo = Arc::new(auth_repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_auth_repo()
            .times(1)
            .returning(move || auth_repo.clone());
        app_state
            .expect_users_repo()
            .times(1)
            .returning(move || users_repo.clone());
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let otp = if user.get_mfa_token().unwrap() == "111111" {
            "222222"
        } else {
            "111111"
        }
        .to_string();

        let payload = serde_json::to_string(&OtpUserInputHelper { otp }).unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(sub), Some(active_tenant_id))
                ),
            )
            .method("POST")
            .uri("/api/users/otp/verify")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_otp_verify_unauthorized_expired() {
        let sub = Uuid::new_v4();
        let user_email = "testuser@example.com".to_string();
        let active_tenant_id = Uuid::new_v4();

        let user = User {
                    id: sub,
                    email: user_email.clone(),
                    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    phone: Some("+123456789".to_string()),
                    status: "active".to_string(),
                    last_login_at: Some(Utc::now()),
                    profile_picture_url: None,
                    locale: "hu-HU".to_string(),
                    invited_by: None,
                    email_verified_at: Some(Utc::now()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    is_mfa_enabled: false,
                    mfa_secret: None,
                    timezone: "Europe/Budapest".to_string(),
                };

        assert!(!user.is_mfa_enabled());
        let user = user.init_mfa_secret();

        let mut app_state = MockUsersModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let payload = serde_json::to_string(&OtpUserInputHelper {
            otp: user.get_mfa_token().unwrap(),
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt(Some(active_tenant_id))),
            )
            .method("POST")
            .uri("/api/users/otp/verify")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_otp_verify_unauthorized_missing() {
        let sub = Uuid::new_v4();
        let user_email = "testuser@example.com".to_string();

        let user = User {
                    id: sub,
                    email: user_email.clone(),
                    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    phone: Some("+123456789".to_string()),
                    status: "active".to_string(),
                    last_login_at: Some(Utc::now()),
                    profile_picture_url: None,
                    locale: "hu-HU".to_string(),
                    invited_by: None,
                    email_verified_at: Some(Utc::now()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    is_mfa_enabled: false,
                    mfa_secret: None,
                    timezone: "Europe/Budapest".to_string(),
                };

        assert!(!user.is_mfa_enabled());
        let user = user.init_mfa_secret();

        let app_state = MockUsersModule::new();

        let payload = serde_json::to_string(&OtpUserInputHelper {
            otp: user.get_mfa_token().unwrap(),
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/users/otp/verify")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_otp_disable_success() {
        let sub = Uuid::new_v4();
        let user_email = "testuser@example.com".to_string();
        let active_tenant_id = Uuid::new_v4();
        let mut users_repo = MockUsersRepository::new();

        let user = User {
                    id: sub,
                    email: user_email.clone(),
                    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    phone: Some("+123456789".to_string()),
                    status: "active".to_string(),
                    last_login_at: Some(Utc::now()),
                    profile_picture_url: None,
                    locale: "hu-HU".to_string(),
                    invited_by: None,
                    email_verified_at: Some(Utc::now()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    is_mfa_enabled: true,
                    mfa_secret: None,
                    timezone: "Europe/Budapest".to_string(),
                };

        assert!(user.is_mfa_enabled());
        let user = user.init_mfa_secret();

        users_repo
            .expect_get_user_by_id()
            .times(1)
            .with(eq(sub))
            .returning({
                let user_clone = user.clone();
                move |_| Ok(user_clone.clone())
            });
        users_repo
            .expect_update_user()
            .times(1)
            .withf(|user| user.mfa_secret.is_none() && !user.is_mfa_enabled())
            .returning(Ok);

        let mut auth_repo = MockAuthRepository::new();
        auth_repo
            .expect_account_event_log_by_ip_and_event_type_count()
            .times(1)
            .with(
                eq("127.0.0.1".parse::<IpAddr>().unwrap()),
                eq(AccountEventType::MfaDisable),
                eq(120),
            )
            .returning(|_, _, _| Ok(0));
        auth_repo
            .expect_insert_account_event_log()
            .times(1)
            .with(
                eq(Some(sub)),
                eq(Some(user_email)),
                eq(AccountEventType::MfaDisable),
                eq(AccountEventStatus::Success),
                eq(Some("127.0.0.1".parse().unwrap())),
                eq(None),
                eq(None),
            )
            .returning(
                |user_id, identifier, event_type, status, ip_address, user_agent, metadata| {
                    let ip_address = Some(ipnetwork::IpNetwork::from(ip_address.unwrap()));
                    Ok(AccountEventLogEntry {
                        id: Uuid::new_v4(),
                        user_id,
                        identifier,
                        event_type,
                        status,
                        ip_address,
                        user_agent,
                        metadata,
                        created_at: Utc::now(),
                    })
                },
            );

        let mut app_state = MockUsersModule::new();
        let users_repo = Arc::new(users_repo);
        let auth_repo = Arc::new(auth_repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_auth_repo()
            .times(2)
            .returning(move || auth_repo.clone());
        app_state
            .expect_users_repo()
            .times(1)
            .returning(move || users_repo.clone());
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let payload = serde_json::to_string(&OtpUserInputHelper {
            otp: user.get_mfa_token().unwrap(),
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(sub), Some(active_tenant_id))
                ),
            )
            .method("POST")
            .uri("/api/users/otp/disable")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_otp_disable_invalid_user_input() {
        let sub = Uuid::new_v4();
        let user_email = "testuser@example.com".to_string();
        let active_tenant_id = Uuid::new_v4();
        let mut users_repo = MockUsersRepository::new();

        let user = User {
                    id: sub,
                    email: user_email.clone(),
                    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    phone: Some("+123456789".to_string()),
                    status: "active".to_string(),
                    last_login_at: Some(Utc::now()),
                    profile_picture_url: None,
                    locale: "hu-HU".to_string(),
                    invited_by: None,
                    email_verified_at: Some(Utc::now()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    is_mfa_enabled: true,
                    mfa_secret: None,
                    timezone: "Europe/Budapest".to_string(),
                };

        assert!(user.is_mfa_enabled());
        let user = user.init_mfa_secret();

        users_repo
            .expect_get_user_by_id()
            .times(1)
            .with(eq(sub))
            .returning({
                let user_clone = user.clone();
                move |_| Ok(user_clone.clone())
            });
        users_repo.expect_update_user().times(0);

        let mut auth_repo = MockAuthRepository::new();
        auth_repo
            .expect_account_event_log_by_ip_and_event_type_count()
            .times(1)
            .with(
                eq("127.0.0.1".parse::<IpAddr>().unwrap()),
                eq(AccountEventType::MfaDisable),
                eq(120),
            )
            .returning(|_, _, _| Ok(0));
        auth_repo
            .expect_insert_account_event_log()
            .times(1)
            .with(
                eq(Some(sub)),
                eq(Some(user_email)),
                eq(AccountEventType::MfaDisable),
                eq(AccountEventStatus::Error),
                eq(Some("127.0.0.1".parse().unwrap())),
                eq(None),
                eq(Some(json!({
                    "error": "A kétlépcsős azonosításhoz hasznát kód hibás!".to_string()
                }))),
            )
            .returning(
                |user_id, identifier, event_type, status, ip_address, user_agent, metadata| {
                    let ip_address = Some(ipnetwork::IpNetwork::from(ip_address.unwrap()));
                    Ok(AccountEventLogEntry {
                        id: Uuid::new_v4(),
                        user_id,
                        identifier,
                        event_type,
                        status,
                        ip_address,
                        user_agent,
                        metadata,
                        created_at: Utc::now(),
                    })
                },
            );

        let mut app_state = MockUsersModule::new();
        let users_repo = Arc::new(users_repo);
        let auth_repo = Arc::new(auth_repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_auth_repo()
            .times(2)
            .returning(move || auth_repo.clone());
        app_state
            .expect_users_repo()
            .times(1)
            .returning(move || users_repo.clone());
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let otp = if user.get_mfa_token().unwrap() == "111111" {
            "222222"
        } else {
            "111111"
        }
        .to_string();

        let payload = serde_json::to_string(&OtpUserInputHelper { otp }).unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(sub), Some(active_tenant_id))
                ),
            )
            .method("POST")
            .uri("/api/users/otp/disable")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_otp_disable_unauthorized_expired() {
        let sub = Uuid::new_v4();
        let user_email = "testuser@example.com".to_string();
        let active_tenant_id = Uuid::new_v4();

        let user = User {
                    id: sub,
                    email: user_email.clone(),
                    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    phone: Some("+123456789".to_string()),
                    status: "active".to_string(),
                    last_login_at: Some(Utc::now()),
                    profile_picture_url: None,
                    locale: "hu-HU".to_string(),
                    invited_by: None,
                    email_verified_at: Some(Utc::now()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    is_mfa_enabled: true,
                    mfa_secret: None,
                    timezone: "Europe/Budapest".to_string(),
                };

        assert!(user.is_mfa_enabled());
        let user = user.init_mfa_secret();

        let mut app_state = MockUsersModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let payload = serde_json::to_string(&OtpUserInputHelper {
            otp: user.get_mfa_token().unwrap(),
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt(Some(active_tenant_id))),
            )
            .method("POST")
            .uri("/api/users/otp/disable")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_otp_disable_unauthorized_missing() {
        let sub = Uuid::new_v4();
        let user_email = "testuser@example.com".to_string();

        let user = User {
                    id: sub,
                    email: user_email.clone(),
                    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    phone: Some("+123456789".to_string()),
                    status: "active".to_string(),
                    last_login_at: Some(Utc::now()),
                    profile_picture_url: None,
                    locale: "hu-HU".to_string(),
                    invited_by: None,
                    email_verified_at: Some(Utc::now()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    is_mfa_enabled: true,
                    mfa_secret: None,
                    timezone: "Europe/Budapest".to_string(),
                };

        assert!(user.is_mfa_enabled());
        let user = user.init_mfa_secret();

        let payload = serde_json::to_string(&OtpUserInputHelper {
            otp: user.get_mfa_token().unwrap(),
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/users/otp/disable")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(MockUsersModule::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
