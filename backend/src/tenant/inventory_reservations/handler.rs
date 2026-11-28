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

use crate::common::AppStateInterface;
use crate::common::dto::{EmptyType, SimpleMessageResponse, SuccessResponseBuilder, UuidParam};
use crate::common::extractors::{ClientContext, UserInput};
use crate::common::handler::{HandlerResult, map_handler_err};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::BaseService;
use crate::manager::auth::middleware::AuthenticatedUser;
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
use tracing::instrument;

#[utoipa::path(
    get, path = "/api/inventory_reservations/get", tag = "Inventory Reservations",
    summary = "Get an inventory reservation",
    params(("uuid" = uuid::Uuid, Query, description = "Reservation ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Inventory reservation", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::tenant::inventory_reservations::model::InventoryReservation>),
        (status = 401, description = "Authentication required"),
        (status = 404, description = "Reservation not found")
    )
)]
#[instrument(
    name = "obvia::tenant::inventory_reservations::handler::get",
    skip(app_state)
)]
pub async fn get<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let result = map_handler_err(service.get(payload.uuid).await, app_state.clone()).await?;
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
    get, path = "/api/inventory_reservations/get_resolved", tag = "Inventory Reservations",
    summary = "Get a reservation with resolved details",
    params(("uuid" = uuid::Uuid, Query, description = "Reservation ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Resolved reservation", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::tenant::inventory_reservations::model::InventoryReservationResolved>),
        (status = 401, description = "Authentication required"),
        (status = 404, description = "Reservation not found")
    )
)]
#[instrument(
    name = "obvia::tenant::inventory_reservations::handler::get_resolved",
    skip(app_state)
)]
pub async fn get_resolved<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let result =
        map_handler_err(service.get_resolved(payload.uuid).await, app_state.clone()).await?;
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
    post, path = "/api/inventory_reservations/create", tag = "Inventory Reservations",
    summary = "Create an inventory reservation",
    request_body = InventoryReservationUserInputHelper,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Reservation created", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::tenant::inventory_reservations::model::InventoryReservation>),
        (status = 401, description = "Authentication required"),
        (status = 422, description = "Invalid input")
    )
)]
#[instrument(
    name = "obvia::tenant::inventory_reservations::handler::create",
    skip(app_state)
)]
pub async fn create<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    UserInput(user_input, _): UserInput<
        InventoryReservationUserInput,
        InventoryReservationUserInputHelper,
    >,
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
    put, path = "/api/inventory_reservations/update", tag = "Inventory Reservations",
    summary = "Update an inventory reservation",
    request_body = InventoryReservationUserInputHelper,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Reservation updated", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::tenant::inventory_reservations::model::InventoryReservation>),
        (status = 401, description = "Authentication required"),
        (status = 404, description = "Reservation not found"),
        (status = 422, description = "Invalid input")
    )
)]
#[instrument(
    name = "obvia::tenant::inventory_reservations::handler::update",
    skip(app_state)
)]
pub async fn update<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    UserInput(user_input, _): UserInput<
        InventoryReservationUserInput,
        InventoryReservationUserInputHelper,
    >,
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
    delete, path = "/api/inventory_reservations/delete", tag = "Inventory Reservations",
    summary = "Delete an inventory reservation",
    params(("uuid" = uuid::Uuid, Query, description = "Reservation ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Reservation deleted", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, crate::common::dto::SimpleMessageResponse>),
        (status = 401, description = "Authentication required"),
        (status = 404, description = "Reservation not found")
    )
)]
#[instrument(
    name = "obvia::tenant::inventory_reservations::handler::delete",
    skip(app_state)
)]
pub async fn delete<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    map_handler_err(service.delete(payload.uuid).await, app_state.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(SimpleMessageResponse::new(
                "A készletfoglalás törlése sikeresen megtörtént",
            ))
            .build(),
        app_state,
    )
    .await?
    .into_response())
}

