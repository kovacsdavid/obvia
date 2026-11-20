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
use std::{fmt::Debug, sync::Arc};
use uuid::Uuid;

use crate::{
    common::{AppState, BaseModule, database::PoolManager, error::RepositoryResult},
    tenant::owner_profile::repository::OwnerProfileRepository,
};

pub mod dto;
mod handler;
pub mod model;
pub mod repository;
pub mod routes;
pub mod service;
pub mod types;

pub use dto::{
    print::{Print, test_handover_and_acceptance_full_print_builder},
    user_input::{UserInput, UserInputError, UserInputHelper},
};
pub use repository::Repository;
pub use routes::routes;
pub use service::{Service, ServiceError};

// TODO: define this interface
pub trait ModuleInterface: BaseModule {
    fn handover_and_acceptance_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn Repository + Send + Sync>>;
    fn owner_profile_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn OwnerProfileRepository + Send + Sync>>;
}

// TODO: implement the interface
impl<P, T> ModuleInterface for AppState<P, T>
where
    P: PoolManager + Debug,
    T: AsyncTransport<Ok = Response, Error = Error> + Debug + Send + Sync,
    T::Error: Debug,
{
    fn handover_and_acceptance_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn Repository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }

    fn owner_profile_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn OwnerProfileRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::common::{ConfigProvider, MailTransporter, config::AppConfig};
    use lettre::Message;

    use super::*;

    use mockall::mock;

    mock!(
        pub Module {}
        impl Debug for Module {
            fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a> ) -> std::fmt::Result;
        }
        impl ConfigProvider for Module {
            type Cfg = AppConfig;
            fn config(&self) -> &<Self as ConfigProvider>::Cfg;
        }
        impl MailTransporter for Module {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl BaseModule for Module {}
        impl ModuleInterface for Module {
            fn handover_and_acceptance_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn Repository + Send + Sync>>;
            fn owner_profile_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn OwnerProfileRepository + Send + Sync>>;
        }
    );
}
