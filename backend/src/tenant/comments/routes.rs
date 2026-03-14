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

use std::sync::Arc;

use crate::manager::auth::middleware::require_auth;
use crate::tenant::comments::CommentsModule;
use crate::tenant::comments::handler::post as comments_post;
use axum::middleware::from_fn_with_state;
use axum::{Router, routing::post};

pub fn routes(comments_module: Arc<dyn CommentsModule>) -> Router {
    Router::new().nest(
        "/comments",
        Router::new()
            .route("/post", post(comments_post))
            .layer(from_fn_with_state(comments_module.config(), require_auth))
            .with_state(comments_module),
    )
}
