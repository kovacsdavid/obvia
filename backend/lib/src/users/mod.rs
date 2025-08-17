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

mod error;
mod handler;
pub(crate) mod model;
mod repository;
mod routes;
mod service;

pub fn init_users_module() -> UsersModuleBuilder {
    UsersModuleBuilder::default()
}

pub struct UsersModule {}

pub struct UsersModuleBuilder {}

impl UsersModuleBuilder {
    pub fn new() -> Self {
        Self {}
    }
    pub fn build(self) -> Result<UsersModule, String> {
        Ok(UsersModule {})
    }
}

#[cfg(not(test))]
impl Default for UsersModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for UsersModuleBuilder {
        fn default() -> Self {
            UsersModuleBuilder {}
        }
    }
}
