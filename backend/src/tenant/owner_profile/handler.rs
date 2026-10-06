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

use crate::common::dto::{EmptyType, SuccessResponseBuilder};
use crate::common::extractors::{ClientContext, UserInput};
use crate::common::handler::{HandlerResult, map_handler_err};
use crate::common::service::Service;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::owner_profile::OwnerProfileModuleInterface;
use crate::tenant::owner_profile::dto::user_input::{
    OwnerProfileUserInput, OwnerProfileUserInputHelper,
};
use crate::tenant::owner_profile::service::OwnerProfileService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::instrument;

#[instrument(
    name = "obvia::tenant::owner_profile::handler::get_full",
    skip(owner_profile_module)
)]
pub async fn get_full<M: OwnerProfileModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(owner_profile_module): State<Arc<M>>,
    _client_context: ClientContext,
) -> HandlerResult {
    let service = Service::new(Some(&claims), owner_profile_module.clone());
    let result = map_handler_err(service.get_full().await, owner_profile_module.clone()).await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        owner_profile_module,
    )
    .await?
    .into_response())
}

#[instrument(
    name = "obvia::tenant::owner_profile::handler::update",
    skip(owner_profile_module)
)]
pub async fn update<M: OwnerProfileModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(owner_profile_module): State<Arc<M>>,
    _client_context: ClientContext,
    UserInput(user_input, _): UserInput<OwnerProfileUserInput, OwnerProfileUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), owner_profile_module.clone());
    let result = map_handler_err(
        service.update(&user_input).await,
        owner_profile_module.clone(),
    )
    .await?;
    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        owner_profile_module,
    )
    .await?
    .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::error::RepositoryError;
    use crate::common::handler::tests::{
        extract_json_response, generate_expired_jwt, generate_jwt_with_invalid_signature,
        generate_valid_jwt,
    };
    use crate::tenant::address::model::test_address_resolved_builder;
    use crate::tenant::owner_profile::model::tests::{
        test_owner_profile_builder, test_owner_profile_full_builder,
    };
    use crate::{
        common::config::tests::test_app_config_builder,
        tenant::owner_profile::{
            self, repository::MockOwnerProfileRepository, tests::MockOwnerProfileModule,
        },
    };
    use axum::body::Body;
    use axum::{Router, http::Request};
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_full_success() {
        let active_tenant_id = Uuid::now_v7();
        let owner_profile_id = Uuid::now_v7();
        let billing_address_id = Uuid::now_v7();
        let mailing_address_id = Uuid::now_v7();

        let owner_profile_full = test_owner_profile_full_builder()
            .id(owner_profile_id)
            .billing_address(Some(
                test_address_resolved_builder()
                    .id(billing_address_id)
                    .build()
                    .unwrap(),
            ))
            .mailing_address(Some(
                test_address_resolved_builder()
                    .id(mailing_address_id)
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let mut owner_profile_repo = MockOwnerProfileRepository::new();
        owner_profile_repo.expect_get_full().times(1).returning({
            let owner_profile_full = owner_profile_full.clone();
            move || Ok(owner_profile_full.clone())
        });

        let mut app_state = MockOwnerProfileModule::new();
        let owner_profile_repo = Arc::new(owner_profile_repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_owner_profile_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(owner_profile_repo.clone()));
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
                "/api/owner_profile/get_full?uuid={owner_profile_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(owner_profile::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": owner_profile_full
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_get_full_unauthorized_expired() {
        let owner_profile_id = Uuid::now_v7();

        let mut app_state = MockOwnerProfileModule::new();
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
                "/api/owner_profile/get_full?uuid={owner_profile_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(owner_profile::routes::routes(Arc::new(app_state))),
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
        let owner_profile_id = Uuid::now_v7();

        let mut app_state = MockOwnerProfileModule::new();
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
                "/api/owner_profile/get_full?uuid={owner_profile_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(owner_profile::routes::routes(Arc::new(app_state))),
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
        let owner_profile_id = Uuid::now_v7();
        let app_state = MockOwnerProfileModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/owner_profile/get_full?uuid={owner_profile_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(owner_profile::routes::routes(Arc::new(app_state))),
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
        let owner_profile_id = Uuid::now_v7();

        let mut repo = MockOwnerProfileRepository::new();
        repo.expect_get_full()
            .times(1)
            .with()
            .returning(|| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockOwnerProfileModule::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_owner_profile_repo()
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
                "/api/owner_profile/get_full?uuid={owner_profile_id}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(owner_profile::routes::routes(Arc::new(app_state))),
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
    async fn test_update_success() {
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let owner_profile_id = Uuid::now_v7();

        let user_input_helper = OwnerProfileUserInputHelper {
            id: Some(owner_profile_id.to_string()),
            name: "Test Owner".to_string(),
            contact_name: "".to_string(),
            email: "test.owner@example.com".to_string(),
            website: "https://example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            owner_profile_type: "natural".to_string(),
            billing_address: None,
            mailing_address: None,
        };
        let user_input = OwnerProfileUserInput::try_from(user_input_helper.clone()).unwrap();

        let owner_profile = test_owner_profile_builder()
            .id(owner_profile_id)
            .build()
            .unwrap();

        let mut repo = MockOwnerProfileRepository::new();
        repo.expect_update()
            .times(1)
            .with(eq(user_input), eq(user_id))
            .returning({
                let owner_profile = owner_profile.clone();
                move |_, _| Ok(owner_profile.clone())
            });

        let mut app_state = MockOwnerProfileModule::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_owner_profile_repo()
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
            .uri("/api/owner_profile/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(owner_profile::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": owner_profile
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_update_invalid_user_input() {
        let active_tenant_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let user_input_helper = OwnerProfileUserInputHelper {
            id: Some("".to_string()),
            name: "Test Owner".to_string(),
            contact_name: "".to_string(),
            email: "test.owner@example.com".to_string(),
            website: "https://example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            owner_profile_type: "natural".to_string(),
            billing_address: None,
            mailing_address: None,
        };

        let mut app_state = MockOwnerProfileModule::new();
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
            .uri("/api/owner_profile/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(owner_profile::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = OwnerProfileUserInputHelper {
            id: Some(Uuid::now_v7().to_string()),
            name: "Test Owner".to_string(),
            contact_name: "".to_string(),
            email: "test.owner_profile@example.com".to_string(),
            website: "https://example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            owner_profile_type: "natural".to_string(),
            billing_address: None,
            mailing_address: None,
        };

        let mut app_state = MockOwnerProfileModule::new();
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
            .uri("/api/owner_profile/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(owner_profile::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = OwnerProfileUserInputHelper {
            id: Some(Uuid::now_v7().to_string()),
            name: "Test Owner".to_string(),
            contact_name: "".to_string(),
            email: "test.owner_profile@example.com".to_string(),
            website: "https://example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            owner_profile_type: "natural".to_string(),
            billing_address: None,
            mailing_address: None,
        };

        let mut app_state = MockOwnerProfileModule::new();
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
            .uri("/api/owner_profile/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(owner_profile::routes::routes(Arc::new(app_state))),
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
        let user_input_helper = OwnerProfileUserInputHelper {
            id: Some(Uuid::now_v7().to_string()),
            name: "Test Owner".to_string(),
            contact_name: "".to_string(),
            email: "test.owner@example.com".to_string(),
            website: "https://example.com".to_string(),
            phone_number: "+36301234567".to_string(),
            owner_profile_type: "natural".to_string(),
            billing_address: None,
            mailing_address: None,
        };

        let app_state = MockOwnerProfileModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("PUT")
            .uri("/api/owner_profile/update")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(owner_profile::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }
}
