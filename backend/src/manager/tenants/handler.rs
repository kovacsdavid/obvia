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

use crate::common::dto::{
    EmptyType, HandlerResult, OrderingParams, PaginatorParams, QueryParam, SuccessResponseBuilder,
};
use crate::common::error::FriendlyError;
use crate::common::error::IntoFriendlyError;
use crate::common::extractors::{UserInput, ValidJson};
use crate::common::types::Order;
use crate::common::types::ValueObject;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::tenants::TenantsModule;
use crate::manager::tenants::dto::{
    CreateTenant, CreateTenantHelper, FilteringParams, PublicTenantManaged, PublicTenantSelfHosted,
    TenantActivateRequest,
};
use crate::manager::tenants::service::TenantsService;
use crate::manager::tenants::types::TenantsOrderBy;
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use std::sync::Arc;

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tenants_module): State<Arc<dyn TenantsModule>>,
    UserInput(user_input, _): UserInput<CreateTenant, CreateTenantHelper>,
) -> HandlerResult {
    if user_input.is_self_hosted() {
        let result =
            match TenantsService::create_self_hosted(&claims, &user_input, tenants_module.clone())
                .await
            {
                Ok(r) => r,
                Err(e) => return Err(e.into_friendly_error(tenants_module).await.into_response()),
            };

        match SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(PublicTenantSelfHosted::from(result))
            .build()
        {
            Ok(r) => Ok(r.into_response()),
            Err(e) => Err(e.into_friendly_error(tenants_module).await.into_response()),
        }
    } else {
        let result = match TenantsService::create_managed(
            &claims,
            &user_input,
            tenants_module.clone(),
        )
        .await
        {
            Ok(r) => r,
            Err(e) => return Err(e.into_friendly_error(tenants_module).await.into_response()),
        };

        match SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(PublicTenantManaged::from(result))
            .build()
        {
            Ok(r) => Ok(r.into_response()),
            Err(e) => Err(e.into_friendly_error(tenants_module).await.into_response()),
        }
    }
}

pub async fn get(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(_tenants_module): State<Arc<dyn TenantsModule>>,
) -> Response {
    todo!();
}

pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tenants_module): State<Arc<dyn TenantsModule>>,
    Query(payload): Query<QueryParam>,
) -> HandlerResult {
    let (meta, data) = match TenantsService::get_paged_list(
        &PaginatorParams::try_from(&payload).unwrap_or(PaginatorParams::default()),
        &OrderingParams::try_from(&payload).unwrap_or(OrderingParams {
            order_by: ValueObject::new(TenantsOrderBy("name".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
            order: ValueObject::new(Order("asc".to_string()))
                .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
        }),
        &FilteringParams::from(&payload),
        &claims,
        tenants_module.tenants_repo(),
    )
    .await
    {
        Ok((m, d)) => (m, d),
        Err(e) => return Err(e.into_friendly_error(tenants_module).await.into_response()),
    };

    match SuccessResponseBuilder::new()
        .status_code(StatusCode::OK)
        .meta(meta)
        .data(data)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e.into_friendly_error(tenants_module).await.into_response()),
    }
}

