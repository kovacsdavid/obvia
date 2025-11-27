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

use crate::manager::auth::middleware::require_auth;
use crate::tenant::worksheets::WorksheetsModule;
use crate::tenant::worksheets::handler::{
    create as worksheets_create, delete as worksheets_delete, get as worksheets_get,
    get_resolved as worksheets_get_resolved, list as worksheets_list,
    select_list as worksheets_select_list, update as worksheets_update,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn routes(worksheets_module: Arc<dyn WorksheetsModule>) -> Router {
    Router::new().nest(
        "/worksheets",
        Router::new()
            .route("/get", get(worksheets_get))
            .route("/get_resolved", get(worksheets_get_resolved))
            .route("/list", get(worksheets_list))
            .route("/select_list", get(worksheets_select_list))
            .route("/create", post(worksheets_create))
            .route("/update", put(worksheets_update))
            .route("/delete", delete(worksheets_delete))
            .layer(from_fn_with_state(worksheets_module.config(), require_auth))
            .with_state(worksheets_module),
    )
}
