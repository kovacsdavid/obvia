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

use crate::app::config::AppConfig;
use crate::app::database::PgPoolManagerTrait;
use crate::auth::repository::AuthRepository;

pub(crate) mod dto;
mod handler;
pub(crate) mod middleware;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;

/// `AuthModule` is a structure that represents the authentication module in the application.
/// It encapsulates the required components for managing authentication operations.
///
/// # Fields
///
/// * `pool_manager` - An `Arc`-wrapped implementation of the `PgPoolManagerTrait` trait, providing the interface
///   for database connection pooling. This is used to manage database interactions securely and efficiently.
///
/// * `password_hasher` - An `Arc`-wrapped implementation of the `AuthPasswordHasher` trait, responsible for
///   hashing and verifying passwords. This ensures that user passwords are stored and processed securely.
///
/// * `config` - An `Arc`-wrapped instance of the `AppConfig` structure, which holds the application's configuration
///   setting necessary for configuring the authentication behavior.
///
/// # Notes
///
/// Using `Arc` for the fields allows safe sharing of these components across multiple threads.
pub struct AuthModule {
    pub pool_manager: Arc<dyn PgPoolManagerTrait>,
    pub config: Arc<AppConfig>,
    pub repo_factory: Box<dyn Fn() -> Box<dyn AuthRepository + Send + Sync> + Send + Sync>,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::app::database::MockPgPoolManagerTrait;
    use crate::auth::repository::MockAuthRepository;

    impl Default for AuthModule {
        fn default() -> Self {
            AuthModule {
                pool_manager: Arc::new(MockPgPoolManagerTrait::new()),
                config: Arc::new(AppConfig::default()),
                repo_factory: Box::new(|| Box::new(MockAuthRepository::new())),
            }
        }
    }
}
