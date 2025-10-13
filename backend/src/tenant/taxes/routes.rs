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
use crate::tenant::taxes::TaxesModule;
use crate::tenant::taxes::handler::{
    create as taxes_create, delete as taxes_delete, get as taxes_get,
    get_resolved as taxes_get_resolved, list as taxes_list, select_list as taxes_select_list,
    update as taxes_update,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn routes(taxes_module: Arc<TaxesModule>) -> Router {
    Router::new().nest(
        "/taxes",
        Router::new()
            .route("/get", get(taxes_get))
            .route("/get_resolved", get(taxes_get_resolved))
            .route("/list", get(taxes_list))
            .route("/select_list", get(taxes_select_list))
            .route("/create", post(taxes_create))
            .route("/update", put(taxes_update))
            .route("/delete", delete(taxes_delete))
            .layer(from_fn_with_state(
                taxes_module.config.clone(),
                require_auth,
            ))
            .with_state(taxes_module),
    )
}
