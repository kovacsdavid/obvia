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
use lettre::{
    AsyncTransport,
    transport::smtp::{Error, response::Response},
};
use std::fmt::Debug;

pub(crate) mod dto;
mod handler;
pub(crate) mod middleware;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod types;

pub trait AuthModuleInterface: AuthRepository + BaseModule {}

impl<P, T> AuthModuleInterface for AppState<P, T>
where
    P: PoolManager + Send + Sync + 'static,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync + Send + Sync + 'static,
    T::Error: Debug,
{
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::{BaseModule, ConfigProvider, MailTransporter};
    use crate::manager::auth::dto::claims::Claims;
    use crate::manager::auth::model::{
        AccountEventLogEntry, AccountEventStatus, AccountEventType, EmailVerification,
        ForgottenPassword, RefreshToken,
    };
    use crate::manager::auth::repository::AuthRepository;
    use crate::manager::tenants::model::UserTenant;
    use crate::manager::users::model::User;
    use crate::{
        common::{config::AppConfig, error::RepositoryResult},
        manager::auth::dto::register::RegisterRequest,
    };
    use lettre::{
        Message,
        transport::smtp::{Error, response::Response},
    };
    use mockall::mock;
    use std::net::IpAddr;
    use uuid::Uuid;

    mock!(
        pub AuthModule {}
        impl ConfigProvider for AuthModule {
            type Cfg = AppConfig;
            fn config(&self) -> &<Self as ConfigProvider>::Cfg;
        }
        impl MailTransporter for AuthModule {
            fn send(
                &self,
                message: Message,
            ) -> impl Future<Output = Result<Option<Response>, Error>> + Send;
        }
        impl BaseModule for AuthModule {}
        impl AuthRepository for AuthModule {
            fn insert_user(
                &self,
                payload: &RegisterRequest,
                password_hash: &str,
            ) -> impl Future<Output = RepositoryResult<User>> + Send;
            fn get_user_by_email(&self, email: &str)
            -> impl Future<Output = RepositoryResult<User>> + Send;

            fn get_user_by_id(&self, user_id: Uuid) -> impl Future<Output = RepositoryResult<User>> + Send;

            fn update_user(&self, user: User) -> impl Future<Output = RepositoryResult<User>> + Send;

            fn update_user_last_login_at(
                &self,
                user_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<()>> + Send;

            fn get_user_active_tenant(
                &self,
                user_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<Option<UserTenant>>> + Send;
            fn insert_email_verification(
                &self,
                user_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<EmailVerification>> + Send;
            fn get_email_verification(
                &self,
                email_verification_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<EmailVerification>> + Send;
            fn invalidate_email_verification(
                &self,
                email_verification_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<()>> + Send;
            fn insert_forgotten_password(
                &self,
                user_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<ForgottenPassword>> + Send;
            fn get_forgotten_password(
                &self,
                forgotten_password_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<ForgottenPassword>> + Send;
            fn invalidate_forgotten_password(
                &self,
                forgotten_password_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<()>> + Send;
            fn insert_refresh_token(
                &self,
                claims: &Claims,
            ) -> impl Future<Output = RepositoryResult<RefreshToken>> + Send;
            fn get_refresh_token(
                &self,
                jti: Uuid,
            ) -> impl Future<Output = RepositoryResult<RefreshToken>> + Send;
            fn consume_refresh_token(
                &self,
                jti: Uuid,
                new_jti: Uuid,
            ) -> impl Future<Output = RepositoryResult<()>> + Send;
            fn revoke_refresh_tokens_by_user_id(
                &self,
                user_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<()>> + Send;
            fn revoke_refresh_tokens_by_family_id(
                &self,
                family_id: Uuid,
            ) -> impl Future<Output = RepositoryResult<()>> + Send;
            #[allow(clippy::too_many_arguments)]
            fn insert_account_event_log(
                &self,
                user_id: Option<Uuid>,
                identifier: Option<String>,
                event_type: AccountEventType,
                event_status: AccountEventStatus,
                ip_address: Option<IpAddr>,
                user_agent: Option<String>,
                metadata: Option<serde_json::Value>,
            ) -> impl Future<Output = RepositoryResult<AccountEventLogEntry>> + Send;
            fn account_event_log_ip_and_event_status_count(
                &self,
                ip_address: IpAddr,
                event_status: AccountEventStatus,
                interval_mins: i64,
            ) -> impl Future<Output = RepositoryResult<i64>> + Send;
            fn account_event_log_by_ip_and_event_type_count(
                &self,
                ip_address: IpAddr,
                event_type: AccountEventType,
                interval_mins: i64,
            ) -> impl Future<Output = RepositoryResult<i64>> + Send;
        }
        impl AuthModuleInterface for AuthModule {}
    );
}
