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
use crate::tenant::customers::CustomersModuleInterface;
use crate::tenant::customers::dto::print::CustomerResolvedPrint;
use crate::tenant::customers::dto::user_input::{CustomerUserInput, CustomerUserInputHelper};
use crate::tenant::customers::service::CustomerService;
use crate::tenant::customers::types::customer::{CustomerFilterBy, CustomerOrderBy};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use std::str::FromStr;
use std::sync::Arc;

pub async fn get_resolved<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let result = map_handler_err(
        service.get_resolved(payload.uuid).await,
        customers_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        customers_module,
    )
    .await?
    .into_response())
}

pub async fn get<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let result = map_handler_err(service.get(payload.uuid).await, customers_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        customers_module,
    )
    .await?
    .into_response())
}

pub async fn create<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<CustomerUserInput, CustomerUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let result =
        map_handler_err(service.insert(&user_input).await, customers_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::CREATED)
            .data(result)
            .build(),
        customers_module,
    )
    .await?
    .into_response())
}

pub async fn update<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<CustomerUserInput, CustomerUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let result =
        map_handler_err(service.update(&user_input).await, customers_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        customers_module,
    )
    .await?
    .into_response())
}

pub async fn delete<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    map_handler_err(service.delete(payload.uuid).await, customers_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(SimpleMessageResponse::new(
                "A vevő törlése sikeresen megtörtént",
            ))
            .build(),
        customers_module,
    )
    .await?
    .into_response())
}

pub async fn list<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<CustomerOrderBy, CustomerFilterBy>::from_str(payload.q()),
        customers_module.clone(),
    )
    .await?;
    let (meta, data) = map_handler_err(
        service.get_paged(&resource_query).await,
        customers_module.clone(),
    )
    .await?;

    Ok(map_handler_err(
        SuccessResponseBuilder::new()
            .status_code(StatusCode::OK)
            .meta(meta)
            .data(data)
            .build(),
        customers_module,
    )
    .await?
    .into_response())
}

