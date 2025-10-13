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
use crate::tenant::services::ServicesModule;
use crate::tenant::services::handler::{
    create as services_create, delete as services_delete, get as services_get,
    get_resolved as services_get_resolved, list as services_list,
    select_list as services_select_list, update as services_update,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn routes(services_module: Arc<ServicesModule>) -> Router {
    Router::new().nest(
        "/services",
        Router::new()
            .route("/get", get(services_get))
            .route("/get_resolved", get(services_get_resolved))
            .route("/list", get(services_list))
            .route("/select_list", get(services_select_list))
            .route("/create", post(services_create))
            .route("/update", put(services_update))
            .route("/delete", delete(services_delete))
            .layer(from_fn_with_state(
                services_module.config.clone(),
                require_auth,
            ))
            .with_state(services_module),
    )
}
