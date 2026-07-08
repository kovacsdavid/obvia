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

use crate::common::dto::{EmptyType, SuccessResponseBuilder};
use crate::common::extractors::{UserInput, ValidJson};
use crate::common::handler::{HandlerResult, map_handler_err};
use crate::common::query_parser::{CommonRawQuery, ResourceQuery};
use crate::common::service::Service;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::tenants::TenantsModuleInterface;
use crate::manager::tenants::dto::{
    CreateTenant, CreateTenantHelper, PublicTenantManaged, TenantIdRequest,
};
use crate::manager::tenants::service::TenantService;
use crate::manager::tenants::types::{TenantFilterBy, TenantOrderBy};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use std::str::FromStr;
use std::sync::Arc;

pub async fn create<M: TenantsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tenants_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<CreateTenant, CreateTenantHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tenants_module.clone());
    let result = map_handler_err(
        service.create_managed(&user_input).await,
        tenants_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(PublicTenantManaged::from(result))
            .build(),
        tenants_module,
    )
    .await?
    .into_response())
}

pub async fn get<M: TenantsModuleInterface>(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(_tenants_module): State<Arc<M>>,
) -> Response {
    todo!();
}

pub async fn list<M: TenantsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tenants_module): State<Arc<M>>,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tenants_module.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<TenantOrderBy, TenantFilterBy>::from_str(payload.q()),
        tenants_module.clone(),
    )
    .await?;
    let (meta, data) = map_handler_err(
        service.get_paged(&resource_query).await,
        tenants_module.clone(),
    )
    .await?;

    Ok(map_handler_err(
        SuccessResponseBuilder::new()
            .status_code(StatusCode::OK)
            .meta(meta)
            .data(data)
            .build(),
        tenants_module,
    )
    .await?
    .into_response())
}

pub async fn activate<M: TenantsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tenants_module): State<Arc<M>>,
    ValidJson(payload): ValidJson<TenantIdRequest>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tenants_module.clone());
    let result = map_handler_err(service.activate(&payload).await, tenants_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        tenants_module,
    )
    .await?
    .into_response())
}

