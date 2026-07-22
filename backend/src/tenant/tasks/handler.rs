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
use crate::common::handler::{HandlerResult, map_handler_err};
use crate::common::query_parser::{CommonRawQuery, ResourceQuery};
use crate::common::service::Service;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::tasks::TasksModule;
use crate::tenant::tasks::dto::print::TaskResolvedPrint;
use crate::tenant::tasks::dto::user_input::{TaskUserInput, TaskUserInputHelper};
use crate::tenant::tasks::service::TaskService;
use crate::tenant::tasks::types::task::{TaskFilterBy, TaskOrderBy};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub async fn get_resolved<M: TasksModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tasks_module.clone());
    let result = map_handler_err(
        service.get_resolved(payload.uuid).await,
        tasks_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        tasks_module,
    )
    .await?
    .into_response())
}

pub async fn get<M: TasksModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tasks_module.clone());
    let result = map_handler_err(service.get(payload.uuid).await, tasks_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        tasks_module,
    )
    .await?
    .into_response())
}

pub async fn update<M: TasksModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<TaskUserInput, TaskUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tasks_module.clone());
    let result = map_handler_err(service.update(&user_input).await, tasks_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        tasks_module,
    )
    .await?
    .into_response())
}

pub async fn delete<M: TasksModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tasks_module.clone());
    map_handler_err(service.delete(payload.uuid).await, tasks_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(SimpleMessageResponse::new(
                "A feladat törlése sikeresen megtörtént",
            ))
            .build(),
        tasks_module,
    )
    .await?
    .into_response())
}

pub async fn create<M: TasksModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<TaskUserInput, TaskUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tasks_module.clone());
    let result = map_handler_err(service.insert(&user_input).await, tasks_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(result)
            .build(),
        tasks_module,
    )
    .await?
    .into_response())
}

pub async fn select_list<M: TasksModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<M>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tasks_module.clone());
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));

    let result = map_handler_err(
        service.get_select_list_items(&list_type).await,
        tasks_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        tasks_module,
    )
    .await?
    .into_response())
}

pub async fn list<M: TasksModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<M>>,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tasks_module.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<TaskOrderBy, TaskFilterBy>::from_str(payload.q()),
        tasks_module.clone(),
    )
    .await?;
    let (meta, data) = map_handler_err(
        service.get_paged(&resource_query).await,
        tasks_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::new()
            .status_code(StatusCode::OK)
            .meta(meta)
            .data(data)
            .build(),
        tasks_module,
    )
    .await?
    .into_response())
}

