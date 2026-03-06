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
    EmptyType, HandlerResult, SimpleMessageResponse, SuccessResponseBuilder, UuidParam,
};
use crate::common::error::FriendlyError;
use crate::common::error::IntoFriendlyError;
use crate::common::extractors::UserInput;
use crate::common::query_parser::{CommonRawQuery, GetQuery};
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::projects::ProjectsModule;
use crate::tenant::projects::dto::{ProjectUserInput, ProjectUserInputHelper};
use crate::tenant::projects::service::ProjectsService;
use crate::tenant::projects::types::project::{ProjectFilterBy, ProjectOrderBy};
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::str::FromStr;
use std::sync::Arc;

#[debug_handler]
pub async fn get_resolved(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(projects_module): State<Arc<dyn ProjectsModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let result = match ProjectsService::get_resolved_by_id(
        &claims,
        &payload,
        projects_module.projects_repo(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => return Err(e.into_friendly_error(projects_module).await.into_response()),
    };
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e.into_friendly_error(projects_module).await.into_response()),
    }
}

#[debug_handler]
pub async fn get(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(projects_module): State<Arc<dyn ProjectsModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let result =
        match ProjectsService::get(&claims, &payload, projects_module.projects_repo()).await {
            Ok(r) => r,
            Err(e) => return Err(e.into_friendly_error(projects_module).await.into_response()),
        };
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e.into_friendly_error(projects_module).await.into_response()),
    }
}

#[debug_handler]
pub async fn update(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(projects_module): State<Arc<dyn ProjectsModule>>,
    UserInput(user_input, _): UserInput<ProjectUserInput, ProjectUserInputHelper>,
) -> HandlerResult {
    let result = match ProjectsService::update(
        &claims,
        &user_input,
        projects_module.projects_repo(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => return Err(e.into_friendly_error(projects_module).await.into_response()),
    };
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e.into_friendly_error(projects_module).await.into_response()),
    }
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(projects_module): State<Arc<dyn ProjectsModule>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    match ProjectsService::delete(&claims, &payload, projects_module.projects_repo()).await {
        Ok(_) => (),
        Err(e) => return Err(e.into_friendly_error(projects_module).await.into_response()),
    };

    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::OK)
        .data(SimpleMessageResponse::new(
            "A projekt törlése sikeresen megtörtént",
        ))
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e.into_friendly_error(projects_module).await.into_response()),
    }
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(projects_module): State<Arc<dyn ProjectsModule>>,
    UserInput(user_input, _): UserInput<ProjectUserInput, ProjectUserInputHelper>,
) -> HandlerResult {
    let result = match ProjectsService::create(&claims, &user_input, projects_module.clone()).await
    {
        Ok(r) => r,
        Err(e) => return Err(e.into_friendly_error(projects_module).await.into_response()),
    };
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::CREATED)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e.into_friendly_error(projects_module).await.into_response()),
    }
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(projects_module): State<Arc<dyn ProjectsModule>>,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let (meta, data) = match ProjectsService::get_paged_list(
        &GetQuery::<ProjectOrderBy, ProjectFilterBy>::from_str(payload.as_str())
            .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
        &claims,
        projects_module.projects_repo(),
    )
    .await
    {
        Ok((m, d)) => (m, d),
        Err(e) => return Err(e.into_friendly_error(projects_module).await.into_response()),
    };

    match SuccessResponseBuilder::new()
        .status_code(StatusCode::OK)
        .meta(meta)
        .data(data)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e.into_friendly_error(projects_module).await.into_response()),
    }
}
