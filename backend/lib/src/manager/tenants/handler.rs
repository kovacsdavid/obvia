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

use crate::common::error::FriendlyError;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::common::dto::{
    OkResponse, OrderingParams, PaginatorParams, QueryParam, SimpleMessageResponse,
};
use crate::manager::tenants::TenantsModule;
use crate::manager::tenants::dto::{
    CreateTenant, CreateTenantHelper, FilteringParams, TenantActivateRequest,
};
use crate::manager::tenants::service::TenantsService;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{Json, debug_handler};
use std::sync::Arc;
use tracing::Level;

/// Handles the creation of a tenant.
///
/// This asynchronous function processes a request to create a tenant, performing
/// authentication, state handling, and validation of the payload before delegating the core
/// process to the `create_inner` function.
///
/// # Parameters
/// - `AuthenticatedUser(claims)`: Represents the authenticated user's claims, required for
///   authorization and context.
/// - `State(tenants_module)`: A shared state containing the `TenantsModule`
///   object. This module provides access to necessary services and utilities related to
///   tenants.
/// - `payload`: The input payload wrapped in a `Result` object, which contains either:
///     - `Json<CreateRequestHelper>`: A valid JSON payload for creating a tenant.
///     - `JsonRejection`: An error generated during JSON deserialization or validation.
///
/// # Returns
/// A `Response` object representing the outcome of the tenant creation process:
/// - A successful response if the creation operation completes successfully.
/// - An appropriate error response if any step of the process fails (e.g., authentication error,
///   invalid payload, or data processing failure).
///
/// # Implementation Details
/// This function does the following:
/// 1. Extracts the authenticated user's claims.
/// 2. Accesses the `TenantsModule` state.
/// 3. Validates and processes the incoming JSON payload.
/// 4. Invokes the `create_inner` function to perform the core logic of creating the tenant.
///    - Passes a closure that asynchronously generates a repository implementation (`PoolWrapper`),
///      which is used to interact with the data layer.
#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tenants_module): State<Arc<TenantsModule>>,
    payload: Result<Json<CreateTenantHelper>, JsonRejection>,
) -> Response {
    match payload {
        Ok(Json(payload)) => match CreateTenant::try_from(payload) {
            Ok(user_input) => {
                match TenantsService::try_create(claims, user_input, tenants_module).await {
                    Ok(_) => (
                        StatusCode::CREATED,
                        Json(OkResponse::new(SimpleMessageResponse {
                            message: String::from(
                                "Szervezeti egység létrehozása sikeresen megtörtént!",
                            ),
                        })),
                    )
                        .into_response(),
                    Err(e) => e.into_response(),
                }
            }
            Err(e) => e.into_response(),
        },
        Err(_) => FriendlyError::UserFacing(
            StatusCode::BAD_REQUEST,
            "ORGANIZATIONAL_UNITS/HANDLER/CREATE".to_string(),
            "Invalid JSON".to_string(),
        )
        .trace(Level::DEBUG)
        .into_response(),
    }
}

/// Handles the HTTP GET request for a tenant
///
/// This asynchronous function is designed to handle requests that require
/// an authenticated user and access to the `TenantsModule` state.
/// The implementation of this function is currently not provided (`todo!` macro),
/// and should be implemented in the future to define its behavior.
///
/// # Arguments
///
/// * `AuthenticatedUser(_claims)` - Represents the authenticated user making the request.
///   The `_claims` parameter holds the claims or credentials associated with the user,
///   but it is currently unused in the function.
///
/// * `State(_tenants_module)` - Provides access to the shared state of the
///   `TenantsModule`. The state is wrapped in an `Arc` for thread-safe sharing,
///   but it is currently unused in the function.
///
/// # Returns
///
/// A `Response` object representing the HTTP response to be sent to the client.
/// The exact contents and behavior of the response are not yet defined as the
/// implementation is pending.
pub async fn get(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(_tenants_module): State<Arc<TenantsModule>>,
) -> Response {
    todo!();
}

