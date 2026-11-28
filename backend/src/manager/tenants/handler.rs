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

use crate::common::AppStateInterface;
use crate::common::dto::{EmptyType, SuccessResponseBuilder};
use crate::common::extractors::{ClientContext, UserInput, ValidJson};
use crate::common::handler::{HandlerResult, map_handler_err};
use crate::common::query_parser::{CommonRawQuery, ResourceQuery};
use crate::common::service::BaseService;
use crate::manager::auth::middleware::AuthenticatedUser;
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
use tracing::instrument;

#[utoipa::path(
    post, path = "/api/tenants/create", tag = "Tenants",
    summary = "Create a managed tenant",
    request_body = CreateTenantHelper,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Tenant created", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::manager::tenants::dto::PublicTenantManaged>),
        (status = 401, description = "Authentication required"),
        (status = 422, description = "Invalid input")
    )
)]
#[instrument(name = "obvia::manager::tenants::handler::create", skip(app_state))]
pub async fn create<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    UserInput(user_input, _): UserInput<CreateTenant, CreateTenantHelper>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let result =
        map_handler_err(service.create_managed(&user_input).await, app_state.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(PublicTenantManaged::from(result))
            .build(),
        app_state,
    )
    .await?
    .into_response())
}

#[utoipa::path(
    get, path = "/api/tenants/get", tag = "Tenants",
    summary = "Get the active tenant",
    security(("bearer_auth" = [])),
    responses(
        (status = 501, description = "Not implemented")
    )
)]
#[instrument(name = "obvia::manager::tenants::handler::get", skip(_app_state))]
pub async fn get<A: AppStateInterface>(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(_app_state): State<Arc<A>>,
    _client_context: ClientContext,
) -> Response {
    todo!();
}

#[utoipa::path(
    get, path = "/api/tenants/get_resolved", tag = "Tenants",
    summary = "Get the active tenant with resolved details",
    security(("bearer_auth" = [])),
    responses(
        (status = 501, description = "Not implemented")
    )
)]
#[instrument(
    name = "obvia::manager::tenants::handler::get_resolved",
    skip(_app_state)
)]
pub async fn get_resolved<A: AppStateInterface>(
    AuthenticatedUser(_claims): AuthenticatedUser,
    State(_app_state): State<Arc<A>>,
    _client_context: ClientContext,
) -> Response {
    todo!();
}

#[utoipa::path(
    get, path = "/api/tenants/list", tag = "Tenants",
    summary = "List tenants",
    params(("q" = Option<String>, Query, description = "Tenant filtering, ordering, and paging expression")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Page of tenants", body = crate::common::dto::SuccessResponse<crate::common::dto::PaginatorMeta, Vec<crate::manager::tenants::dto::PublicTenant>>),
        (status = 401, description = "Authentication required")
    )
)]
#[instrument(name = "obvia::manager::tenants::handler::list", skip(app_state))]
pub async fn list<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<TenantOrderBy, TenantFilterBy>::from_str(payload.q()),
        app_state.clone(),
    )
    .await?;
    let (meta, data) =
        map_handler_err(service.get_paged(&resource_query).await, app_state.clone()).await?;

    Ok(map_handler_err(
        SuccessResponseBuilder::new()
            .status_code(StatusCode::OK)
            .meta(meta)
            .data(data)
            .build(),
        app_state,
    )
    .await?
    .into_response())
}

#[utoipa::path(
    post, path = "/api/tenants/activate", tag = "Tenants",
    summary = "Activate a tenant",
    request_body = TenantIdRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Tenant activated", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::manager::tenants::dto::NewTokenResponse>),
        (status = 401, description = "Authentication required"),
        (status = 422, description = "Invalid tenant ID")
    )
)]
#[instrument(name = "obvia::manager::tenants::handler::activate", skip(app_state))]
pub async fn activate<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    ValidJson(payload): ValidJson<TenantIdRequest>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let result = map_handler_err(service.activate(&payload).await, app_state.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        app_state,
    )
    .await?
    .into_response())
}

