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

use super::AuthModule;
use crate::common::dto::{EmptyType, SimpleMessageResponse, SuccessResponseBuilder};
use crate::common::extractors::{ClientContext, UserInput};
use crate::common::handler::{ErrorMapper, ErrorMapperInterface, HandlerResult};
use crate::common::service::Service;
use crate::manager::auth::dto::login::LoginResponse;
use crate::manager::auth::dto::register::{
    ForgottenPasswordRequest, ForgottenPasswordRequestHelper, NewPasswordRequest,
    NewPasswordRequestHelper, RegisterRequestHelper, ResendEmailValidationRequest,
    ResendEmailValidationRequestHelper,
};
use crate::manager::auth::dto::{login::LoginRequest, register::RegisterRequest};
use crate::manager::auth::service::{AuthService, gen_refresh_cookie};
use axum::extract::Query;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn login<M: AuthModule>(
    State(auth_module): State<Arc<M>>,
    jar: CookieJar,
    client_context: ClientContext,
    Json(payload): Json<LoginRequest>,
) -> HandlerResult {
    let service = Service::new(None, auth_module.clone());
    let error_mapper = ErrorMapper::new(auth_module);
    let (access_token, access_claims, refresh_token, _, user_public) = error_mapper
        .or_handler_error(service.try_login(&payload, &client_context).await)
        .await?;
    let response = error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(LoginResponse::new(access_claims, user_public, access_token))
                .build(),
        )
        .await?
        .into_response();

    let refresh_cookie = error_mapper
        .or_handler_error(gen_refresh_cookie(
            refresh_token,
            !matches!(service.module().config().server().environment(), "dev"),
            service
                .module()
                .config()
                .auth()
                .refresh_token_expiration_mins(),
        ))
        .await?;
    Ok((jar.add(refresh_cookie), response).into_response())
}

pub async fn refresh<M: AuthModule>(
    State(auth_module): State<Arc<M>>,
    jar: CookieJar,
    client_context: ClientContext,
) -> HandlerResult {
    let service = Service::new(None, auth_module.clone());
    let error_mapper = ErrorMapper::new(auth_module);
    let (access_token, access_claims, refresh_token, _, user_public) = error_mapper
        .or_handler_error(service.refresh(jar.clone(), &client_context).await)
        .await?;
    let response = error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(LoginResponse::new(access_claims, user_public, access_token))
                .build(),
        )
        .await?
        .into_response();

    let refresh_cookie = error_mapper
        .or_handler_error(gen_refresh_cookie(
            refresh_token,
            !matches!(service.module().config().server().environment(), "dev"),
            service
                .module()
                .config()
                .auth()
                .refresh_token_expiration_mins(),
        ))
        .await?;
    Ok((jar.add(refresh_cookie), response).into_response())
}

pub async fn logout<M: AuthModule>(
    State(auth_module): State<Arc<M>>,
    jar: CookieJar,
    client_context: ClientContext,
) -> HandlerResult {
    let service = Service::new(None, auth_module.clone());
    let error_mapper = ErrorMapper::new(auth_module);
    error_mapper
        .or_handler_error(service.logout(jar.clone(), &client_context).await)
        .await?;

    Ok(jar
        .remove(Cookie::build("refresh_token").path("/api/auth/t"))
        .into_response())
}

pub async fn register<M: AuthModule>(
    State(auth_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<RegisterRequest, RegisterRequestHelper>,
) -> HandlerResult {
    let service = Service::new(None, auth_module.clone());
    let error_mapper = ErrorMapper::new(auth_module);
    error_mapper
        .or_handler_error(service.try_register(&user_input).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::CREATED)
                .data(SimpleMessageResponse::new(
                    "A felhasználó sikeresen létrehozva",
                ))
                .build(),
        )
        .await?
        .into_response())
}

