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
use crate::tenant::tasks::TasksModule;
use crate::tenant::tasks::handler::{
    create as tasks_create, delete as tasks_delete, get as tasks_get,
    get_resolved as tasks_get_resolved, list as tasks_list, select_list as tasks_select_list,
    update as tasks_update,
};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn routes(tasks_module: Arc<TasksModule>) -> Router {
    Router::new().nest(
        "/tasks",
        Router::new()
            .route("/get", get(tasks_get))
            .route("/get_resolved", get(tasks_get_resolved))
            .route("/list", get(tasks_list))
            .route("/select_list", get(tasks_select_list))
            .route("/create", post(tasks_create))
            .route("/update", put(tasks_update))
            .route("/delete", delete(tasks_delete))
            .layer(from_fn_with_state(
                tasks_module.config.clone(),
                require_auth,
            ))
            .with_state(tasks_module),
    )
}
