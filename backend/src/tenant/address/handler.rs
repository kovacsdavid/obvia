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

use crate::common::dto::QueryParam;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::address::AddressModule;
use crate::tenant::address::dto::CreateAddress;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::{Json, debug_handler};
use std::sync::Arc;

#[debug_handler]
pub async fn get(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(address_module): State<Arc<AddressModule>>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn create(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(address_module): State<Arc<AddressModule>>,
    payload: Result<Json<CreateAddress>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn update(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(address_module): State<Arc<AddressModule>>,
    payload: Result<Json<CreateAddress>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn delete(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(address_module): State<Arc<AddressModule>>,
    payload: Result<Json<CreateAddress>, JsonRejection>,
) -> Response {
    todo!()
}

#[debug_handler]
pub async fn list(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(address_module): State<Arc<AddressModule>>,
    Query(payload): Query<QueryParam>,
) -> Response {
    todo!()
}