pub async fn activate(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tenants_module): State<Arc<dyn TenantsModule>>,
    ValidJson(payload): ValidJson<TenantActivateRequest>,
) -> HandlerResult {
    let result = match TenantsService::activate(
        &payload,
        &claims,
        tenants_module.tenants_repo(),
        tenants_module.config(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => return Err(e.into_friendly_error(tenants_module).await.into_response()),
    };
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e.into_friendly_error(tenants_module).await.into_response()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manager::app::config::{
        AppConfigBuilder, DatabasePoolSizeProvider, DatabaseUrlProvider,
    };
    use crate::manager::app::database::{MockConnectionTester, MockDatabaseMigrator};
    use crate::manager::auth::dto::claims::Claims;
    use crate::manager::tenants;
    use crate::manager::tenants::dto::NewTokenResponse;
    use crate::manager::tenants::model::{Tenant, UserTenant};
    use crate::manager::tenants::repository::MockTenantsRepository;
    use crate::manager::tenants::tests::MockTenantsModule;
    use crate::manager::users::model::User as ManagerUser;
    use crate::manager::users::repository::MockUsersRepository as MockManagerUserRepository;
    use crate::tenant::users::repository::MockUsersRepository as MockTenantUserRepository;
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
        let new_tenant_id = Uuid::new_v4();
        let now = Local::now();

        let mut tenants_repo = MockTenantsRepository::new();
        tenants_repo
            .expect_setup_managed()
            .times(1)
            .withf(|_, name, _, _, _| name == "test")
            .returning(move |_, _, _, _, _| {
                Ok(Tenant {
                    id: new_tenant_id,
                    name: "test".to_string(),
                    is_self_hosted: false,
                    db_host: "localhost".to_string(),
                    db_port: 5432,
                    db_name: "database".to_string(),
                    db_user: "user".to_string(),
                    db_password: "password".to_string(),
                    db_max_pool_size: 5,
                    db_ssl_mode: "disable".to_string(),
                    created_at: now,
                    updated_at: now,
                    deleted_at: None,
                })
            });

        let mut manager_user_repo = MockManagerUserRepository::new();

        manager_user_repo
            .expect_get_by_uuid()
            .times(1)
            .returning(|_| {
                Ok(ManagerUser {
                    id: Uuid::new_v4(),
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
                    is_mfa_enabled: false,
                    mfa_secret: None,
                })
            });

        let mut tenant_user_repo = MockTenantUserRepository::new();
        tenant_user_repo
            .expect_insert_from_manager()
            .times(1)
            .returning(|user, _| Ok(user));

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
            format!("{}-api", config.auth().jwt_audience().to_string()),
            Uuid::new_v4(),
            None,
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

        let tenants_repo = Arc::new(tenants_repo);
        let tenant_user_repo = Arc::new(tenant_user_repo);
        let manager_user_repo = Arc::new(manager_user_repo);
        let migrator = Arc::new(migrator);
        let connection_tester = Arc::new(connection_tester);

        let mut tenants_module = MockTenantsModule::new();
        tenants_module
            .expect_config()
            .returning(move || config.clone());
        tenants_module
            .expect_tenants_repo()
            .returning(move || tenants_repo.clone());
        tenants_module
            .expect_tenant_user_repo()
            .returning(move || tenant_user_repo.clone());
        tenants_module
            .expect_manager_user_repo()
            .returning(move || manager_user_repo.clone());
        tenants_module
            .expect_migrator()
            .returning(move || migrator.clone());
        tenants_module
            .expect_connection_tester()
            .returning(move || connection_tester.clone());
        tenants_module
            .expect_add_tenant_pool()
            .times(1)
            .returning(|tenant_id, _| Ok(tenant_id));

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();

        let expected_response = serde_json::to_string(
            &SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::CREATED)
                .data(PublicTenantSelfHosted {
                    id: new_tenant_id,
                    name: "test".to_string(),
                    is_self_hosted: false,
                    db_host: "[MANAGED]".to_string(),
                    db_port: 0,
                    db_name: "[MANAGED]".to_string(),
                    db_user: "[MANAGED]".to_string(),
                    db_password: "[REDACTED]".to_string(),
                    db_max_pool_size: 5,
                    db_ssl_mode: "disable".to_string(),
                    created_at: now,
                    updated_at: now,
                    deleted_at: None,
                })
                .build()
                .unwrap(),
        )
        .unwrap();

        assert_eq!(&body[..], expected_response.as_bytes());
    }

    #[tokio::test]
    async fn test_call_endpoint_with_invalid_bearer() {
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

        let repo = Arc::new(repo);
        let migrator = Arc::new(migrator);
        let connection_tester = Arc::new(connection_tester);

        let mut tenants_module = MockTenantsModule::new();
        tenants_module
            .expect_config()
            .returning(move || config.clone());
        tenants_module
            .expect_tenants_repo()
            .returning(move || repo.clone());
        tenants_module
            .expect_migrator()
            .returning(move || migrator.clone());
        tenants_module
            .expect_connection_tester()
            .returning(move || connection_tester.clone());

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    #[ignore]
    async fn test_create_self_hosted_success() {
        let new_tenant_id = Uuid::new_v4();
        let now = Local::now();

        let mut tenants_repo = MockTenantsRepository::new();
        tenants_repo
            .expect_setup_self_hosted()
            .times(1)
            .withf(|name, _, _| name == "test")
            .returning(move |_, _, _| {
                Ok(Tenant {
                    id: new_tenant_id,
                    name: "test".to_string(),
                    is_self_hosted: true,
                    db_host: "example.com".to_string(),
                    db_port: 5432,
                    db_name: "tenant_1234567890".to_string(),
                    db_user: "tenant_1234567890".to_string(),
                    db_password: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
                    db_max_pool_size: 5,
                    db_ssl_mode: "verify-full".to_string(),
                    created_at: now,
                    updated_at: now,
                    deleted_at: None,
                })
            });

        let mut manager_user_repo = MockManagerUserRepository::new();

        manager_user_repo
            .expect_get_by_uuid()
            .times(1)
            .returning(|_| {
                Ok(ManagerUser {
                    id: Uuid::new_v4(),
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
                    is_mfa_enabled: false,
                    mfa_secret: None,
                })
            });

        let mut tenant_user_repo = MockTenantUserRepository::new();

        tenant_user_repo
            .expect_insert_from_manager()
            .times(1)
            .returning(|user, _| Ok(user));

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
            format!("{}-api", config.auth().jwt_audience().to_string()),
            Uuid::new_v4(),
            None,
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

        let tenants_repo = Arc::new(tenants_repo);
        let tenant_user_repo = Arc::new(tenant_user_repo);
        let manager_user_repo = Arc::new(manager_user_repo);
        let migrator = Arc::new(migrator);
        let connection_tester = Arc::new(connection_tester);

        let mut tenants_module = MockTenantsModule::new();
        tenants_module
            .expect_config()
            .returning(move || config.clone());
        tenants_module
            .expect_tenants_repo()
            .returning(move || tenants_repo.clone());
        tenants_module
            .expect_tenant_user_repo()
            .returning(move || tenant_user_repo.clone());
        tenants_module
            .expect_manager_user_repo()
            .returning(move || manager_user_repo.clone());
        tenants_module
            .expect_migrator()
            .returning(move || migrator.clone());
        tenants_module
            .expect_connection_tester()
            .returning(move || connection_tester.clone());
        tenants_module
            .expect_add_tenant_pool()
            .times(1)
            .returning(|tenant_id, _| Ok(tenant_id));

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();

        let expected_response = serde_json::to_string(
            &SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::CREATED)
                .data(PublicTenantSelfHosted {
                    id: new_tenant_id,
                    name: "test".to_string(),
                    is_self_hosted: true,
                    db_host: "example.com".to_string(),
                    db_port: 5432,
                    db_name: "tenant_1234567890".to_string(),
                    db_user: "tenant_1234567890".to_string(),
                    db_password: "[REDACTED]".to_string(),
                    db_max_pool_size: 5,
                    db_ssl_mode: "verify-full".to_string(),
                    created_at: now,
                    updated_at: now,
                    deleted_at: None,
                })
                .build()
                .unwrap(),
        )
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
            format!("{}-api", config.auth().jwt_audience().to_string()),
            jti,
            None,
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

        let repo = Arc::new(repo);
        let migrator = Arc::new(migrator);
        let connection_tester = Arc::new(connection_tester);
        let config_clone = config.clone();

        let mut tenants_module = MockTenantsModule::new();
        tenants_module
            .expect_config()
            .returning(move || config_clone.clone());
        tenants_module
            .expect_tenants_repo()
            .returning(move || repo.clone());
        tenants_module
            .expect_migrator()
            .returning(move || migrator.clone());
        tenants_module
            .expect_connection_tester()
            .returning(move || connection_tester.clone());

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

        let expected_response = serde_json::to_string(
            &SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(NewTokenResponse {
                    token: claims_new
                        .to_token(config.auth().jwt_secret().as_bytes())
                        .unwrap(),
                    claims: claims_new,
                })
                .build()
                .unwrap(),
        )
        .unwrap();

        assert_eq!(&body[..], expected_response.as_bytes());
    }
}
