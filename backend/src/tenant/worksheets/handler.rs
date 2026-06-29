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

use crate::common::dto::{EmptyType, SimpleMessageResponse, SuccessResponseBuilder, UuidParam};
use crate::common::extractors::UserInput;
use crate::common::handler::{ErrorMapper, ErrorMapperInterface, HandlerResult};
use crate::common::query_parser::{CommonRawQuery, ResourceQuery};
use crate::common::service::Service;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::worksheets::WorksheetsModuleInterface;
use crate::tenant::worksheets::dto::print::WorksheetResolvedPrint;
use crate::tenant::worksheets::dto::user_input::{WorksheetUserInput, WorksheetUserInputHelper};
use crate::tenant::worksheets::service::WorksheetService;
use crate::tenant::worksheets::types::worksheet::{WorksheetFilterBy, WorksheetOrderBy};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub async fn get_resolved<M: WorksheetsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), worksheets_module.clone());
    let error_mapper = ErrorMapper::new(worksheets_module);
    let result = error_mapper
        .or_handler_error(service.get_resolved(payload.uuid).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(result)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn get<M: WorksheetsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), worksheets_module.clone());
    let error_mapper = ErrorMapper::new(worksheets_module);
    let result = error_mapper
        .or_handler_error(service.get(payload.uuid).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(result)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn update<M: WorksheetsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<WorksheetUserInput, WorksheetUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), worksheets_module.clone());
    let error_mapper = ErrorMapper::new(worksheets_module);
    let result = error_mapper
        .or_handler_error(service.update(&user_input).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(result)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn delete<M: WorksheetsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), worksheets_module.clone());
    let error_mapper = ErrorMapper::new(worksheets_module);
    error_mapper
        .or_handler_error(service.delete(payload.uuid).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A munkalap törlése sikeresen megtörtént",
                ))
                .build(),
        )
        .await?
        .into_response())
}