pub async fn delete<M: TenantsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tenants_module): State<Arc<M>>,
    ValidJson(payload): ValidJson<TenantIdRequest>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tenants_module.clone());
    let result =
        map_handler_err(service.delete(payload.uuid).await, tenants_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        tenants_module,
    )
    .await?
    .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::config::tests::AppConfigBuilder;
    use crate::common::handler::tests::{
        extract_json_response, generate_expired_jwt, generate_jwt_with_invalid_signature,
        generate_valid_jwt,
    };
    use crate::manager::auth::dto::claims::Claims;
    use crate::manager::tenants;
    use crate::manager::tenants::model::{Tenant, UserTenant};
    use crate::manager::tenants::repository::MockTenantsRepository;
    use crate::manager::tenants::tests::MockTenantsModule;
    use crate::manager::users::model::User as ManagerUser;
    use crate::manager::users::repository::MockUsersRepository as MockManagerUserRepository;
    use crate::tenant::users::model::User as TenantUser;
    use crate::tenant::users::repository::MockUsersRepository as MockTenantUserRepository;
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use chrono::Utc;
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::future::ready;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_managed_success() {
        let new_tenant_id = Uuid::new_v4();
        let now = Utc::now();
        let sub = Uuid::new_v4();
        let manager_user = ManagerUser {
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
                    timezone: "Europe/Budapest".to_string()
                };

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
            .with(eq(sub))
            .times(1)
            .returning({
                let manager_user = manager_user.clone();
                move |_| Ok(manager_user.clone())
            });

        let mut tenant_user_repo = MockTenantUserRepository::new();
        tenant_user_repo
            .expect_insert_from_manager()
            .with(eq(TenantUser::from(manager_user)))
            .times(1)
            .returning(Ok);

        let payload = serde_json::to_string(&CreateTenantHelper {
            name: "test".to_string(),
        })
        .unwrap();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_valid_jwt(Some(sub), None)),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/create")
            .body(Body::from(payload))
            .unwrap();

        let tenants_repo = Arc::new(tenants_repo);
        let tenant_user_repo = Arc::new(tenant_user_repo);
        let manager_user_repo = Arc::new(manager_user_repo);

        let test_config = AppConfigBuilder::default().build().unwrap();
        let mut tenants_module = MockTenantsModule::new();
        tenants_module
            .expect_config()
            .times(2)
            .return_const(test_config);
        tenants_module
            .expect_tenants_repo()
            .times(1)
            .returning(move || tenants_repo.clone());
        tenants_module
            .expect_tenant_user_repo()
            .times(1)
            .with(eq(new_tenant_id))
            .returning(move |_| Ok(tenant_user_repo.clone()));
        tenants_module
            .expect_manager_user_repo()
            .times(1)
            .returning(move || manager_user_repo.clone());
        tenants_module
            .expect_add_tenant_pool()
            .times(1)
            .returning(|tenant_id, _| Box::pin(ready(Ok(tenant_id))));
        tenants_module
            .expect_migrate_tenant_db()
            .times(1)
            .returning(|_| Box::pin(ready(Ok(()))));

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response_body = extract_json_response(response).await;
        let expected_data = PublicTenantManaged {
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
        };
        let expected_body = json!({
            "meta": null,
            "data": expected_data
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_managed_unauthorized_expired() {
        let payload = serde_json::to_string(&CreateTenantHelper {
            name: "test".to_string(),
        })
        .unwrap();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/create")
            .body(Body::from(payload))
            .unwrap();

        let mut tenants_module = MockTenantsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        tenants_module
            .expect_config()
            .times(1)
            .return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Hozzáférés megtagadva!"
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_managed_unauthorized_invalid_signature() {
        let payload = serde_json::to_string(&CreateTenantHelper {
            name: "test".to_string(),
        })
        .unwrap();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/create")
            .body(Body::from(payload))
            .unwrap();

        let mut tenants_module = MockTenantsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        tenants_module
            .expect_config()
            .times(1)
            .return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Hozzáférés megtagadva!"
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_managed_unauthorized_missing() {
        let payload = serde_json::to_string(&CreateTenantHelper {
            name: "test".to_string(),
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(MockTenantsModule::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_activate_success() {
        let sub = Uuid::new_v4();
        let active_tenant_id = Uuid::new_v4();

        let mut repo = MockTenantsRepository::new();
        repo.expect_get_user_active_tenant_by_id()
            .times(1)
            .withf(move |user_id, tenant_id| *user_id == sub && *tenant_id == active_tenant_id)
            .returning(|user_id, tenant_id| {
                Ok(Some(UserTenant {
                    id: Uuid::new_v4(),
                    user_id,
                    tenant_id,
                    role: "owner".to_string(),
                    invited_by: None,
                    last_activated: Utc::now(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                }))
            });

        let test_config = AppConfigBuilder::default().build().unwrap();

        let payload = serde_json::to_string(&TenantIdRequest {
            uuid: active_tenant_id,
        })
        .unwrap();

        let original_bearer = generate_valid_jwt(Some(sub), None);

        let original_claims = Claims::from_token(
            &original_bearer,
            test_config.auth().jwt_secret().as_bytes(),
            test_config.auth().jwt_issuer(),
            &format!("{}-api", test_config.auth().jwt_audience()),
        )
        .unwrap();

        let request = Request::builder()
            .header("Authorization", format!("Bearer {}", original_bearer))
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/activate")
            .body(Body::from(payload))
            .unwrap();

        let repo = Arc::new(repo);

        let mut tenants_module = MockTenantsModule::new();
        tenants_module
            .expect_config()
            .times(2)
            .return_const(test_config.clone());
        tenants_module
            .expect_tenants_repo()
            .times(1)
            .returning(move || repo.clone());

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let new_claims = original_claims
            .clone()
            .set_active_tenant(Some(active_tenant_id));
        let new_token = new_claims
            .to_token(test_config.auth().jwt_secret().as_bytes())
            .unwrap();

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": {
                "token": new_token,
                "claims": new_claims
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_activate_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();

        let test_config = AppConfigBuilder::default().build().unwrap();

        let payload = serde_json::to_string(&TenantIdRequest {
            uuid: active_tenant_id,
        })
        .unwrap();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/activate")
            .body(Body::from(payload))
            .unwrap();

        let mut tenants_module = MockTenantsModule::new();
        tenants_module
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Hozzáférés megtagadva!"
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_activate_unauthorized_invalid_signature() {
        let active_tenant_id = Uuid::new_v4();

        let test_config = AppConfigBuilder::default().build().unwrap();

        let payload = serde_json::to_string(&TenantIdRequest {
            uuid: active_tenant_id,
        })
        .unwrap();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/activate")
            .body(Body::from(payload))
            .unwrap();

        let mut tenants_module = MockTenantsModule::new();
        tenants_module
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(tenants_module))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Hozzáférés megtagadva!"
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_activate_unauthorized_missing() {
        let active_tenant_id = Uuid::new_v4();

        let payload = serde_json::to_string(&TenantIdRequest {
            uuid: active_tenant_id,
        })
        .unwrap();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/activate")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(MockTenantsModule::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
}
