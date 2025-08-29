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
use crate::tenant::address::AddressModule;
use crate::tenant::address::handler::{
    create as address_create, delete as address_delete, get as address_get, list as address_list,
    update as address_update,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn routes(address_module: Arc<AddressModule>) -> Router {
    Router::new().nest(
        "/addresses",
        Router::new()
            .route("/get", get(address_get))
            .route("/list", get(address_list))
            .route("/create", post(address_create))
            .route("/update", post(address_update))
            .route("/delete", post(address_delete))
            .layer(from_fn_with_state(
                address_module.config.clone(),
                require_auth,
            ))
            .with_state(address_module),
    )
}