/// Handles the listing of tenants for an authenticated user.
///
/// This asynchronous function processes a request to list tenants, ensuring that
/// the user is authenticated before proceeding. The function currently contains a placeholder
/// (`todo!`) and needs implementation to fulfill its intended purpose.
///
/// # Parameters
/// - `AuthenticatedUser(_claims)`: The `_claims` represent the authentication
///   claims of the user. Currently unused.
/// - `State(_tenants_module)`: Shared application state of type `Arc<TenantsModule>`,
///   used to facilitate the interaction with the data layer.
///
/// # Returns
/// - `Response`: An HTTP response that will eventually return the results of listing tenants or an appropriate error response if issues occur.
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tenants_module): State<Arc<TenantsModule>>,
    Query(payload): Query<QueryParam>,
) -> Response {
    match TenantsService::get_paged_list(
        &PaginatorParams::try_from(&payload).unwrap_or(PaginatorParams::default()),
        &OrderingParams::try_from(&payload).unwrap_or(OrderingParams {
            order_by: "name".to_string(),
            order: "asc".to_string(),
        }),
        &FilteringParams::from(&payload),
        &claims,
        tenants_module.tenants_repo.clone(),
    )
    .await
    {
        Ok(res) => (StatusCode::OK, Json(OkResponse::new(res))).into_response(),
        Err(e) => e.into_response(),
    }
}

