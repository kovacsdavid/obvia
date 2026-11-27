/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

use std::{collections::HashMap, str::FromStr, sync::Arc};

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use tracing::instrument;

use crate::{
    common::{
        AppStateInterface,
        dto::{EmptyType, SimpleMessageResponse, SuccessResponseBuilder, UuidParam},
        extractors,
        handler::{HandlerResult, map_handler_err},
        query_parser::{CommonRawQuery, ResourceQuery},
        service::BaseService,
    },
    manager::auth::middleware::AuthenticatedUser,
    tenant::handover_and_acceptance::{
        Service, UserInput, UserInputHelper,
        types::{FilterBy, OrderBy},
    },
};

#[utoipa::path(
    get, path = "/api/handover_and_acceptance/get_full", tag = "Handover and Acceptance",
    summary = "Get a handover and acceptance record",
    params(("uuid" = uuid::Uuid, Query, description = "Record ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Full record", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::common::dto::JsonObjectSchema>),
        (status = 401, description = "Authentication required"),
        (status = 404, description = "Record not found")
    )
)]
#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::get_full",
    skip(app_state)
)]
pub async fn get_full<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: extractors::ClientContext,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let result = map_handler_err(service.get_full(payload.uuid).await, app_state.clone()).await?;
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
    get, path = "/api/handover_and_acceptance/list", tag = "Handover and Acceptance",
    summary = "List handover and acceptance records",
    params(("q" = Option<String>, Query, description = "Optional filtering, ordering, and paging expression")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Page of records", body = crate::common::dto::SuccessResponse<crate::common::dto::PaginatorMeta, Vec<crate::common::dto::JsonObjectSchema>>),
        (status = 401, description = "Authentication required")
    )
)]
#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::list",
    skip(app_state)
)]
pub async fn list<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: extractors::ClientContext,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<OrderBy, FilterBy>::from_str(payload.q()),
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
    get, path = "/api/handover_and_acceptance/select_list", tag = "Handover and Acceptance",
    summary = "Get handover and acceptance select-list items",
    params(("list" = Option<String>, Query, description = "Select-list type"), ("param" = Option<String>, Query, description = "Optional select-list parameter")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Select-list items", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, Vec<crate::common::dto::JsonObjectSchema>>),
        (status = 401, description = "Authentication required")
    )
)]
#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::select_list",
    skip(app_state)
)]
pub async fn select_list<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: extractors::ClientContext,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let list_type = payload.get("list").map(|v| v.as_str());
    let list_param = payload.get("param").map(|v| v.as_str());

    let result = map_handler_err(
        service.get_select_list_items(list_type, list_param).await,
        app_state.clone(),
    )
    .await?;
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
    post, path = "/api/handover_and_acceptance/create", tag = "Handover and Acceptance",
    summary = "Create a handover and acceptance record",
    request_body = UserInputHelper,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Record created", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::common::dto::JsonObjectSchema>),
        (status = 401, description = "Authentication required"),
        (status = 422, description = "Invalid input")
    )
)]
#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::create",
    skip(app_state)
)]
pub async fn create<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: extractors::ClientContext,
    extractors::UserInput(user_input, _): extractors::UserInput<UserInput, UserInputHelper>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let result = map_handler_err(service.insert(&user_input).await, app_state.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(result)
            .build(),
        app_state,
    )
    .await?
    .into_response())
}

#[utoipa::path(
    put, path = "/api/handover_and_acceptance/update", tag = "Handover and Acceptance",
    summary = "Update a handover and acceptance record",
    request_body = UserInputHelper,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Record updated", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::common::dto::JsonObjectSchema>),
        (status = 401, description = "Authentication required"),
        (status = 404, description = "Record not found"),
        (status = 422, description = "Invalid input")
    )
)]
#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::update",
    skip(app_state)
)]
pub async fn update<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: extractors::ClientContext,
    extractors::UserInput(user_input, _): extractors::UserInput<UserInput, UserInputHelper>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let result = map_handler_err(service.update(&user_input).await, app_state.clone()).await?;
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
    delete, path = "/api/handover_and_acceptance/delete", tag = "Handover and Acceptance",
    summary = "Delete a handover and acceptance record",
    params(("uuid" = uuid::Uuid, Query, description = "Record ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Record deleted", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::common::dto::JsonObjectSchema>),
        (status = 401, description = "Authentication required"),
        (status = 404, description = "Record not found")
    )
)]
#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::delete",
    skip(app_state)
)]
pub async fn delete<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: extractors::ClientContext,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    map_handler_err(service.delete(payload.uuid).await, app_state.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(SimpleMessageResponse::new(
                "A vevő törlése sikeresen megtörtént",
            ))
            .build(),
        app_state,
    )
    .await?
    .into_response())
}

