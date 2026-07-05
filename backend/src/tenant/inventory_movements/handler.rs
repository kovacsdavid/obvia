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
use crate::common::query_parser::ResourceQuery;
use crate::common::service::Service;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::inventory_movements::InventoryMovementsModuleInterface;
use crate::tenant::inventory_movements::dto::print::InventoryMovementsResolvedPrint;
use crate::tenant::inventory_movements::dto::user_input::{
    InventoryMovementUserInput, InventoryMovementUserInputHelper, InventoryMovementsRawQuery,
};
use crate::tenant::inventory_movements::service::InventoryMovementService;
use crate::tenant::inventory_movements::types::{
    InventoryMovementFilterBy, InventoryMovementOrderBy,
};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub async fn get<M: InventoryMovementsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_movements_module.clone());
    let result = map_handler_err(
        service.get(payload.uuid).await,
        inventory_movements_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        inventory_movements_module,
    )
    .await?
    .into_response())
}

pub async fn get_resolved<M: InventoryMovementsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_movements_module.clone());
    let result = map_handler_err(
        service.get_resolved(payload.uuid).await,
        inventory_movements_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        inventory_movements_module,
    )
    .await?
    .into_response())
}

pub async fn create<M: InventoryMovementsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<
        InventoryMovementUserInput,
        InventoryMovementUserInputHelper,
    >,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_movements_module.clone());
    let result = map_handler_err(
        service.insert(&user_input).await,
        inventory_movements_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(result)
            .build(),
        inventory_movements_module,
    )
    .await?
    .into_response())
}

pub async fn update<M: InventoryMovementsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<
        InventoryMovementUserInput,
        InventoryMovementUserInputHelper,
    >,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_movements_module.clone());
    let result = map_handler_err(
        service.update(&user_input).await,
        inventory_movements_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        inventory_movements_module,
    )
    .await?
    .into_response())
}

pub async fn delete<M: InventoryMovementsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_movements_module.clone());
    map_handler_err(
        service.delete(payload.uuid).await,
        inventory_movements_module.clone(),
    )
    .await?;

    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(SimpleMessageResponse::new(
                "A készletmozgás törlése sikeresen megtörtént",
            ))
            .build(),
        inventory_movements_module,
    )
    .await?
    .into_response())
}

pub async fn list<M: InventoryMovementsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<M>>,
    Query(payload): Query<InventoryMovementsRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_movements_module.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<InventoryMovementOrderBy, InventoryMovementFilterBy>::from_str(payload.q()),
        inventory_movements_module.clone(),
    )
    .await?;
    let (meta, data) = map_handler_err(
        service
            .get_paged(&resource_query, payload.inventory_id())
            .await,
        inventory_movements_module.clone(),
    )
    .await?;

    Ok(map_handler_err(
        SuccessResponseBuilder::new()
            .status_code(StatusCode::OK)
            .meta(meta)
            .data(data)
            .build(),
        inventory_movements_module,
    )
    .await?
    .into_response())
}

pub async fn select_list<M: InventoryMovementsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<M>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_movements_module.clone());
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));
    let result = map_handler_err(
        service.get_select_list_items(&list_type).await,
        inventory_movements_module.clone(),
    )
    .await?;

    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        inventory_movements_module,
    )
    .await?
    .into_response())
}

