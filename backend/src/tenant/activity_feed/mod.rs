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
use crate::common::error::RepositoryResult;
use crate::common::{AppState, BaseModule};
use crate::tenant::activity_feed::repository::ActivityFeedRepository;
use lettre::{
    AsyncTransport,
    transport::smtp::{Error, response::Response},
};
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

mod dto;
mod handler;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub trait ActivityFeedModuleInterface: BaseModule {
    fn activity_feed_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn ActivityFeedRepository + Send + Sync>>;
}

impl<P, T> ActivityFeedModuleInterface for AppState<P, T>
where
    P: PoolManager,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync,
    T::Error: Debug,
{
    fn activity_feed_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn ActivityFeedRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::config::AppConfig;
    use crate::common::error::RepositoryResult;
    use crate::common::{BaseModule, ConfigProvider, MailTransporter};
    use lettre::{
        Message,
        transport::smtp::{Error, response::Response},
    };
    use mockall::mock;

    mock!(
        pub ActivityFeedModule {}
        impl ConfigProvider for ActivityFeedModule {
            type Cfg = AppConfig;
            fn config(&self) -> &<Self as ConfigProvider>::Cfg;
        }
        impl MailTransporter for ActivityFeedModule {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl BaseModule for ActivityFeedModule {}
        impl ActivityFeedModuleInterface for ActivityFeedModule {
            fn activity_feed_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn ActivityFeedRepository + Send + Sync>>;
        }
    );
}