pub async fn print<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let customer_resolved_print = CustomerResolvedPrint::from_customer_revolved(
        map_handler_err(
            service.get_resolved(payload.uuid).await,
            customers_module.clone(),
        )
        .await?,
        map_handler_err(claims.tz(), customers_module.clone()).await?,
    );
    let customer_id = customer_resolved_print.id();
    let pdf = map_handler_err(
        service.print(&[customer_resolved_print]).await,
        customers_module,
    )
    .await?;
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!(r#"inline; filename="vevo_{customer_id}.pdf""#)
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
    use crate::tenant::customers::model::CustomerResolved;
    use crate::{
        common::config::tests::AppConfigBuilder,
        tenant::customers::{
            self, model::Customer, repository::MockCustomersRepository, tests::MockCustomersModule,
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
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let customer = Customer {
            id: customer_id,
            name: "Test customer".to_string(),
            contact_name: None,
            email: "test_customer@example.com".to_string(),
            phone_number: Some("+36301234567".to_string()),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
            created_by_id,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockCustomersRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(customer_id))
            .returning({
                let customer = customer.clone();
                move |_| Ok(customer.clone())
            });

        let mut app_state = MockCustomersModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_customers_repo()
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
            .uri(format!("/api/customers/get?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": customer
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_unauthorized_expired() {
        let customer_id = Uuid::new_v4();

        let mut app_state = MockCustomersModule::new();
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
            .uri(format!("/api/customers/get?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();

        let mut app_state = MockCustomersModule::new();
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
            .uri(format!("/api/customers/get?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();
        let app_state = MockCustomersModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/customers/get?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();

        let mut repo = MockCustomersRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(customer_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockCustomersModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_customers_repo()
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
            .uri(format!("/api/customers/get?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let customer_resolved = CustomerResolved {
            id: customer_id,
            name: "Test customer".to_string(),
            contact_name: None,
            email: "test_customer@example.com".to_string(),
            phone_number: Some("+36301234567".to_string()),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockCustomersRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(customer_id))
            .returning({
                let customer_resolved = customer_resolved.clone();
                move |_| Ok(customer_resolved.clone())
            });

        let mut app_state = MockCustomersModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_customers_repo()
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
            .uri(format!("/api/customers/get_resolved?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": customer_resolved
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_expired() {
        let customer_id = Uuid::new_v4();

        let mut app_state = MockCustomersModule::new();
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
            .uri(format!("/api/customers/get_resolved?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();

        let mut app_state = MockCustomersModule::new();
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
            .uri(format!("/api/customers/get_resolved?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();
        let app_state = MockCustomersModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/customers/get_resolved?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();

        let mut repo = MockCustomersRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(customer_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockCustomersModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_customers_repo()
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
            .uri(format!("/api/customers/get_resolved?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let paginator_meta = PaginatorMeta {
            page: 1,
            limit: 25,
            total: 100,
        };
        let customer_resolved = CustomerResolved {
            id: customer_id,
            name: "Test customer".to_string(),
            contact_name: None,
            email: "test_customer@example.com".to_string(),
            phone_number: Some("+36301234567".to_string()),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockCustomersRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<CustomerOrderBy, CustomerFilterBy>>()
                .unwrap()))
            .returning({
                let customer_resolved = customer_resolved.clone();
                move |_| Ok((paginator_meta, vec![customer_resolved.clone()]))
            });

        let mut app_state = MockCustomersModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_customers_repo()
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
            .uri("/api/customers/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": paginator_meta,
            "data": vec![customer_resolved]
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let mut app_state = MockCustomersModule::new();
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
            .uri("/api/customers/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let mut app_state = MockCustomersModule::new();
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
            .uri("/api/customers/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let app_state = MockCustomersModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/customers/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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

        let mut repo = MockCustomersRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<CustomerOrderBy, CustomerFilterBy>>()
                .unwrap()))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockCustomersModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_customers_repo()
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
            .uri("/api/customers/list")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = CustomerUserInputHelper {
            id: None,
            name: "Test Customer".to_string(),
            contact_name: "".to_string(),
            email: "test.customer@example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
        };
        let user_input = CustomerUserInput::try_from(user_input_helper.clone()).unwrap();

        let customer = Customer {
            id: customer_id,
            name: "Test Customer".to_string(),
            contact_name: None,
            email: "test.customer@example.com".to_string(),
            phone_number: Some("36301234567".to_string()),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
            created_by_id,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockCustomersRepository::new();
        repo.expect_insert()
            .times(1)
            .withf({
                let user_input_expected = user_input.clone();
                move |user_input, user_id_inner| {
                    user_input.name == user_input_expected.name
                        && user_input.contact_name == user_input_expected.contact_name
                        && user_input.email == user_input_expected.email
                        && user_input.phone_number == user_input_expected.phone_number
                        && user_input.status == user_input_expected.status
                        && user_input.customer_type == user_input_expected.customer_type
                        && user_id == *user_id_inner
                }
            })
            .returning({
                let customer = customer.clone();
                move |_, _| Ok(customer.clone())
            });

        let mut app_state = MockCustomersModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_customers_repo()
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
            .uri("/api/customers/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": customer,
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = CustomerUserInputHelper {
            id: None,
            name: "Test Customer".to_string(),
            contact_name: "".to_string(),
            email: "test.customer@example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            status: "activee".to_string(),
            customer_type: "natural".to_string(),
        };

        let mut app_state = MockCustomersModule::new();
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
            .uri("/api/customers/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let field_errors = CustomerUserInput::try_from(user_input_helper).unwrap_err();

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
        let user_input_helper = CustomerUserInputHelper {
            id: None,
            name: "Test Customer".to_string(),
            contact_name: "".to_string(),
            email: "test.customer@example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
        };

        let mut app_state = MockCustomersModule::new();
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
            .uri("/api/customers/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = CustomerUserInputHelper {
            id: None,
            name: "Test Customer".to_string(),
            contact_name: "".to_string(),
            email: "test.customer@example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
        };

        let mut app_state = MockCustomersModule::new();
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
            .uri("/api/customers/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = CustomerUserInputHelper {
            id: None,
            name: "Test Customer".to_string(),
            contact_name: "".to_string(),
            email: "test.customer@example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
        };

        let app_state = MockCustomersModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/customers/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = CustomerUserInputHelper {
            id: Some(customer_id.to_string()),
            name: "Test Customer".to_string(),
            contact_name: "".to_string(),
            email: "test.customer@example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
        };
        let user_input = CustomerUserInput::try_from(user_input_helper.clone()).unwrap();

        let customer = Customer {
            id: customer_id,
            name: "Test Customer".to_string(),
            contact_name: None,
            email: "test.customer@example.com".to_string(),
            phone_number: Some("36301234567".to_string()),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
            created_by_id,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        };

        let mut repo = MockCustomersRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input))
            .returning({
                let customer = customer.clone();
                move |_| Ok(customer.clone())
            });

        let mut app_state = MockCustomersModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_customers_repo()
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
            .uri("/api/customers/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": customer
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = CustomerUserInputHelper {
            id: None,
            name: "Test Customer".to_string(),
            contact_name: "".to_string(),
            email: "test.customer@example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
        };

        let mut app_state = MockCustomersModule::new();
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
            .uri("/api/customers/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = CustomerUserInputHelper {
            id: None,
            name: "Test Customer".to_string(),
            contact_name: "".to_string(),
            email: "test.customer@example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
        };

        let mut app_state = MockCustomersModule::new();
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
            .uri("/api/customers/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = CustomerUserInputHelper {
            id: None,
            name: "Test Customer".to_string(),
            contact_name: "".to_string(),
            email: "test.customer@example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
        };

        let mut app_state = MockCustomersModule::new();
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
            .uri("/api/customers/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = CustomerUserInputHelper {
            id: None,
            name: "Test Customer".to_string(),
            contact_name: "".to_string(),
            email: "test.customer@example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
        };

        let app_state = MockCustomersModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/customers/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();
        let mut repo = MockCustomersRepository::new();

        repo.expect_delete_by_id()
            .times(1)
            .with(eq(customer_id))
            .returning(move |_| Ok(()));

        let mut app_state = MockCustomersModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_customers_repo()
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
            .uri(format!("/api/customers/delete?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut app_state = MockCustomersModule::new();
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
            .uri("/api/customers/delete?uuid=invalid_user_input")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_expired() {
        let customer_id = Uuid::new_v4();

        let mut app_state = MockCustomersModule::new();
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
            .uri(format!("/api/customers/delete?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();

        let mut app_state = MockCustomersModule::new();
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
            .uri(format!("/api/customers/delete?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();

        let app_state = MockCustomersModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!("/api/customers/delete?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse().unwrap();
        let created_by_id: Uuid = "97054cdb-781c-4f40-a489-b43373d75bf0".parse().unwrap();
        let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z".parse().unwrap();

        let customer_resolved = CustomerResolved {
            id: customer_id,
            name: "Test Customer".to_string(),
            contact_name: None,
            email: "test.customer@example.com".to_string(),
            phone_number: Some("+36301234567".to_string()),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
        };

        let mut repo = MockCustomersRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(customer_id))
            .returning({
                let customer_resolved = customer_resolved.clone();
                move |_| Ok(customer_resolved.clone())
            });

        let mut app_state = MockCustomersModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_customers_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let pdf_gen_payload_expected = vec![CustomerResolvedPrint::from_customer_revolved(
            customer_resolved,
            "Europe/Budapest".parse().unwrap(),
        )];

        let _m = PDF_GENERATOR_TEST_SYNC.lock();
        let pdf_gen = MockPdfGenerator::gen_pdf_temporary_context();
        pdf_gen
            .expect::<Vec<CustomerResolvedPrint>>()
            .times(1)
            .with(eq(PdfTemplates::CustomerView), eq(pdf_gen_payload_expected))
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
            .uri(format!("/api/customers/print?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let snapshot_path = Path::new("testing/pdf/snapshots/customers_test.pdf");
        let mut file = File::open(snapshot_path).unwrap();
        let mut snapshot = vec![];
        file.read_to_end(&mut snapshot).unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        let test_result_path = Path::new("testing/pdf/test_results/customers_test.pdf");
        let mut file = File::create(test_result_path).unwrap();
        file.write_all(&body_bytes).unwrap();

        assert_eq!(
            extract_pdf_text(&body_bytes).unwrap(),
            extract_pdf_text(&snapshot).unwrap()
        );
    }

    #[tokio::test]
    async fn test_print_unauthorized_expired() {
        let customer_id = Uuid::new_v4();

        let mut app_state = MockCustomersModule::new();
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
            .uri(format!("/api/customers/print?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();

        let mut app_state = MockCustomersModule::new();
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
            .uri(format!("/api/customers/print?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
        let customer_id = Uuid::new_v4();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/customers/print?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(
                MockCustomersModule::new(),
            ))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
}
