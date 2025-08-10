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

/// A builder struct for constructing the `AppState` structure with optional modules.
///
/// `AppStateBuilder` provides a convenient way to assemble the application state by configuring
/// its optional components such as authentication, configuration, users, and organizational units.
#[derive(Default)]
pub struct AppStateBuilder {
    pub auth_module: Option<Arc<AuthModule>>,
    pub config_module: Option<Arc<AppConfig>>,
    pub users_module: Option<Arc<UsersModule>>,
    pub organizational_units_module: Option<Arc<OrganizationalUnitsModule>>,
}

impl AppStateBuilder {
    /// Creates a new instance of the struct with all its module fields initialized to `None`.
    ///
    /// # Returns
    /// A new instance of the struct with the following default field values:
    /// - `auth_module`: None
    /// - `config_module`: None
    /// - `users_module`: None
    /// - `organizational_units_module`: None
    pub fn new() -> Self {
        Self {
            auth_module: None,
            config_module: None,
            users_module: None,
            organizational_units_module: None,
        }
    }
    /// Sets the authentication module for the current instance and returns the updated instance.
    ///
    /// This method allows attaching an instance of `AuthModule` to the current structure,
    /// enabling authentication functionalities as defined by the `AuthModule`. The provided
    /// `AuthModule` instance is wrapped in an `Arc` to allow shared ownership across threads.
    ///
    /// # Arguments
    ///
    /// * `self` - The current instance of the structure implementating the method, passed in mutable form.
    /// * `auth_module` - An `Arc` wrapped instance of the `AuthModule` to be applied to the current structure.
    ///
    /// # Returns
    ///
    /// Returns the updated instance of the structure with the specified `AuthModule` set.
    ///
    /// # Note
    ///
    /// This method consumes the input instance (`self`) and returns an updated instance with the
    /// `AuthModule` applied. Make sure to handle the return value appropriately to maintain the state of the instance.
    pub fn auth_module(mut self, auth_module: Arc<AuthModule>) -> Self {
        self.auth_module = Some(auth_module);
        self
    }
    /// Sets the configuration module for the current instance.
    ///
    /// This method takes an `Arc<AppConfig>` as input and assigns it to the
    /// internal `config_module` field. The module is then wrapped in an `Option`
    /// to allow for potential absence of a configuration at runtime. The method
    /// leverages method chaining, returning `self` after updating the field.
    ///
    /// # Arguments
    /// * `config_module` - An `Arc<AppConfig>` that provides access to the application's configuration in a thread-safe manner.
    ///
    /// # Returns
    /// Returns the updated instance of `Self` to enable method chaining.
    pub fn config_module(mut self, config_module: Arc<AppConfig>) -> Self {
        self.config_module = Some(config_module);
        self
    }
    /// Sets the `users_module` for the current instance and returns the updated instance.
    ///
    /// This method allows injecting an `Arc<UsersModule>` into the current object, typically used
    /// to manage user-related functionality or dependencies associated with the `UsersModule`.
    ///
    /// # Arguments
    ///
    /// * `users_module` - An `Arc`-wrapped `UsersModule` instance that will be assigned to the
    ///   `users_module` field of the object.
    ///
    /// # Returns
    ///
    /// Returns the updated instance of the object to enable method chaining.
    pub fn users_module(mut self, users_module: Arc<UsersModule>) -> Self {
        self.users_module = Some(users_module);
        self
    }
    /// Sets the `organizational_units_module` for the current instance.
    ///
    /// This method accepts an `Arc<OrganizationalUnitsModule>` and updates the associated
    /// field in the struct. It returns the modified instance of the struct for method
    /// chaining.
    ///
    /// # Arguments
    ///
    /// * `organizational_units_module` - An `Arc`-wrapped instance of `OrganizationalUnitsModule`
    ///   that will be set for the current instance.
    ///
    /// # Returns
    ///
    /// Returns the modified instance of `Self` after the `organizational_units_module`
    /// has been set.
    pub fn organizational_units_module(
        mut self,
        organizational_units_module: Arc<OrganizationalUnitsModule>,
    ) -> Self {
        self.organizational_units_module = Some(organizational_units_module);
        self
    }
    /// Builds an `AppState` instance by initializing its required modules.
    ///
    /// This method validates that all mandatory modules (`auth_module`, `config_module`,
    /// `users_module`, and `organizational_units_module`) have been provided. If any of these
    /// modules are missing, it returns an `Err` with a descriptive error message.
    ///
    /// # Returns
    /// - `Ok(AppState)`: If all required modules are initialized.
    /// - `Err(String)`: If any required module is not provided.
    ///
    /// # Errors
    /// This method returns an error in the following cases:
    /// - If the `auth_module` is `None`, an error stating "Auth module is required" is returned.
    /// - If the `config_module` is `None`, an error stating "Config module is required" is returned.
    /// - If the `users_module` is `None`, an error stating "Users module is required" is returned.
    /// - If the `organizational_units_module` is `None`, an error stating "Organizational units module is required" is returned.
    pub fn build(self) -> Result<AppState, String> {
        Ok(AppState {
            auth_module: self.auth_module.ok_or("Auth module is required")?,
            config_module: self.config_module.ok_or("Config module is required")?,
            users_module: self.users_module.ok_or("Users module is required")?,
            organizational_units_module: self
                .organizational_units_module
                .ok_or("Organizational units module is required")?,
        })
    }
}
