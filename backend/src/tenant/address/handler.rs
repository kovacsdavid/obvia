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

use std::{collections::HashMap, sync::Arc};

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::instrument;

use crate::common::dto::EmptyType;
use crate::tenant::address::service::AddressService;
use crate::{
    common::{
        dto::SuccessResponseBuilder,
        extractors::ClientContext,
        handler::{HandlerResult, map_handler_err},
        service::Service,
    },
    manager::auth::middleware::AuthenticatedUser,
    tenant::address::AddressModuleInterface,
};

#[instrument(
    name = "obvia::tenant::address::handler::select_list",
    skip(address_module)
)]
pub async fn select_list<M: AddressModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(address_module): State<Arc<M>>,
    _client_context: ClientContext,
    Query(payload): Query<HashMap<String, String>>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), address_module.clone());
    let list_type = payload
        .get("list")
        .cloned()
        .unwrap_or(String::from("missing_list"));
    let result = map_handler_err(
        service.get_select_list_items(&list_type).await,
        address_module.clone(),
    )
    .await?;

    Ok(map_handler_err(
        SuccessResponseBuilder::<EmptyType, _>::new()
            .status_code(StatusCode::OK)
            .data(result)
            .build(),
        address_module,
    )
    .await?
    .into_response())
}

#[cfg(test)]
mod tests {
    use axum::{Router, http::Request};
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        common::{
            config::tests::test_app_config_builder,
            handler::tests::{
                extract_json_response, generate_expired_jwt, generate_jwt_with_invalid_signature,
                generate_valid_jwt,
            },
            model::SelectOption,
        },
        tenant::address::{repository::MockAddressRepository, tests::MockAddressModule},
    };

    use super::*;

    #[tokio::test]
    async fn test_select_list_success() {
        let active_tenant_id = Uuid::now_v7();
        let list = "countries";

        let select_options = vec![
            SelectOption {
                value: "HU".to_string(),
                title: "Magyarország".to_string(),
            },
            SelectOption {
                value: "AT".to_string(),
                title: "Ausztria".to_string(),
            },
        ];

        let mut repo = MockAddressRepository::new();
        repo.expect_get_all_countries_select_list_items()
            .times(1)
            .returning({
                let select_options = select_options.clone();
                move || Ok(select_options.clone())
            });

        let mut app_state = MockAddressModule::new();
        let repo = Arc::new(repo);
        let test_config = test_app_config_builder().build().unwrap();
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());
        app_state
            .expect_address_repo()
            .with(eq(active_tenant_id))
            .times(1)
            .returning(move |_| Ok(repo.clone()));

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
            .uri(format!("/api/address/select_list?list={list}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(crate::tenant::address::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "meta": null,
            "data": [{
                "value": "HU",
                "title": "Magyarország"
            }, {
                "value": "AT",
                "title": "Ausztria"
            }]
        });

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_select_list_unauthorized_expired() {
        let list = "countries";

        let mut app_state = MockAddressModule::new();
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
            .uri(format!("/api/address/select_list?list={list}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(crate::tenant::address::routes::routes(Arc::new(app_state))),
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
    async fn test_select_list_unauthorized_invalid_signature() {
        let list = "countries";

        let mut app_state = MockAddressModule::new();
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
            .uri(format!("/api/address/select_list?list={list}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(crate::tenant::address::routes::routes(Arc::new(app_state))),
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
    async fn test_select_list_unauthorized_missing() {
        let list = "countries";
        let app_state = MockAddressModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/address/select_list?list={list}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(crate::tenant::address::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({});

        assert_eq!(response_body, expected_body);
    }

    #[tokio::test]
    async fn test_select_list_invalid_list() {
        let active_tenant_id = Uuid::now_v7();
        let list = "invalid_list";
        let mut app_state = MockAddressModule::new();
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
                    generate_valid_jwt(None, Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!("/api/address/select_list?list={list}"))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(crate::tenant::address::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let response_body = extract_json_response(response).await;
        let expected_body = json!({
            "error": {
                "message": "A lista nem létezik"
            }
        });

        assert_eq!(response_body, expected_body);
    }
}
