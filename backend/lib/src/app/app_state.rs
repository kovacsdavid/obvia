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

use crate::app::config::AppConfig;
use crate::auth::AuthModule;
use crate::organizational_units::OrganizationalUnitsModule;
use crate::users::UsersModule;
use std::sync::Arc;

/// `AppState` is a shared state structure that holds references to various application modules.
/// It is intended to be used across the application to provide shared access to core components.
///
/// # Notes
///
/// * All fields are wrapped in `Arc` (Atomic Reference Counting smart pointer) to enable thread-safe access
///   to shared resources across the application.
/// * This structure is commonly utilized within asynchronous or multi-threaded contexts where shared state
///   is required.
/// * Reorganization may be required in the future because I assume most of the modules will use mostly the same resources
pub struct AppState {
    pub auth_module: Arc<AuthModule>,
    pub config_module: Arc<AppConfig>,
    pub users_module: Arc<UsersModule>,
    pub organizational_units_module: Arc<OrganizationalUnitsModule>,
}
