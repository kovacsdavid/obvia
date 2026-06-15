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
use crate::tenant::comments::CommentsModule;
use crate::tenant::comments::dto::{CommentUserInput, CommentUserInputHelper};
use crate::tenant::comments::service::CommentService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn post<M: CommentsModule>(
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