#[utoipa::path(
    post, path = "/api/tenants/delete", tag = "Tenants",
    summary = "Delete a tenant",
    request_body = TenantIdRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Tenant deleted", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::manager::tenants::dto::NewTokenResponse>),
        (status = 401, description = "Authentication required"),
        (status = 422, description = "Invalid tenant ID")
    )
)]
#[instrument(name = "obvia::manager::tenants::handler::delete", skip(app_state))]
pub async fn delete<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    ValidJson(payload): ValidJson<TenantIdRequest>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let result = map_handler_err(service.delete(payload.uuid).await, app_state.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        app_state,
    )
    .await?
    .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::config::tests::test_app_config_builder;
    use crate::common::dto::PaginatorMeta;
    use crate::common::handler::tests::{
        extract_json_response, generate_expired_jwt, generate_jwt_with_invalid_signature,
        generate_valid_jwt,
    };
    use crate::common::tests::MockAppState;
    use crate::manager::auth::dto::claims::Claims;
    use crate::manager::tenants;
    use crate::manager::tenants::dto::PublicTenant;
    use crate::manager::tenants::model::{Tenant, UserTenant};
    use crate::manager::tenants::repository::MockTenantsRepository;
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
        let new_tenant_id = Uuid::now_v7();
        let now = Utc::now();
        let sub = Uuid::now_v7();
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

        let test_config = test_app_config_builder().build().unwrap();
        let mut app_state = MockAppState::new();
        app_state.expect_config().times(2).return_const(test_config);
        app_state
            .expect_tenants_repo()
            .times(1)
            .returning(move || tenants_repo.clone());

        app_state
            .expect_tenant_users_repo()
            .times(1)
            .with(eq(new_tenant_id))
            .returning(move |_| Ok(tenant_user_repo.clone()));
        app_state
            .expect_manager_users_repo()
            .times(1)
            .returning(move || manager_user_repo.clone());
        app_state
            .expect_add_tenant_pool()
            .times(1)
            .returning(|tenant_id, _| Box::pin(ready(Ok(tenant_id))));
        app_state
            .expect_migrate_tenant_db()
            .times(1)
            .returning(|_| Box::pin(ready(Ok(()))));

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
            Router::new().merge(tenants::routes::routes(Arc::new(MockAppState::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    // TODO: test_get_success

    #[tokio::test]
    async fn test_get_unauthorized_expired() {
        let tenant_id = Uuid::now_v7();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tenants/get?uuid=${tenant_id}"))
            .body("".to_string())
            .unwrap();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
    async fn test_get_unauthorized_invalid_signature() {
        let tenant_id = Uuid::now_v7();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tenants/get?uuid=${tenant_id}"))
            .body("".to_string())
            .unwrap();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
    async fn test_get_unauthorized_missing() {
        let tenant_id = Uuid::now_v7();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tenants/get?uuid=${tenant_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(MockAppState::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    // TODO: test_get_resolved_success

    #[tokio::test]
    async fn test_get_resolved_unauthorized_expired() {
        let tenant_id = Uuid::now_v7();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tenants/get_resolved?uuid=${tenant_id}"))
            .body("".to_string())
            .unwrap();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
    async fn test_get_resolved_unauthorized_invalid_signature() {
        let tenant_id = Uuid::now_v7();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tenants/get_resolved?uuid=${tenant_id}"))
            .body("".to_string())
            .unwrap();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
    async fn test_get_resolved_unauthorized_missing() {
        let tenant_id = Uuid::now_v7();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tenants/get_resolved?uuid=${tenant_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(MockAppState::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_success() {
        let sub = Uuid::now_v7();
        let tenant_id = Uuid::now_v7();
        let mut repo = MockTenantsRepository::new();
        let utc_now = Utc::now();
        let paginator_meta = PaginatorMeta {
            page: 1,
            limit: 25,
            total: 100,
        };
        let tenant = Tenant {
            id: tenant_id,
            name: "Test tenant".to_string(),
            is_self_hosted: false,
            db_host: "localhost".to_string(),
            db_port: 5432,
            db_name: "test_db".to_string(),
            db_user: "test_user".to_string(),
            db_password: "test_password".to_string(),
            db_max_pool_size: 3,
            db_ssl_mode: "verify-full".to_string(),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        repo.expect_get_all_by_user_id()
            .times(1)
            .withf(move |user_id, _| *user_id == sub)
            .returning({
                let tenant = tenant.clone();
                move |_, _| Ok((paginator_meta, vec![tenant.clone()]))
            });
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_valid_jwt(Some(sub), None)),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/tenants/list")
            .body("".to_string())
            .unwrap();

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        app_state
            .expect_tenants_repo()
            .times(1)
            .returning(move || repo.clone());
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": paginator_meta,
            "data": [PublicTenant::from(tenant)]
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/tenants/list")
            .body("".to_string())
            .unwrap();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
    async fn test_list_unauthorized_invalid_signature() {
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/tenants/list")
            .body("".to_string())
            .unwrap();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
    async fn test_list_unauthorized_missing() {
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/tenants/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(MockAppState::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_activate_success() {
        let sub = Uuid::now_v7();
        let active_tenant_id = Uuid::now_v7();

        let mut repo = MockTenantsRepository::new();
        repo.expect_get_user_active_tenant_by_id()
            .times(1)
            .withf(move |user_id, tenant_id| *user_id == sub && *tenant_id == active_tenant_id)
            .returning(|user_id, tenant_id| {
                Ok(Some(UserTenant {
                    id: Uuid::now_v7(),
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

        let test_config = test_app_config_builder().build().unwrap();

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

        let mut app_state = MockAppState::new();
        app_state
            .expect_config()
            .times(2)
            .return_const(test_config.clone());
        app_state
            .expect_tenants_repo()
            .times(1)
            .returning(move || repo.clone());

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
        let active_tenant_id = Uuid::now_v7();

        let test_config = test_app_config_builder().build().unwrap();

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

        let mut app_state = MockAppState::new();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
        let active_tenant_id = Uuid::now_v7();

        let test_config = test_app_config_builder().build().unwrap();

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

        let mut app_state = MockAppState::new();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
        let active_tenant_id = Uuid::now_v7();

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
            Router::new().merge(tenants::routes::routes(Arc::new(MockAppState::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_success() {
        let sub = Uuid::now_v7();
        let tenant_id = Uuid::now_v7();
        let utc_now = Utc::now();

        let mut repo = MockTenantsRepository::new();
        repo.expect_delete()
            .times(1)
            .with(eq(tenant_id), eq(sub))
            .returning(move |_, _| {
                Ok(Tenant {
                    id: tenant_id,
                    name: "Test tenant".to_string(),
                    is_self_hosted: false,
                    db_host: "localhost".to_string(),
                    db_port: 5432,
                    db_name: "test_db".to_string(),
                    db_user: "test_user".to_string(),
                    db_password: "test_password".to_string(),
                    db_max_pool_size: 3,
                    db_ssl_mode: "verify-full".to_string(),
                    created_at: utc_now,
                    updated_at: utc_now,
                    deleted_at: None,
                })
            });

        let repo = Arc::new(repo);
        let mut app_state = MockAppState::new();
        app_state
            .expect_tenants_repo()
            .times(1)
            .returning(move || repo.clone());
        app_state
            .expect_delete_tenant_pool()
            .times(1)
            .with(eq(tenant_id))
            .returning(|_| Box::pin(ready(Ok(()))));
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(2)
            .return_const(test_config.clone());

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
        );

        let payload = json!({
            "uuid": tenant_id
        });

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_valid_jwt(Some(sub), None)),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tenants/delete")
            .body(Body::from(payload.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let data = response_body.get("data").unwrap();
        let claims = Claims::from_token(
            data["token"].as_str().unwrap(),
            test_config.auth().jwt_secret().as_bytes(),
            test_config.auth().jwt_issuer(),
            &format!("{}-api", test_config.auth().jwt_audience()),
        )
        .unwrap();

        assert_eq!(claims.active_tenant(), None);

        let expected_body = json!({
            "meta": null,
            "data": {
                "token": data["token"],
                "claims": claims
            },
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_expired() {
        let tenant_id = Uuid::now_v7();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tenants/delete?uuid=${tenant_id}"))
            .body("".to_string())
            .unwrap();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
    async fn test_delete_unauthorized_invalid_signature() {
        let tenant_id = Uuid::now_v7();

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tenants/delete?uuid=${tenant_id}"))
            .body("".to_string())
            .unwrap();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state.expect_config().times(1).return_const(test_config);

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(app_state))),
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
    async fn test_delete_unauthorized_missing() {
        let tenant_id = Uuid::now_v7();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tenants/delete?uuid=${tenant_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tenants::routes::routes(Arc::new(MockAppState::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
}
