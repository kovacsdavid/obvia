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

use crate::common::error::FriendlyError;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::common::dto::{OkResponse, QueryParam, SimpleMessageResponse};
use crate::tenant::tasks::TasksModule;
use crate::tenant::tasks::dto::{CreateTask, CreateTaskHelper};
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, debug_handler};
use std::sync::Arc;
use tracing::Level;

#[debug_handler]
pub async fn get(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<TasksModule>>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<TasksModule>>,
    payload: Result<Json<CreateTaskHelper>, JsonRejection>,
) -> Response {
    match payload {
        Ok(Json(payload)) => match CreateTask::try_from(payload) {
            Ok(_) => (
                StatusCode::CREATED,
                Json(OkResponse::new(SimpleMessageResponse {
                    message: String::from("TEST!!!!"), //TODO: implement
                })),
            )
                .into_response(),
            Err(e) => e.into_response(),
        },
        Err(_) => FriendlyError::UserFacing(
            StatusCode::BAD_REQUEST,
            "ORGANIZATIONAL_UNITS/HANDLER/CREATE".to_string(),
            "Invalid JSON".to_string(),
        )
        .trace(Level::DEBUG)
        .into_response(),
    }
}

#[debug_handler]
pub async fn update(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<TasksModule>>,
    payload: Result<Json<CreateTask>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<TasksModule>>,
    payload: Result<Json<CreateTask>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(tasks_module): State<Arc<TasksModule>>,
    Query(payload): Query<QueryParam>,
) -> Response {
    todo!()
}