pub async fn create<M: WorksheetsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<WorksheetUserInput, WorksheetUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), worksheets_module.clone());
    let error_mapper = ErrorMapper::new(worksheets_module);
    let result = error_mapper
        .or_handler_error(service.insert(&user_input).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::CREATED)
                .data(result)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn select_list<M: WorksheetsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<M>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), worksheets_module.clone());
    let error_mapper = ErrorMapper::new(worksheets_module);
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));

    let result = error_mapper
        .or_handler_error(service.get_select_list_items(&list_type).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(result)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn list<M: WorksheetsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<M>>,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), worksheets_module.clone());
    let error_mapper = ErrorMapper::new(worksheets_module);
    let resource_query = error_mapper
        .or_handler_error(
            ResourceQuery::<WorksheetOrderBy, WorksheetFilterBy>::from_str(payload.q()),
        )
        .await?;
    let (meta, data) = error_mapper
        .or_handler_error(service.get_paged(&resource_query).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::new()
                .status_code(StatusCode::OK)
                .meta(meta)
                .data(data)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn print<M: WorksheetsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(worksheets_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), worksheets_module.clone());
    let error_mapper = ErrorMapper::new(worksheets_module);
    let worksheet_resolved_print = WorksheetResolvedPrint::from_worksheet_resolved(
        error_mapper
            .or_handler_error(service.get_resolved(payload.uuid).await)
            .await?,
        error_mapper.or_handler_error(claims.tz()).await?,
    );
    let pdf = error_mapper
        .or_handler_error(service.print(&[worksheet_resolved_print]).await)
        .await?;
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
    use super::*;
    use crate::common::dto::PaginatorMeta;
    use crate::common::error::RepositoryError;
    use crate::common::handler::tests::{
        generate_expired_jwt, generate_jwt_with_invalid_signature, generate_valid_jwt,
    };
    use crate::common::pdf::tests::PDF_GENERATOR_TEST_SYNC;
    use crate::common::pdf::{MockPdfGenerator, PdfTemplates};
    use crate::tenant::worksheets::model::WorksheetResolved;
    use crate::{
        common::config::tests::AppConfigBuilder,
        tenant::worksheets::{
            self, model::Worksheet, repository::MockWorksheetsRepository,
            tests::MockWorksheetsModule,
        },
    };
    use axum::body::Body;
    use axum::{Router, http::Request};
    use chrono::Utc;
    use mockall::predicate::eq;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_success() {
        let active_tenant_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockWorksheetsRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(worksheet_id))
            .returning(move |worksheet_id| {
                Ok(Worksheet {
                    id: worksheet_id,
                    name: "Test worksheet".to_string(),
                    description: None,
                    customer_id,
                    project_id: None,
                    created_by_id,
                    status: "active".to_string(),
                    created_at: utc_now,
                    updated_at: utc_now,
                    deleted_at: None,
                })
            });

        let mut app_state = MockWorksheetsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_worksheets_repo()
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
            .uri(format!("/api/worksheets/get?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_unauthorized_expired() {
        let worksheet_id = Uuid::new_v4();

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri(format!("/api/worksheets/get?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_unauthorized_invalid_signature() {
        let worksheet_id = Uuid::new_v4();

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri(format!("/api/worksheets/get?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_unauthorized_missing() {
        let worksheet_id = Uuid::new_v4();
        let app_state = MockWorksheetsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/worksheets/get?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();

        let mut repo = MockWorksheetsRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(worksheet_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockWorksheetsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_worksheets_repo()
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
            .uri(format!("/api/worksheets/get?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_resolved_success() {
        let active_tenant_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockWorksheetsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(worksheet_id))
            .returning(move |worksheet_id| {
                Ok(WorksheetResolved {
                    id: worksheet_id,
                    name: "Test worksheet".to_string(),
                    description: None,
                    customer_id,
                    customer: "Test customer".to_string(),
                    project_id: None,
                    project: None,
                    created_by_id,
                    created_by: "Test user".to_string(),
                    status: "active".to_string(),
                    created_at: utc_now,
                    updated_at: utc_now,
                    deleted_at: None,
                    net_material_cost: "10".parse().unwrap(),
                    gross_material_cost: "20".parse().unwrap(),
                    net_work_cost: "30".parse().unwrap(),
                    gross_work_cost: "40".parse().unwrap(),
                })
            });

        let mut app_state = MockWorksheetsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_worksheets_repo()
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
            .uri(format!("/api/worksheets/get_resolved?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_expired() {
        let worksheet_id = Uuid::new_v4();

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri(format!("/api/worksheets/get_resolved?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_invalid_signature() {
        let worksheet_id = Uuid::new_v4();

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri(format!("/api/worksheets/get_resolved?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_missing() {
        let worksheet_id = Uuid::new_v4();
        let app_state = MockWorksheetsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/worksheets/get_resolved?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_get_resolved_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();

        let mut repo = MockWorksheetsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(worksheet_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockWorksheetsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_worksheets_repo()
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
            .uri(format!("/api/worksheets/get_resolved?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_success() {
        let active_tenant_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockWorksheetsRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<WorksheetOrderBy, WorksheetFilterBy>>()
                .unwrap()))
            .returning(move |_| {
                Ok((
                    PaginatorMeta {
                        page: 1,
                        limit: 25,
                        total: 100,
                    },
                    vec![WorksheetResolved {
                        id: worksheet_id,
                        name: "Test worksheet".to_string(),
                        description: None,
                        customer_id,
                        customer: "Test customer".to_string(),
                        project_id: None,
                        project: None,
                        created_by_id,
                        created_by: "Test user".to_string(),
                        status: "active".to_string(),
                        created_at: utc_now,
                        updated_at: utc_now,
                        deleted_at: None,
                        net_material_cost: "10".parse().unwrap(),
                        gross_material_cost: "20".parse().unwrap(),
                        net_work_cost: "30".parse().unwrap(),
                        gross_work_cost: "40".parse().unwrap(),
                    }],
                ))
            });

        let mut app_state = MockWorksheetsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_worksheets_repo()
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
            .uri("/api/worksheets/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri("/api/worksheets/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_unauthorized_invalid_signature() {
        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri("/api/worksheets/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_unauthorized_missing() {
        let app_state = MockWorksheetsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/worksheets/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_list_not_found() {
        let active_tenant_id = Uuid::new_v4();

        let mut repo = MockWorksheetsRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<WorksheetOrderBy, WorksheetFilterBy>>()
                .unwrap()))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockWorksheetsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_worksheets_repo()
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
            .uri("/api/worksheets/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = WorksheetUserInputHelper {
            id: None,
            name: "Test worksheet".to_string(),
            description: "".to_string(),
            customer_id: customer_id.to_string(),
            project_id: "".to_string(),
            status: "active".to_string(),
        };
        let user_input = WorksheetUserInput::try_from(user_input_helper.clone()).unwrap();

        let mut repo = MockWorksheetsRepository::new();
        repo.expect_insert()
            .times(1)
            .withf({
                let user_input_expected = user_input.clone();
                move |user_input, user_id_inner| {
                    user_input.name == user_input_expected.name
                        && user_input.description == user_input_expected.description
                        && user_input.customer_id == user_input_expected.customer_id
                        && user_input.project_id == user_input_expected.project_id
                        && user_input.status == user_input_expected.status
                        && user_id == *user_id_inner
                }
            })
            .returning(move |_, _| {
                Ok(Worksheet {
                    id: Uuid::new_v4(),
                    name: "Test worksheet".to_string(),
                    description: None,
                    customer_id,
                    project_id: None,
                    created_by_id,
                    status: "active".to_string(),
                    created_at: utc_now,
                    updated_at: utc_now,
                    deleted_at: None,
                })
            });

        let mut app_state = MockWorksheetsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_worksheets_repo()
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
            .uri("/api/worksheets/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = WorksheetUserInputHelper {
            id: None,
            name: "Test worksheet".to_string(),
            description: "".to_string(),
            customer_id: customer_id.to_string(),
            project_id: "".to_string(),
            status: "activee".to_string(),
        };

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri("/api/worksheets/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_create_unauthorized_expired() {
        let user_input_helper = WorksheetUserInputHelper {
            id: None,
            name: "Test worksheet".to_string(),
            description: "".to_string(),
            customer_id: Uuid::new_v4().to_string(),
            project_id: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri("/api/worksheets/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_create_unauthorized_invalid_signature() {
        let user_input_helper = WorksheetUserInputHelper {
            id: None,
            name: "Test worksheet".to_string(),
            description: "".to_string(),
            customer_id: Uuid::new_v4().to_string(),
            project_id: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri("/api/worksheets/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_create_unauthorized_missing() {
        let user_input_helper = WorksheetUserInputHelper {
            id: None,
            name: "Test worksheet".to_string(),
            description: "".to_string(),
            customer_id: Uuid::new_v4().to_string(),
            project_id: "".to_string(),
            status: "active".to_string(),
        };

        let app_state = MockWorksheetsModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/worksheets/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_update_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = WorksheetUserInputHelper {
            id: Some(worksheet_id.to_string()),
            name: "Test worksheet".to_string(),
            description: "".to_string(),
            customer_id: customer_id.to_string(),
            project_id: "".to_string(),
            status: "active".to_string(),
        };
        let user_input = WorksheetUserInput::try_from(user_input_helper.clone()).unwrap();

        let mut repo = MockWorksheetsRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input))
            .returning(move |_| {
                Ok(Worksheet {
                    id: worksheet_id,
                    name: "Test worksheet".to_string(),
                    description: None,
                    customer_id,
                    project_id: None,
                    created_by_id,
                    status: "active".to_string(),
                    created_at: utc_now,
                    updated_at: utc_now,
                    deleted_at: None,
                })
            });

        let mut app_state = MockWorksheetsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_worksheets_repo()
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
            .uri("/api/worksheets/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = WorksheetUserInputHelper {
            id: None,
            name: "Test worksheet".to_string(),
            description: "".to_string(),
            customer_id: Uuid::new_v4().to_string(),
            project_id: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri("/api/worksheets/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
    #[tokio::test]
    async fn test_update_unauthorized_expired() {
        let worksheet_id = Uuid::new_v4();
        let user_input_helper = WorksheetUserInputHelper {
            id: Some(worksheet_id.to_string()),
            name: "Test worksheet".to_string(),
            description: "".to_string(),
            customer_id: Uuid::new_v4().to_string(),
            project_id: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri("/api/worksheets/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_update_unauthorized_invalid_signature() {
        let worksheet_id = Uuid::new_v4();
        let user_input_helper = WorksheetUserInputHelper {
            id: Some(worksheet_id.to_string()),
            name: "Test worksheet".to_string(),
            description: "".to_string(),
            customer_id: Uuid::new_v4().to_string(),
            project_id: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri("/api/worksheets/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_update_unauthorized_missing() {
        let worksheet_id = Uuid::new_v4();
        let user_input_helper = WorksheetUserInputHelper {
            id: Some(worksheet_id.to_string()),
            name: "Test worksheet".to_string(),
            description: "".to_string(),
            customer_id: Uuid::new_v4().to_string(),
            project_id: "".to_string(),
            status: "active".to_string(),
        };

        let app_state = MockWorksheetsModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/worksheets/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_delete_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let mut repo = MockWorksheetsRepository::new();

        repo.expect_delete_by_id()
            .times(1)
            .with(eq(worksheet_id))
            .returning(move |_| Ok(()));

        let mut app_state = MockWorksheetsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_worksheets_repo()
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
            .uri(format!("/api/worksheets/delete?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri("/api/worksheets/delete?uuid=invalid_user_input")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_expired() {
        let worksheet_id = Uuid::new_v4();

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri(format!("/api/worksheets/delete?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_invalid_signature() {
        let worksheet_id = Uuid::new_v4();

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri(format!("/api/worksheets/delete?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_missing() {
        let worksheet_id = Uuid::new_v4();

        let app_state = MockWorksheetsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!("/api/worksheets/delete?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_print_success() {
        let active_tenant_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let customer_resolved = WorksheetResolved {
            id: worksheet_id,
            name: "Test worksheet".to_string(),
            description: None,
            customer_id,
            customer: "Test customer".to_string(),
            project_id: None,
            project: None,
            created_by_id,
            created_by: "Test user".to_string(),
            status: "active".to_string(),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
            net_material_cost: "10".parse().unwrap(),
            gross_material_cost: "20".parse().unwrap(),
            net_work_cost: "30".parse().unwrap(),
            gross_work_cost: "40".parse().unwrap(),
        };

        let mut repo = MockWorksheetsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(worksheet_id))
            .returning({
                let customer_resolved = customer_resolved.clone();
                move |_| Ok(customer_resolved.clone())
            });

        let mut app_state = MockWorksheetsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_worksheets_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let pdf_gen_payload_expected = vec![WorksheetResolvedPrint::from_worksheet_resolved(
            customer_resolved,
            "Europe/Budapest".parse().unwrap(),
        )];

        let _m = PDF_GENERATOR_TEST_SYNC.lock();
        let pdf_gen = MockPdfGenerator::gen_pdf_temporary_context();
        pdf_gen
            .expect::<Vec<WorksheetResolvedPrint>>()
            .times(1)
            .with(
                eq(PdfTemplates::WorksheetView),
                eq(pdf_gen_payload_expected),
            )
            .returning(|_, _| Ok(vec![]));

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
            .uri(format!("/api/worksheets/print?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_print_unauthorized_expired() {
        let worksheet_id = Uuid::new_v4();

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri(format!("/api/worksheets/print?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_print_unauthorized_invalid_signature() {
        let worksheet_id = Uuid::new_v4();

        let mut app_state = MockWorksheetsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
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
            .uri(format!("/api/worksheets/print?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_print_unauthorized_missing() {
        let worksheet_id = Uuid::new_v4();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/worksheets/print?uuid={worksheet_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(worksheets::routes::routes(Arc::new(
                MockWorksheetsModule::new(),
            ))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
