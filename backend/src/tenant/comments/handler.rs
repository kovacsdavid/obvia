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

use crate::common::dto::{EmptyType, HandlerResult, SuccessResponseBuilder};
use crate::common::error::IntoFriendlyError;
use crate::common::extractors::UserInput;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::comments::CommentsModule;
use crate::tenant::comments::dto::{CommentUserInput, CommentUserInputHelper};
use crate::tenant::comments::service::CommentsService;
use axum::debug_handler;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

#[debug_handler]
pub async fn post(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(comments_module): State<Arc<dyn CommentsModule>>,
    UserInput(user_input, _): UserInput<CommentUserInput, CommentUserInputHelper>,
) -> HandlerResult {
    let result =
        match CommentsService::post(&claims, &user_input, comments_module.comments_repo()).await {
            Ok(r) => r,
            Err(e) => {
                return Err(e.into_friendly_error(comments_module).await.into_response());
            }
        };
    match SuccessResponseBuilder::<EmptyType, _>::new()
        .status_code(StatusCode::CREATED)
        .data(result)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e.into_friendly_error(comments_module).await.into_response()),
    }
}