#[utoipa::path(
    get, path = "/api/handover_and_acceptance/print", tag = "Handover and Acceptance",
    summary = "Print a record as PDF",
    params(("uuid" = uuid::Uuid, Query, description = "Record ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "PDF document", body = crate::common::api_docs::PdfDocument, content_type = "application/pdf", headers(("Content-Disposition" = String))),
        (status = 401, description = "Authentication required"),
        (status = 404, description = "Record not found")
    )
)]
#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::print",
    skip(app_state)
)]
pub async fn print<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: extractors::ClientContext,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());

    let pdf = map_handler_err(service.print(payload.uuid).await, app_state).await?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!(r#"inline; filename="{}""#, payload.uuid)
            .parse()
            .unwrap(),
    );
    Ok((StatusCode::OK, headers, pdf).into_response())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{Read, Write},
        path::Path,
    };

    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
    };
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        common::{
            TEST_TIME, TEST_TZ,
            config::tests::test_app_config_builder,
            dto::PaginatorMeta,
            error::RepositoryError,
            handler::tests::{
                extract_json_response, generate_expired_jwt, generate_jwt_with_invalid_signature,
                generate_valid_jwt,
            },
            pdf::{
                MockPdfGenerator, PdfGenerator, PdfTemplates,
                tests::{PDF_GENERATOR_TEST_SYNC, extract_pdf_text},
            },
            query_parser::ResourceQuery,
            tests::MockAppState,
        },
        tenant::{
            address::model::test_address_resolved_builder,
            customers::model::tests::test_customer_full_builder,
            handover_and_acceptance::{
                self, Print, UserInput, UserInputHelper,
                model::tests::{
                    test_handover_and_acceptance_builder, test_handover_and_acceptance_full_builder,
                },
                repository::MockRepository,
            },
            owner_profile::{
                dto::print::OwnerProfileFullPrint, model::tests::test_owner_profile_full_builder,
                repository::MockOwnerProfileRepository,
            },
            users::model::tests::test_user_builder,
        },
    };

    use super::*;

    #[tokio::test]
    async fn test_get_full_success() {
        let active_tenant_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let handover_and_acceptance_full = test_handover_and_acceptance_full_builder(
            test_customer_full_builder().build().unwrap(),
        )
        .id(handover_and_acceptance_id)
        .build()
        .unwrap();

        let mut handover_and_acceptance_repo = MockRepository::new();
        handover_and_acceptance_repo
            .expect_get_full()
            .times(1)
            .with(eq(handover_and_acceptance_id))
            .returning({
                let handover_and_acceptance_full = handover_and_acceptance_full.clone();
                move |_| Ok(handover_and_acceptance_full.clone())
            });

        let mut app_state = MockAppState::new();
        let handover_and_acceptance_repo = Arc::new(handover_and_acceptance_repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_handover_and_acceptance_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(handover_and_acceptance_repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(None, Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/handover_and_acceptance/get_full?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": handover_and_acceptance_full
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_full_unauthorized_expired() {
        let handover_and_acceptance_id = Uuid::now_v7();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/handover_and_acceptance/get_full?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
    async fn test_get_full_unauthorized_invalid_signature() {
        let handover_and_acceptance_id = Uuid::now_v7();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/handover_and_acceptance/get_full?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
    async fn test_get_full_unauthorized_missing() {
        let handover_and_acceptance_id = Uuid::now_v7();
        let app_state = MockAppState::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/handover_and_acceptance/get_full?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_get_full_not_found() {
        let active_tenant_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let mut repo = MockRepository::new();
        repo.expect_get_full()
            .times(1)
            .with(eq(handover_and_acceptance_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_handover_and_acceptance_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(None, Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/handover_and_acceptance/get_full?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Nem található"
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_success() {
        assert_list_success("").await;
    }

    #[tokio::test]
    async fn test_list_success_with_display_id_filter() {
        assert_list_success(
            "filtering:display_id-DEMO-AT-2026-1 ordering:created_at-asc paging:1-25",
        )
        .await;
    }

    async fn assert_list_success(query: &str) {
        let active_tenant_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let paginator_meta = PaginatorMeta {
            page: 1,
            limit: 25,
            total: 100,
        };

        let handover_and_acceptance_full = test_handover_and_acceptance_full_builder(
            test_customer_full_builder().build().unwrap(),
        )
        .id(handover_and_acceptance_id)
        .build()
        .unwrap();

        let expected_query = query
            .parse::<ResourceQuery<
                handover_and_acceptance::types::OrderBy,
                handover_and_acceptance::types::FilterBy,
            >>()
            .unwrap();
        if !query.is_empty() {
            assert_eq!(expected_query.filtering().filter_by(), Some("display_id"));
            assert_eq!(
                expected_query.filtering().value_unchecked(),
                Some("DEMO-AT-2026-1")
            );
            assert_eq!(
                expected_query.ordering().order_by().as_deref(),
                Some("created_at")
            );
            assert_eq!(expected_query.paging().page(), Some(1));
            assert_eq!(expected_query.paging().limit(), Some(25));
        }

        let mut handover_and_acceptance_repo = MockRepository::new();
        handover_and_acceptance_repo
            .expect_get_paged()
            .times(1)
            .with(eq(expected_query))
            .returning({
                let handover_and_acceptance_resolved = handover_and_acceptance_full.clone();
                move |_| {
                    Ok((
                        paginator_meta,
                        vec![handover_and_acceptance_resolved.clone()],
                    ))
                }
            });

        let mut app_state = MockAppState::new();
        let handover_and_acceptance_repo = Arc::new(handover_and_acceptance_repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_handover_and_acceptance_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(handover_and_acceptance_repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(None, Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/handover_and_acceptance/list?q={}",
                query.replace(' ', "%20")
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": paginator_meta,
            "data": vec![handover_and_acceptance_full]
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/handover_and_acceptance/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/handover_and_acceptance/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
        let app_state = MockAppState::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/handover_and_acceptance/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_list_not_found() {
        let active_tenant_id = Uuid::now_v7();

        let mut repo = MockRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<
                    handover_and_acceptance::types::OrderBy,
                    handover_and_acceptance::types::FilterBy,
                >>()
                .unwrap()))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_handover_and_acceptance_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(None, Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/handover_and_acceptance/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Nem található"
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_success() {
        let customer_id = Uuid::now_v7();
        let item_make_id = Uuid::now_v7();
        let item_model_id = Uuid::now_v7();
        let item_identifier_type_id = Uuid::now_v7();
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let user_input_helper = UserInputHelper {
            id: None,
            customer_id: customer_id.to_string(),
            item_make_id: item_make_id.to_string(),
            new_item_make: "".to_string(),
            item_model_id: item_model_id.to_string(),
            new_item_model: "".to_string(),
            item_identifier_type_id: item_identifier_type_id.to_string(),
            new_item_identifier_type: "".to_string(),
            item_identifier_value: "ASD123456789".to_string(),
            reported_issue: "Some reported issue".to_string(),
            condition_at_receipt: "Condition at receipt".to_string(),
            included_accessories: vec!["chargeing cable".to_string(), "laptop bag".to_string()],
            authorized_work: "Some work to do.".to_string(),
            repair_limit: "1000".to_string(),
            currency_code: "HUF".to_string(),
            terms_version: "v1".to_string(),
            received_at: TEST_TIME.to_rfc3339(),
        };
        let user_input = UserInput::try_from(user_input_helper.clone()).unwrap();

        let handover_and_acceptance = test_handover_and_acceptance_builder()
            .id(handover_and_acceptance_id)
            .build()
            .unwrap();

        let mut repo = MockRepository::new();
        repo.expect_insert()
            .times(1)
            .withf({
                let user_input_expected = user_input.clone();
                move |user_input, user_id_inner| {
                    user_input.id == user_input_expected.id
                        && user_input.customer_id == user_input_expected.customer_id
                        && user_input.item_make_id == user_input_expected.item_make_id
                        && user_input.new_item_make == user_input_expected.new_item_make
                        && user_input.item_model_id == user_input_expected.item_model_id
                        && user_input.new_item_model == user_input_expected.new_item_model
                        && user_input.item_identifier_type_id
                            == user_input_expected.item_identifier_type_id
                        && user_input.new_item_identifier_type
                            == user_input_expected.new_item_identifier_type
                        && user_input.item_identifier_value
                            == user_input_expected.item_identifier_value
                        && user_input.reported_issue == user_input_expected.reported_issue
                        && user_input.condition_at_receipt
                            == user_input_expected.condition_at_receipt
                        && user_input.included_accessories
                            == user_input_expected.included_accessories
                        && user_input.authorized_work == user_input_expected.authorized_work
                        && user_input.repair_limit == user_input_expected.repair_limit
                        && user_input.currency_code == user_input_expected.currency_code
                        && user_input.terms_version == user_input_expected.terms_version
                        && user_input.received_at == user_input_expected.received_at
                        && user_id == *user_id_inner
                }
            })
            .returning({
                let handover_and_acceptance = handover_and_acceptance.clone();
                move |_, _| Ok(handover_and_acceptance.clone())
            });

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_handover_and_acceptance_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(user_id), Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/handover_and_acceptance/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": handover_and_acceptance,
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_invalid_user_input() {
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let user_input_helper = UserInputHelper {
            id: None,
            customer_id: "".to_string(),
            item_make_id: "other".to_string(),
            new_item_make: "".to_string(),
            item_model_id: "asdf".to_string(),
            new_item_model: "".to_string(),
            item_identifier_type_id: "".to_string(),
            new_item_identifier_type: "".to_string(),
            item_identifier_value: "".to_string(),
            reported_issue: "Some reported issue".to_string(),
            condition_at_receipt: "Condition at receipt".to_string(),
            included_accessories: vec!["chargeing cable".to_string(), "laptop bag".to_string()],
            authorized_work: "Some work to do.".to_string(),
            repair_limit: "1000".to_string(),
            currency_code: "HUF".to_string(),
            terms_version: "v1".to_string(),
            received_at: TEST_TIME.to_rfc3339(),
        };

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(user_id), Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/handover_and_acceptance/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let field_errors = UserInput::try_from(user_input_helper).unwrap_err();

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Kérjük ellenőrizze a hibás mezőket!",
                "fields": field_errors
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_unauthorized_expired() {
        let customer_id = Uuid::now_v7();
        let item_make_id = Uuid::now_v7();
        let item_model_id = Uuid::now_v7();
        let item_identifier_type_id = Uuid::now_v7();

        let user_input_helper = UserInputHelper {
            id: None,
            customer_id: customer_id.to_string(),
            item_make_id: item_make_id.to_string(),
            new_item_make: "".to_string(),
            item_model_id: item_model_id.to_string(),
            new_item_model: "".to_string(),
            item_identifier_type_id: item_identifier_type_id.to_string(),
            new_item_identifier_type: "".to_string(),
            item_identifier_value: "ASD123456789".to_string(),
            reported_issue: "Some reported issue".to_string(),
            condition_at_receipt: "Condition at receipt".to_string(),
            included_accessories: vec!["chargeing cable".to_string(), "laptop bag".to_string()],
            authorized_work: "Some work to do.".to_string(),
            repair_limit: "1000".to_string(),
            currency_code: "HUF".to_string(),
            terms_version: "v1".to_string(),
            received_at: TEST_TIME.to_rfc3339(),
        };

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/handover_and_acceptance/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
    async fn test_create_unauthorized_invalid_signature() {
        let customer_id = Uuid::now_v7();
        let item_make_id = Uuid::now_v7();
        let item_model_id = Uuid::now_v7();
        let item_identifier_type_id = Uuid::now_v7();

        let user_input_helper = UserInputHelper {
            id: None,
            customer_id: customer_id.to_string(),
            item_make_id: item_make_id.to_string(),
            new_item_make: "".to_string(),
            item_model_id: item_model_id.to_string(),
            new_item_model: "".to_string(),
            item_identifier_type_id: item_identifier_type_id.to_string(),
            new_item_identifier_type: "".to_string(),
            item_identifier_value: "ASD123456789".to_string(),
            reported_issue: "Some reported issue".to_string(),
            condition_at_receipt: "Condition at receipt".to_string(),
            included_accessories: vec!["chargeing cable".to_string(), "laptop bag".to_string()],
            authorized_work: "Some work to do.".to_string(),
            repair_limit: "1000".to_string(),
            currency_code: "HUF".to_string(),
            terms_version: "v1".to_string(),
            received_at: TEST_TIME.to_rfc3339(),
        };

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/handover_and_acceptance/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
    async fn test_create_unauthorized_missing() {
        let customer_id = Uuid::now_v7();
        let item_make_id = Uuid::now_v7();
        let item_model_id = Uuid::now_v7();
        let item_identifier_type_id = Uuid::now_v7();

        let user_input_helper = UserInputHelper {
            id: None,
            customer_id: customer_id.to_string(),
            item_make_id: item_make_id.to_string(),
            new_item_make: "".to_string(),
            item_model_id: item_model_id.to_string(),
            new_item_model: "".to_string(),
            item_identifier_type_id: item_identifier_type_id.to_string(),
            new_item_identifier_type: "".to_string(),
            item_identifier_value: "ASD123456789".to_string(),
            reported_issue: "Some reported issue".to_string(),
            condition_at_receipt: "Condition at receipt".to_string(),
            included_accessories: vec!["chargeing cable".to_string(), "laptop bag".to_string()],
            authorized_work: "Some work to do.".to_string(),
            repair_limit: "1000".to_string(),
            currency_code: "HUF".to_string(),
            terms_version: "v1".to_string(),
            received_at: TEST_TIME.to_rfc3339(),
        };

        let app_state = MockAppState::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/handover_and_acceptance/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_success() {
        let customer_id = Uuid::now_v7();
        let item_make_id = Uuid::now_v7();
        let item_model_id = Uuid::now_v7();
        let item_identifier_type_id = Uuid::now_v7();
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let user_input_helper = UserInputHelper {
            id: Some(handover_and_acceptance_id.to_string()),
            customer_id: customer_id.to_string(),
            item_make_id: item_make_id.to_string(),
            new_item_make: "".to_string(),
            item_model_id: item_model_id.to_string(),
            new_item_model: "".to_string(),
            item_identifier_type_id: item_identifier_type_id.to_string(),
            new_item_identifier_type: "".to_string(),
            item_identifier_value: "ASD123456789".to_string(),
            reported_issue: "Some reported issue".to_string(),
            condition_at_receipt: "Condition at receipt".to_string(),
            included_accessories: vec!["chargeing cable".to_string(), "laptop bag".to_string()],
            authorized_work: "Some work to do.".to_string(),
            repair_limit: "1000".to_string(),
            currency_code: "HUF".to_string(),
            terms_version: "v1".to_string(),
            received_at: TEST_TIME.to_rfc3339(),
        };
        let user_input = UserInput::try_from(user_input_helper.clone()).unwrap();

        let handover_and_acceptance = test_handover_and_acceptance_builder()
            .id(handover_and_acceptance_id)
            .build()
            .unwrap();

        let mut repo = MockRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input), eq(user_id))
            .returning({
                let handover_and_acceptance = handover_and_acceptance.clone();
                move |_, _| Ok(handover_and_acceptance.clone())
            });

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_handover_and_acceptance_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(user_id), Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/handover_and_acceptance/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": handover_and_acceptance
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_invalid_user_input() {
        let customer_id = Uuid::now_v7();
        let item_make_id = Uuid::now_v7();
        let item_model_id = Uuid::now_v7();
        let item_identifier_type_id = Uuid::now_v7();
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let user_input_helper = UserInputHelper {
            id: None,
            customer_id: customer_id.to_string(),
            item_make_id: item_make_id.to_string(),
            new_item_make: "".to_string(),
            item_model_id: item_model_id.to_string(),
            new_item_model: "".to_string(),
            item_identifier_type_id: item_identifier_type_id.to_string(),
            new_item_identifier_type: "".to_string(),
            item_identifier_value: "ASD123456789".to_string(),
            reported_issue: "Some reported issue".to_string(),
            condition_at_receipt: "Condition at receipt".to_string(),
            included_accessories: vec!["chargeing cable".to_string(), "laptop bag".to_string()],
            authorized_work: "Some work to do.".to_string(),
            repair_limit: "1000".to_string(),
            currency_code: "HUF".to_string(),
            terms_version: "v1".to_string(),
            received_at: TEST_TIME.to_rfc3339(),
        };

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(user_id), Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/handover_and_acceptance/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Hiba történt az adatok feldolgozása során: Az azonosító megadása kötelező!",
            }
        });

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_update_unauthorized_expired() {
        let customer_id = Uuid::now_v7();
        let item_make_id = Uuid::now_v7();
        let item_model_id = Uuid::now_v7();
        let item_identifier_type_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let user_input_helper = UserInputHelper {
            id: Some(handover_and_acceptance_id.to_string()),
            customer_id: customer_id.to_string(),
            item_make_id: item_make_id.to_string(),
            new_item_make: "".to_string(),
            item_model_id: item_model_id.to_string(),
            new_item_model: "".to_string(),
            item_identifier_type_id: item_identifier_type_id.to_string(),
            new_item_identifier_type: "".to_string(),
            item_identifier_value: "ASD123456789".to_string(),
            reported_issue: "Some reported issue".to_string(),
            condition_at_receipt: "Condition at receipt".to_string(),
            included_accessories: vec!["chargeing cable".to_string(), "laptop bag".to_string()],
            authorized_work: "Some work to do.".to_string(),
            repair_limit: "1000".to_string(),
            currency_code: "HUF".to_string(),
            terms_version: "v1".to_string(),
            received_at: TEST_TIME.to_rfc3339(),
        };

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/handover_and_acceptance/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
    async fn test_update_unauthorized_invalid_signature() {
        let customer_id = Uuid::now_v7();
        let item_make_id = Uuid::now_v7();
        let item_model_id = Uuid::now_v7();
        let item_identifier_type_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let user_input_helper = UserInputHelper {
            id: Some(handover_and_acceptance_id.to_string()),
            customer_id: customer_id.to_string(),
            item_make_id: item_make_id.to_string(),
            new_item_make: "".to_string(),
            item_model_id: item_model_id.to_string(),
            new_item_model: "".to_string(),
            item_identifier_type_id: item_identifier_type_id.to_string(),
            new_item_identifier_type: "".to_string(),
            item_identifier_value: "ASD123456789".to_string(),
            reported_issue: "Some reported issue".to_string(),
            condition_at_receipt: "Condition at receipt".to_string(),
            included_accessories: vec!["chargeing cable".to_string(), "laptop bag".to_string()],
            authorized_work: "Some work to do.".to_string(),
            repair_limit: "1000".to_string(),
            currency_code: "HUF".to_string(),
            terms_version: "v1".to_string(),
            received_at: TEST_TIME.to_rfc3339(),
        };

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/handover_and_acceptance/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
    async fn test_update_unauthorized_missing() {
        let customer_id = Uuid::now_v7();
        let item_make_id = Uuid::now_v7();
        let item_model_id = Uuid::now_v7();
        let item_identifier_type_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let user_input_helper = UserInputHelper {
            id: Some(handover_and_acceptance_id.to_string()),
            customer_id: customer_id.to_string(),
            item_make_id: item_make_id.to_string(),
            new_item_make: "".to_string(),
            item_model_id: item_model_id.to_string(),
            new_item_model: "".to_string(),
            item_identifier_type_id: item_identifier_type_id.to_string(),
            new_item_identifier_type: "".to_string(),
            item_identifier_value: "ASD123456789".to_string(),
            reported_issue: "Some reported issue".to_string(),
            condition_at_receipt: "Condition at receipt".to_string(),
            included_accessories: vec!["chargeing cable".to_string(), "laptop bag".to_string()],
            authorized_work: "Some work to do.".to_string(),
            repair_limit: "1000".to_string(),
            currency_code: "HUF".to_string(),
            terms_version: "v1".to_string(),
            received_at: TEST_TIME.to_rfc3339(),
        };

        let app_state = MockAppState::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/handover_and_acceptance/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_delete_success() {
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();
        let mut repo = MockRepository::new();

        repo.expect_delete_by_id()
            .times(1)
            .with(eq(handover_and_acceptance_id))
            .returning(move |_| Ok(()));

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_handover_and_acceptance_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(user_id), Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!(
                "/api/handover_and_acceptance/delete?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": {"message": "A vevő törlése sikeresen megtörtént"}
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_invalid_user_input() {
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(user_id), Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri("/api/handover_and_acceptance/delete?uuid=invalid_user_input")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_expired() {
        let handover_and_acceptance_id = Uuid::now_v7();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!(
                "/api/handover_and_acceptance/delete?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
        let handover_and_acceptance_id = Uuid::now_v7();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!(
                "/api/handover_and_acceptance/delete?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
        let handover_and_acceptance_id = Uuid::now_v7();

        let app_state = MockAppState::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!(
                "/api/handover_and_acceptance/delete?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_print_success() {
        let active_tenant_id = Uuid::now_v7();
        let handover_and_acceptance_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse().unwrap();
        let owner_profile_id = "de6c4563-640e-483c-ad2d-d47b1c9115b7".parse().unwrap();
        let customer_id = "01a1f891-a6e5-733e-b525-41a33aa00cb3".parse().unwrap();

        let handover_and_acceptance_full = test_handover_and_acceptance_full_builder(
            test_customer_full_builder()
                .id(customer_id)
                .billing_address(Some(test_address_resolved_builder().build().unwrap()))
                .build()
                .unwrap(),
        )
        .id(handover_and_acceptance_id)
        .accepted_by(test_user_builder().build().unwrap())
        .build()
        .unwrap();

        let mut repo = MockRepository::new();
        repo.expect_get_full()
            .times(1)
            .with(eq(handover_and_acceptance_id))
            .returning({
                let handover_and_acceptance_full = handover_and_acceptance_full.clone();
                move |_| Ok(handover_and_acceptance_full.clone())
            });

        let mut owner_profile_repo = MockOwnerProfileRepository::new();
        let owner_profile_full = test_owner_profile_full_builder()
            .id(owner_profile_id)
            .billing_address(Some(test_address_resolved_builder().build().unwrap()))
            .build()
            .unwrap();
        owner_profile_repo.expect_get_full().times(1).returning({
            let owner_profile_full = owner_profile_full.clone();
            move || Ok(owner_profile_full.clone())
        });

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let owner_profile_repo = Arc::new(owner_profile_repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_handover_and_acceptance_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_owner_profile_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(owner_profile_repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let owner_profile_full = test_owner_profile_full_builder()
            .id(owner_profile_id)
            .billing_address(Some(test_address_resolved_builder().build().unwrap()))
            .build()
            .unwrap();

        let owner_profile_full_print = OwnerProfileFullPrint::new(owner_profile_full, *TEST_TZ);

        let pdf_gen_payload_expected = Print::new(
            handover_and_acceptance_full,
            owner_profile_full_print,
            "Europe/Budapest".parse().unwrap(),
        );

        let _m = PDF_GENERATOR_TEST_SYNC.lock();
        let pdf_gen = MockPdfGenerator::gen_pdf_temporary_context();
        pdf_gen
            .expect::<Print>()
            .times(1)
            .with(
                eq(PdfTemplates::HandoverAndAcceptanceView),
                eq(pdf_gen_payload_expected),
            )
            .returning(|template, payload| {
                Ok(PdfGenerator::gen_pdf_temporary(template, payload).unwrap())
            });

        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(None, Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/handover_and_acceptance/print?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let snapshot_path = Path::new("testing/pdf/snapshots/handover_and_acceptance_test.pdf");
        let mut file = File::open(snapshot_path).unwrap();
        let mut snapshot = vec![];
        file.read_to_end(&mut snapshot).unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        let test_result_path =
            Path::new("testing/pdf/test_results/handover_and_acceptance_test.pdf");
        let mut file = File::create(test_result_path).unwrap();
        file.write_all(&body_bytes).unwrap();

        assert_eq!(
            extract_pdf_text(&body_bytes).unwrap(),
            extract_pdf_text(&snapshot).unwrap()
        );
    }

    #[tokio::test]
    async fn test_print_unauthorized_expired() {
        let handover_and_acceptance_id = Uuid::now_v7();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/handover_and_acceptance/print?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
    async fn test_print_unauthorized_invalid_signature() {
        let handover_and_acceptance_id = Uuid::now_v7();

        let mut app_state = MockAppState::new();
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_jwt_with_invalid_signature()),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/handover_and_acceptance/print?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(app_state))),
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
    async fn test_print_unauthorized_missing() {
        let handover_and_acceptance_id = Uuid::now_v7();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/handover_and_acceptance/print?uuid={handover_and_acceptance_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes(Arc::new(
                MockAppState::new(),
            ))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
}
