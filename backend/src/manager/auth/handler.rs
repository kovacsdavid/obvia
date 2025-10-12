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
use crate::common::dto::{EmptyType, HandlerResult, SimpleMessageResponse, SuccessResponseBuilder};
use crate::common::extractors::UserInput;
use crate::manager::auth::dto::register::RegisterRequestHelper;
use crate::manager::auth::dto::{login::LoginRequest, register::RegisterRequest};
use crate::manager::auth::service::AuthService;
use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

/// Handles the login process for a user in an asynchronous manner.
///
/// # Parameters
/// - `State(auth_module)`: An instance of `AuthModule` wrapped in an `Arc`, providing access
///   to authentication-related functionality such as pool management and configurations.
///   It's extracted from the application state.
/// - `Json(payload)`: A `LoginRequest` object extracted from the JSON body of the request.
///   This contains the credentials or necessary data for authentication.
///
/// # Returns
/// A `Response` object, wrapped in an `async` function, which represents the outcome of the
/// login operation. This response may contain a success token, error message, or other
/// authentication-related information depending on the login process's result.
///
/// # Functionality
/// - Delegates the core login logic to the `login_inner` function to handle authentication.
/// - The repository for authentication (`AuthRepository`) is created using a `PoolWrapper`
///   that is initialized with the main database connection pool via the `pool_manager`.
/// - The `login_inner` function handles the login operation utilizing the provided
///   repository and `payload`.
#[debug_handler]
pub async fn login(
    State(auth_module): State<Arc<AuthModule>>,
    Json(payload): Json<LoginRequest>,
) -> HandlerResult {
    let res = AuthService::try_login(auth_module.clone(), payload)
        .await
        .map_err(|e| e.into_response())?;
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(res)
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

/// Handles user registration requests.
///
/// This function is an HTTP handler that processes a user registration request by:
/// 1. Validating the incoming payload.
/// 2. Passing the data to the appropriate module for handling the registration logic.
///
/// # Parameters
/// - `State(auth_module)`: Provides a shared reference to the `AuthModule`,
///   which contains the necessary components for handling authentication and user
///   management. The `AuthModule` is wrapped in an `Arc` for thread-safe shared access.
/// - `payload`: The incoming user registration request payload. This is wrapped in a
///   `Result` to handle potential payload rejections due to deserialization errors.
///
/// # Returns
/// An asynchronous HTTP `Response` containing the result of the registration process. The response
/// includes the appropriate status code and/or error messages.
///
/// # Implementation Details
/// - This handler calls an internal function `register_inner`, which manages the logic for
///   processing the registration request.
/// - For repository interaction, the function dynamically constructs an instance of `PoolWrapper`
///   using the pool managed by the `AuthModule`. `PoolWrapper` implements the `AuthRepository` trait
///   to abstract database access.
///
/// # Errors
/// - Returns appropriate error responses if:
///   - The payload is invalid or rejected (e.g., malformed JSON).
///   - There is any issue during the registration process (e.g., database connectivity issues).
#[debug_handler]
pub async fn register(
    State(auth_module): State<Arc<AuthModule>>,
    UserInput(user_input, _): UserInput<RegisterRequest, RegisterRequestHelper>,
) -> HandlerResult {
    AuthService::try_register(auth_module.auth_repo.clone(), user_input)
        .await
        .map_err(|e| e.into_response())?;
    Ok(SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::CREATED)
        .data(SimpleMessageResponse::new(
            "A felhasználó sikeresen létrehozva",
        ))
        .build()
        .map_err(|e| e.into_response())?
        .into_response())
}

#[cfg(test)]
mod tests {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use chrono::Local;
    use mockall::predicate::*;
    use sqlx::error::{DatabaseError, ErrorKind};
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};
    use std::sync::Arc;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::common::error::RepositoryError;
    use crate::common::types::value_object::ValueObject;
    use crate::common::types::{Email, FirstName, LastName, Password};
    use crate::manager::app::config::AppConfigBuilder;
    use crate::manager::auth::dto::claims::Claims;
    use crate::manager::auth::dto::register::RegisterRequestHelper;
    use crate::manager::tenants::model::UserTenant;
    use crate::manager::{
        auth,
        auth::{
            AuthModule,
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
                last_login_at: Some(Local::now()),
                profile_picture_url: None,
                locale: Some("hu-HU".to_string()),
                invited_by: None,
                email_verified_at: Some(Local::now()),
                created_at: Local::now(),
                updated_at: Local::now(),
                deleted_at: None,
            }));
        repo.expect_get_user_active_tenant()
            .with(eq(user_id2))
            .returning(|_| Ok(None));
        let auth_module = AuthModule {
            config: Arc::new(AppConfigBuilder::default().build().unwrap()),
            auth_repo: Arc::new(repo),
        };
        let payload = serde_json::to_string(&LoginRequest {
            email: "testuser@example.com".to_string(),
            password: "correctpassword".to_string(),
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
                config.auth().jwt_audience(),
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
                last_login_at: Some(Local::now()),
                profile_picture_url: None,
                locale: Some("hu-HU".to_string()),
                invited_by: None,
                email_verified_at: Some(Local::now()),
                created_at: Local::now(),
                updated_at: Local::now(),
                deleted_at: None,
            }));
        repo.expect_get_user_active_tenant()
            .with(eq(user_id2))
            .returning(|_| Ok(None));
        let auth_module = AuthModule {
            config: Arc::new(AppConfigBuilder::default().build().unwrap()),
            auth_repo: Arc::new(repo),
        };
        let payload = serde_json::to_string(&LoginRequest {
            email: "testuser@example.com".to_string(),
            password: "invalidpassword".to_string(),
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
        repo.expect_insert_user()
            .withf(move |payload_param, hashed_password| {
                *payload_param
                    == RegisterRequest {
                        email: ValueObject::new(Email("testuser@example.com".to_string())).unwrap(),
                        first_name: ValueObject::new(FirstName("Test".to_string())).unwrap(),
                        last_name: ValueObject::new(LastName("User".to_string())).unwrap(),
                        password: ValueObject::new(Password("Password1!".to_string())).unwrap(),
                    }
                    && Argon2::default()
                        .verify_password(
                            b"Password1!",
                            &PasswordHash::new(&hashed_password).unwrap(),
                        )
                        .is_ok()
            })
            .returning(|_, _| Ok(()));

        let auth_module = AuthModule {
            config: Arc::new(AppConfigBuilder::default().build().unwrap()),
            auth_repo: Arc::new(repo),
        };

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

        let auth_module = AuthModule {
            config: Arc::new(AppConfigBuilder::default().build().unwrap()),
            auth_repo: Arc::new(repo),
        };

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
                last_login_at: Some(Local::now()),
                profile_picture_url: None,
                locale: Some("hu-HU".to_string()),
                invited_by: None,
                email_verified_at: Some(Local::now()),
                created_at: Local::now(),
                updated_at: Local::now(),
                deleted_at: None,
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
                    last_activated: Local::now(),
                    created_at: Local::now(),
                    updated_at: Local::now(),
                    deleted_at: None,
                }))
            });
        let auth_module = AuthModule {
            config: Arc::new(AppConfigBuilder::default().build().unwrap()),
            auth_repo: Arc::new(repo),
        };
        let payload = serde_json::to_string(&LoginRequest {
            email: "testuser@example.com".to_string(),
            password: "correctpassword".to_string(),
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
            config.auth().jwt_audience(),
        );

        assert!(claims.is_ok());

        assert_eq!(claims.unwrap().active_tenant().unwrap(), active_tenant_id2)
    }
}
