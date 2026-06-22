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
    let error_mapper = ErrorMapper::new(customers_module);
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

pub async fn get<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
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

pub async fn create<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<CustomerUserInput, CustomerUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
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

pub async fn update<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<CustomerUserInput, CustomerUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
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

pub async fn delete<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
    error_mapper
        .or_handler_error(service.delete(payload.uuid).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A vevő törlése sikeresen megtörtént",
                ))
                .build(),
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
    let error_mapper = ErrorMapper::new(customers_module);
    let resource_query = error_mapper
        .or_handler_error(ResourceQuery::<CustomerOrderBy, CustomerFilterBy>::from_str(payload.q()))
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

pub async fn print<M: CustomersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
    let customer_resolved_print = CustomerResolvedPrint::from_customer_revolved(
        error_mapper
            .or_handler_error(service.get_resolved(payload.uuid).await)
            .await?,
        error_mapper.or_handler_error(claims.tz()).await?,
    );
    let pdf = error_mapper
        .or_handler_error(service.print(&[customer_resolved_print]).await)
        .await?;
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        r#"inline; filename="invoice.pdf""#.parse().unwrap(),
    );
    Ok((StatusCode::OK, headers, pdf).into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::dto::PaginatorMeta;
    use crate::common::error::RepositoryError;
    use crate::tenant::customers::model::CustomerResolved;
    use crate::{
        common::config::tests::AppConfigBuilder,
        manager::auth::dto::claims::Claims,
        tenant::customers::{
            self, model::Customer, repository::MockCustomersRepository, tests::MockCustomersModule,
        },
    };
    use axum::body::Body;
    use axum::{Router, http::Request};
    use chrono::Utc;
    use mockall::predicate::eq;
    use std::ops::{Add, Sub};
    use std::time::Duration;
    use tower::ServiceExt;
    use uuid::Uuid;

    fn generate_valid_token(sub: Option<Uuid>, active_tenant_id: Option<Uuid>) -> String {
        let config = AppConfigBuilder::default().build().unwrap();
        let sub = match sub {
            Some(v) => v,
            None => Uuid::new_v4(),
        };
        let exp = Utc::now().add(Duration::from_secs(100)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        Claims::new(
            sub,
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            format!("{}-api", config.auth().jwt_audience()),
            Uuid::new_v4(),
            "hu-HU".to_string(),
            "Europe/Budapest".parse().unwrap(),
            None,
            active_tenant_id,
        )
        .to_token(config.auth().jwt_secret().as_bytes())
        .unwrap()
    }

    fn generate_expired_token(active_tenant_id: Option<Uuid>) -> String {
        let config = AppConfigBuilder::default().build().unwrap();
        let exp = Utc::now().sub(Duration::from_secs(100)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            format!("{}-api", config.auth().jwt_audience()),
            Uuid::new_v4(),
            "hu-HU".to_string(),
            "Europe/Budapest".parse().unwrap(),
            None,
            active_tenant_id,
        )
        .to_token(config.auth().jwt_secret().as_bytes())
        .unwrap()
    }

    #[tokio::test]
    async fn test_get_success() {
        let active_tenant_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockCustomersRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(customer_id))
            .returning(move |customer_id| {
                Ok(Customer {
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
                })
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
                    generate_valid_token(None, Some(active_tenant_id))
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
    }

    #[tokio::test]
    async fn test_get_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();
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
                format!("Bearer {}", generate_expired_token(Some(active_tenant_id))),
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
    }
    #[tokio::test]
    async fn test_get_unauthorized_not_found() {
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
                    generate_valid_token(None, Some(active_tenant_id))
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
    }

    #[tokio::test]
    async fn test_get_resolved_success() {
        let active_tenant_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockCustomersRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(customer_id))
            .returning(move |customer_id| {
                Ok(CustomerResolved {
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
                })
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
                    generate_valid_token(None, Some(active_tenant_id))
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
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();
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
                format!("Bearer {}", generate_expired_token(Some(active_tenant_id))),
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
                    generate_valid_token(None, Some(active_tenant_id))
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
    }

    #[tokio::test]
    async fn test_list_success() {
        let active_tenant_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockCustomersRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<CustomerOrderBy, CustomerFilterBy>>()
                .unwrap()))
            .returning(move |_| {
                Ok((
                    PaginatorMeta {
                        page: 1,
                        limit: 25,
                        total: 100,
                    },
                    vec![CustomerResolved {
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
                    }],
                ))
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
                    generate_valid_token(None, Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/customers/list?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();
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
                format!("Bearer {}", generate_expired_token(Some(active_tenant_id))),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/customers/list?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_unauthorized_missing() {
        let customer_id = Uuid::new_v4();
        let app_state = MockCustomersModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/customers/list?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_list_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();

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
                    generate_valid_token(None, Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/customers/list?uuid={customer_id}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
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
            .returning(move |_, _| {
                Ok(Customer {
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
                })
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
                    generate_valid_token(Some(user_id), Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri(format!("/api/customers/create?uuid={customer_id}"))
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();

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
                    generate_valid_token(Some(user_id), Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri(format!("/api/customers/create?uuid={customer_id}"))
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(customers::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
