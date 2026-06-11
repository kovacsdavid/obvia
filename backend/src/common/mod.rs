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

#[cfg(test)]
use mockall::automock;
use sqlx::PgPool;
use std::fmt::Debug;
use uuid::Uuid;

use crate::{
    common::{
        config::{AppConfig, BasicDatabaseConfig},
        database::{DatabaseMigrator, PoolManager},
        error::RepositoryResult,
    },
    manager::tenants::model::Tenant,
};
use lettre::{
    AsyncTransport, Message,
    message::header::{Subject, To},
    transport::smtp::{Error, response::Response},
};
use tracing::{error, info};

pub(crate) mod config;
pub(crate) mod database;
pub(crate) mod dto;
pub(crate) mod error;
pub(crate) mod extractors;
pub(crate) mod handler;
pub(crate) mod init;
pub(crate) mod macros;
pub(crate) mod model;
pub(crate) mod pdf;
pub(crate) mod query_parser;
pub(crate) mod service;
pub(crate) mod types;
pub(crate) mod utils;
pub(crate) mod value_object;

pub trait BaseModule:
    ConfigProvider<Cfg = AppConfig> + MailTransporter + Send + Sync + 'static
{
}

pub trait ConfigProvider {
    type Cfg;
    fn config(&self) -> &Self::Cfg;
}

pub trait MailTransporter {
    fn send(
        &self,
        message: Message,
    ) -> impl Future<Output = Result<Option<Response>, Error>> + Send;
}

pub struct AppState<P, T>
where
    P: Send + Sync + 'static,
    T: Send + Sync + 'static,
{
    config: AppConfig,
    pool_manager: P,
    smtp_transport: T,
}

impl<P, T> AppState<P, T>
where
    P: Send + Sync,
    T: Send + Sync,
{
    pub async fn new(
        config: AppConfig,
        pool_manager: P,
        smtp_transport: T,
    ) -> anyhow::Result<Self> {
        // let pool_manager = Self::init_pool_manager(&config).await?;
        // let smtp_transport = Self::init_smpt_transport(&config)?;
        Ok(Self {
            config,
            pool_manager,
            smtp_transport,
        })
    }
    pub fn pool_manager(&self) -> &P {
        &self.pool_manager
    }
}

impl<P, T> ConfigProvider for AppState<P, T>
where
    P: Send + Sync,
    T: Send + Sync,
{
    type Cfg = AppConfig;
    fn config(&self) -> &Self::Cfg {
        &self.config
    }
}

#[cfg_attr(test, automock)]
impl<P, T> MailTransporter for AppState<P, T>
where
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync,
    T::Error: Debug + Send + Sync,
    P: Send + Sync,
{
    async fn send(&self, message: Message) -> Result<Option<Response>, Error> {
        let subject = message.headers().get::<Subject>();
        let to = message.headers().get::<To>();
        if self.config.mail().mail_enabled() {
            match self.smtp_transport.send(message).await {
                Ok(r) => {
                    info!("Mail sent: subject={:?} to={:?}", subject, to);
                    Ok(Some(r))
                }
                Err(e) => {
                    error!(
                        "Mail transport error: subject={:?} to={:?} error={:?}",
                        subject, to, e
                    );
                    Err(e)
                }
            }
        } else {
            info!(
                "Mail sent (dry run; cfg:mail_enabled=false): subject={:?} to={:?}",
                subject, to
            );
            Ok(None)
        }
    }
}

impl<P, T> DatabaseMigrator for AppState<P, T>
where
    P: DatabaseMigrator + Send + Sync,
    T: Send + Sync,
{
    async fn migrate_main_db(&self) -> error::RepositoryResult<()> {
        self.pool_manager().migrate_main_db().await
    }
    async fn migrate_tenant_db(&self, tenant_id: uuid::Uuid) -> RepositoryResult<()> {
        self.pool_manager().migrate_tenant_db(tenant_id).await
    }
    async fn migrate_all_tenant_dbs(&self, tenants: &[Tenant]) -> error::RepositoryResult<()> {
        self.pool_manager().migrate_all_tenant_dbs(tenants).await
    }
}

impl<P, T> PoolManager for AppState<P, T>
where
    P: PoolManager + Send + Sync + 'static,
    T: Send + Sync + 'static,
{
    fn get_main_pool(&self) -> PgPool {
        self.pool_manager().get_main_pool()
    }
    fn get_tenant_pool(&self, tenant_id: Uuid) -> RepositoryResult<PgPool> {
        self.pool_manager().get_tenant_pool(tenant_id)
    }
    async fn add_tenant_pool(
        &self,
        tenant_id: Uuid,
        config: &BasicDatabaseConfig,
    ) -> RepositoryResult<Uuid> {
        self.pool_manager().add_tenant_pool(tenant_id, config).await
    }
    async fn delete_tenant_pool(&self, tenant_id: Uuid) -> RepositoryResult<()> {
        self.pool_manager().delete_tenant_pool(tenant_id).await
    }
}

impl<P, T> BaseModule for AppState<P, T>
where
    P: PoolManager + Send + Sync + 'static,
    T: AsyncTransport<Ok = Response, Error = Error> + Send + Sync + 'static,
    T::Error: Debug,
{
}