/// Activates a tenant for the authenticated user.
///
/// This endpoint is responsible for setting a specific tenant as the active tenant
/// for the currently authenticated user. The resulting JWT will be updated to include
/// the active tenant information.
///
/// # Arguments
///
/// * `AuthenticatedUser(claims)` - Represents the authenticated user's claims containing user details.
/// * `State(tenants_module)` - Shared state containing the `TenantsModule`, which provides tenant-related functionality.
/// * `payload` - Result containing either the JSON payload (`TenantActivateRequest`) or a JSON parsing error.
///
/// # Returns
///
/// A `Response` indicating the result of the activation operation:
///
/// - **`200 OK`**: If the tenant is successfully activated, returns a new JWT token with the updated active tenant.
/// - **`400 BAD REQUEST`**: If the JSON payload is invalid, an error message is returned to the client.
/// - **`401 UNAUTHORIZED`**: If the user is not authorized to activate the given tenant, an error message is returned.
/// - **`500 INTERNAL SERVER ERROR`**: If an unexpected error occurs during the process, an internal error message is returned.
///
/// # Behavior
///
/// 1. The function first attempts to parse the JSON payload into a `TenantActivateRequest` structure. - If parsing fails, it responds with `400 BAD REQUEST`.
/// 2. On successful parsing, it uses the tenant repository to fetch the user's association with the specified tenant ID. - If no association is found, it responds with `401 UNAUTHORIZED`.
/// 3. If the user is associated with the tenant, the function updates the user's claims to set the new active tenant. - A new JWT token is then generated and returned in the response.
/// 4. Any internal errors during repository access will result in a `500 INTERNAL SERVER ERROR`.
///
/// # Errors
///
/// This function generates the following errors:
/// - `Unauthorized`: If the user doesn't have access to the specified tenant ID.
/// - `Bad Request`: If the incoming JSON payload is malformed.
/// - `Internal Error`: If an unexpected error occurs, such as database or repository failures.
pub async fn activate(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tenants_module): State<Arc<TenantsModule>>,
    payload: Result<Json<TenantActivateRequest>, JsonRejection>,
) -> Response {
    match payload {
        Ok(payload) => {
            match TenantsService::activate(
                &payload,
                &claims,
                tenants_module.tenants_repo.clone(),
                tenants_module.config.clone(),
            )
            .await
            {
                Ok(res) => (StatusCode::OK, Json(OkResponse::new(res))).into_response(),
                Err(e) => e.into_response(),
            }
        }
        Err(_) => FriendlyError::UserFacing(
            StatusCode::BAD_REQUEST,
            "ORGANIZATIONAL_UNITS/HANDLER/ACTIVATE".to_string(),
            "Invalid JSON".to_string(),
        )
        .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manager::app::config::{
        AppConfigBuilder, DatabaseConfigBuilder, DatabasePoolSizeProvider, DatabaseUrlProvider,
    };
    use crate::manager::app::database::{
        MockConnectionTester, MockDatabaseMigrator, MockPgPoolManagerTrait,
    };
    use crate::manager::auth::dto::claims::Claims;
    use crate::manager::common::dto::{OkResponse, SimpleMessageResponse};
    use crate::manager::tenants;
    use crate::manager::tenants::TenantsModuleBuilder;
    use crate::manager::tenants::model::{Tenant, UserTenant};
    use crate::manager::tenants::repository::MockTenantsRepository;
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use chrono::Local;
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
    use std::ops::{Add, Sub};
    use std::str::FromStr;
    use std::time::Duration;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_managed_success() {
        let mut pool_manager_mock = MockPgPoolManagerTrait::new();
        pool_manager_mock
            .expect_add_tenant_pool()
            .times(1)
            .returning(|tenant_id, _| Ok(tenant_id));
        pool_manager_mock
            .expect_get_tenant_pool()
            .times(1)
            .returning(|_| {
                let database_config = DatabaseConfigBuilder::default().build().unwrap();
                Ok(Some(
                    PgPoolOptions::new()
                        .connect_lazy(&database_config.url())
                        .unwrap(),
                ))
            });
        let pool_manager_mock = Arc::new(pool_manager_mock);

        let mut repo = MockTenantsRepository::new();
        repo.expect_setup_managed()
            .times(1)
            .withf(|_, name, _, _, _| name == "test")
            .returning(|uuid: Uuid, _, _, _, _| {
                Ok(Tenant {
                    id: uuid,
                    name: "test".to_string(),
                    is_self_hosted: false,
                    db_host: "localhost".to_string(),
                    db_port: 5432,
                    db_name: "database".to_string(),
                    db_user: "user".to_string(),
                    db_password: "password".to_string(),
                    db_max_pool_size: 5,
                    db_ssl_mode: "disable".to_string(),
                    created_at: Local::now(),
                    updated_at: Local::now(),
                    deleted_at: None,
                })
            });

        let mut migrator = MockDatabaseMigrator::new();
        migrator
            .expect_migrate_tenant_db()
            .times(1)
            .returning(|_| Ok(()));

        let connection_tester = MockConnectionTester::new();

        let config = Arc::new(AppConfigBuilder::default().build().unwrap());

        let payload = serde_json::to_string(&CreateTenantHelper {
            name: String::from("test"),
            is_self_hosted: false,
            db_host: None,
            db_port: None,
            db_name: None,
            db_user: None,
            db_password: None,
        })
        .unwrap();

        let exp = Local::now().add(Duration::from_secs(100)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().timestamp();

        let bearer = Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            None,
        )
        .to_token(config.auth().jwt_secret().as_bytes())
        .unwrap();

        let request = Request::builder()
            .header("Authorization", format!("Bearer {}", bearer))
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/create")
            .body(Body::from(payload))
            .unwrap();

        let tenants_module = TenantsModuleBuilder::default()
            .pool_manager(pool_manager_mock.clone())
            .config(config.clone())
            .tenants_repo(Arc::new(repo))
            .migrator(Arc::new(migrator))
            .connection_tester(Arc::new(connection_tester))
            .build()
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();

        let expected_response = serde_json::to_string(&OkResponse::new(SimpleMessageResponse {
            message: String::from("Szervezeti egység létrehozása sikeresen megtörtént!"),
        }))
        .unwrap();

        assert_eq!(&body[..], expected_response.as_bytes());
    }

    #[tokio::test]
    async fn test_call_endpoint_with_invalid_bearer() {
        let mut pool_manager_mock = MockPgPoolManagerTrait::new();
        pool_manager_mock
            .expect_add_tenant_pool()
            .times(0)
            .returning(|tenant_id, _| Ok(tenant_id));
        pool_manager_mock
            .expect_get_tenant_pool()
            .times(0)
            .returning(|_| {
                let database_config = DatabaseConfigBuilder::default().build().unwrap();
                Ok(Some(
                    PgPoolOptions::new()
                        .connect_lazy(&database_config.url())
                        .unwrap(),
                ))
            });
        let pool_manager_mock = Arc::new(pool_manager_mock);

        let mut repo = MockTenantsRepository::new();
        repo.expect_setup_managed()
            .times(0)
            .withf(|_, name, _, _, _| name == "test")
            .returning(|uuid: Uuid, _, _, _, _| {
                Ok(Tenant {
                    id: uuid,
                    name: "test".to_string(),
                    is_self_hosted: false,
                    db_host: "localhost".to_string(),
                    db_port: 5432,
                    db_name: "database".to_string(),
                    db_user: "user".to_string(),
                    db_password: "password".to_string(),
                    db_max_pool_size: 5,
                    db_ssl_mode: "disable".to_string(),
                    created_at: Local::now(),
                    updated_at: Local::now(),
                    deleted_at: None,
                })
            });

        let mut migrator = MockDatabaseMigrator::new();
        migrator
            .expect_migrate_tenant_db()
            .times(0)
            .returning(|_| Ok(()));

        let connection_tester = MockConnectionTester::new();

        let config = Arc::new(AppConfigBuilder::default().build().unwrap());

        let payload = serde_json::to_string(&CreateTenantHelper {
            name: String::from("test"),
            is_self_hosted: false,
            db_host: None,
            db_port: None,
            db_name: None,
            db_user: None,
            db_password: None,
        })
        .unwrap();

        let exp = Local::now().sub(Duration::from_secs(61)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().timestamp();

        let bearer = Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            None,
        )
        .to_token(config.auth().jwt_secret().as_bytes())
        .unwrap();

        let request = Request::builder()
            .header("Authorization", format!("Bearer {}", bearer))
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/create")
            .body(Body::from(payload))
            .unwrap();

        let tenants_module = TenantsModuleBuilder::default()
            .pool_manager(pool_manager_mock.clone())
            .config(config.clone())
            .tenants_repo(Arc::new(repo))
            .migrator(Arc::new(migrator))
            .connection_tester(Arc::new(connection_tester))
            .build()
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn test_create_self_hosted_success() {
        let mut pool_manager_mock = MockPgPoolManagerTrait::new();
        pool_manager_mock
            .expect_add_tenant_pool()
            .times(1)
            .returning(|tenant_id, _| Ok(tenant_id));
        pool_manager_mock
            .expect_get_tenant_pool()
            .times(1)
            .returning(|_| {
                let database_config = DatabaseConfigBuilder::default().build().unwrap();
                Ok(Some(
                    PgPoolOptions::new()
                        .connect_lazy(&database_config.url())
                        .unwrap(),
                ))
            });
        let pool_manager_mock = Arc::new(pool_manager_mock);

        let mut repo = MockTenantsRepository::new();
        repo.expect_setup_self_hosted()
            .times(1)
            .withf(|name, _, _| name == "test")
            .returning(|_, _, _| {
                Ok(Tenant {
                    id: Uuid::new_v4(),
                    name: "test".to_string(),
                    is_self_hosted: true,
                    db_host: "example.com".to_string(),
                    db_port: 5432,
                    db_name: "tenant_1234567890".to_string(),
                    db_user: "tenant_1234567890".to_string(),
                    db_password: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
                    db_max_pool_size: 5,
                    db_ssl_mode: "verify-full".to_string(),
                    created_at: Local::now(),
                    updated_at: Local::now(),
                    deleted_at: None,
                })
            });

        let mut migrator = MockDatabaseMigrator::new();
        migrator
            .expect_migrate_tenant_db()
            .times(1)
            .returning(|_| Ok(()));

        let mut connection_tester = MockConnectionTester::new();
        connection_tester
            .expect_test_connect()
            .times(1)
            .returning(|config, ssl_mode| {
                let conn = PgConnectOptions::from_str(&config.url())?.ssl_mode(ssl_mode);
                let pool = PgPoolOptions::new()
                    .max_connections(config.max_pool_size())
                    .acquire_timeout(Duration::from_secs(3))
                    .connect_lazy_with(conn);
                Ok(pool)
            });

        connection_tester
            .expect_is_empty_database()
            .times(1)
            .returning(|_| Ok(()));

        let config = Arc::new(AppConfigBuilder::default().build().unwrap());

        let payload = serde_json::to_string(&CreateTenantHelper {
            name: String::from("test"),
            is_self_hosted: true,
            db_host: Some(String::from("example.com")),
            db_port: Some(5432),
            db_name: Some(String::from("tenant_1234567890")),
            db_user: Some(String::from("tenant_1234567890")),
            db_password: Some(String::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")),
        })
        .unwrap();

        let exp = Local::now().add(Duration::from_secs(100)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().timestamp();

        let bearer = Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            None,
        )
        .to_token(config.auth().jwt_secret().as_bytes())
        .unwrap();

        let request = Request::builder()
            .header("Authorization", format!("Bearer {}", bearer))
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/create")
            .body(Body::from(payload))
            .unwrap();

        let tenants_module = TenantsModuleBuilder::default()
            .pool_manager(pool_manager_mock.clone())
            .config(config.clone())
            .tenants_repo(Arc::new(repo))
            .migrator(Arc::new(migrator))
            .connection_tester(Arc::new(connection_tester))
            .build()
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();

        let expected_response = serde_json::to_string(&OkResponse::new(SimpleMessageResponse {
            message: String::from("Szervezeti egység létrehozása sikeresen megtörtént!"),
        }))
        .unwrap();

        assert_eq!(&body[..], expected_response.as_bytes());
    }
    #[tokio::test]
    async fn test_activate_success() {
        let active_tenant_id1 = Uuid::new_v4();
        let active_tenant_id2 = active_tenant_id1;
        let user_id1 = Uuid::new_v4();
        let user_id2 = user_id1;

        let mut repo = MockTenantsRepository::new();
        repo.expect_get_user_active_tenant_by_id()
            .times(1)
            .withf(move |user_id, tenant_id| {
                *user_id == user_id1 && *tenant_id == active_tenant_id1
            })
            .returning(|user_id, tenant_id| {
                Ok(Some(UserTenant {
                    id: Uuid::new_v4(),
                    user_id,
                    tenant_id,
                    role: "owner".to_string(),
                    invited_by: None,
                    last_activated: Local::now(),
                    created_at: Local::now(),
                    updated_at: Local::now(),
                    deleted_at: None,
                }))
            });

        let migrator = MockDatabaseMigrator::new();

        let connection_tester = MockConnectionTester::new();

        let config = Arc::new(AppConfigBuilder::default().build().unwrap());

        let payload = serde_json::to_string(&TenantActivateRequest {
            new_tenant_id: active_tenant_id2,
        })
        .unwrap();

        let exp = Local::now().add(Duration::from_secs(100)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().timestamp();

        let jti = Uuid::new_v4();

        let claims_original = Claims::new(
            user_id2,
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            jti,
            None,
        );

        let bearer = claims_original
            .to_token(config.auth().jwt_secret().as_bytes())
            .unwrap();

        let request = Request::builder()
            .header("Authorization", format!("Bearer {}", bearer))
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/activate")
            .body(Body::from(payload))
            .unwrap();

        let tenants_module = TenantsModuleBuilder::default()
            .pool_manager(Arc::new(MockPgPoolManagerTrait::new()))
            .config(config.clone())
            .tenants_repo(Arc::new(repo))
            .migrator(Arc::new(migrator))
            .connection_tester(Arc::new(connection_tester))
            .build()
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();

        let claims_new = claims_original
            .clone()
            .set_active_tenant(Some(active_tenant_id2));

        let expected_response = serde_json::to_string(&OkResponse::new(
            claims_new
                .to_token(config.auth().jwt_secret().as_bytes())
                .unwrap(),
        ))
        .unwrap();

        assert_eq!(&body[..], expected_response.as_bytes());
    }
}
