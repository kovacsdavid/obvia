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
        crate::manager::auth::handler::login,
        crate::manager::auth::handler::refresh,
        crate::manager::auth::handler::logout,
        crate::manager::auth::handler::register,
        crate::manager::auth::handler::verify_email,
        crate::manager::auth::handler::resend_email_verification,
        crate::manager::auth::handler::forgotten_password,
        crate::manager::auth::handler::new_password,
        crate::manager::users::handler::get_claims,
        crate::manager::users::handler::otp_enable,
        crate::manager::users::handler::otp_verify,
        crate::manager::users::handler::otp_disable,
        crate::manager::tenants::handler::create,
        crate::manager::tenants::handler::get,
        crate::manager::tenants::handler::get_resolved,
        crate::manager::tenants::handler::list,
        crate::manager::tenants::handler::activate,
        crate::manager::tenants::handler::delete,
        crate::tenant::activity_feed::handler::list,
        crate::tenant::address::handler::select_list,
        crate::tenant::comments::handler::post,
        crate::tenant::comments::handler::delete,
        crate::tenant::customers::handler::get,
        crate::tenant::customers::handler::get_resolved,
        crate::tenant::customers::handler::get_full,
        crate::tenant::customers::handler::list,
        crate::tenant::customers::handler::create,
        crate::tenant::customers::handler::update,
        crate::tenant::customers::handler::delete,
        crate::tenant::customers::handler::print,
        crate::tenant::handover_and_acceptance::handler::get_full,
        crate::tenant::handover_and_acceptance::handler::list,
        crate::tenant::handover_and_acceptance::handler::select_list,
        crate::tenant::handover_and_acceptance::handler::create,
        crate::tenant::handover_and_acceptance::handler::update,
        crate::tenant::handover_and_acceptance::handler::delete,
        crate::tenant::handover_and_acceptance::handler::print,
        crate::tenant::inventory::handler::get,
        crate::tenant::inventory::handler::get_resolved,
        crate::tenant::inventory::handler::list,
        crate::tenant::inventory::handler::select_list,
        crate::tenant::inventory::handler::create,
        crate::tenant::inventory::handler::update,
        crate::tenant::inventory::handler::delete,
        crate::tenant::inventory::handler::print,
        crate::tenant::inventory_movements::handler::get,
        crate::tenant::inventory_movements::handler::get_resolved,
        crate::tenant::inventory_movements::handler::list,
        crate::tenant::inventory_movements::handler::select_list,
        crate::tenant::inventory_movements::handler::create,
        crate::tenant::inventory_movements::handler::update,
        crate::tenant::inventory_movements::handler::delete,
        crate::tenant::inventory_movements::handler::print,
        crate::tenant::inventory_reservations::handler::get,
        crate::tenant::inventory_reservations::handler::get_resolved,
        crate::tenant::inventory_reservations::handler::list,
        crate::tenant::inventory_reservations::handler::select_list,
        crate::tenant::inventory_reservations::handler::create,
        crate::tenant::inventory_reservations::handler::update,
        crate::tenant::inventory_reservations::handler::delete,
        crate::tenant::inventory_reservations::handler::print,
        crate::tenant::owner_profile::handler::get_full,
        crate::tenant::owner_profile::handler::update,
        crate::tenant::products::handler::get,
        crate::tenant::products::handler::get_resolved,
        crate::tenant::products::handler::list,
        crate::tenant::products::handler::select_list,
        crate::tenant::products::handler::create,
        crate::tenant::products::handler::update,
        crate::tenant::products::handler::delete,
        crate::tenant::products::handler::print,
        crate::tenant::services::handler::get,
        crate::tenant::services::handler::get_resolved,
        crate::tenant::services::handler::list,
        crate::tenant::services::handler::select_list,
        crate::tenant::services::handler::create,
        crate::tenant::services::handler::update,
        crate::tenant::services::handler::delete,
        crate::tenant::services::handler::print,
        crate::tenant::tasks::handler::get,
        crate::tenant::tasks::handler::get_resolved,
        crate::tenant::tasks::handler::list,
        crate::tenant::tasks::handler::select_list,
        crate::tenant::tasks::handler::create,
        crate::tenant::tasks::handler::update,
        crate::tenant::tasks::handler::delete,
        crate::tenant::tasks::handler::print,
        crate::tenant::taxes::handler::get,
        crate::tenant::taxes::handler::get_resolved,
        crate::tenant::taxes::handler::list,
        crate::tenant::taxes::handler::select_list,
        crate::tenant::taxes::handler::create,
        crate::tenant::taxes::handler::update,
        crate::tenant::taxes::handler::delete,
        crate::tenant::taxes::handler::print,
        crate::tenant::warehouses::handler::get,
        crate::tenant::warehouses::handler::get_resolved,
        crate::tenant::warehouses::handler::list,
        crate::tenant::warehouses::handler::create,
        crate::tenant::warehouses::handler::update,
        crate::tenant::warehouses::handler::delete,
        crate::tenant::warehouses::handler::print,
        crate::tenant::worksheets::handler::get,
        crate::tenant::worksheets::handler::get_resolved,
        crate::tenant::worksheets::handler::list,
        crate::tenant::worksheets::handler::select_list,
        crate::tenant::worksheets::handler::create,
        crate::tenant::worksheets::handler::update,
        crate::tenant::worksheets::handler::delete,
        crate::tenant::worksheets::handler::print
    ),
    modifiers(&BearerSecurity),
    tags(
        (name = "Authentication", description = "Public registration, login, and token operations"),
        (name = "Users", description = "Authenticated user and two-factor authentication operations"),
        (name = "Tenants", description = "Tenant management operations"),
        (name = "Activity Feed", description = "Tenant activity feed operations"),
        (name = "Addresses", description = "Address selection operations"),
        (name = "Comments", description = "Comment operations"),
        (name = "Customers", description = "Customer operations for the active tenant in the bearer token"),
        (name = "Handover and Acceptance", description = "Handover and acceptance records"),
        (name = "Inventory", description = "Inventory operations"),
        (name = "Inventory Movements", description = "Inventory movement operations"),
        (name = "Inventory Reservations", description = "Inventory reservation operations"),
        (name = "Owner Profile", description = "Tenant owner profile operations"),
        (name = "Products", description = "Product operations"),
        (name = "Services", description = "Service operations"),
        (name = "Tasks", description = "Task operations"),
        (name = "Taxes", description = "Tax operations"),
        (name = "Warehouses", description = "Warehouse operations"),
        (name = "Worksheets", description = "Worksheet operations")
    )
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
    fn backend_openapi_contract() {
        let document = serde_json::to_value(ApiDoc::openapi()).unwrap();
        assert_local_references_resolve(&document, &document);
        let paths = document["paths"].as_object().unwrap();
        assert_eq!(paths.len(), 110);
        for (prefix, expected_count) in [
            ("/api/auth/", 8),
            ("/api/users/", 4),
            ("/api/tenants/", 6),
            ("/api/activity_feed/", 1),
            ("/api/address/", 1),
            ("/api/comments/", 2),
            ("/api/customers/", 8),
            ("/api/handover_and_acceptance/", 7),
            ("/api/inventory/", 8),
            ("/api/inventory_movements/", 8),
            ("/api/inventory_reservations/", 8),
            ("/api/owner_profile/", 2),
            ("/api/products/", 8),
            ("/api/services/", 8),
            ("/api/tasks/", 8),
            ("/api/taxes/", 8),
            ("/api/warehouses/", 7),
            ("/api/worksheets/", 8),
        ] {
            assert_eq!(
                paths.keys().filter(|path| path.starts_with(prefix)).count(),
                expected_count,
                "unexpected operation count for {prefix}"
            );
        }
        for (path, path_item) in paths {
            for (method, operation) in path_item.as_object().unwrap() {
                assert!(
                    operation["summary"].is_string(),
                    "missing summary: {method} {path}"
                );
                assert!(
                    operation["responses"]
                        .as_object()
                        .is_some_and(|responses| !responses.is_empty()),
                    "missing responses: {method} {path}"
                );
                if path != "/api/auth/t/logout"
                    && (operation["responses"].get("200").is_some()
                        || operation["responses"].get("201").is_some())
                {
                    let success = operation["responses"]
                        .get("200")
                        .or_else(|| operation["responses"].get("201"))
                        .unwrap();
                    assert!(
                        success["content"].as_object().is_some(),
                        "missing response schema: {method} {path}"
                    );
                }
                if path.starts_with("/api/auth/") {
                    assert!(operation.get("security").is_none(), "public path {path}");
                } else {
                    assert_eq!(
                        operation["security"][0]["bearer_auth"],
                        serde_json::json!([]),
                        "{path}"
                    );
                }
                if method == "put"
                    || (method == "post"
                        && path != "/api/auth/t/refresh"
                        && path != "/api/auth/t/logout")
                    || path == "/api/auth/resend_email_verification"
                {
                    assert!(
                        operation["requestBody"]["content"]["application/json"]["schema"]
                            .is_object(),
                        "missing request schema: {method} {path}"
                    );
                }
            }
        }
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
        assert!(schemas.get("JsonObjectSchema").is_none());
        for schema in [
            "LoginResponse",
            "Claims",
            "PublicTenantManaged",
            "PublicTenant",
            "NewTokenResponse",
            "ActivityFeedResolved",
            "SelectOption",
            "Comment",
            "HandoverAndAcceptance",
            "HandoverAndAcceptanceFull",
            "Inventory",
            "InventoryResolved",
            "InventoryMovement",
            "InventoryMovementResolved",
            "InventoryReservation",
            "InventoryReservationResolved",
            "OwnerProfileFull",
            "Product",
            "ProductResolved",
            "Service",
            "ServiceResolved",
            "Task",
            "TaskResolved",
            "Tax",
            "TaxResolved",
            "Warehouse",
            "WarehouseResolved",
            "Worksheet",
            "WorksheetResolved",
        ] {
            assert!(
                schemas.get(schema).is_some(),
                "missing response schema {schema}"
            );
        }
        let tax_select_list = &paths["/api/taxes/select_list"]["get"]["responses"]["200"]["content"]
            ["application/json"]["schema"];
        assert!(
            tax_select_list["$ref"]
                .as_str()
                .unwrap()
                .contains("SelectOption")
        );
        assert!(schemas["SelectOption"]["properties"]["value"].is_object());
        assert!(schemas["SelectOption"]["properties"]["title"].is_object());
        let tax_page = &paths["/api/taxes/list"]["get"]["responses"]["200"]["content"]["application/json"]
            ["schema"]["$ref"];
        assert!(
            tax_page
                .as_str()
                .unwrap()
                .contains("SuccessResponse_PaginatorMeta_Vec_TaxResolved")
        );
        let login = &paths["/api/auth/login"]["post"]["responses"]["200"]["content"]["application/json"]
            ["schema"]["$ref"];
        assert!(login.as_str().unwrap().contains("LoginResponse"));
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
