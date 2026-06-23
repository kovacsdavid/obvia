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
use crate::common::query_parser::ResourceQuery;
use crate::common::service::Service;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::inventory_reservations::InventoryReservationsModuleInterface;
use crate::tenant::inventory_reservations::dto::print::InventoryReservationResolvedPrint;
use crate::tenant::inventory_reservations::dto::user_input::{
    InventoryReservationUserInput, InventoryReservationUserInputHelper,
    InventoryReservationsRawQuery,
};
use crate::tenant::inventory_reservations::service::InventoryReservationService;
use crate::tenant::inventory_reservations::types::{
    InventoryReservationFilterBy, InventoryReservationOrderBy,
};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub async fn get<M: InventoryReservationsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_reservations_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_reservations_module.clone());
    let error_mapper = ErrorMapper::new(inventory_reservations_module);
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

pub async fn get_resolved<M: InventoryReservationsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_reservations_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_reservations_module.clone());
    let error_mapper = ErrorMapper::new(inventory_reservations_module);
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

pub async fn create<M: InventoryReservationsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_reservations_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<
        InventoryReservationUserInput,
        InventoryReservationUserInputHelper,
    >,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_reservations_module.clone());
    let error_mapper = ErrorMapper::new(inventory_reservations_module);
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

pub async fn update<M: InventoryReservationsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_reservations_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<
        InventoryReservationUserInput,
        InventoryReservationUserInputHelper,
    >,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_reservations_module.clone());
    let error_mapper = ErrorMapper::new(inventory_reservations_module);
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

pub async fn delete<M: InventoryReservationsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_reservations_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_reservations_module.clone());
    let error_mapper = ErrorMapper::new(inventory_reservations_module);
    error_mapper
        .or_handler_error(service.delete(payload.uuid).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A készletfoglalás törlése sikeresen megtörtént",
                ))
                .build(),
        )
        .await?
        .into_response())
}

pub async fn list<M: InventoryReservationsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_reservations_module): State<Arc<M>>,
    Query(payload): Query<InventoryReservationsRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_reservations_module.clone());
    let error_mapper = ErrorMapper::new(inventory_reservations_module);
    let resource_query = error_mapper
        .or_handler_error(ResourceQuery::<
            InventoryReservationOrderBy,
            InventoryReservationFilterBy,
        >::from_str(payload.q()))
        .await?;
    let (meta, data) = error_mapper
        .or_handler_error(
            service
                .get_paged(&resource_query, payload.inventory_id())
                .await,
        )
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

pub async fn select_list<M: InventoryReservationsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_reservations_module): State<Arc<M>>,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_reservations_module.clone());
    let error_mapper = ErrorMapper::new(inventory_reservations_module);
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

