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
use crate::tenant::inventory::InventoryModuleInterface;
use crate::tenant::inventory::dto::print::InventoryResolvedPrint;
use crate::tenant::inventory::dto::user_input::{InventoryUserInput, InventoryUserInputHelper};
use crate::tenant::inventory::service::InventoryService;
use crate::tenant::inventory::types::inventory::{InventoryFilterBy, InventoryOrderBy};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub async fn get_resolved<M: InventoryModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_module.clone());
    let result = map_handler_err(
        service.get_resolved(payload.uuid).await,
        inventory_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        inventory_module,
    )
    .await?
    .into_response())
}

pub async fn get<M: InventoryModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_module.clone());
    let result = map_handler_err(service.get(payload.uuid).await, inventory_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        inventory_module,
    )
    .await?
    .into_response())
}

pub async fn update<M: InventoryModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<InventoryUserInput, InventoryUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_module.clone());
    let result =
        map_handler_err(service.update(&user_input).await, inventory_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        inventory_module,
    )
    .await?
    .into_response())
}

pub async fn delete<M: InventoryModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_module.clone());
    map_handler_err(service.delete(payload.uuid).await, inventory_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(SimpleMessageResponse::new(
                "A raktárkészlet törlése sikeresen megtörtént",
            ))
            .build(),
        inventory_module,
    )
    .await?
    .into_response())
}

pub async fn create<M: InventoryModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<InventoryUserInput, InventoryUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_module.clone());
    let result =
        map_handler_err(service.insert(&user_input).await, inventory_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(result)
            .build(),
        inventory_module,
    )
    .await?
    .into_response())
}

pub async fn list<M: InventoryModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<M>>,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_module.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<InventoryOrderBy, InventoryFilterBy>::from_str(payload.q()),
        inventory_module.clone(),
    )
    .await?;
    let (meta, data) = map_handler_err(
        service.get_paged(&resource_query).await,
        inventory_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::new()
            .status_code(StatusCode::OK)
            .meta(meta)
            .data(data)
            .build(),
        inventory_module,
    )
    .await?
    .into_response())
}

pub async fn select_list<M: InventoryModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<M>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_module.clone());
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));
    let result = map_handler_err(
        service.get_select_list_items(&list_type).await,
        inventory_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        inventory_module,
    )
    .await?
    .into_response())
}

