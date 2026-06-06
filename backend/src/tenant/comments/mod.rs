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

use crate::common::database::PoolManager;
use crate::common::{AppState, BaseModule};
use crate::tenant::comments::repository::CommentsRepository;
use lettre::{
    AsyncTransport,
    transport::smtp::{Error, response::Response},
};
use std::fmt::Debug;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub trait CommentsModule: CommentsRepository + BaseModule {}

impl<P, T> CommentsModule for AppState<P, T>
where
    P: PoolManager + Send + Sync + 'static,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync + Send + Sync + 'static,
    T::Error: Debug,
{
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
        pub CommentsModule {}
        impl ConfigProvider for CommentsModule {
            fn config(&self) -> Arc<AppConfig>;
        }
        #[async_trait]
        impl MailTransporter for CommentsModule {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl CommentsModule for CommentsModule {
            fn comments_repo(&self) -> Arc<dyn CommentsRepository>;
        }
    );
}
*/
