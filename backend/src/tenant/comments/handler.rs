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
use crate::common::handler::{HandlerResult, init_handler};
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::comments::CommentsModule;
use crate::tenant::comments::dto::{CommentUserInput, CommentUserInputHelper};
use crate::tenant::comments::service::CommentService;
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
    let (service, error_mapper) = init_handler(Some(&claims), comments_module);
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