pub async fn print<M: InventoryModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_module.clone());
    let inventory_resolved_print = InventoryResolvedPrint::from_inventory_resolved(
        map_handler_err(
            service.get_resolved(payload.uuid).await,
            inventory_module.clone(),
        )
        .await?,
        map_handler_err(claims.tz(), inventory_module.clone()).await?,
    );
    let pdf = map_handler_err(
        service.print(&[inventory_resolved_print]).await,
        inventory_module.clone(),
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
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;

    use super::*;
    use crate::common::dto::PaginatorMeta;
    use crate::common::error::RepositoryError;
    use crate::common::handler::tests::{
        extract_json_response, generate_expired_jwt, generate_jwt_with_invalid_signature,
        generate_valid_jwt,
    };
    use crate::common::pdf::tests::{PDF_GENERATOR_TEST_SYNC, extract_pdf_text};
    use crate::common::pdf::{MockPdfGenerator, PdfGenerator, PdfTemplates};
    use crate::tenant::inventory::model::InventoryResolved;
    use crate::{
        common::config::tests::AppConfigBuilder,
        tenant::inventory::{
            self, model::Inventory, repository::MockInventoryRepository, tests::MockInventoryModule,
        },
    };
    use axum::body::Body;
    use axum::{Router, http::Request};
    use chrono::{DateTime, Utc};
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_success() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let inventory = Inventory {
            id: inventory_id,
            product_id,
            warehouse_id,
            quantity_on_hand: "10".parse().unwrap(),
            quantity_reserved: "20".parse().unwrap(),
            quantity_available: "30".parse().unwrap(),
            minimum_stock: None,
            maximum_stock: None,
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockInventoryRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(inventory_id))
            .returning({
                let inventory = inventory.clone();
                move |_| Ok(inventory.clone())
            });

        let mut app_state = MockInventoryModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_repo()
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
            .uri(format!("/api/inventory/get?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": inventory,
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_unauthorized_expired() {
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryModule::new();
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
            .uri(format!("/api/inventory/get?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryModule::new();
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
            .uri(format!("/api/inventory/get?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();
        let app_state = MockInventoryModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/inventory/get?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();

        let mut repo = MockInventoryRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(inventory_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockInventoryModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_repo()
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
            .uri(format!("/api/inventory/get?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let inventory_resolved = InventoryResolved {
            id: inventory_id,
            product_id,
            product: "Test product".to_string(),
            warehouse_id,
            warehouse: "Test warehouse".to_string(),
            quantity_on_hand: "10".parse().unwrap(),
            quantity_reserved: "20".parse().unwrap(),
            quantity_available: "30".parse().unwrap(),
            minimum_stock: None,
            maximum_stock: None,
            currency_code: "HUF".to_string(),
            currency: "Forint".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockInventoryRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_id))
            .returning({
                let inventory_resolved = inventory_resolved.clone();
                move |_| Ok(inventory_resolved.clone())
            });

        let mut app_state = MockInventoryModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_repo()
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
            .uri(format!("/api/inventory/get_resolved?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": inventory_resolved
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_expired() {
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryModule::new();
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
            .uri(format!("/api/inventory/get_resolved?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryModule::new();
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
            .uri(format!("/api/inventory/get_resolved?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();
        let app_state = MockInventoryModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/inventory/get_resolved?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();

        let mut repo = MockInventoryRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockInventoryModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_repo()
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
            .uri(format!("/api/inventory/get_resolved?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let paginator_meta = PaginatorMeta {
            page: 1,
            limit: 25,
            total: 100,
        };
        let inventory_resolved = InventoryResolved {
            id: inventory_id,
            product_id,
            product: "Test product".to_string(),
            warehouse_id,
            warehouse: "Test warehouse".to_string(),
            quantity_on_hand: "10".parse().unwrap(),
            quantity_reserved: "20".parse().unwrap(),
            quantity_available: "30".parse().unwrap(),
            minimum_stock: None,
            maximum_stock: None,
            currency_code: "HUF".to_string(),
            currency: "Forint".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockInventoryRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<InventoryOrderBy, InventoryFilterBy>>()
                .unwrap()))
            .returning({
                let inventory_resolved = inventory_resolved.clone();
                move |_| Ok((paginator_meta, vec![inventory_resolved.clone()]))
            });

        let mut app_state = MockInventoryModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_repo()
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
            .uri("/api/inventory/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": paginator_meta,
            "data": [inventory_resolved],
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let mut app_state = MockInventoryModule::new();
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
            .uri("/api/inventory/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let mut app_state = MockInventoryModule::new();
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
            .uri("/api/inventory/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let app_state = MockInventoryModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/inventory/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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

        let mut repo = MockInventoryRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<InventoryOrderBy, InventoryFilterBy>>()
                .unwrap()))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockInventoryModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_repo()
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
            .uri("/api/inventory/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = InventoryUserInputHelper {
            id: None,
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: "".to_string(),
            maximum_stock: "".to_string(),
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
        };
        let user_input = InventoryUserInput::try_from(user_input_helper.clone()).unwrap();
        let inventory = Inventory {
            id: inventory_id,
            product_id,
            warehouse_id,
            quantity_on_hand: "10".parse().unwrap(),
            quantity_reserved: "20".parse().unwrap(),
            quantity_available: "30".parse().unwrap(),
            minimum_stock: None,
            maximum_stock: None,
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockInventoryRepository::new();
        repo.expect_insert()
            .times(1)
            .withf({
                let user_input_expected = user_input.clone();
                move |user_input, user_id_inner| {
                    user_input.product_id == user_input_expected.product_id
                        && user_input.warehouse_id == user_input_expected.warehouse_id
                        && user_input.minimum_stock == user_input_expected.minimum_stock
                        && user_input.maximum_stock == user_input_expected.maximum_stock
                        && user_input.currency_code == user_input_expected.currency_code
                        && user_input.status == user_input_expected.status
                        && user_id == *user_id_inner
                }
            })
            .returning({
                let inventory = inventory.clone();
                move |_, _| Ok(inventory.clone())
            });

        let mut app_state = MockInventoryModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_repo()
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
            .uri("/api/inventory/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": inventory
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_invalid_user_input() {
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = InventoryUserInputHelper {
            id: None,
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: "".to_string(),
            maximum_stock: "".to_string(),
            currency_code: "HUF".to_string(),
            status: "activee".to_string(),
        };

        let mut app_state = MockInventoryModule::new();
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
            .uri("/api/inventory/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let user_input_error = InventoryUserInput::try_from(user_input_helper).unwrap_err();

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Kérjük ellenőrizze a hibás mezőket!",
                "fields": user_input_error
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_unauthorized_expired() {
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();

        let user_input_helper = InventoryUserInputHelper {
            id: None,
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: "".to_string(),
            maximum_stock: "".to_string(),
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockInventoryModule::new();
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
            .uri("/api/inventory/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();

        let user_input_helper = InventoryUserInputHelper {
            id: None,
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: "".to_string(),
            maximum_stock: "".to_string(),
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockInventoryModule::new();
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
            .uri("/api/inventory/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();

        let user_input_helper = InventoryUserInputHelper {
            id: None,
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: "".to_string(),
            maximum_stock: "".to_string(),
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
        };

        let app_state = MockInventoryModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/inventory/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = InventoryUserInputHelper {
            id: Some(inventory_id.to_string()),
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: "".to_string(),
            maximum_stock: "".to_string(),
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
        };
        let user_input = InventoryUserInput::try_from(user_input_helper.clone()).unwrap();
        let inventory = Inventory {
            id: inventory_id,
            product_id,
            warehouse_id,
            quantity_on_hand: "10".parse().unwrap(),
            quantity_reserved: "20".parse().unwrap(),
            quantity_available: "30".parse().unwrap(),
            minimum_stock: None,
            maximum_stock: None,
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockInventoryRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input))
            .returning({
                let inventory = inventory.clone();
                move |_| Ok(inventory.clone())
            });

        let mut app_state = MockInventoryModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_repo()
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
            .uri("/api/inventory/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": inventory
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = InventoryUserInputHelper {
            id: None,
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: "".to_string(),
            maximum_stock: "".to_string(),
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockInventoryModule::new();
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
            .uri("/api/inventory/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();

        let user_input_helper = InventoryUserInputHelper {
            id: Some(inventory_id.to_string()),
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: "".to_string(),
            maximum_stock: "".to_string(),
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockInventoryModule::new();
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
            .uri("/api/inventory/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();

        let user_input_helper = InventoryUserInputHelper {
            id: Some(inventory_id.to_string()),
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: "".to_string(),
            maximum_stock: "".to_string(),
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockInventoryModule::new();
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
            .uri("/api/inventory/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();

        let user_input_helper = InventoryUserInputHelper {
            id: Some(inventory_id.to_string()),
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: "".to_string(),
            maximum_stock: "".to_string(),
            currency_code: "HUF".to_string(),
            status: "active".to_string(),
        };

        let app_state = MockInventoryModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/inventory/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();
        let mut repo = MockInventoryRepository::new();

        repo.expect_delete_by_id()
            .times(1)
            .with(eq(inventory_id))
            .returning(move |_| Ok(()));

        let mut app_state = MockInventoryModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_repo()
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
            .uri(format!("/api/inventory/delete?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": {
                "message": "A raktárkészlet törlése sikeresen megtörtént"
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut app_state = MockInventoryModule::new();
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
            .uri("/api/inventory/delete?uuid=invalid_user_input")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_expired() {
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryModule::new();
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
            .uri(format!("/api/inventory/delete?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryModule::new();
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
            .uri(format!("/api/inventory/delete?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();

        let app_state = MockInventoryModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!("/api/inventory/delete?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse().unwrap();
        let product_id = "0237354a-21ab-46f4-a4ca-b21cb08561d7".parse().unwrap();
        let warehouse_id = "521f9728-f59f-435d-8656-69ba4273254c".parse().unwrap();
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0".parse().unwrap();
        let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z".parse().unwrap();

        let inventory_resolved = InventoryResolved {
            id: inventory_id,
            product_id,
            product: "Test product".to_string(),
            warehouse_id,
            warehouse: "Test warehouse".to_string(),
            quantity_on_hand: "10".parse().unwrap(),
            quantity_reserved: "20".parse().unwrap(),
            quantity_available: "30".parse().unwrap(),
            minimum_stock: None,
            maximum_stock: None,
            currency_code: "HUF".to_string(),
            currency: "Forint".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
        };

        let mut repo = MockInventoryRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_id))
            .returning({
                let inventory_resolved = inventory_resolved.clone();
                move |_| Ok(inventory_resolved.clone())
            });

        let mut app_state = MockInventoryModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let pdf_gen_payload_expected = vec![InventoryResolvedPrint::from_inventory_resolved(
            inventory_resolved,
            "Europe/Budapest".parse().unwrap(),
        )];

        let _m = PDF_GENERATOR_TEST_SYNC.lock();
        let pdf_gen = MockPdfGenerator::gen_pdf_temporary_context();
        pdf_gen
            .expect::<Vec<InventoryResolvedPrint>>()
            .times(1)
            .with(
                eq(PdfTemplates::InventoryView),
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
            .uri(format!("/api/inventory/print?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let snapshot_path = Path::new("testing/pdf/snapshots/inventory_test.pdf");
        let mut file = File::open(snapshot_path).unwrap();
        let mut snapshot = vec![];
        file.read_to_end(&mut snapshot).unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        let test_result_path = Path::new("testing/pdf/test_results/inventory_test.pdf");
        let mut file = File::create(test_result_path).unwrap();
        file.write_all(&body_bytes).unwrap();

        assert_eq!(
            extract_pdf_text(&body_bytes).unwrap(),
            extract_pdf_text(&snapshot).unwrap()
        );
    }

    #[tokio::test]
    async fn test_print_unauthorized_expired() {
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryModule::new();
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
            .uri(format!("/api/inventory/print?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryModule::new();
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
            .uri(format!("/api/inventory/print?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::new_v4();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/inventory/print?uuid={inventory_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory::routes::routes(Arc::new(
                MockInventoryModule::new(),
            ))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
}