pub async fn print<M: InventoryReservationsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(inventory_reservations_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), inventory_reservations_module.clone());
    let error_mapper = ErrorMapper::new(inventory_reservations_module);
    let inventory_reservations_resolved_print =
        InventoryReservationResolvedPrint::from_inventory_reservation_resolved(
            error_mapper
                .or_handler_error(service.get_resolved(payload.uuid).await)
                .await?,
            error_mapper.or_handler_error(claims.tz()).await?,
        );
    let pdf = error_mapper
        .or_handler_error(
            service
                .print(&[inventory_reservations_resolved_print])
                .await,
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
    use crate::common::handler::tests::{generate_expired_jwt, generate_valid_jwt};
    use crate::tenant::inventory_reservations::model::InventoryReservationResolved;
    use crate::{
        common::config::tests::AppConfigBuilder,
        tenant::inventory_reservations::{
            self, model::InventoryReservation, repository::MockInventoryReservationsRepository,
            tests::MockInventoryReservationsModule,
        },
    };
    use axum::body::Body;
    use axum::{Router, http::Request};
    use chrono::{Duration, Utc};
    use mockall::predicate::eq;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_success() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Some(Uuid::new_v4());
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning(move |inventory_reservation_id| {
                Ok(InventoryReservation {
                    id: inventory_reservation_id,
                    inventory_id,
                    quantity: "10".parse().unwrap(),
                    reference_type: Some("worksheets".to_string()),
                    reference_id,
                    reserved_until: None,
                    status: "active".to_string(),
                    created_by_id,
                    created_at: utc_now,
                    updated_at: utc_now,
                })
            });

        let mut app_state = MockInventoryReservationsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_reservations_repo()
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
                "/api/inventory_reservations/get?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();

        let mut app_state = MockInventoryReservationsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt(Some(active_tenant_id))),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_reservations/get?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_unauthorized_missing() {
        let inventory_reservation_id = Uuid::new_v4();
        let app_state = MockInventoryReservationsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_reservations/get?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockInventoryReservationsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_reservations_repo()
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
                "/api/inventory_reservations/get?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_resolved_success() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Some(Uuid::new_v4());
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning(move |inventory_reservation_id| {
                Ok(InventoryReservationResolved {
                    id: inventory_reservation_id,
                    inventory_id,
                    quantity: "10".parse().unwrap(),
                    reference_type: Some("worksheets".to_string()),
                    reference_id,
                    reserved_until: None,
                    status: "active".to_string(),
                    created_by_id,
                    created_by: "Test User".to_string(),
                    created_at: utc_now,
                    updated_at: utc_now,
                })
            });

        let mut app_state = MockInventoryReservationsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_reservations_repo()
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
                "/api/inventory_reservations/get_resolved?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();

        let mut app_state = MockInventoryReservationsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt(Some(active_tenant_id))),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_reservations/get_resolved?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_missing() {
        let inventory_reservation_id = Uuid::new_v4();
        let app_state = MockInventoryReservationsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_reservations/get_resolved?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_get_resolved_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockInventoryReservationsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_reservations_repo()
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
                "/api/inventory_reservations/get_resolved?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_success() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Some(Uuid::new_v4());
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<InventoryReservationOrderBy, InventoryReservationFilterBy>>()
                .unwrap()),
                eq(inventory_id))
            .returning(move |_, _| {
                Ok((
                    PaginatorMeta {
                        page: 1,
                        limit: 25,
                        total: 100,
                    },
                    vec![InventoryReservationResolved {
                        id: inventory_reservation_id,
                        inventory_id,
                        quantity: "10".parse().unwrap(),
                        reference_type: Some("worksheets".to_string()),
                        reference_id,
                        reserved_until: None,
                        status: "active".to_string(),
                        created_by_id,
                        created_by: "Test User".to_string(),
                        created_at: utc_now,
                        updated_at: utc_now,
                    }],
                ))
            });

        let mut app_state = MockInventoryReservationsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_reservations_repo()
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
                "/api/inventory_reservations/list?inventory_id={inventory_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();

        let mut app_state = MockInventoryReservationsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt(Some(active_tenant_id))),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_reservations/list?inventory_id={inventory_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_unauthorized_missing() {
        let inventory_id = Uuid::new_v4();

        let app_state = MockInventoryReservationsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_reservations/list?inventory_id={inventory_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_list_not_found() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_paged()
            .times(1)
            .withf(|resource_query, _| {
                *resource_query == ""
                .parse::<ResourceQuery<InventoryReservationOrderBy, InventoryReservationFilterBy>>()
                .unwrap()
            })
            .returning(|_, _| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockInventoryReservationsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_reservations_repo()
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
                "/api/inventory_reservations/list?inventory_id={inventory_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: (utc_now + Duration::days(2)).date_naive().to_string(),
            status: "active".to_string(),
        };
        let user_input =
            InventoryReservationUserInput::try_from(user_input_helper.clone()).unwrap();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_insert()
            .times(1)
            .withf({
                let user_input_expected = user_input.clone();
                move |user_input, user_id_inner| {
                    user_input.inventory_id == user_input_expected.inventory_id
                        && user_input.quantity == user_input_expected.quantity
                        && user_input.reference_type == user_input_expected.reference_type
                        && user_input.reference_id == user_input_expected.reference_id
                        && user_input.reserved_until == user_input_expected.reserved_until
                        && user_input.status == user_input_expected.status
                        && user_id == *user_id_inner
                }
            })
            .returning(move |_, _| {
                Ok(InventoryReservation {
                    id: inventory_reservation_id,
                    inventory_id,
                    quantity: "10".parse().unwrap(),
                    reference_type: Some("worksheets".to_string()),
                    reference_id: Some(reference_id),
                    reserved_until: None,
                    status: "active".to_string(),
                    created_by_id,
                    created_at: utc_now,
                    updated_at: utc_now,
                })
            });

        let mut app_state = MockInventoryReservationsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_reservations_repo()
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
            .uri("/api/inventory_reservations/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "activee".to_string(),
        };

        let mut app_state = MockInventoryReservationsModule::new();
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
            .uri("/api/inventory_reservations/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
    #[tokio::test]
    async fn test_create_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockInventoryReservationsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt(Some(active_tenant_id))),
            )
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/inventory_reservations/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn test_create_unauthorized_missing() {
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
        };

        let app_state = MockInventoryReservationsModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/inventory_reservations/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_update_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: Some(inventory_reservation_id.to_string()),
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: (utc_now + Duration::days(2)).date_naive().to_string(),
            status: "active".to_string(),
        };
        let user_input =
            InventoryReservationUserInput::try_from(user_input_helper.clone()).unwrap();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input))
            .returning(move |_| {
                Ok(InventoryReservation {
                    id: inventory_reservation_id,
                    inventory_id,
                    quantity: "10".parse().unwrap(),
                    reference_type: Some("worksheets".to_string()),
                    reference_id: Some(reference_id),
                    reserved_until: None,
                    status: "active".to_string(),
                    created_by_id,
                    created_at: utc_now,
                    updated_at: utc_now,
                })
            });

        let mut app_state = MockInventoryReservationsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_reservations_repo()
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
            .uri("/api/inventory_reservations/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockInventoryReservationsModule::new();
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
            .uri("/api/inventory_reservations/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
    #[tokio::test]
    async fn test_update_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: Some(inventory_reservation_id.to_string()),
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
        };

        let mut app_state = MockInventoryReservationsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt(Some(active_tenant_id))),
            )
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/inventory_reservations/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn test_update_unauthorized_missing() {
        let inventory_reservation_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: Some(inventory_reservation_id.to_string()),
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
        };

        let app_state = MockInventoryReservationsModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/inventory_reservations/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_delete_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();
        let mut repo = MockInventoryReservationsRepository::new();

        repo.expect_delete_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning(move |_| Ok(()));

        let mut app_state = MockInventoryReservationsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_inventory_reservations_repo()
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
                "/api/inventory_reservations/delete?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut app_state = MockInventoryReservationsModule::new();
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
            .uri("/api/inventory_reservations/delete?uuid=invalid_user_input")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_expired() {
        let active_tenant_id = Uuid::new_v4();
        let inventory_reservation_id = Uuid::new_v4();

        let mut app_state = MockInventoryReservationsModule::new();
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        let request = Request::builder()
            .header(
                "Authorization",
                format!("Bearer {}", generate_expired_jwt(Some(active_tenant_id))),
            )
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!(
                "/api/inventory_reservations/delete?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn test_delete_unauthorized_missing() {
        let inventory_reservation_id = Uuid::new_v4();

        let app_state = MockInventoryReservationsModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("DELETE")
            .uri(format!(
                "/api/inventory_reservations/delete?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
