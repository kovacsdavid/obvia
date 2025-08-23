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

use crate::auth::middleware::AuthenticatedUser;
use crate::common::dto::{OkResponse, OrderingParams, PagedResult, PaginatorParams, QueryParam};
use crate::common::error::FriendlyError;
use crate::tenants::TenantsModule;
use crate::tenants::dto::{
    FilteringParams, PublicTenant, TenantCreateRequest, TenantCreateRequestHelper,
};
use crate::tenants::service::try_create;
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
    payload: Result<Json<TenantCreateRequestHelper>, JsonRejection>,
) -> Response {
    match payload {
        Ok(Json(payload)) => match TenantCreateRequest::try_from(payload) {
            Ok(user_input) => {
                let mut repo = (tenants_module.repo_factory)();
                let migrator = (tenants_module.migrator_factory)();
                match try_create(&mut *repo, &*migrator, claims, user_input, tenants_module).await {
                    Ok(resp) => (StatusCode::CREATED, Json(resp)).into_response(),
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
    let repo = (tenants_module.repo_factory)();

    let paginator = PaginatorParams::try_from(&payload).unwrap_or(PaginatorParams::default());
    let orderding = OrderingParams::try_from(&payload).unwrap_or(OrderingParams {
        order_by: "name".to_string(),
        order: "asc".to_string(),
    });
    let filtering = FilteringParams::from(&payload);

    match repo
        .get_all_by_user_id(claims.sub(), paginator, orderding, filtering)
        .await
    {
        Ok(result) => {
            let mut public_tenants = vec![];
            for tenant in result.data {
                public_tenants.push(PublicTenant::from(tenant))
            }
            let result = PagedResult {
                page: result.page,
                limit: result.limit,
                total: result.total,
                data: public_tenants,
            };
            (StatusCode::OK, Json(OkResponse::new(result))).into_response()
        }
        Err(_e) => {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::config::{
        AppConfigBuilder, DatabaseConfigBuilder, DatabasePoolSizeProvider, DatabaseUrlProvider,
    };
    use crate::app::database::{
        ConnectionTester, DatabaseMigrator, MockConnectionTester, MockDatabaseMigrator,
        MockPgPoolManagerTrait,
    };
    use crate::auth::dto::claims::Claims;
    use crate::common::dto::{OkResponse, SimpleMessageResponse};
    use crate::tenants;
    use crate::tenants::TenantsModuleBuilder;
    use crate::tenants::model::Tenant;
    use crate::tenants::repository::{MockTenantsRepository, TenantsRepository};
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

        let repo_factory = Box::new(|| {
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
            Box::new(repo) as Box<dyn TenantsRepository + Send + Sync>
        });

        let migrator_factory = Box::new(|| {
            let mut migrator = MockDatabaseMigrator::new();
            migrator
                .expect_migrate_tenant_db()
                .times(1)
                .returning(|_| Ok(()));
            Box::new(migrator) as Box<dyn DatabaseMigrator + Send + Sync>
        });

        let connection_tester_factory = Box::new(|| {
            Box::new(MockConnectionTester::new()) as Box<dyn ConnectionTester + Send + Sync>
        });

        let config = Arc::new(AppConfigBuilder::default().build().unwrap());

        let payload = serde_json::to_string(&TenantCreateRequestHelper {
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
            .repo_factory(repo_factory)
            .migrator_factory(migrator_factory)
            .connection_tester_factory(connection_tester_factory)
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

        let repo_factory = Box::new(|| {
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
            Box::new(repo) as Box<dyn TenantsRepository + Send + Sync>
        });

        let migrator_factory = Box::new(|| {
            let mut migrator = MockDatabaseMigrator::new();
            migrator
                .expect_migrate_tenant_db()
                .times(0)
                .returning(|_| Ok(()));
            Box::new(migrator) as Box<dyn DatabaseMigrator + Send + Sync>
        });

        let connection_tester_factory = Box::new(|| {
            Box::new(MockConnectionTester::new()) as Box<dyn ConnectionTester + Send + Sync>
        });

        let config = Arc::new(AppConfigBuilder::default().build().unwrap());

        let payload = serde_json::to_string(&TenantCreateRequestHelper {
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
            .repo_factory(repo_factory)
            .migrator_factory(migrator_factory)
            .connection_tester_factory(connection_tester_factory)
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

        let repo_factory = Box::new(|| {
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
            Box::new(repo) as Box<dyn TenantsRepository + Send + Sync>
        });

        let migrator_factory = Box::new(|| {
            let mut migrator = MockDatabaseMigrator::new();
            migrator
                .expect_migrate_tenant_db()
                .times(1)
                .returning(|_| Ok(()));
            Box::new(migrator) as Box<dyn DatabaseMigrator + Send + Sync>
        });

        let connection_tester_factory = Box::new(|| {
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

            Box::new(connection_tester) as Box<dyn ConnectionTester + Send + Sync>
        });

        let config = Arc::new(AppConfigBuilder::default().build().unwrap());

        let payload = serde_json::to_string(&TenantCreateRequestHelper {
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
            .repo_factory(repo_factory)
            .migrator_factory(migrator_factory)
            .connection_tester_factory(connection_tester_factory)
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
}
