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

use crate::common::dto::SuccessResponseBuilder;
use crate::common::handler::{HandlerResult, map_handler_err};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::Service;
use crate::common::types::Empty;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::activity_feed::ActivityFeedModuleInterface;
use crate::tenant::activity_feed::dto::ActivityFeedRawQuery;
use crate::tenant::activity_feed::service::ActivityFeedService;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::str::FromStr;
use std::sync::Arc;

pub async fn list<M: ActivityFeedModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(activity_feed_module): State<Arc<M>>,
    Query(payload): Query<ActivityFeedRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), activity_feed_module.clone());
    let resource_query = map_handler_err(
        ResourceQuery::<Empty, Empty>::from_str(payload.q()),
        activity_feed_module.clone(),
    )
    .await?;
    let (meta, data) = map_handler_err(
        service
            .get_all_paged(
                &resource_query,
                payload.resource_id(),
                &map_handler_err(payload.resource_type(), activity_feed_module.clone()).await?,
            )
            .await,
        activity_feed_module.clone(),
    )
    .await?;

    Ok(map_handler_err(
        SuccessResponseBuilder::new()
            .status_code(StatusCode::OK)
            .meta(meta)
            .data(data)
            .build(),
        activity_feed_module,
    )
    .await?
    .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::dto::PaginatorMeta;
    use crate::common::error::RepositoryError;
    use crate::common::handler::tests::{
        generate_expired_jwt, generate_jwt_with_invalid_signature, generate_valid_jwt,
    };
    use crate::common::value_object::ValueObjectRequired;
    use crate::tenant::activity_feed::model::ActivityFeedResolved;
    use crate::tenant::activity_feed::types::ResourceType;
    use crate::{
        common::config::tests::AppConfigBuilder,
        tenant::activity_feed::{
            self, repository::MockActivityFeedRepository, tests::MockActivityFeedModule,
        },
    };
    use axum::{Router, http::Request};
    use chrono::Utc;
    use mockall::predicate::eq;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_list_success() {
        let active_tenant_id = Uuid::new_v4();
        let activity_feed_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();
        let resource_type = "worksheets"
            .parse::<ValueObjectRequired<ResourceType>>()
            .unwrap();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let mut repo = MockActivityFeedRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(
                eq("".parse::<ResourceQuery<Empty, Empty>>().unwrap()),
                eq(resource_id),
                eq(resource_type.clone()),
            )
            .returning({
                let resource_type_clone = resource_type.clone();
                move |_, _, _| {
                    Ok((
                        PaginatorMeta {
                            page: 1,
                            limit: 25,
                            total: 100,
                        },
                        vec![ActivityFeedResolved {
                            id: activity_feed_id,
                            resource_id,
                            resource_type: resource_type_clone.to_string(),
                            activity_type: "comment".to_string(),
                            content: "Test content".to_string(),
                            created_by_id,
                            created_by: "Test User".to_string(),
                            created_at: utc_now,
                            updated_at: utc_now,
                            deleted_at: None,
                        }],
                    ))
                }
            });

        let mut app_state = MockActivityFeedModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_activity_feed_repo()
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
                "/api/activity_feed/list?resource_id={resource_id}&resource_type={resource_type}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(activity_feed::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_unauthorized_expired() {
        let resource_id = Uuid::new_v4();
        let resource_type = "worksheets"
            .parse::<ValueObjectRequired<ResourceType>>()
            .unwrap();

        let mut app_state = MockActivityFeedModule::new();
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
            .uri(format!(
                "/api/activity_feed/list?resource_id={resource_id}&resource_type={resource_type}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(activity_feed::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_unauthorized_invalid_signature() {
        let resource_id = Uuid::new_v4();
        let resource_type = "worksheets"
            .parse::<ValueObjectRequired<ResourceType>>()
            .unwrap();

        let mut app_state = MockActivityFeedModule::new();
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
            .uri(format!(
                "/api/activity_feed/list?resource_id={resource_id}&resource_type={resource_type}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(activity_feed::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_unauthorized_missing() {
        let resource_id = Uuid::new_v4();
        let resource_type = "worksheets"
            .parse::<ValueObjectRequired<ResourceType>>()
            .unwrap();
        let app_state = MockActivityFeedModule::new();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("GET")
            .uri(format!(
                "/api/activity_feed/list?resource_id={resource_id}&resource_type={resource_type}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(activity_feed::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_list_not_found() {
        let resource_id = Uuid::new_v4();
        let resource_type = "worksheets"
            .parse::<ValueObjectRequired<ResourceType>>()
            .unwrap();
        let active_tenant_id = Uuid::new_v4();

        let mut repo = MockActivityFeedRepository::new();
        repo.expect_get_paged()
            .times(1)
            .with(
                eq("".parse::<ResourceQuery<Empty, Empty>>().unwrap()),
                eq(resource_id),
                eq(resource_type.clone()),
            )
            .returning(|_, _, _| Err(RepositoryError::Database(sqlx::Error::RowNotFound)));

        let mut app_state = MockActivityFeedModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_activity_feed_repo()
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
                "/api/activity_feed/list?resource_id={resource_id}&resource_type={resource_type}"
            ))
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(activity_feed::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
