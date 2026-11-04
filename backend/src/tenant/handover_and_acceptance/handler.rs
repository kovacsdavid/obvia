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

use axum::extract::{Query, State};
use tracing::instrument;

use crate::{
    common::{
        dto::UuidParam,
        extractors::{ClientContext, UserInput},
        handler::HandlerResult,
        query_parser::CommonRawQuery,
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
    todo!();
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
    todo!();
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
    todo!();
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
    todo!();
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
    todo!();
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
    todo!();
}

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        common::{
            config::tests::test_app_config_builder,
            error::RepositoryError,
            handler::tests::{
                extract_json_response, generate_expired_jwt, generate_jwt_with_invalid_signature,
                generate_valid_jwt,
            },
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
}
