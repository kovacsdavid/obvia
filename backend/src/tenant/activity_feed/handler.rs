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

use crate::common::dto::{HandlerResult, SuccessResponseBuilder};
use crate::common::error::{FriendlyError, IntoFriendlyError};
use crate::common::query_parser::{CommonRawQuery, GetQuery};
use crate::common::types::{EmptyFilterBy, EmptyOrderBy};
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::activity_feed::ActivityFeedModule;
use crate::tenant::activity_feed::service::ActivityFeedService;
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::str::FromStr;
use std::sync::Arc;

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(activity_feed_module): State<Arc<dyn ActivityFeedModule>>,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let (meta, data) = match ActivityFeedService::get_all_paged(
        &GetQuery::<EmptyOrderBy, EmptyFilterBy>::from_str(payload.as_str())
            .map_err(|e| FriendlyError::internal(file!(), e.to_string()).into_response())?,
        &claims,
        activity_feed_module.activity_feed_repo(),
    )
    .await
    {
        Ok((m, d)) => (m, d),
        Err(e) => {
            return Err(e
                .into_friendly_error(activity_feed_module)
                .await
                .into_response());
        }
    };

    match SuccessResponseBuilder::new()
        .status_code(StatusCode::OK)
        .meta(meta)
        .data(data)
        .build()
    {
        Ok(r) => Ok(r.into_response()),
        Err(e) => Err(e
            .into_friendly_error(activity_feed_module)
            .await
            .into_response()),
    }
}
