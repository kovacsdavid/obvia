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
use crate::tenant::products::ProductsModule;
use crate::tenant::products::handler::{
    create as products_create, delete as products_delete, get as products_get,
    get_resolved as products_get_resolved, list as products_list,
    select_list as products_select_list, update as products_update,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn routes(products_module: Arc<ProductsModule>) -> Router {
    Router::new().nest(
        "/products",
        Router::new()
            .route("/get", get(products_get))
            .route("/get_resolved", get(products_get_resolved))
            .route("/list", get(products_list))
            .route("/select_list", get(products_select_list))
            .route("/create", post(products_create))
            .route("/update", post(products_update))
            .route("/delete", post(products_delete))
            .layer(from_fn_with_state(
                products_module.config.clone(),
                require_auth,
            ))
            .with_state(products_module),
    )
}
