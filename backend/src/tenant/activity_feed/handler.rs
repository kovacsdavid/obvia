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
use crate::common::error::FriendlyError;
use crate::common::handler::{ErrorMapper, ErrorMapperInterface, HandlerResult};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::Service;
use crate::common::types::Empty;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::activity_feed::ActivityFeedModule;
use crate::tenant::activity_feed::dto::ActivityFeedRawQuery;
use crate::tenant::activity_feed::service::ActivityFeedService;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::str::FromStr;
use std::sync::Arc;

pub async fn list<M: ActivityFeedModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(activity_feed_module): State<Arc<M>>,
    Query(payload): Query<ActivityFeedRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), activity_feed_module.clone());
    let error_mapper = ErrorMapper::new(activity_feed_module);
    let resource_query = error_mapper
        .or_handler_error(ResourceQuery::<Empty, Empty>::from_str(payload.q()))
        .await?;
    let (meta, data) = error_mapper
        .or_handler_error(
            service
                .get_all_paged(
                    &resource_query,
                    payload.resource_id(),
                    &payload.resource_type().map_err(|e| {
                        FriendlyError::internal(file!(), e.to_string()).into_response()
                    })?,
                )
                .await,
        )
        .await?;

    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::new()
                .status_code(StatusCode::OK)
                .meta(meta)
                .data(data)
                .build(),
        )
        .await?
        .into_response())
}