pub async fn verify_email<M: AuthModule>(
    State(auth_module): State<Arc<M>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let service = Service::new(None, auth_module.clone());
    let error_mapper = ErrorMapper::new(auth_module);
    let token = payload
        .get("id")
        .cloned()
        .unwrap_or(String::from("missing_token"));
    error_mapper
        .or_handler_error(service.verify_email(&token).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "Az e-mail cím megerősítése sikeresen megtörtént",
                ))
                .build(),
        )
        .await?
        .into_response())
}

pub async fn resend_email_verification<M: AuthModule>(
    State(auth_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<
        ResendEmailValidationRequest,
        ResendEmailValidationRequestHelper,
    >,
) -> HandlerResult {
    let service = Service::new(None, auth_module.clone());
    let error_mapper = ErrorMapper::new(auth_module);
    error_mapper
        .or_handler_error(service.resend_email_verification(user_input).await)
        .await?;

    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A megerősítő e-mail újraküldése sikeresen megtörtént",
                ))
                .build(),
        )
        .await?
        .into_response())
}

pub async fn forgotten_password<M: AuthModule>(
    State(auth_module): State<Arc<M>>,
    client_context: ClientContext,
    UserInput(user_input, _): UserInput<ForgottenPasswordRequest, ForgottenPasswordRequestHelper>,
) -> HandlerResult {
    let service = Service::new(None, auth_module.clone());
    let error_mapper = ErrorMapper::new(auth_module);
    error_mapper
        .or_handler_error(
            service
                .forgotten_password(user_input, &client_context)
                .await,
        )
        .await?;
    Ok(error_mapper.or_handler_error(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(SimpleMessageResponse::new(
            "Ha a megadott e-mail cím helyes, a jelszó helyreállításához szükséges levél elküldésre került.",
        ))
        .build()).await?.into_response())
}

