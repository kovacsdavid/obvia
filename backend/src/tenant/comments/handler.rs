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
use crate::common::extractors::UserInput;
use crate::common::handler::{ErrorMapper, ErrorMapperInterface, HandlerResult};
use crate::common::service::Service;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::comments::CommentsModuleInterface;
use crate::tenant::comments::dto::{CommentUserInput, CommentUserInputHelper};
use crate::tenant::comments::service::CommentService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn post<M: CommentsModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(comments_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<CommentUserInput, CommentUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), comments_module.clone());
    let error_mapper = ErrorMapper::new(comments_module);
    let result = error_mapper
        .or_handler_error(service.post(&user_input).await)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::handler::tests::{
        generate_expired_jwt, generate_jwt_with_invalid_signature, generate_valid_jwt,
    };
    use crate::{
        common::config::tests::AppConfigBuilder,
        tenant::comments::{
            self, model::Comment, repository::MockCommentsRepository, tests::MockCommentsModule,
        },
    };
    use axum::body::Body;
    use axum::{Router, http::Request};
    use chrono::Utc;
    use mockall::predicate::eq;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_post_success() {
        let active_tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let comment_id = Uuid::new_v4();
        let commentable_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let utc_now = Utc::now();

        let user_input_helper = CommentUserInputHelper {
            id: None,
            commentable_type: "worksheets".to_string(),
            commentable_id: commentable_id.to_string(),
            comment: "Test comment".to_string(),
        };
        let user_input = CommentUserInput::try_from(user_input_helper.clone()).unwrap();

        let mut repo = MockCommentsRepository::new();
        repo.expect_post()
            .times(1)
            .withf({
                let user_input_expected = user_input.clone();
                move |user_input, user_id_inner| {
                    user_input.commentable_type == user_input_expected.commentable_type
                        && user_input.commentable_id == user_input_expected.commentable_id
                        && user_input.comment == user_input_expected.comment
                        && user_id == *user_id_inner
                }
            })
            .returning(move |_, _| {
                Ok(Comment {
                    id: comment_id,
                    commentable_type: "worksheets".to_string(),
                    commentable_id,
                    comment: "Test comment".to_string(),
                    created_by_id,
                    created_at: utc_now,
                    updated_at: utc_now,
                    deleted_at: None,
                })
            });

        let mut app_state = MockCommentsModule::new();
        let repo = Arc::new(repo);
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_comments_repo()
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
            .uri("/api/comments/post")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(comments::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_post_invalid_user_input() {
        let active_tenant_id = Uuid::new_v4();
        let commentable_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let user_input_helper = CommentUserInputHelper {
            id: None,
            commentable_type: "worksheets".to_string(),
            commentable_id: commentable_id.to_string(),
            comment: "".to_string(),
        };

        let mut app_state = MockCommentsModule::new();
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
            .uri("/api/comments/post")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(comments::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_post_unauthorized_expired() {
        let commentable_id = Uuid::new_v4();

        let user_input_helper = CommentUserInputHelper {
            id: None,
            commentable_type: "worksheets".to_string(),
            commentable_id: commentable_id.to_string(),
            comment: "Test comment".to_string(),
        };

        let mut app_state = MockCommentsModule::new();
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
            .uri("/api/comments/post")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(comments::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_post_unauthorized_invalid_signature() {
        let commentable_id = Uuid::new_v4();

        let user_input_helper = CommentUserInputHelper {
            id: None,
            commentable_type: "worksheets".to_string(),
            commentable_id: commentable_id.to_string(),
            comment: "Test comment".to_string(),
        };

        let mut app_state = MockCommentsModule::new();
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
            .uri("/api/comments/post")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(comments::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_post_unauthorized_missing() {
        let commentable_id = Uuid::new_v4();
        let user_input_helper = CommentUserInputHelper {
            id: None,
            commentable_type: "worksheets".to_string(),
            commentable_id: commentable_id.to_string(),
            comment: "Test comment".to_string(),
        };

        let app_state = MockCommentsModule::new();
        let payload = serde_json::to_string(&user_input_helper).unwrap();
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .method("POST")
            .uri("/api/comments/post")
            .body(Body::from(payload))
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(comments::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
