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
use crate::manager::app::config::AppConfig;
use crate::manager::app::database::PgPoolManagerTrait;
use std::sync::Arc;

mod dto;
pub(crate) mod model;
pub(crate) mod types;

pub fn init_default_address_module(
    pool_manager_config: Arc<dyn PgPoolManagerTrait>,
    config: Arc<AppConfig>,
) {
    // TODO: Addbuilder default here
}

pub struct AddressModule {}

pub struct AddressModuleBuilder {}

impl AddressModuleBuilder {
    pub fn new() -> Self {
        Self {}
    }
    pub fn build(self) -> Result<AddressModule, String> {
        Ok(AddressModule {})
    }
}

#[cfg(not(test))]
impl Default for AddressModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl Default for AddressModuleBuilder {
        fn default() -> Self {
            todo!()
        }
    }
}