pub async fn print<M: InventoryMovementsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_movements_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_movements_module.clone());
    let inventory_movements_resolved_print =
        InventoryMovementsResolvedPrint::from_inventory_movements_resolved(
            map_handler_err(
                service.get_resolved(payload.uuid).await,
                inventory_movements_module.clone(),
            )
            .await?,
            map_handler_err(claims.tz(), inventory_movements_module.clone()).await?,
        );

    let pdf = map_handler_err(
        service.print(&[inventory_movements_resolved_print]).await,
        inventory_movements_module,
    )
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
    use crate::tenant::inventory_movements::model::InventoryMovementResolved;
    use crate::{
        common::config::tests::AppConfigBuilder,
        tenant::inventory_movements::{
            self, model::InventoryMovement, repository::MockInventoryMovementsRepository,
            tests::MockInventoryMovementsModule,
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
        let inventory_movement_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Some(Uuid::new_v4());
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockInventoryMovementsRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(inventory_movement_id))
            .returning(move |inventory_movement_id| {
                Ok(InventoryMovement {
                    id: inventory_movement_id,
                    inventory_id,
                    movement_type: "in".to_string(),
                    quantity: "10".parse().unwrap(),
                    reference_type: Some("worksheets".to_string()),
                    reference_id,
                    unit_price: Some("20".parse().unwrap()),
                    total_price: Some("30".parse().unwrap()),
                    tax_id,
                    movement_date: utc_now,
                    created_by_id,
                    created_at: utc_now,
                })
            });

        let mut app_state = MockInventoryMovementsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_movements_repo()
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
                "/api/inventory_movements/get?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_unauthorized_expired() {
        let inventory_movement_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri(format!(
                "/api/inventory_movements/get?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_unauthorized_invalid_signature() {
        let inventory_movement_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri(format!(
                "/api/inventory_movements/get?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_unauthorized_missing() {
        let inventory_movement_id = Uuid::new_v4();
        let app_state = MockInventoryMovementsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_movements/get?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_movement_id = Uuid::new_v4();

        let mut repo = MockInventoryMovementsRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(inventory_movement_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockInventoryMovementsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_movements_repo()
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
                "/api/inventory_movements/get?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_resolved_success() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_movement_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Some(Uuid::new_v4());
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockInventoryMovementsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_movement_id))
            .returning(move |inventory_movement_id| {
                Ok(InventoryMovementResolved {
                    id: inventory_movement_id,
                    inventory_id,
                    movement_type: "in".to_string(),
                    quantity: "10".parse().unwrap(),
                    reference_type: Some("worksheets".to_string()),
                    reference_id,
                    unit_price: Some("20".parse().unwrap()),
                    total_price: Some("30".parse().unwrap()),
                    tax_id,
                    tax: Some("Test Tax".to_string()),
                    movement_date: utc_now,
                    created_by_id,
                    created_by: "Test User".to_string(),
                    created_at: utc_now,
                })
            });

        let mut app_state = MockInventoryMovementsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_movements_repo()
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
                "/api/inventory_movements/get_resolved?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_expired() {
        let inventory_movement_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri(format!(
                "/api/inventory_movements/get_resolved?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_invalid_signature() {
        let inventory_movement_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri(format!(
                "/api/inventory_movements/get_resolved?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_missing() {
        let inventory_movement_id = Uuid::new_v4();
        let app_state = MockInventoryMovementsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_movements/get_resolved?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_get_resolved_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_movement_id = Uuid::new_v4();

        let mut repo = MockInventoryMovementsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_movement_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockInventoryMovementsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_movements_repo()
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
                "/api/inventory_movements/get_resolved?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_success() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_movement_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Some(Uuid::new_v4());
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockInventoryMovementsRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(
                eq(""
                    .parse::<ResourceQuery<InventoryMovementOrderBy, InventoryMovementFilterBy>>()
                    .unwrap()),
                eq(inventory_id),
            )
            .returning(move |_, _| {
                Ok((
                    PaginatorMeta {
                        page: 1,
                        limit: 25,
                        total: 100,
                    },
                    vec![InventoryMovementResolved {
                        id: inventory_movement_id,
                        inventory_id,
                        movement_type: "in".to_string(),
                        quantity: "10".parse().unwrap(),
                        reference_type: Some("worksheets".to_string()),
                        reference_id,
                        unit_price: Some("20".parse().unwrap()),
                        total_price: Some("30".parse().unwrap()),
                        tax_id,
                        tax: Some("Test Tax".to_string()),
                        movement_date: utc_now,
                        created_by_id,
                        created_by: "Test User".to_string(),
                        created_at: utc_now,
                    }],
                ))
            });

        let mut app_state = MockInventoryMovementsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_movements_repo()
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
                "/api/inventory_movements/list?inventory_id={inventory_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri(format!(
                "/api/inventory_movements/list?inventory_id={inventory_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_unauthorized_invalid_signature() {
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri(format!(
                "/api/inventory_movements/list?inventory_id={inventory_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_unauthorized_missing() {
        let inventory_id = Uuid::new_v4();
        let app_state = MockInventoryMovementsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_movements/list?inventory_id={inventory_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_list_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();

        let mut repo = MockInventoryMovementsRepository::new();
        repo.expect_get_paged()
            .times(1)
            .withf(|resource_query, _| {
                *resource_query == ""
                .parse::<ResourceQuery<InventoryMovementOrderBy, InventoryMovementFilterBy>>()
                .unwrap()
            })
            .returning(|_, _| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockInventoryMovementsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_movements_repo()
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
                "/api/inventory_movements/list?inventory_id={inventory_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let inventory_movement_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = InventoryMovementUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            movement_type: "in".to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            unit_price: "20".to_string(),
            tax_id: tax_id.to_string(),
        };
        let user_input = InventoryMovementUserInput::try_from(user_input_helper.clone()).unwrap();

        let mut repo = MockInventoryMovementsRepository::new();
        repo.expect_insert()
            .times(1)
            .withf({
                let user_input_expected = user_input.clone();
                move |user_input, user_id_inner| {
                    user_input.inventory_id == user_input_expected.inventory_id
                        && user_input.movement_type == user_input_expected.movement_type
                        && user_input.quantity == user_input_expected.quantity
                        && user_input.reference_type == user_input_expected.reference_type
                        && user_input.reference_id == user_input_expected.reference_id
                        && user_input.unit_price == user_input_expected.unit_price
                        && user_input.tax_id == user_input_expected.tax_id
                        && user_id == *user_id_inner
                }
            })
            .returning(move |_, _| {
                Ok(InventoryMovement {
                    id: inventory_movement_id,
                    inventory_id,
                    movement_type: "in".to_string(),
                    quantity: "10".parse().unwrap(),
                    reference_type: Some("worksheets".to_string()),
                    reference_id: Some(reference_id),
                    unit_price: Some("20".parse().unwrap()),
                    total_price: Some("30".parse().unwrap()),
                    tax_id,
                    movement_date: utc_now,
                    created_by_id,
                    created_at: utc_now,
                })
            });

        let mut app_state = MockInventoryMovementsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_movements_repo()
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
            .uri("/api/inventory_movements/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = InventoryMovementUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            movement_type: "inn".to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            unit_price: "20".to_string(),
            tax_id: tax_id.to_string(),
        };

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri("/api/inventory_movements/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_create_unauthorized_expired() {
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();

        let user_input_helper = InventoryMovementUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            movement_type: "in".to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            unit_price: "20".to_string(),
            tax_id: tax_id.to_string(),
        };

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri("/api/inventory_movements/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_create_unauthorized_invalid_signature() {
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();

        let user_input_helper = InventoryMovementUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            movement_type: "in".to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            unit_price: "20".to_string(),
            tax_id: tax_id.to_string(),
        };

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri("/api/inventory_movements/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_create_unauthorized_missing() {
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let user_input_helper = InventoryMovementUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            movement_type: "in".to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            unit_price: "20".to_string(),
            tax_id: tax_id.to_string(),
        };

        let app_state = MockInventoryMovementsModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/inventory_movements/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_update_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let inventory_movement_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = InventoryMovementUserInputHelper {
            id: Some(inventory_movement_id.to_string()),
            inventory_id: inventory_id.to_string(),
            movement_type: "in".to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            unit_price: "20".to_string(),
            tax_id: tax_id.to_string(),
        };
        let user_input = InventoryMovementUserInput::try_from(user_input_helper.clone()).unwrap();

        let mut repo = MockInventoryMovementsRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input))
            .returning(move |_| {
                Ok(InventoryMovement {
                    id: inventory_movement_id,
                    inventory_id,
                    movement_type: "in".to_string(),
                    quantity: "10".parse().unwrap(),
                    reference_type: Some("worksheets".to_string()),
                    reference_id: Some(reference_id),
                    unit_price: Some("20".parse().unwrap()),
                    total_price: Some("30".parse().unwrap()),
                    tax_id,
                    movement_date: utc_now,
                    created_by_id,
                    created_at: utc_now,
                })
            });

        let mut app_state = MockInventoryMovementsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_movements_repo()
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
            .uri("/api/inventory_movements/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = InventoryMovementUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            movement_type: "in".to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            unit_price: "20".to_string(),
            tax_id: tax_id.to_string(),
        };

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri("/api/inventory_movements/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_update_unauthorized_expired() {
        let inventory_movement_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();

        let user_input_helper = InventoryMovementUserInputHelper {
            id: Some(inventory_movement_id.to_string()),
            inventory_id: inventory_id.to_string(),
            movement_type: "in".to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            unit_price: "20".to_string(),
            tax_id: tax_id.to_string(),
        };

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri("/api/inventory_movements/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_update_unauthorized_invalid_signature() {
        let inventory_movement_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();

        let user_input_helper = InventoryMovementUserInputHelper {
            id: Some(inventory_movement_id.to_string()),
            inventory_id: inventory_id.to_string(),
            movement_type: "in".to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            unit_price: "20".to_string(),
            tax_id: tax_id.to_string(),
        };

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri("/api/inventory_movements/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_update_unauthorized_missing() {
        let inventory_movement_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();

        let user_input_helper = InventoryMovementUserInputHelper {
            id: Some(inventory_movement_id.to_string()),
            inventory_id: inventory_id.to_string(),
            movement_type: "in".to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            unit_price: "20".to_string(),
            tax_id: tax_id.to_string(),
        };

        let app_state = MockInventoryMovementsModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/inventory_movements/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_delete_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let inventory_movement_id = Uuid::new_v4();
        let mut repo = MockInventoryMovementsRepository::new();

        repo.expect_delete_by_id()
            .times(1)
            .with(eq(inventory_movement_id))
            .returning(move |_| Ok(()));

        let mut app_state = MockInventoryMovementsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_movements_repo()
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
                "/api/inventory_movements/delete?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri("/api/inventory_movements/delete?uuid=invalid_user_input")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_expired() {
        let inventory_movement_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri(format!(
                "/api/inventory_movements/delete?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_invalid_signature() {
        let inventory_movement_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri(format!(
                "/api/inventory_movements/delete?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_missing() {
        let inventory_movement_id = Uuid::new_v4();

        let app_state = MockInventoryMovementsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!(
                "/api/inventory_movements/delete?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_print_success() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_movement_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let inventory_movement_resolved = InventoryMovementResolved {
            id: inventory_movement_id,
            inventory_id,
            movement_type: "in".to_string(),
            quantity: "10".parse().unwrap(),
            reference_type: Some("worksheets".to_string()),
            reference_id: Some(reference_id),
            unit_price: Some("20".parse().unwrap()),
            total_price: Some("30".parse().unwrap()),
            tax_id,
            tax: Some("Test Tax".to_string()),
            movement_date: utc_now,
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: utc_now,
        };

        let mut repo = MockInventoryMovementsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_movement_id))
            .returning({
                let inventory_movement_resolved = inventory_movement_resolved.clone();
                move |_| Ok(inventory_movement_resolved.clone())
            });

        let mut app_state = MockInventoryMovementsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_movements_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let pdf_gen_payload_expected = vec![
            InventoryMovementsResolvedPrint::from_inventory_movements_resolved(
                inventory_movement_resolved,
                "Europe/Budapest".parse().unwrap(),
            ),
        ];

        let _m = PDF_GENERATOR_TEST_SYNC.lock();
        let pdf_gen = MockPdfGenerator::gen_pdf_temporary_context();
        pdf_gen
            .expect::<Vec<InventoryMovementsResolvedPrint>>()
            .times(1)
            .with(
                eq(PdfTemplates::InventoryMovementView),
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
            .uri(format!(
                "/api/inventory_movements/print?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_print_unauthorized_expired() {
        let inventory_movement_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri(format!(
                "/api/inventory_movements/print?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_print_unauthorized_invalid_signature() {
        let inventory_movement_id = Uuid::new_v4();

        let mut app_state = MockInventoryMovementsModule::new();
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
            .uri(format!(
                "/api/inventory_movements/print?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_print_unauthorized_missing() {
        let inventory_movement_id = Uuid::new_v4();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_movements/print?uuid={inventory_movement_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_movements::routes::routes(Arc::new(
                MockInventoryMovementsModule::new(),
            ))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