#[utoipa::path(
    get, path = "/api/inventory_reservations/list", tag = "Inventory Reservations",
    summary = "List inventory reservations",
    params(("q" = Option<String>, Query, description = "Optional filtering, ordering, and paging expression"), ("inventory_id" = Option<String>, Query, description = "Filter by inventory ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Page of inventory reservations", body = crate::common::dto::SuccessResponse<crate::common::dto::PaginatorMeta, Vec<crate::tenant::inventory_reservations::model::InventoryReservationResolved>>),
        (status = 401, description = "Authentication required")
    )
)]
#[instrument(
    name = "obvia::tenant::inventory_reservations::handler::list",
    skip(app_state)
)]
pub async fn list<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    Query(payload): Query<InventoryReservationsRawQuery>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<InventoryReservationOrderBy, InventoryReservationFilterBy>::from_str(
            payload.q(),
        ),
        app_state.clone(),
    )
    .await?;
    let (meta, data) = map_handler_err(
        service
            .get_paged(&resource_query, payload.inventory_id())
            .await,
        app_state.clone(),
    )
    .await?;
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
    get, path = "/api/inventory_reservations/select_list", tag = "Inventory Reservations",
    summary = "Get reservation select-list items",
    params(("list" = Option<String>, Query, description = "Select-list type")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Select-list items", body = crate::common::dto::SuccessResponse<crate::common::dto::EmptyType, Vec<crate::common::model::SelectOption>>),
        (status = 401, description = "Authentication required")
    )
)]
#[instrument(
    name = "obvia::tenant::inventory_reservations::handler::select_list",
    skip(app_state)
)]
pub async fn select_list<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let service = BaseService::new(Some(&claims), app_state.clone());
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));

    let result = map_handler_err(
        service.get_select_list_items(&list_type).await,
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
    get, path = "/api/inventory_reservations/print", tag = "Inventory Reservations",
    summary = "Print a reservation as PDF",
    params(("uuid" = uuid::Uuid, Query, description = "Reservation ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "PDF document", body = crate::common::api_docs::PdfDocument, content_type = "application/pdf", headers(("Content-Disposition" = String))),
        (status = 401, description = "Authentication required"),
        (status = 404, description = "Reservation not found")
    )
)]
#[instrument(
    name = "obvia::tenant::inventory_reservations::handler::print",
    skip(app_state)
)]
pub async fn print<A: AppStateInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(app_state): State<Arc<A>>,
    _client_context: ClientContext,
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
    use super::*;
    use crate::common::TEST_TZ;
    use crate::common::dto::PaginatorMeta;
    use crate::common::error::RepositoryError;
    use crate::common::handler::tests::{
        extract_json_response, generate_expired_jwt, generate_jwt_with_invalid_signature,
        generate_valid_jwt,
    };
    use crate::common::pdf::tests::{PDF_GENERATOR_TEST_SYNC, extract_pdf_text};
    use crate::common::pdf::{MockPdfGenerator, PdfGenerator, PdfTemplates};
    use crate::common::tests::MockAppState;
    use crate::tenant::inventory::dto::print::InventoryResolvedPrint;
    use crate::tenant::inventory::model::tests::test_inventory_resolved_builder;
    use crate::tenant::inventory::repository::MockInventoryRepository;
    use crate::tenant::inventory_reservations::dto::print::InventoryReservationResolvedPrint;
    use crate::tenant::inventory_reservations::model::InventoryReservationResolved;
    use crate::tenant::inventory_reservations::model::tests::test_inventory_reservation_resolved_builder;
    use crate::tenant::products::dto::print::ProductsResolvedPrint;
    use crate::tenant::products::model::tests::test_product_resolved_builder;
    use crate::tenant::products::repository::MockProductsRepository;
    use crate::tenant::warehouses::dto::print::WarehouseResolvedPrint;
    use crate::tenant::warehouses::model::tests::test_warehouse_resolved_builder;
    use crate::tenant::warehouses::repository::MockWarehousesRepository;
    use crate::{
        common::config::tests::test_app_config_builder,
        tenant::inventory_reservations::{
            self, model::InventoryReservation, repository::MockInventoryReservationsRepository,
        },
    };
    use axum::body::Body;
    use axum::{Router, http::Request};
    use chrono::{Duration, Utc};
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
        let active_tenant_id = Uuid::now_v7();
        let inventory_reservation_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();
        let reference_id = Some(Uuid::now_v7());
        let created_by_id = Uuid::now_v7();
        let utc_now = Utc::now();
        let inventory_reservation = InventoryReservation {
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
        };

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning({
                let inventory_reservation = inventory_reservation.clone();
                move |_| Ok(inventory_reservation.clone())
            });

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": inventory_reservation,
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_unauthorized_expired() {
        let inventory_reservation_id = Uuid::now_v7();

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
        let inventory_reservation_id = Uuid::now_v7();

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
        let inventory_reservation_id = Uuid::now_v7();
        let app_state = MockAppState::new();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let active_tenant_id = Uuid::now_v7();
        let inventory_reservation_id = Uuid::now_v7();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Nem található"
            },
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_resolved_success() {
        let active_tenant_id = Uuid::now_v7();
        let inventory_reservation_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();
        let reference_id = Some(Uuid::now_v7());
        let created_by_id = Uuid::now_v7();
        let utc_now = Utc::now();
        let inventory_reservation_resolved = InventoryReservationResolved {
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
        };

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning({
                let inventory_reservation_resolved = inventory_reservation_resolved.clone();
                move |_| Ok(inventory_reservation_resolved.clone())
            });

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": inventory_reservation_resolved,
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_resolved_unauthorized_expired() {
        let inventory_reservation_id = Uuid::now_v7();

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
        let inventory_reservation_id = Uuid::now_v7();

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
        let inventory_reservation_id = Uuid::now_v7();
        let app_state = MockAppState::new();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_get_resolved_not_found() {
        let active_tenant_id = Uuid::now_v7();
        let inventory_reservation_id = Uuid::now_v7();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning(|_| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Nem található"
            },
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_success() {
        let active_tenant_id = Uuid::now_v7();
        let inventory_reservation_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();
        let reference_id = Some(Uuid::now_v7());
        let created_by_id = Uuid::now_v7();
        let utc_now = Utc::now();
        let paginator_meta = PaginatorMeta {
            page: 1,
            limit: 25,
            total: 100,
        };
        let inventory_reservation_resolved = InventoryReservationResolved {
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
        };

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(eq(""
                .parse::<ResourceQuery<InventoryReservationOrderBy, InventoryReservationFilterBy>>()
                .unwrap()),
                eq(inventory_id))
            .returning({
                let inventory_reservation_resolved = inventory_reservation_resolved.clone();
                move |_, _| {
                Ok((
                    paginator_meta,
                    vec![inventory_reservation_resolved.clone()],
                ))
            }});

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": paginator_meta,
            "data": [inventory_reservation_resolved],
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let inventory_id = Uuid::now_v7();

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
        let inventory_id = Uuid::now_v7();

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
        let inventory_id = Uuid::now_v7();

        let app_state = MockAppState::new();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_list_not_found() {
        let active_tenant_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_get_paged()
            .times(1)
            .withf(|resource_query, _| {
                *resource_query == ""
                .parse::<ResourceQuery<InventoryReservationOrderBy, InventoryReservationFilterBy>>()
                .unwrap()
            })
            .returning(|_, _| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Nem található"
            },
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_success() {
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();
        let reference_id = Uuid::now_v7();
        let inventory_reservation_id = Uuid::now_v7();
        let created_by_id = Uuid::now_v7();
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

        let inventory_reservation = InventoryReservation {
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
        };

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
            .returning({
                let inventory_reservation = inventory_reservation.clone();
                move |_, _| Ok(inventory_reservation.clone())
            });

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": inventory_reservation
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_invalid_user_input() {
        let active_tenant_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();
        let reference_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "activee".to_string(),
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
            .uri("/api/inventory_reservations/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let user_input_error =
            InventoryReservationUserInput::try_from(user_input_helper).unwrap_err();

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Kérjük ellenőrizze a hibás mezőket!",
                "fields":user_input_error
            }
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_create_unauthorized_expired() {
        let inventory_id = Uuid::now_v7();
        let reference_id = Uuid::now_v7();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
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
            .uri("/api/inventory_reservations/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::now_v7();
        let reference_id = Uuid::now_v7();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
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
            .uri("/api/inventory_reservations/create")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
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
        let inventory_id = Uuid::now_v7();
        let reference_id = Uuid::now_v7();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
        };

        let app_state = MockAppState::new();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_update_success() {
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let inventory_reservation_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();
        let reference_id = Uuid::now_v7();
        let created_by_id = Uuid::now_v7();
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
        let inventory_reservation = InventoryReservation {
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
        };

        let mut repo = MockInventoryReservationsRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input))
            .returning({
                let inventory_reservation = inventory_reservation.clone();
                move |_| Ok(inventory_reservation.clone())
            });

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": inventory_reservation,
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_invalid_user_input() {
        let active_tenant_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();
        let reference_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
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
            .uri("/api/inventory_reservations/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "Hiba történt az adatok feldolgozása során: Az azonosító megadása kötelező!"
            },
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_unauthorized_expired() {
        let inventory_reservation_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();
        let reference_id = Uuid::now_v7();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: Some(inventory_reservation_id.to_string()),
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
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
            .uri("/api/inventory_reservations/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
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
        let inventory_reservation_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();
        let reference_id = Uuid::now_v7();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: Some(inventory_reservation_id.to_string()),
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
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
            .uri("/api/inventory_reservations/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
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
        let inventory_reservation_id = Uuid::now_v7();
        let inventory_id = Uuid::now_v7();
        let reference_id = Uuid::now_v7();

        let user_input_helper = InventoryReservationUserInputHelper {
            id: Some(inventory_reservation_id.to_string()),
            inventory_id: inventory_id.to_string(),
            quantity: "10".to_string(),
            reference_type: "worksheets".to_string(),
            reference_id: reference_id.to_string(),
            reserved_until: "".to_string(),
            status: "active".to_string(),
        };

        let app_state = MockAppState::new();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
    #[tokio::test]
    async fn test_delete_success() {
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let inventory_reservation_id = Uuid::now_v7();
        let mut repo = MockInventoryReservationsRepository::new();

        repo.expect_delete_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning(move |_| Ok(()));

        let mut app_state = MockAppState::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": {
                "message": "A készletfoglalás törlése sikeresen megtörtént"
            },
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
            .uri("/api/inventory_reservations/delete?uuid=invalid_user_input")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_delete_unauthorized_expired() {
        let inventory_reservation_id = Uuid::now_v7();

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
        let inventory_reservation_id = Uuid::now_v7();

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
        let inventory_reservation_id = Uuid::now_v7();

        let app_state = MockAppState::new();
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

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_print_success() {
        let active_tenant_id = Uuid::now_v7();
        let inventory_reservation_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse().unwrap();
        let inventory_id = "ac55ca9c-2cd1-4cdf-8b44-ed4df798c750".parse().unwrap();
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0".parse().unwrap();
        let reference_id = "fd48ade1-a817-431b-8ada-6faea8c9f9dd".parse().unwrap();
        let product_id = "4f882e4a-5e44-4b46-bbfd-d813ccffdae3".parse().unwrap();
        let warehouse_id = "c8b67f0a-2613-4dca-ba9c-3e411ac3fb23".parse().unwrap();

        let inventory_reservation_resolved = test_inventory_reservation_resolved_builder()
            .id(inventory_reservation_id)
            .reference_id(Some(reference_id))
            .created_by_id(created_by_id)
            .build()
            .unwrap();

        let mut inventory_reservations_repo = MockInventoryReservationsRepository::new();
        inventory_reservations_repo
            .expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_reservation_id))
            .returning({
                let inventory_reservation_resolved = inventory_reservation_resolved.clone();
                move |_| Ok(inventory_reservation_resolved.clone())
            });

        let inventory_resolved = test_inventory_resolved_builder()
            .id(inventory_id)
            .created_by_id(created_by_id)
            .product_id(product_id)
            .warehouse_id(warehouse_id)
            .build()
            .unwrap();

        let mut inventory_repo = MockInventoryRepository::new();
        inventory_repo
            .expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_reservation_resolved.inventory_id))
            .returning({
                let inventory_resolved = inventory_resolved.clone();
                move |_| Ok(inventory_resolved.clone())
            });

        let product_resolved = test_product_resolved_builder()
            .id(product_id)
            .created_by_id(created_by_id)
            .build()
            .unwrap();
        let mut products_repo = MockProductsRepository::new();
        products_repo
            .expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_resolved.product_id))
            .returning({
                let product_resolved = product_resolved.clone();
                move |_| Ok(product_resolved.clone())
            });

        let warehouse_resolved = test_warehouse_resolved_builder()
            .id(warehouse_id)
            .created_by_id(created_by_id)
            .build()
            .unwrap();
        let mut warehouses_repo = MockWarehousesRepository::new();
        warehouses_repo
            .expect_get_resolved_by_id()
            .times(1)
            .with(eq(inventory_resolved.warehouse_id))
            .returning({
                let warehouse_resolved = warehouse_resolved.clone();
                move |_| Ok(warehouse_resolved.clone())
            });

        let mut app_state = MockAppState::new();
        let inventory_reservations_repo = Arc::new(inventory_reservations_repo);
        let inventory_repo = Arc::new(inventory_repo);
        let products_repo = Arc::new(products_repo);
        let warehouses_repo = Arc::new(warehouses_repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_inventory_reservations_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(inventory_reservations_repo.clone()));
        app_state
            .expect_inventory_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(inventory_repo.clone()));
        app_state
            .expect_products_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(products_repo.clone()));
        app_state
            .expect_warehouses_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(warehouses_repo.clone()));
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let pdf_gen_payload_expected = InventoryReservationResolvedPrint::new(
            inventory_reservation_resolved,
            InventoryResolvedPrint::new(
                inventory_resolved,
                ProductsResolvedPrint::new(product_resolved, *TEST_TZ),
                WarehouseResolvedPrint::new(warehouse_resolved, *TEST_TZ),
                *TEST_TZ,
            ),
            *TEST_TZ,
        );

        let _m = PDF_GENERATOR_TEST_SYNC.lock();
        let pdf_gen = MockPdfGenerator::gen_pdf_temporary_context();
        pdf_gen
            .expect::<InventoryReservationResolvedPrint>()
            .times(1)
            .with(
                eq(PdfTemplates::InventoryReservationView),
                eq(pdf_gen_payload_expected.clone()),
            )
            .returning(move |template, payload| {
                assert_eq!(pdf_gen_payload_expected, payload);
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
                "/api/inventory_reservations/print?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let snapshot_path = Path::new("testing/pdf/snapshots/inventory_reservations_test.pdf");
        let mut file = File::open(snapshot_path).unwrap();
        let mut snapshot = vec![];
        file.read_to_end(&mut snapshot).unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        let test_result_path =
            Path::new("testing/pdf/test_results/inventory_reservations_test.pdf");
        let mut file = File::create(test_result_path).unwrap();
        file.write_all(&body_bytes).unwrap();

        assert_eq!(
            extract_pdf_text(&body_bytes).unwrap(),
            extract_pdf_text(&snapshot).unwrap()
        );
    }

    #[tokio::test]
    async fn test_print_unauthorized_expired() {
        let inventory_reservation_id = Uuid::now_v7();

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
                "/api/inventory_reservations/print?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
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
        let inventory_reservation_id = Uuid::now_v7();

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
                "/api/inventory_reservations/print?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(app_state))),
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
        let inventory_reservation_id = Uuid::now_v7();

        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/inventory_reservations/print?uuid={inventory_reservation_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(inventory_reservations::routes::routes(Arc::new(
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
