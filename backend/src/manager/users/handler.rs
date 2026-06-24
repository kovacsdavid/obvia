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

use super::UsersModuleInterface;
use super::service::UserService;
use crate::common::dto::{EmptyType, SimpleMessageResponse, SuccessResponseBuilder};
use crate::common::extractors::{ClientContext, UserInput};
use crate::common::handler::{ErrorMapper, ErrorMapperInterface, HandlerResult};
use crate::common::service::Service;
use crate::manager::auth::dto::login::{OtpUserInput, OtpUserInputHelper};
use crate::manager::auth::middleware::AuthenticatedUser;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

pub async fn get_claims<M: UsersModuleInterface>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(users_module): State<Arc<M>>,
) -> HandlerResult {
    let error_mapper = ErrorMapper::new(users_module);
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(claims)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn otp_enable<M: UsersModuleInterface>(
    State(users_module): State<Arc<M>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> HandlerResult {
    let service = Service::new(Some(&claims), users_module.clone());
    let error_mapper = ErrorMapper::new(users_module);
    let response = error_mapper
        .or_handler_error(service.otp_enable(&client_context).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(response)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn otp_verify<M: UsersModuleInterface>(
    State(users_module): State<Arc<M>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
    UserInput(user_input, _): UserInput<OtpUserInput, OtpUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), users_module.clone());
    let error_mapper = ErrorMapper::new(users_module);
    error_mapper
        .or_handler_error(service.otp_verify(&user_input, &client_context).await)
        .await?;

    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A kétlépcsős azonosítás aktiválása megtörtént!",
                ))
                .build(),
        )
        .await?
        .into_response())
}

pub async fn otp_disable<M: UsersModuleInterface>(
    State(users_module): State<Arc<M>>,
    client_context: ClientContext,
    AuthenticatedUser(claims): AuthenticatedUser,
    UserInput(user_input, _): UserInput<OtpUserInput, OtpUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), users_module.clone());
    let error_mapper = ErrorMapper::new(users_module);
    error_mapper
        .or_handler_error(service.otp_disable(&user_input, &client_context).await)
        .await?;

    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A kétlépcsős azonosítás kikapcsolása megtörtént!",
                ))
                .build(),
        )
        .await?
        .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::config::tests::AppConfigBuilder;
    use crate::common::handler::tests::generate_valid_jwt;
    use crate::manager::auth::repository::MockAuthRepository;
    use crate::manager::users::{
        self, model::User, repository::MockUsersRepository, tests::MockUsersModule,
    };
    use axum::{Router, http::Request};
    use chrono::Utc;
    use mockall::predicate::eq;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_otp_enable_success() {
        let sub = Uuid::new_v4();
        let active_tenant_id = Uuid::new_v4();

        let mut repo = MockUsersRepository::new();
        repo.expect_get_user_by_id()
            .times(1)
            .with(eq(sub))
            .returning(|sub| {
                Ok(User {
                    id: sub,
                    email: "testuser@example.com".to_string(),
                    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$13WsVCFEv98dFpY+OIm6vHiQvmQ5nLhlxNKktlDvlvs".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    phone: Some("+123456789".to_string()),
                    status: "active".to_string(),
                    last_login_at: Some(Utc::now()),
                    profile_picture_url: None,
                    locale: "hu-HU".to_string(),
                    invited_by: None,
                    email_verified_at: Some(Utc::now()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    is_mfa_enabled: false,
                    mfa_secret: None,
                    timezone: "Europe/Budapest".to_string(),
                })
            });
        repo.expect_update_user()
            .times(1)
            .withf(|user| {
                if let Some(mfa_secret) = &user.mfa_secret
                    && mfa_secret.len() == 32
                {
                    true
                } else {
                    false
                }
            })
            .returning(Ok);

        let mut app_state = MockUsersModule::new();
        let users_repo = Arc::new(repo);
        let auth_repo = Arc::new(MockAuthRepository::new());
        let test_config = AppConfigBuilder::default().build().unwrap();
        app_state
            .expect_auth_repo()
            .times(1)
            .returning(move || auth_repo.clone());
        app_state
            .expect_users_repo()
            .times(1)
            .returning(move || users_repo.clone());
        app_state
            .expect_config()
            .times(1)
            .return_const(test_config.clone());

        let request = Request::builder()
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    generate_valid_jwt(Some(sub), Some(active_tenant_id))
                ),
            )
            .header("Content-Type", "application/json")
            .method("GET")
            .uri("/api/users/otp/enable")
            .body("".to_string())
            .unwrap();

        let app = Router::new().nest(
            "/api",
            Router::new().merge(users::routes::routes(Arc::new(app_state))),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
