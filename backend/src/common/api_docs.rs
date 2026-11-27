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

use crate::tenant::customers::handler;
use axum::Router;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(ToSchema)]
#[schema(value_type = String, format = Binary)]
pub(crate) struct PdfDocument(#[allow(dead_code)] Vec<u8>);

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::get,
        handler::get_resolved,
        handler::get_full,
        handler::list,
        handler::create,
        handler::update,
        handler::delete,
        handler::print
    ),
    modifiers(&BearerSecurity),
    tags((name = "Customers", description = "Customer operations for the active tenant in the bearer token"))
)]
pub(crate) struct ApiDoc;

struct BearerSecurity;

impl Modify for BearerSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

pub(crate) fn routes() -> Router {
    SwaggerUi::new("/api/swagger-ui")
        .url("/api/api-docs/openapi.json", ApiDoc::openapi())
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::MockAppState;
    use crate::tenant::customers;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::Value;
    use std::sync::Arc;
    use tower::ServiceExt;

    fn assert_local_references_resolve(value: &Value, document: &Value) {
        match value {
            Value::Object(object) => {
                if let Some(reference) = object.get("$ref").and_then(Value::as_str)
                    && let Some(pointer) = reference.strip_prefix('#')
                {
                    assert!(
                        document.pointer(pointer).is_some(),
                        "unresolved reference: {reference}"
                    );
                }
                for child in object.values() {
                    assert_local_references_resolve(child, document);
                }
            }
            Value::Array(array) => {
                for child in array {
                    assert_local_references_resolve(child, document);
                }
            }
            _ => {}
        }
    }

    #[test]
    fn customer_openapi_contract() {
        let document = serde_json::to_value(ApiDoc::openapi()).unwrap();
        assert_local_references_resolve(&document, &document);
        let paths = document["paths"].as_object().unwrap();
        let operations = [
            ("get", "get"),
            ("get_resolved", "get"),
            ("get_full", "get"),
            ("list", "get"),
            ("create", "post"),
            ("update", "put"),
            ("delete", "delete"),
            ("print", "get"),
        ];
        assert_eq!(paths.len(), operations.len());
        for (endpoint, method) in operations {
            let operation = &paths[&format!("/api/customers/{endpoint}")][method];
            assert!(operation.is_object(), "missing {method} {endpoint}");
            assert_eq!(
                operation["security"][0]["bearer_auth"],
                serde_json::json!([])
            );
            assert!(operation["responses"]["400"].is_object());
            assert!(operation["responses"]["401"]["content"]["text/plain"].is_object());
            assert!(operation["responses"]["401"]["content"]["application/json"].is_object());
        }
        assert_eq!(
            document["components"]["securitySchemes"]["bearer_auth"]["scheme"],
            "bearer"
        );
        assert_eq!(
            document["components"]["securitySchemes"]["bearer_auth"]["bearerFormat"],
            "JWT"
        );

        let get = &paths["/api/customers/get"]["get"];
        assert_eq!(get["parameters"][0]["name"], "uuid");
        assert_eq!(get["parameters"][0]["in"], "query");
        assert_eq!(get["parameters"][0]["required"], true);
        let list = &paths["/api/customers/list"]["get"];
        assert_eq!(list["parameters"][0]["name"], "q");
        assert_eq!(list["parameters"][0]["required"], false);

        let create = &paths["/api/customers/create"]["post"];
        assert!(create["requestBody"]["content"]["application/json"]["schema"].is_object());
        assert!(create["responses"]["201"]["content"]["application/json"].is_object());
        assert!(create["responses"]["409"].is_object());
        assert!(create["responses"]["422"].is_object());
        let print = &paths["/api/customers/print"]["get"]["responses"]["200"];
        assert!(print["content"]["application/pdf"].is_object());
        assert!(print["headers"]["Content-Disposition"].is_object());

        let schemas = &document["components"]["schemas"];
        assert_eq!(
            get["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
            "#/components/schemas/SuccessResponse_EmptyType_Customer"
        );
        assert!(
            schemas["SuccessResponse_EmptyType_Customer"]["properties"]["data"]["properties"]["id"]
                .is_object()
        );
        assert!(schemas["SuccessResponse_EmptyType_Customer"]["properties"]["meta"].is_object());
        assert!(
            schemas["SuccessResponse_EmptyType_Customer"]["properties"]
                .get("status_code")
                .is_none()
        );
        assert!(schemas["SuccessResponse_PaginatorMeta_Vec_CustomerFull"]["properties"]["meta"]["properties"]["total"].is_object());
        let error_response_ref =
            create["responses"]["422"]["content"]["application/json"]["schema"]["$ref"]
                .as_str()
                .unwrap();
        let error_response_name = error_response_ref.rsplit('/').next().unwrap();
        let error_response = &schemas[error_response_name];
        assert!(error_response["properties"].get("status_code").is_none());
        let error_body = &error_response["properties"]["error"];
        assert!(error_body["properties"]["message"].is_object());
        assert!(error_body["properties"]["fields"].is_object());
        assert!(schemas["CustomerUserInputHelper"]["properties"]["billing_address"].is_object());
        assert!(schemas["AddressUserInputHelper"]["properties"]["type"].is_object());
        assert!(schemas["AddressResolved"]["properties"]["type"].is_object());
        assert!(schemas["CustomerUserInputError"]["properties"]["billing_address"].is_object());
        assert_eq!(schemas["PdfDocument"]["type"], "string");
        assert_eq!(schemas["PdfDocument"]["format"], "binary");
    }

    #[tokio::test]
    async fn docs_routes_are_public_and_customer_routes_remain_protected() {
        let app = Router::new().merge(routes()).nest(
            "/api",
            customers::routes::routes(Arc::new(MockAppState::new())),
        );

        for path in [
            "/api/swagger-ui/",
            "/api/swagger-ui/swagger-ui-bundle.js",
            "/api/swagger-ui/swagger-initializer.js",
            "/api/api-docs/openapi.json",
        ] {
            let response = app
                .clone()
                .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{path}");
            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            assert!(!body.is_empty(), "{path}");
            if path.ends_with("swagger-initializer.js") {
                assert!(String::from_utf8_lossy(&body).contains("/api/api-docs/openapi.json"));
            }
            if path.ends_with("openapi.json") {
                let document: Value = serde_json::from_slice(&body).unwrap();
                assert!(document["paths"]["/api/customers/get"].is_object());
            }
        }

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/swagger-ui")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(response.headers()["location"], "/api/swagger-ui/");
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/customers/get?uuid=00000000-0000-0000-0000-000000000000")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
