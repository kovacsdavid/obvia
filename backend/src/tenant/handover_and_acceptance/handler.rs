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

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode, header},
};
use tracing::instrument;

use crate::{
    common::{
        dto::{EmptyType, SimpleMessageResponse, SuccessResponseBuilder, UuidParam},
        extractors::{ClientContext, UserInput},
        handler::{HandlerResult, map_handler_err},
        query_parser::{CommonRawQuery, ResourceQuery},
        service::Service,
    },
    manager::auth::middleware::AuthenticatedUser,
    tenant::handover_and_acceptance::HandoverAndAcceptanceModuleInterface,
};

#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::get_full",
    skip(handover_and_acceptance_module)
)]
pub async fn get_full<M: HandoverAndAcceptanceModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(handover_and_acceptance_module): State<Arc<M>>,
    _client_context: ClientContext,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), handover_and_acceptance_module.clone());
    let result = map_handler_err(
        service.get_full(payload.uuid).await,
        handover_and_acceptance_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        handover_and_acceptance_module,
    )
    .await?
    .into_response())
}

#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::list",
    skip(handover_and_acceptance_module)
)]
pub async fn list<M: HandoverAndAcceptanceModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(handover_and_acceptance_module): State<Arc<M>>,
    _client_context: ClientContext,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), handover_and_acceptance_module.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<HandoverAndAcceptanceOrderBy, HandoverAndAcceptanceFilterBy>::from_str(
            payload.q(),
        ),
        handover_and_acceptance_module.clone(),
    )
    .await?;
    let (meta, data) = map_handler_err(
        service.get_paged(&resource_query).await,
        handover_and_acceptance_module.clone(),
    )
    .await?;

    Ok(map_handler_err(
        SuccessResponseBuilder::new()
            .status_code(StatusCode::OK)
            .meta(meta)
            .data(data)
            .build(),
        handover_and_acceptance_module,
    )
    .await?
    .into_response())
}

#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::create",
    skip(handover_and_acceptance_module)
)]
pub async fn create<M: HandoverAndAcceptanceModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(handover_and_acceptance_module): State<Arc<M>>,
    _client_context: ClientContext,
    UserInput(user_input, _): UserInput<
        HandoverAndAcceptanceUserInput,
        HandoverAndAcceptanceUserInputHelper,
    >,
) -> HandlerResult {
    let service = Service::new(Some(&claims), handover_and_acceptance_module.clone());
    let result = map_handler_err(
        service.insert(&user_input).await,
        handover_and_acceptance_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(result)
            .build(),
        handover_and_acceptance_module,
    )
    .await?
    .into_response())
}

#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::update",
    skip(handover_and_acceptance_module)
)]
pub async fn update<M: HandoverAndAcceptanceModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(handover_and_acceptance_module): State<Arc<M>>,
    _client_context: ClientContext,
    UserInput(user_input, _): UserInput<
        HandoverAndAcceptanceUserInput,
        HandoverAndAcceptanceUserInputHelper,
    >,
) -> HandlerResult {
    let service = Service::new(Some(&claims), handover_and_acceptance_module.clone());
    let result = map_handler_err(
        service.update(&user_input).await,
        handover_and_acceptance_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        handover_and_acceptance_module,
    )
    .await?
    .into_response())
}

#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::delete",
    skip(handover_and_acceptance_module)
)]
pub async fn delete<M: HandoverAndAcceptanceModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(handover_and_acceptance_module): State<Arc<M>>,
    _client_context: ClientContext,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), handover_and_acceptance_module.clone());
    map_handler_err(
        service.delete(payload.uuid).await,
        handover_and_acceptance_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(SimpleMessageResponse::new(
                "A vevő törlése sikeresen megtörtént",
            ))
            .build(),
        handover_and_acceptance_module,
    )
    .await?
    .into_response())
}

