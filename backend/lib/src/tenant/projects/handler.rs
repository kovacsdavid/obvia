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

use crate::manager::auth::middleware::AuthenticatedUser;
use crate::manager::common::dto::QueryParam;
use crate::tenant::projects::ProjectsModule;
use crate::tenant::projects::dto::CreateProject;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::{Json, debug_handler};
use std::sync::Arc;

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
    payload: Result<Json<CreateProject>, JsonRejection>,
) -> Response {
    todo!()
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
) -> Response {
    todo!()
}
