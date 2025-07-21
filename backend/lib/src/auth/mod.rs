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

use std::sync::Arc;

use service::AuthPasswordHasher;

use crate::app::config::AppConfig;
use crate::app::database::PgPoolManagerTrait;

pub mod dto;
pub mod handler;
pub mod middleware;
pub mod repository;
pub mod routes;
pub mod service;
#[cfg(test)]
pub mod tests;

pub struct AuthModule {
    pub db_pools: Arc<dyn PgPoolManagerTrait>,
    pub password_hasher: Arc<dyn AuthPasswordHasher>,
    pub config: Arc<AppConfig>,
}