#[instrument(
    name = "obvia::tenant::handover_and_acceptance::handler::print",
    skip(handover_and_acceptance_module)
)]
pub async fn print<M: HandoverAndAcceptanceModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(handover_and_acceptance_module): State<Arc<M>>,
    _client_context: ClientContext,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), handover_and_acceptance_module.clone());
    let handover_and_acceptance_resolved_print = HandoverAndAcceptanceFullPrint::new(
        map_handler_err(
            service.get_full(payload.uuid).await,
            handover_and_acceptance_module.clone(),
        )
        .await?,
        map_handler_err(claims.tz(), handover_and_acceptance_module.clone()).await?,
    );
    let handover_and_acceptance_id = handover_and_acceptance_resolved_print.id();
    let pdf = map_handler_err(
        service
            .print(&[handover_and_acceptance_resolved_print])
            .await,
        handover_and_acceptance_module,
    )
    .await?;
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!(r#"inline; filename="vevo_{handover_and_acceptance_id}.pdf""#)
            .parse()
            .unwrap(),
    );
    Ok((StatusCode::OK, headers, pdf).into_response())
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write, path::Path};

    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
    };
    use mockall::predicate::eq;
    use serde_json::json;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        common::{
            config::tests::test_app_config_builder,
            dto::PaginatorMeta,
            error::RepositoryError,
            handler::tests::{
                extract_json_response, generate_expired_jwt, generate_jwt_with_invalid_signature,
                generate_valid_jwt,
            },
            pdf::{
                MockPdfGenerator, PdfTemplates,
                tests::{PDF_GENERATOR_TEST_SYNC, extract_pdf_text},
            },
            query_parser::ResourceQuery,
        },
        tenant::handover_and_acceptance::tests::MockHandoverAndAcceptanceModule,
    };

    use super::*;

    #[tokio::test]
    async fn test_get_full_success() {
        todo!();
        let active_tenant_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let handover_and_acceptance_full = test_handover_and_acceptance_full_builder()
            .id(handover_and_acceptance_id)
            .build()
            .unwrap();

        let mut handover_and_acceptance_repo = MockHandoverAndAcceptanceRepository::new();
        handover_and_acceptance_repo
            .expect_get_full()
            .times(1)
            .with(eq(handover_and_acceptance_id))
            .returning({
                let handover_and_acceptance_full = handover_and_acceptance_full.clone();
                move |_| Ok(handover_and_acceptance_full.clone())
            });

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let mut repo = MockHandoverAndAcceptanceModule::new();
        repo.expect_get_full()
            .times(1)
            .with(eq(handover_and_acceptance_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let active_tenant_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let paginator_meta = PaginatorMeta {
            page: 1,
            limit: 25,
            total: 100,
        };

        let handover_and_acceptance_resolved = test_handover_and_acceptance_resolved_builder()
            .id(handover_and_acceptance_id)
            .build()
            .unwrap();

        let mut handover_and_acceptance_repo = MockHandoverAndAcceptanceRepository::new();
        handover_and_acceptance_repo
            .expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<HandoverAndAcceptanceOrderBy, HandoverAndAcceptanceFilterBy>>()
                .unwrap()))
            .returning({
                let handover_and_acceptance_resolved = handover_and_acceptance_resolved.clone();
                move |_| Ok((paginator_meta, vec![handover_and_acceptance_resolved.clone()]))
            });

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            .uri("/api/handover_and_acceptance/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let handover_and_acceptance_full = handover_and_acceptance_resolved.into_full();

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": paginator_meta,
            "data": vec![handover_and_acceptance_full]
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let app_state = MockHandoverAndAcceptanceModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/handover_and_acceptance/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let mut repo = MockHandoverAndAcceptanceRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<HandoverAndAcceptanceOrderBy, HandoverAndAcceptanceFilterBy>>()
                .unwrap()))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let user_input_helper = HandoverAndAcceptanceUserInputHelper {};
        let user_input =
            HandoverAndAcceptanceUserInput::try_from(user_input_helper.clone()).unwrap();

        let handover_and_acceptance = test_handover_and_acceptance_builder()
            .id(handover_and_acceptance_id)
            .build()
            .unwrap();

        let mut repo = MockHandoverAndAcceptanceRepository::new();
        repo.expect_insert()
            .times(1)
            .withf({
                let user_input_expected = user_input.clone();
                // TODO: adjust this to HandoverAndAcceptance and figure out why did not you use
                // PartialEq for this check
                move |user_input, user_id_inner| {
                    user_input.name == user_input_expected.name
                        && user_input.contact_name == user_input_expected.contact_name
                        && user_input.email == user_input_expected.email
                        && user_input.phone_number == user_input_expected.phone_number
                        && user_input.status == user_input_expected.status
                        && user_id == *user_id_inner
                        && todo!()
                }
            })
            .returning({
                let handover_and_acceptance = handover_and_acceptance.clone();
                move |_, _| Ok(handover_and_acceptance.clone())
            });

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let user_input_helper = HandoverAndAcceptanceUserInputHelper {};

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let field_errors = HandoverAndAcceptanceUserInput::try_from(user_input_helper).unwrap_err();

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
        let user_input_helper = HandoverAndAcceptanceUserInputHelper {};

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = HandoverAndAcceptanceUserInputHelper {};

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = HandoverAndAcceptanceUserInputHelper {};

        let app_state = MockHandoverAndAcceptanceModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/handover_and_acceptance/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_success() {
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let handover_and_acceptance_id = Uuid::now_v7();

        let user_input_helper = HandoverAndAcceptanceUserInputHelper {};
        let user_input =
            HandoverAndAcceptanceUserInput::try_from(user_input_helper.clone()).unwrap();

        let handover_and_acceptance = test_handover_and_acceptance_builder()
            .id(handover_and_acceptance_id)
            .build()
            .unwrap();

        let mut repo = MockHandoverAndAcceptanceRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input), eq(user_id))
            .returning({
                let handover_and_acceptance = handover_and_acceptance.clone();
                move |_, _| Ok(handover_and_acceptance.clone())
            });

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let user_input_helper = HandoverAndAcceptanceUserInputHelper {};

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = HandoverAndAcceptanceUserInputHelper {};

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = HandoverAndAcceptanceUserInputHelper {};

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = HandoverAndAcceptanceUserInputHelper {};

        let app_state = MockHandoverAndAcceptanceModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/handover_and_acceptance/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
        let mut repo = MockHandoverAndAcceptanceRepository::new();

        repo.expect_delete_by_id()
            .times(1)
            .with(eq(handover_and_acceptance_id))
            .returning(move |_| Ok(()));

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let handover_and_acceptance_full = test_handover_and_acceptance_full_builder()
            .id(handover_and_acceptance_id)
            .build()
            .unwrap();

        let mut repo = MockHandoverAndAcceptanceRepository::new();
        repo.expect_get_full()
            .times(1)
            .with(eq(handover_and_acceptance_id))
            .returning({
                let handover_and_acceptance_resolved = handover_and_acceptance_full.clone();
                move |_| Ok(handover_and_acceptance_resolved.clone())
            });

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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

        let pdf_gen_payload_expected = vec![HandoverAndAcceptanceFullPrint::new(
            handover_and_acceptance_full,
            "Europe/Budapest".parse().unwrap(),
        )];

        let _m = PDF_GENERATOR_TEST_SYNC.lock();
        let pdf_gen = MockPdfGenerator::gen_pdf_temporary_context();
        pdf_gen
            .expect::<Vec<HandoverAndAcceptanceFullPrint>>()
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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

        let mut app_state = MockHandoverAndAcceptanceModule::new();
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(app_state))),
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
            Router::new().merge(handover_and_acceptance::routes::routes(Arc::new(
                MockHandoverAndAcceptanceModule::new(),
            ))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
}
