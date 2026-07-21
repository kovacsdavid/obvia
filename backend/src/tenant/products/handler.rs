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
use crate::tenant::products::ProductsModuleInterface;
use crate::tenant::products::dto::print::ProductsResolvedPrint;
use crate::tenant::products::dto::user_input::{ProductUserInput, ProductUserInputHelper};
use crate::tenant::products::service::ProductService;
use crate::tenant::products::types::product::{ProductFilterBy, ProductOrderBy};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub async fn get_resolved<M: ProductsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), products_module.clone());
    let result = map_handler_err(
        service.get_resolved(payload.uuid).await,
        products_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        products_module,
    )
    .await?
    .into_response())
}

pub async fn get<M: ProductsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), products_module.clone());
    let result = map_handler_err(service.get(payload.uuid).await, products_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        products_module,
    )
    .await?
    .into_response())
}

pub async fn update<M: ProductsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<ProductUserInput, ProductUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), products_module.clone());
    let result =
        map_handler_err(service.update(&user_input).await, products_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        products_module,
    )
    .await?
    .into_response())
}

pub async fn delete<M: ProductsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), products_module.clone());
    map_handler_err(service.delete(payload.uuid).await, products_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(SimpleMessageResponse::new(
                "A termék törlése sikeresen megtörtént",
            ))
            .build(),
        products_module,
    )
    .await?
    .into_response())
}

pub async fn create<M: ProductsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<M>>,
    UserInput(mut user_input, _): UserInput<ProductUserInput, ProductUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), products_module.clone());
    let result = map_handler_err(
        service.insert(&mut user_input).await,
        products_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(result)
            .build(),
        products_module,
    )
    .await?
    .into_response())
}

pub async fn list<M: ProductsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<M>>,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), products_module.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<ProductOrderBy, ProductFilterBy>::from_str(payload.q()),
        products_module.clone(),
    )
    .await?;
    let (meta, data) = map_handler_err(
        service.get_paged(&resource_query).await,
        products_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::new()
            .status_code(StatusCode::OK)
            .meta(meta)
            .data(data)
            .build(),
        products_module,
    )
    .await?
    .into_response())
}

pub async fn select_list<M: ProductsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<M>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), products_module.clone());
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));

    let result = map_handler_err(
        service.get_select_list_items(&list_type).await,
        products_module.clone(),
    )
    .await?;

    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        products_module,
    )
    .await?
    .into_response())
}