pub async fn new_password<M: AuthModule>(
    State(auth_module): State<Arc<M>>,
    client_context: ClientContext,
    UserInput(user_input, _): UserInput<NewPasswordRequest, NewPasswordRequestHelper>,
) -> HandlerResult {
    let service = Service::new(None, auth_module.clone());
    let error_mapper = ErrorMapper::new(auth_module);
    error_mapper
        .or_handler_error(service.new_password(user_input, &client_context).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A jelszó megváltoztatása sikeresen megtörtént",
                ))
                .build(),
        )
        .await?
        .into_response())
}
/*
#[cfg(test)]
mod tests {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use chrono::Utc;
    use ipnetwork::IpNetwork;
    use lettre::transport::smtp::response::Category;
    use lettre::transport::smtp::response::Code;
    use lettre::transport::smtp::response::Detail;
    use lettre::transport::smtp::response::Response;
    use lettre::transport::smtp::response::Severity;
    use mockall::predicate::*;
    use sqlx::error::{DatabaseError, ErrorKind};
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};
    use std::net::Ipv4Addr;
    use std::sync::Arc;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::common::config::tests::AppConfigBuilder;
    use crate::common::error::RepositoryError;
    use crate::common::types::{Email, FirstName, LastName, Password};
    use crate::common::value_object::ValueObjectRequired;
    use crate::manager::auth::dto::claims::Claims;
    use crate::manager::auth::dto::register::RegisterRequestHelper;
    use crate::manager::auth::model::AccountEventLogEntry;
    use crate::manager::auth::model::AccountEventStatus;
    use crate::manager::auth::model::AccountEventType;
    use crate::manager::auth::model::EmailVerification;
    use crate::manager::auth::model::RefreshToken;
    use crate::manager::auth::tests::MockAuthModule;
    use crate::manager::tenants::model::UserTenant;
    use crate::manager::{
        auth,
        auth::{
            dto::{login::LoginRequest, register::RegisterRequest},
            repository::MockAuthRepository,
        },
        users::model::User,
    };

    #[tokio::test]
    async fn test_login_success() {
        let mut repo = MockAuthRepository::new();
        let user_id1 = Uuid::new_v4();
        let user_id2 = user_id1;
        repo.expect_get_user_by_email()
            .with(eq("testuser@example.com"))
            .returning(move |_| Ok(User {
                id: user_id1,
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
                is_mfa_enabled: false,
                mfa_secret: None,
            }));
        repo.expect_get_user_active_tenant()
            .with(eq(user_id2))
            .returning(|_| Ok(None));

        repo.expect_update_user_last_login_at()
            .with(eq(user_id2))
            .returning(|_| Ok(()));
        repo.expect_insert_refresh_token().times(1).returning(|_| {
            Ok(RefreshToken {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                family_id: Uuid::new_v4(),
                jti: Uuid::new_v4(),
                iat: Utc::now(),
                exp: Utc::now(),
                replaced_by: None,
                consumed_at: None,
                revoked_at: None,
            })
        });

        repo.expect_account_event_log_ip_and_event_status_count()
            .times(1)
            .returning(|_, _, _| Ok(0));

        repo.expect_insert_account_event_log()
            .times(1)
            .returning(|_, _, _, _, _, _, _| {
                Ok(AccountEventLogEntry {
                    id: Uuid::new_v4(),
                    user_id: Some(Uuid::new_v4()),
                    identifier: Some("test@example.com".to_string()),
                    event_type: AccountEventType::Login,
                    status: AccountEventStatus::Success,
                    ip_address: Some(
                        IpNetwork::new(Ipv4Addr::new(127, 0, 0, 1).into(), 32).unwrap(),
                    ),
                    user_agent: None,
                    metadata: None,
                    created_at: Utc::now(),
                })
            });

        let repo = Arc::new(repo);

        let mut auth_module = MockAuthModule::new();
        auth_module
            .expect_config()
            .returning(|| Arc::new(AppConfigBuilder::default().build().unwrap()));
        auth_module
            .expect_auth_repo()
            .returning(move || repo.clone());

        let payload = serde_json::to_string(&LoginRequest {
            email: "testuser@example.com".to_string(),
            password: "correctpassword".to_string(),
            otp: None,
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/auth/login")
            .body(Body::from(payload))
            .unwrap();

        let config = Arc::new(AppConfigBuilder::default().build().unwrap());

        let app = Router::new().nest(
            "/api",
            Router::new().merge(auth::routes::routes(Arc::new(auth_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        let data = body.get("data").unwrap();

        assert!(
            Claims::from_token(
                data["token"].as_str().unwrap(),
                config.auth().jwt_secret().as_bytes(),
                config.auth().jwt_issuer(),
                &format!("{}-api", config.auth().jwt_audience()),
            )
            .is_ok()
        );
    }

    #[tokio::test]
    async fn test_login_failure() {
        let mut repo = MockAuthRepository::new();
        let user_id1 = Uuid::new_v4();
        let user_id2 = user_id1;
        repo.expect_get_user_by_email()
            .with(eq("testuser@example.com"))
            .returning(move |_| Ok(User {
                id: user_id1,
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
                is_mfa_enabled: false,
                mfa_secret: None,
            }));
        repo.expect_get_user_active_tenant()
            .with(eq(user_id2))
            .returning(|_| Ok(None));

        repo.expect_account_event_log_ip_and_event_status_count()
            .times(1)
            .returning(|_, _, _| Ok(0));

        repo.expect_insert_account_event_log()
            .times(1)
            .returning(|_, _, _, _, _, _, _| {
                Ok(AccountEventLogEntry {
                    id: Uuid::new_v4(),
                    user_id: Some(Uuid::new_v4()),
                    identifier: Some("test@example.com".to_string()),
                    event_type: AccountEventType::Login,
                    status: AccountEventStatus::Failure,
                    ip_address: Some(
                        IpNetwork::new(Ipv4Addr::new(127, 0, 0, 1).into(), 32).unwrap(),
                    ),
                    user_agent: None,
                    metadata: None,
                    created_at: Utc::now(),
                })
            });

        let repo = Arc::new(repo);

        let mut auth_module = MockAuthModule::new();
        auth_module
            .expect_config()
            .returning(|| Arc::new(AppConfigBuilder::default().build().unwrap()));
        auth_module
            .expect_auth_repo()
            .returning(move || repo.clone());

        let payload = serde_json::to_string(&LoginRequest {
            email: "testuser@example.com".to_string(),
            password: "invalidpassword".to_string(),
            otp: None,
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/auth/login")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(auth::routes::routes(Arc::new(auth_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    // ===== REGISTER =====

    #[tokio::test]
    async fn test_register_success() {
        let payload = serde_json::to_string(&RegisterRequestHelper {
            email: "testuser@example.com".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            password: "Password1!".to_string(),
            password_confirm: "Password1!".to_string(),
        })
        .unwrap();

        let mut repo = MockAuthRepository::new();
        let test_user_uuid = Uuid::new_v4();
        let test_user_uuid_copy = test_user_uuid;
        repo.expect_insert_user()
            .withf(move |payload_param, hashed_password| {
                *payload_param
                    == RegisterRequest {
                        email: "testuser@example.com".parse::<ValueObjectRequired<Email>>().unwrap(),
                        first_name: "Test".parse::<ValueObjectRequired<FirstName>>().unwrap(),
                        last_name: "User".parse::<ValueObjectRequired<LastName>>().unwrap(),
                        password: "Password1!".parse::<ValueObjectRequired<Password>>().unwrap(),
                    }
                    && Argon2::default()
                        .verify_password(
                            b"Password1!",
                            &PasswordHash::new(hashed_password).unwrap(),
                        )
                        .is_ok()
            })
            .returning(move |_, _| Ok(User {
                    id: test_user_uuid,
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
                    is_mfa_enabled: false,
                    mfa_secret: None,
                }));
        repo.expect_insert_email_verification()
            .times(1)
            .withf(move |user_id| *user_id == test_user_uuid_copy)
            .returning(|user_id| {
                Ok(EmailVerification {
                    id: Uuid::new_v4(),
                    user_id,
                    valid_until: chrono::Utc::now() + chrono::Duration::days(1),
                    created_at: chrono::Utc::now(),
                    deleted_at: None,
                })
            });

        let repo = Arc::new(repo);

        let mut auth_module = MockAuthModule::new();
        auth_module
            .expect_config()
            .returning(|| Arc::new(AppConfigBuilder::default().build().unwrap()));
        auth_module
            .expect_auth_repo()
            .returning(move || repo.clone());
        auth_module.expect_send().times(1).returning(|_| {
            Ok(Some(Response::new(
                Code::new(
                    Severity::PositiveIntermediate,
                    Category::Connections,
                    Detail::One,
                ),
                vec![],
            )))
        });

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/auth/register")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(auth::routes::routes(Arc::new(auth_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }
    #[tokio::test]
    async fn test_register_user_already_exists() {
        let payload = serde_json::to_string(&RegisterRequestHelper {
            email: "testuser@example.com".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            password: "Password1!".to_string(),
            password_confirm: "Password1!".to_string(),
        })
        .unwrap();

        pub struct DummyDatabaseError;

        impl Error for DummyDatabaseError {}
        impl Debug for DummyDatabaseError {
            fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
                unimplemented!()
            }
        }
        impl Display for DummyDatabaseError {
            fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
                unimplemented!()
            }
        }
        impl DatabaseError for DummyDatabaseError {
            fn message(&self) -> &str {
                unimplemented!()
            }

            fn as_error(&self) -> &(dyn Error + Send + Sync + 'static) {
                unimplemented!()
            }

            fn as_error_mut(&mut self) -> &mut (dyn Error + Send + Sync + 'static) {
                unimplemented!()
            }

            fn into_error(self: Box<Self>) -> Box<dyn Error + Send + Sync + 'static> {
                unimplemented!()
            }

            fn kind(&self) -> ErrorKind {
                unimplemented!()
            }
            fn is_unique_violation(&self) -> bool {
                true
            }
        }

        let mut repo = MockAuthRepository::new();
        repo.expect_insert_user().returning(|_, _| {
            Err(RepositoryError::Database(sqlx::Error::Database(
                Box::new(DummyDatabaseError) as Box<dyn DatabaseError>,
            )))
        });

        let repo = Arc::new(repo);

        let mut auth_module = MockAuthModule::new();
        auth_module
            .expect_config()
            .returning(|| Arc::new(AppConfigBuilder::default().build().unwrap()));
        auth_module
            .expect_auth_repo()
            .returning(move || repo.clone());

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/auth/register")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(auth::routes::routes(Arc::new(auth_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_active_user_tenant() {
        let active_tenant_id1 = Uuid::new_v4();
        let active_tenant_id2 = active_tenant_id1;
        let mut repo = MockAuthRepository::new();
        let user_id1 = Uuid::new_v4();
        let user_id2 = user_id1;
        repo.expect_get_user_by_email()
            .with(eq("testuser@example.com"))
            .returning(move |_| Ok(User {
                id: user_id1,
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
                is_mfa_enabled: false,
                mfa_secret: None,
            }));
        repo.expect_get_user_active_tenant()
            .with(eq(user_id2))
            .returning(move |user_id| {
                Ok(Some(UserTenant {
                    id: Uuid::new_v4(),
                    user_id,
                    tenant_id: active_tenant_id1,
                    role: "owner".to_string(),
                    invited_by: None,
                    last_activated: Utc::now(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                }))
            });

        repo.expect_update_user_last_login_at()
            .with(eq(user_id2))
            .returning(|_| Ok(()));

        repo.expect_insert_refresh_token().times(1).returning(|_| {
            Ok(RefreshToken {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                family_id: Uuid::new_v4(),
                jti: Uuid::new_v4(),
                iat: Utc::now(),
                exp: Utc::now(),
                replaced_by: None,
                consumed_at: None,
                revoked_at: None,
            })
        });

        repo.expect_account_event_log_ip_and_event_status_count()
            .times(1)
            .returning(|_, _, _| Ok(0));

        repo.expect_insert_account_event_log()
            .times(1)
            .returning(|_, _, _, _, _, _, _| {
                Ok(AccountEventLogEntry {
                    id: Uuid::new_v4(),
                    user_id: Some(Uuid::new_v4()),
                    identifier: Some("test@example.com".to_string()),
                    event_type: AccountEventType::Login,
                    status: AccountEventStatus::Success,
                    ip_address: Some(
                        IpNetwork::new(Ipv4Addr::new(127, 0, 0, 1).into(), 32).unwrap(),
                    ),
                    user_agent: None,
                    metadata: None,
                    created_at: Utc::now(),
                })
            });

        let repo = Arc::new(repo);

        let mut auth_module = MockAuthModule::new();
        auth_module
            .expect_config()
            .returning(|| Arc::new(AppConfigBuilder::default().build().unwrap()));
        auth_module
            .expect_auth_repo()
            .returning(move || repo.clone());

        let payload = serde_json::to_string(&LoginRequest {
            email: "testuser@example.com".to_string(),
            password: "correctpassword".to_string(),
            otp: None,
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/auth/login")
            .body(Body::from(payload))
            .unwrap();

        let config = Arc::new(AppConfigBuilder::default().build().unwrap());

        let app = Router::new().nest(
            "/api",
            Router::new().merge(auth::routes::routes(Arc::new(auth_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        let data = body.get("data").unwrap();

        let claims = Claims::from_token(
            data["token"].as_str().unwrap(),
            config.auth().jwt_secret().as_bytes(),
            config.auth().jwt_issuer(),
            &format!("{}-api", config.auth().jwt_audience()),
        );

        assert!(claims.is_ok());

        assert_eq!(claims.unwrap().active_tenant().unwrap(), active_tenant_id2)
    }
}
*/