pub async fn print<M: TasksModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), tasks_module.clone());
    let task_resolved_print = TaskResolvedPrint::from_task_resolved(
        map_handler_err(
            service.get_resolved(payload.uuid).await,
            tasks_module.clone(),
        )
        .await?,
        map_handler_err(claims.tz(), tasks_module.clone()).await?,
    );
    let pdf = map_handler_err(service.print(&[task_resolved_print]).await, tasks_module).await?;
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
        extract_json_response, generate_expired_jwt, generate_jwt_with_invalid_signature,
        generate_valid_jwt,
    };
    use crate::common::pdf::tests::{PDF_GENERATOR_TEST_SYNC, extract_pdf_text};
    use crate::common::pdf::{MockPdfGenerator, PdfGenerator, PdfTemplates};
    use crate::tenant::tasks::model::TaskResolved;
    use crate::{
        common::config::tests::AppConfigBuilder,
        tenant::tasks::{
            self, model::Task, repository::MockTasksRepository, tests::MockTasksModule,
        },
    };
    use axum::body::Body;
    use axum::{Router, http::Request};
    use chrono::{DateTime, Duration, Utc};
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_success() {
        let active_tenant_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();
        let task = Task {
            id: task_id,
            worksheet_id,
            service_id,
            currency_code: "HUF".to_string(),
            quantity: None,
            price: None,
            tax_id,
            created_by_id,
            status: "active".to_string(),
            priority: Some("normal".to_string()),
            due_date: Some(utc_now + Duration::weeks(1)),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
            description: None,
        };

        let mut repo = MockTasksRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(task_id))
            .returning({
                let task = task.clone();
                move |_| Ok(task.clone())
            });

        let mut app_state = MockTasksModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_tasks_repo()
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
            .uri(format!("/api/tasks/get?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": task,
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_unauthorized_expired() {
        let task_id = Uuid::new_v4();

        let mut app_state = MockTasksModule::new();
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
            .uri(format!("/api/tasks/get?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let task_id = Uuid::new_v4();

        let mut app_state = MockTasksModule::new();
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
            .uri(format!("/api/tasks/get?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let task_id = Uuid::new_v4();
        let app_state = MockTasksModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tasks/get?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();

        let mut repo = MockTasksRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(task_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockTasksModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_tasks_repo()
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
            .uri(format!("/api/tasks/get?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
    async fn test_get_resolved_success() {
        let active_tenant_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();
        let task_resolved = TaskResolved {
            id: task_id,
            worksheet_id,
            worksheet: "Test worksheet".to_string(),
            service_id,
            service: "Test service".to_string(),
            currency_code: "HUF".to_string(),
            quantity: None,
            price: None,
            tax_id,
            tax: "Test tax".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            status: "active".to_string(),
            priority: Some("normal".to_string()),
            due_date: Some(utc_now + Duration::weeks(1)),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
            description: None,
        };

        let mut repo = MockTasksRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(task_id))
            .returning({
                let task_resolved = task_resolved.clone();
                move |_| Ok(task_resolved.clone())
            });

        let mut app_state = MockTasksModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_tasks_repo()
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
            .uri(format!("/api/tasks/get_resolved?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": task_resolved
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_expired() {
        let task_id = Uuid::new_v4();

        let mut app_state = MockTasksModule::new();
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
            .uri(format!("/api/tasks/get_resolved?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let task_id = Uuid::new_v4();

        let mut app_state = MockTasksModule::new();
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
            .uri(format!("/api/tasks/get_resolved?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let task_id = Uuid::new_v4();
        let app_state = MockTasksModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tasks/get_resolved?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_get_resolved_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();

        let mut repo = MockTasksRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(task_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockTasksModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_tasks_repo()
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
            .uri(format!("/api/tasks/get_resolved?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let active_tenant_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();
        let paginator_meta = PaginatorMeta {
            page: 1,
            limit: 25,
            total: 100,
        };
        let task_resolved = TaskResolved {
            id: task_id,
            worksheet_id,
            worksheet: "Test worksheet".to_string(),
            service_id,
            service: "Test service".to_string(),
            currency_code: "HUF".to_string(),
            quantity: None,
            price: None,
            tax_id,
            tax: "Test tax".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            status: "active".to_string(),
            priority: Some("normal".to_string()),
            due_date: Some(utc_now + Duration::weeks(1)),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
            description: None,
        };

        let mut repo = MockTasksRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<TaskOrderBy, TaskFilterBy>>()
                .unwrap()))
            .returning({
                let task_resolved = task_resolved.clone();
                move |_| Ok((paginator_meta, vec![task_resolved.clone()]))
            });

        let mut app_state = MockTasksModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_tasks_repo()
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
            .uri("/api/tasks/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": paginator_meta,
            "data": [task_resolved]
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let mut app_state = MockTasksModule::new();
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
            .uri("/api/tasks/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let mut app_state = MockTasksModule::new();
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
            .uri("/api/tasks/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let app_state = MockTasksModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/tasks/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_list_not_found() {
        let active_tenant_id = Uuid::new_v4();

        let mut repo = MockTasksRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<TaskOrderBy, TaskFilterBy>>()
                .unwrap()))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockTasksModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_tasks_repo()
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
            .uri("/api/tasks/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = TaskUserInputHelper {
            id: None,
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: "HUF".to_string(),
            quantity: "".to_string(),
            price: "".to_string(),
            tax_id: tax_id.to_string(),
            status: "active".to_string(),
            priority: "normal".to_string(),
            due_date: "".to_string(),
            description: "".to_string(),
        };
        let user_input = TaskUserInput::try_from(user_input_helper.clone()).unwrap();
        let task = Task {
            id: task_id,
            worksheet_id,
            service_id,
            currency_code: "HUF".to_string(),
            quantity: None,
            price: None,
            tax_id,
            created_by_id,
            status: "active".to_string(),
            priority: Some("normal".to_string()),
            due_date: None,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
            description: None,
        };

        let mut repo = MockTasksRepository::new();
        repo.expect_insert()
            .times(1)
            .withf({
                let user_input_expected = user_input.clone();
                move |user_input, user_id_inner| {
                    user_input.worksheet_id == user_input_expected.worksheet_id
                        && user_input.service_id == user_input_expected.service_id
                        && user_input.currency_code == user_input_expected.currency_code
                        && user_input.quantity == user_input_expected.quantity
                        && user_input.price == user_input_expected.price
                        && user_input.tax_id == user_input_expected.tax_id
                        && user_input.status == user_input_expected.status
                        && user_input.priority == user_input_expected.priority
                        && user_input.due_date == user_input_expected.due_date
                        && user_input.description == user_input_expected.description
                        && user_id == *user_id_inner
                }
            })
            .returning({
                let task = task.clone();
                move |_, _| Ok(task.clone())
            });

        let mut app_state = MockTasksModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_tasks_repo()
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
            .uri("/api/tasks/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": task
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();

        let user_input_helper = TaskUserInputHelper {
            id: None,
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: "HUF".to_string(),
            quantity: "".to_string(),
            price: "".to_string(),
            tax_id: tax_id.to_string(),
            status: "activee".to_string(),
            priority: "normal".to_string(),
            due_date: "".to_string(),
            description: "".to_string(),
        };

        let mut app_state = MockTasksModule::new();
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
            .uri("/api/tasks/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let user_input_error = TaskUserInput::try_from(user_input_helper).unwrap_err();
        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Kérjük ellenőrizze a hibás mezőket!",
                "fields": user_input_error,
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_unauthorized_expired() {
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();

        let user_input_helper = TaskUserInputHelper {
            id: None,
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: "HUF".to_string(),
            quantity: "".to_string(),
            price: "".to_string(),
            tax_id: tax_id.to_string(),
            status: "active".to_string(),
            priority: "normal".to_string(),
            due_date: "".to_string(),
            description: "".to_string(),
        };

        let mut app_state = MockTasksModule::new();
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
            .uri("/api/tasks/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();

        let user_input_helper = TaskUserInputHelper {
            id: None,
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: "HUF".to_string(),
            quantity: "".to_string(),
            price: "".to_string(),
            tax_id: tax_id.to_string(),
            status: "active".to_string(),
            priority: "normal".to_string(),
            due_date: "".to_string(),
            description: "".to_string(),
        };

        let mut app_state = MockTasksModule::new();
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
            .uri("/api/tasks/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();

        let user_input_helper = TaskUserInputHelper {
            id: None,
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: "HUF".to_string(),
            quantity: "".to_string(),
            price: "".to_string(),
            tax_id: tax_id.to_string(),
            status: "active".to_string(),
            priority: "normal".to_string(),
            due_date: "".to_string(),
            description: "".to_string(),
        };

        let app_state = MockTasksModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/tasks/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_update_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();
        let task = Task {
            id: task_id,
            worksheet_id,
            service_id,
            currency_code: "HUF".to_string(),
            quantity: None,
            price: None,
            tax_id,
            created_by_id,
            status: "active".to_string(),
            priority: Some("normal".to_string()),
            due_date: Some(utc_now + Duration::weeks(1)),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
            description: None,
        };

        let user_input_helper = TaskUserInputHelper {
            id: Some(task_id.to_string()),
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: "HUF".to_string(),
            quantity: "".to_string(),
            price: "".to_string(),
            tax_id: tax_id.to_string(),
            status: "active".to_string(),
            priority: "normal".to_string(),
            due_date: "".to_string(),
            description: "".to_string(),
        };
        let user_input = TaskUserInput::try_from(user_input_helper.clone()).unwrap();

        let mut repo = MockTasksRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input))
            .returning({
                let task = task.clone();
                move |_| Ok(task.clone())
            });

        let mut app_state = MockTasksModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_tasks_repo()
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
            .uri("/api/tasks/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": task
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();

        let user_input_helper = TaskUserInputHelper {
            id: None,
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: "HUF".to_string(),
            quantity: "".to_string(),
            price: "".to_string(),
            tax_id: tax_id.to_string(),
            status: "active".to_string(),
            priority: "normal".to_string(),
            due_date: "".to_string(),
            description: "".to_string(),
        };

        let mut app_state = MockTasksModule::new();
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
            .uri("/api/tasks/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Hiba történt az adatok feldolgozása során: Az azonosító megadása kötelező!"
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_unauthorized_expired() {
        let task_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();

        let user_input_helper = TaskUserInputHelper {
            id: Some(task_id.to_string()),
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: "HUF".to_string(),
            quantity: "".to_string(),
            price: "".to_string(),
            tax_id: tax_id.to_string(),
            status: "active".to_string(),
            priority: "normal".to_string(),
            due_date: "".to_string(),
            description: "".to_string(),
        };

        let mut app_state = MockTasksModule::new();
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
            .uri("/api/tasks/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let task_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();

        let user_input_helper = TaskUserInputHelper {
            id: Some(task_id.to_string()),
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: "HUF".to_string(),
            quantity: "".to_string(),
            price: "".to_string(),
            tax_id: tax_id.to_string(),
            status: "active".to_string(),
            priority: "normal".to_string(),
            due_date: "".to_string(),
            description: "".to_string(),
        };

        let mut app_state = MockTasksModule::new();
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
            .uri("/api/tasks/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let task_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();

        let user_input_helper = TaskUserInputHelper {
            id: Some(task_id.to_string()),
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: "HUF".to_string(),
            quantity: "".to_string(),
            price: "".to_string(),
            tax_id: tax_id.to_string(),
            status: "active".to_string(),
            priority: "normal".to_string(),
            due_date: "".to_string(),
            description: "".to_string(),
        };

        let app_state = MockTasksModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/tasks/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_delete_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let mut repo = MockTasksRepository::new();

        repo.expect_delete_by_id()
            .times(1)
            .with(eq(task_id))
            .returning(move |_| Ok(()));

        let mut app_state = MockTasksModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_tasks_repo()
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
            .uri(format!("/api/tasks/delete?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": {
                "message": "A feladat törlése sikeresen megtörtént"
            },
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut app_state = MockTasksModule::new();
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
            .uri("/api/tasks/delete?uuid=invalid_user_input")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_expired() {
        let task_id = Uuid::new_v4();

        let mut app_state = MockTasksModule::new();
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
            .uri(format!("/api/tasks/delete?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let task_id = Uuid::new_v4();

        let mut app_state = MockTasksModule::new();
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
            .uri(format!("/api/tasks/delete?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let task_id = Uuid::new_v4();

        let app_state = MockTasksModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!("/api/tasks/delete?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_print_success() {
        let active_tenant_id = Uuid::new_v4();
        let task_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse().unwrap();
        let worksheet_id = "fd48ade1-a817-431b-8ada-6faea8c9f9dd".parse().unwrap();
        let tax_id = "86097a0b-3f05-42f4-a98d-fd8a4669f02b".parse().unwrap();
        let service_id = "ac55ca9c-2cd1-4cdf-8b44-ed4df798c750".parse().unwrap();
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0".parse().unwrap();
        let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z".parse().unwrap();

        let task_resolved = TaskResolved {
            id: task_id,
            worksheet_id,
            worksheet: "Test worksheet".to_string(),
            service_id,
            service: "Test service".to_string(),
            currency_code: "HUF".to_string(),
            quantity: None,
            price: None,
            tax_id,
            tax: "Test tax".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            status: "active".to_string(),
            priority: Some("normal".to_string()),
            due_date: Some(test_time + Duration::weeks(1)),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
            description: None,
        };

        let mut repo = MockTasksRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(task_id))
            .returning({
                let task_resolved = task_resolved.clone();
                move |_| Ok(task_resolved.clone())
            });

        let mut app_state = MockTasksModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_tasks_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let pdf_gen_payload_expected = vec![TaskResolvedPrint::from_task_resolved(
            task_resolved,
            "Europe/Budapest".parse().unwrap(),
        )];

        let _m = PDF_GENERATOR_TEST_SYNC.lock();
        let pdf_gen = MockPdfGenerator::gen_pdf_temporary_context();
        pdf_gen
            .expect::<Vec<TaskResolvedPrint>>()
            .times(1)
            .with(eq(PdfTemplates::TaskView), eq(pdf_gen_payload_expected))
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
            .uri(format!("/api/tasks/print?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let snapshot_path = Path::new("testing/pdf/snapshots/tasks_test.pdf");
        let mut file = File::open(snapshot_path).unwrap();
        let mut snapshot = vec![];
        file.read_to_end(&mut snapshot).unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        let test_result_path = Path::new("testing/pdf/test_results/tasks_test.pdf");
        let mut file = File::create(test_result_path).unwrap();
        file.write_all(&body_bytes).unwrap();

        assert_eq!(
            extract_pdf_text(&body_bytes).unwrap(),
            extract_pdf_text(&snapshot).unwrap()
        );
    }

    #[tokio::test]
    async fn test_print_unauthorized_expired() {
        let task_id = Uuid::new_v4();

        let mut app_state = MockTasksModule::new();
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
            .uri(format!("/api/tasks/print?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let task_id = Uuid::new_v4();

        let mut app_state = MockTasksModule::new();
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
            .uri(format!("/api/tasks/print?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(app_state))),
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
        let task_id = Uuid::new_v4();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/tasks/print?uuid={task_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(tasks::routes::routes(Arc::new(MockTasksModule::new()))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
}
