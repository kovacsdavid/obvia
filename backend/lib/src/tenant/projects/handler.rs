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

use crate::common::dto::{
    OkResponse, OrderingParams, PaginatorParams, QueryParam, SimpleMessageResponse,
};
use crate::common::error::FriendlyError;
use crate::common::extractors::UserInput;
use crate::common::types::order::Order;
use crate::common::types::value_object::ValueObject;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::tenants::dto::FilteringParams;
use crate::tenant::projects::ProjectsModule;
use crate::tenant::projects::dto::{CreateProject, CreateProjectHelper};
use crate::tenant::projects::service::{ProjectsService, ProjectsServiceError};
use crate::tenant::projects::types::project::ProjectOrderBy;
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
    State(projects_module): State<Arc<ProjectsModule>>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(projects_module): State<Arc<ProjectsModule>>,
    UserInput(user_input, _): UserInput<CreateProject, CreateProjectHelper>,
) -> Result<Response, Response> {
    ProjectsService::create(&claims, &user_input, projects_module)
        .await
        .map_err(|e| {
            match e {
                ProjectsServiceError::Unauthorized => FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    "Hozzáférés megtagadva!",
                ),
                _ => FriendlyError::internal(file!(), e.to_string()),
            }
            .into_response()
        })?;

    Ok((
        StatusCode::CREATED,
        Json(OkResponse::new(SimpleMessageResponse {
            message: String::from("A projekt létrehozása sikeresen megtörtént!"),
        })),
    )
        .into_response())
}

#[debug_handler]
pub async fn update(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(projects_module): State<Arc<ProjectsModule>>,
    payload: Result<Json<CreateProject>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(projects_module): State<Arc<ProjectsModule>>,
    payload: Result<Json<CreateProject>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(projects_module): State<Arc<ProjectsModule>>,
    Query(payload): Query<QueryParam>,
) -> Result<Response, Response> {
    Ok((
        StatusCode::OK,
        Json(OkResponse::new(
            ProjectsService::get_paged_list(
                &PaginatorParams::try_from(&payload).unwrap_or(PaginatorParams::default()),
                &OrderingParams::try_from(&payload).unwrap_or(OrderingParams {
                    order_by: ValueObject::new(ProjectOrderBy("name".to_string())).map_err(
                        |e| FriendlyError::internal(file!(), e.to_string()).into_response(),
                    )?,
                    order: ValueObject::new(Order("asc".to_string())).map_err(|e| {
                        FriendlyError::internal(file!(), e.to_string()).into_response()
                    })?,
                }),
                &FilteringParams::from(&payload),
                &claims,
                projects_module.projects_repo.clone(),
            )
            .await
            .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
        )),
    )
        .into_response())
}
