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
use crate::tenant::inventory_movements::InventoryMovementsModule;
use crate::tenant::inventory_movements::handler::{
    create as inventory_movement_create, delete as inventory_movement_delete,
    get as inventory_movement_get, get_resolved as inventory_movement_get_resolved,
    list as inventory_movement_list, select_list as inventory_movement_select_list,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post};
use std::sync::Arc;

pub fn routes(module: Arc<dyn InventoryMovementsModule>) -> Router {
    Router::new().nest(
        "/inventory_movements",
        Router::new()
            .route("/get", get(inventory_movement_get))
            .route("/get_resolved", get(inventory_movement_get_resolved))
            .route("/list", get(inventory_movement_list))
            .route("/select_list", get(inventory_movement_select_list))
            .route("/create", post(inventory_movement_create))
            .route("/delete", delete(inventory_movement_delete))
            .layer(from_fn_with_state(module.config(), require_auth))
            .with_state(module),
    )
}
