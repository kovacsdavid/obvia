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

use crate::common::database::PoolManager;
use crate::common::{AppState, BaseModule};
use crate::manager::auth::repository::AuthRepository;
use crate::manager::users::repository::UsersRepository;
use lettre::{
    AsyncTransport,
    transport::smtp::{Error, response::Response},
};
use std::fmt::Debug;
use std::sync::Arc;

pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;

pub trait UsersModule: BaseModule {
    fn users_repo(&self) -> Arc<dyn UsersRepository + Send + Sync>;
    fn auth_repo(&self) -> Arc<dyn AuthRepository + Send + Sync>;
}

impl<P, T> UsersModule for AppState<P, T>
where
    P: PoolManager + Send + Sync + 'static,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync + Send + Sync + 'static,
    T::Error: Debug,
{
    fn users_repo(&self) -> Arc<dyn UsersRepository + Send + Sync> {
        self.get_main_pool()
    }
    fn auth_repo(&self) -> Arc<dyn AuthRepository + Send + Sync> {
        self.get_main_pool()
    }
}

/*
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::config::AppConfig;
    use async_trait::async_trait;
    use lettre::{
        Message,
        transport::smtp::{Error, response::Response},
    };
    use mockall::mock;

    mock!(
        pub UsersModule {}
        impl ConfigProvider for UsersModule {
            fn config(&self) -> Arc<AppConfig>;
        }
        #[async_trait]
        impl MailTransporter for UsersModule {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl UsersModule for UsersModule {
            fn users_repo(&self) -> Arc<dyn UsersRepository>;
            fn auth_repo(&self) -> Arc<dyn AuthRepository>;
        }
    );
}
*/