pub async fn print<M: ProductsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(products_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), products_module.clone());
    let product_resolved_print = ProductsResolvedPrint::from_product_resolved(
        map_handler_err(
            service.get_resolved(payload.uuid).await,
            products_module.clone(),
        )
        .await?,
        map_handler_err(claims.tz(), products_module.clone()).await?,
    );
    let pdf = map_handler_err(
        service.print(&[product_resolved_print]).await,
        products_module,
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
    use crate::tenant::products::model::ProductResolved;
    use crate::{
        common::config::tests::AppConfigBuilder,
        tenant::products::{
            self, model::Product, repository::MockProductsRepository, tests::MockProductsModule,
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
        let product_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();
        let product = Product {
            id: product_id,
            name: "Test product".to_string(),
            description: None,
            unit_of_measure_id,
            status: "active".to_string(),
            created_by_id,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockProductsRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(product_id))
            .returning({
                let product = product.clone();
                move |_| Ok(product.clone())
            });

        let mut app_state = MockProductsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_products_repo()
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
            .uri(format!("/api/products/get?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": product
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_unauthorized_expired() {
        let product_id = Uuid::new_v4();

        let mut app_state = MockProductsModule::new();
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
            .uri(format!("/api/products/get?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();

        let mut app_state = MockProductsModule::new();
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
            .uri(format!("/api/products/get?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let app_state = MockProductsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/products/get?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();

        let mut repo = MockProductsRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(product_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockProductsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_products_repo()
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
            .uri(format!("/api/products/get?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();
        let product_resolved = ProductResolved {
            id: product_id,
            name: "Test product".to_string(),
            description: None,
            unit_of_measure_id,
            unit_of_measure: "cm".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockProductsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(product_id))
            .returning({
                let product_resolved = product_resolved.clone();
                move |_| Ok(product_resolved.clone())
            });

        let mut app_state = MockProductsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_products_repo()
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
            .uri(format!("/api/products/get_resolved?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": product_resolved
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_expired() {
        let product_id = Uuid::new_v4();

        let mut app_state = MockProductsModule::new();
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
            .uri(format!("/api/products/get_resolved?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();

        let mut app_state = MockProductsModule::new();
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
            .uri(format!("/api/products/get_resolved?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let app_state = MockProductsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/products/get_resolved?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();

        let mut repo = MockProductsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(product_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockProductsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_products_repo()
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
            .uri(format!("/api/products/get_resolved?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();
        let paginator_meta = PaginatorMeta {
            page: 1,
            limit: 25,
            total: 100,
        };
        let product_resolved = ProductResolved {
            id: product_id,
            name: "Test product".to_string(),
            description: None,
            unit_of_measure_id,
            unit_of_measure: "cm".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockProductsRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<ProductOrderBy, ProductFilterBy>>()
                .unwrap()))
            .returning({
                let product_resolved = product_resolved.clone();
                move |_| Ok((paginator_meta, vec![product_resolved.clone()]))
            });

        let mut app_state = MockProductsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_products_repo()
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
            .uri("/api/products/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": paginator_meta,
            "data": [product_resolved]
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let mut app_state = MockProductsModule::new();
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
            .uri("/api/products/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let mut app_state = MockProductsModule::new();
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
            .uri("/api/products/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let app_state = MockProductsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/products/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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

        let mut repo = MockProductsRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<ProductOrderBy, ProductFilterBy>>()
                .unwrap()))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockProductsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_products_repo()
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
            .uri("/api/products/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();
        let product = Product {
            id: product_id,
            name: "Test product".to_string(),
            description: None,
            unit_of_measure_id,
            status: "active".to_string(),
            created_by_id,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let user_input_helper = ProductUserInputHelper {
            id: None,
            name: "Test product".to_string(),
            description: "".to_string(),
            unit_of_measure_id: unit_of_measure_id.to_string(),
            new_unit_of_measure: "".to_string(),
            status: "active".to_string(),
        };
        let user_input = ProductUserInput::try_from(user_input_helper.clone()).unwrap();

        let mut repo = MockProductsRepository::new();
        repo.expect_insert()
            .times(1)
            .withf({
                let user_input_expected = user_input.clone();
                move |user_input, user_id_inner| {
                    user_input.name == user_input_expected.name
                        && user_input.description == user_input_expected.description
                        && user_input.unit_of_measure_id == user_input_expected.unit_of_measure_id
                        && user_input.new_unit_of_measure == user_input_expected.new_unit_of_measure
                        && user_input.status == user_input_expected.status
                        && user_id == *user_id_inner
                }
            })
            .returning({
                let product = product.clone();
                move |_, _| Ok(product.clone())
            });

        let mut app_state = MockProductsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_products_repo()
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
            .uri("/api/products/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": product
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = ProductUserInputHelper {
            id: None,
            name: "Test product".to_string(),
            description: "".to_string(),
            unit_of_measure_id: unit_of_measure_id.to_string(),
            new_unit_of_measure: "".to_string(),
            status: "activee".to_string(),
        };

        let mut app_state = MockProductsModule::new();
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
            .uri("/api/products/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let user_input_error = ProductUserInput::try_from(user_input_helper).unwrap_err();

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
        let unit_of_measure_id = Uuid::new_v4();

        let user_input_helper = ProductUserInputHelper {
            id: None,
            name: "Test product".to_string(),
            description: "".to_string(),
            unit_of_measure_id: unit_of_measure_id.to_string(),
            new_unit_of_measure: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockProductsModule::new();
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
            .uri("/api/products/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let unit_of_measure_id = Uuid::new_v4();

        let user_input_helper = ProductUserInputHelper {
            id: None,
            name: "Test product".to_string(),
            description: "".to_string(),
            unit_of_measure_id: unit_of_measure_id.to_string(),
            new_unit_of_measure: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockProductsModule::new();
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
            .uri("/api/products/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let unit_of_measure_id = Uuid::new_v4();

        let user_input_helper = ProductUserInputHelper {
            id: None,
            name: "Test product".to_string(),
            description: "".to_string(),
            unit_of_measure_id: unit_of_measure_id.to_string(),
            new_unit_of_measure: "".to_string(),
            status: "active".to_string(),
        };

        let app_state = MockProductsModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/products/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();
        let product = Product {
            id: product_id,
            name: "Test product".to_string(),
            description: None,
            unit_of_measure_id,
            status: "active".to_string(),
            created_by_id,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let user_input_helper = ProductUserInputHelper {
            id: Some(product_id.to_string()),
            name: "Test product".to_string(),
            description: "".to_string(),
            unit_of_measure_id: unit_of_measure_id.to_string(),
            new_unit_of_measure: "".to_string(),
            status: "active".to_string(),
        };
        let user_input = ProductUserInput::try_from(user_input_helper.clone()).unwrap();

        let mut repo = MockProductsRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input))
            .returning({
                let product = product.clone();
                move |_| Ok(product.clone())
            });

        let mut app_state = MockProductsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_products_repo()
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
            .uri("/api/products/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": product,
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = ProductUserInputHelper {
            id: None,
            name: "Test product".to_string(),
            description: "".to_string(),
            unit_of_measure_id: unit_of_measure_id.to_string(),
            new_unit_of_measure: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockProductsModule::new();
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
            .uri("/api/products/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();

        let user_input_helper = ProductUserInputHelper {
            id: Some(product_id.to_string()),
            name: "Test product".to_string(),
            description: "".to_string(),
            unit_of_measure_id: unit_of_measure_id.to_string(),
            new_unit_of_measure: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockProductsModule::new();
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
            .uri("/api/products/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();

        let user_input_helper = ProductUserInputHelper {
            id: Some(product_id.to_string()),
            name: "Test product".to_string(),
            description: "".to_string(),
            unit_of_measure_id: unit_of_measure_id.to_string(),
            new_unit_of_measure: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockProductsModule::new();
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
            .uri("/api/products/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();

        let user_input_helper = ProductUserInputHelper {
            id: Some(product_id.to_string()),
            name: "Test product".to_string(),
            description: "".to_string(),
            unit_of_measure_id: unit_of_measure_id.to_string(),
            new_unit_of_measure: "".to_string(),
            status: "active".to_string(),
        };

        let app_state = MockProductsModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/products/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();
        let mut repo = MockProductsRepository::new();

        repo.expect_delete_by_id()
            .times(1)
            .with(eq(product_id))
            .returning(move |_| Ok(()));

        let mut app_state = MockProductsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_products_repo()
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
            .uri(format!("/api/products/delete?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": {
                "message": "A termék törlése sikeresen megtörtént"
            },
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut app_state = MockProductsModule::new();
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
            .uri("/api/products/delete?uuid=invalid_user_input")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_expired() {
        let product_id = Uuid::new_v4();

        let mut app_state = MockProductsModule::new();
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
            .uri(format!("/api/products/delete?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();

        let mut app_state = MockProductsModule::new();
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
            .uri(format!("/api/products/delete?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();

        let app_state = MockProductsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!("/api/products/delete?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse().unwrap();
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0".parse().unwrap();
        let unit_of_measure_id = "0237354a-21ab-46f4-a4ca-b21cb08561d7".parse().unwrap();
        let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z".parse().unwrap();

        let product_resolved = ProductResolved {
            id: product_id,
            name: "Test product".to_string(),
            description: None,
            unit_of_measure_id,
            unit_of_measure: "cm".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
        };

        let mut repo = MockProductsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(product_id))
            .returning({
                let product_resolved = product_resolved.clone();
                move |_| Ok(product_resolved.clone())
            });

        let mut app_state = MockProductsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_products_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let pdf_gen_payload_expected = vec![ProductsResolvedPrint::from_product_resolved(
            product_resolved,
            "Europe/Budapest".parse().unwrap(),
        )];

        let _m = PDF_GENERATOR_TEST_SYNC.lock();
        let pdf_gen = MockPdfGenerator::gen_pdf_temporary_context();
        pdf_gen
            .expect::<Vec<ProductsResolvedPrint>>()
            .times(1)
            .with(eq(PdfTemplates::ProductView), eq(pdf_gen_payload_expected))
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
            .uri(format!("/api/products/print?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let snapshot_path = Path::new("testing/pdf/snapshots/products_test.pdf");
        let mut file = File::open(snapshot_path).unwrap();
        let mut snapshot = vec![];
        file.read_to_end(&mut snapshot).unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        let test_result_path = Path::new("testing/pdf/test_results/products_test.pdf");
        let mut file = File::create(test_result_path).unwrap();
        file.write_all(&body_bytes).unwrap();

        assert_eq!(
            extract_pdf_text(&body_bytes).unwrap(),
            extract_pdf_text(&snapshot).unwrap()
        );
    }

    #[tokio::test]
    async fn test_print_unauthorized_expired() {
        let product_id = Uuid::new_v4();

        let mut app_state = MockProductsModule::new();
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
            .uri(format!("/api/products/print?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();

        let mut app_state = MockProductsModule::new();
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
            .uri(format!("/api/products/print?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(Arc::new(app_state))),
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
        let product_id = Uuid::new_v4();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/products/print?uuid={product_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(products::routes::routes(
                Arc::new(MockProductsModule::new()),
            )),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
}
