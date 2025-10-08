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
use crate::tenant::warehouses::WarehousesModule;
use crate::tenant::warehouses::handler::{
    create as warehouses_create, delete as warehouses_delete, get as warehouses_get,
    get_resolved as warehouses_get_resolved, list as warehouses_list, update as warehouses_update,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn routes(warehouses_module: Arc<WarehousesModule>) -> Router {
    Router::new().nest(
        "/warehouses",
        Router::new()
            .route("/get", get(warehouses_get))
            .route("/get_resolved", get(warehouses_get_resolved))
            .route("/list", get(warehouses_list))
            .route("/create", post(warehouses_create))
            .route("/update", put(warehouses_update))
            .route("/delete", delete(warehouses_delete))
            .layer(from_fn_with_state(
                warehouses_module.config.clone(),
                require_auth,
            ))
            .with_state(warehouses_module),
    )
}
