/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

use lettre::{
    AsyncTransport,
    transport::smtp::{Error, response::Response},
};
use std::fmt::Debug;

use crate::common::{AppState, BaseModule, database::PoolManager};

pub mod dto;
mod handler;
pub mod model;
pub mod repository;
pub mod routes;
pub mod service;
pub mod types;

// TODO: define this interface
pub trait HandoverAndAcceptanceModuleInterface: BaseModule {}

// TODO: implement the interface
impl<P, T> HandoverAndAcceptanceModuleInterface for AppState<P, T>
where
    P: PoolManager + Debug,
    T: AsyncTransport<Ok = Response, Error = Error> + Debug + Send + Sync,
    T::Error: Debug,
{
}

#[cfg(test)]
pub mod tests {
    use crate::common::{ConfigProvider, MailTransporter, config::AppConfig};
    use lettre::Message;

    use super::*;

    use mockall::mock;

    mock!(
        pub HandoverAndAcceptanceModule {}
        impl Debug for HandoverAndAcceptanceModule {
            fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a> ) -> std::fmt::Result;
        }
        impl ConfigProvider for HandoverAndAcceptanceModule {
            type Cfg = AppConfig;
            fn config(&self) -> &<Self as ConfigProvider>::Cfg;
        }
        impl MailTransporter for HandoverAndAcceptanceModule {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl BaseModule for HandoverAndAcceptanceModule {}
        impl HandoverAndAcceptanceModuleInterface for HandoverAndAcceptanceModule {}
    );
}
