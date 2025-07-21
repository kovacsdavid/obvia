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

use crate::app::app_state::AppState;
use crate::auth::middleware::require_auth;
use crate::organizational_units::handler::{
    create as managed_companies_create, get as managed_companies_get,
    list as managed_companies_list,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new().nest(
        "/organizational_units",
        Router::new()
            .route("/create", post(managed_companies_create))
            .route("/get", get(managed_companies_get))
            .route("/list", get(managed_companies_list))
            .layer(from_fn_with_state(state.clone(), require_auth))
            .with_state(state.organizational_units_module.clone()),
    )
}
