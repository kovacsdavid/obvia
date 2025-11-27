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
use crate::tenant::customers::CustomersModule;
use crate::tenant::customers::handler::{
    create as customers_create, delete as customers_delete, get as customers_get,
    get_resolved as customers_get_resolved, list as customers_list, update as customers_update,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn routes(customers_module: Arc<dyn CustomersModule>) -> Router {
    Router::new().nest(
        "/customers",
        Router::new()
            .route("/get", get(customers_get))
            .route("/get_resolved", get(customers_get_resolved))
            .route("/list", get(customers_list))
            .route("/create", post(customers_create))
            .route("/update", put(customers_update))
            .route("/delete", delete(customers_delete))
            .layer(from_fn_with_state(customers_module.config(), require_auth))
            .with_state(customers_module),
    )
}
