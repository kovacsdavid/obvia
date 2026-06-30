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

use super::InventoryReservationsModuleInterface;
use super::handler;
use crate::manager::auth::middleware::require_auth;
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn routes<M: InventoryReservationsModuleInterface>(
    inventory_reservations_module: Arc<M>,
) -> Router {
    Router::new().nest(
        "/inventory_reservations",
        Router::new()
            .route("/get", get(handler::get::<M>))
            .route("/get_resolved", get(handler::get_resolved::<M>))
            .route("/list", get(handler::list::<M>))
            .route("/select_list", get(handler::select_list::<M>))
            .route("/create", post(handler::create::<M>))
            .route("/update", put(handler::update::<M>))
            .route("/delete", delete(handler::delete::<M>))
            .route("/print", get(handler::print::<M>))
            .layer(from_fn_with_state(
                inventory_reservations_module.clone(),
                require_auth,
            ))
            .with_state(inventory_reservations_module),
    )
}
