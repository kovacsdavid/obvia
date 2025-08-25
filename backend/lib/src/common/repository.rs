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
use crate::app::database::PgPoolManagerTrait;
use std::sync::Arc;

/// A wrapper around a `PoolManager` instance, primarily used to manage the PostgreSQL connection pool.
///
/// The `PoolManagerWrapper` struct provides a convenient way to encapsulate a `PoolWrapper`
/// instance, allowing it to be passed around more easily throughout an application.
pub struct PoolManagerWrapper {
    pub pool_manager: Arc<dyn PgPoolManagerTrait>,
}

impl PoolManagerWrapper {
    /// Creates a new instance of the struct with the provided PostgreSQL connection pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - A `PgPool` representing the connection pool to a PostgreSQL database.
    ///
    /// # Returns
    ///
    /// Returns an instance of the struct initialized with the provided PostgreSQL connection pool.
    pub fn new(pool_manager: Arc<dyn PgPoolManagerTrait>) -> Self {
        Self { pool_manager }
    }
}
