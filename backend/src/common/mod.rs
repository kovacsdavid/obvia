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

use std::sync::Arc;

use crate::{
    common::error::RepositoryResult,
    manager::{
        app::{
            config::{AppConfig, BasicDatabaseConfig},
            database::{
                ConnectionTester, DatabaseMigrator, PgConnectionTester, PgPoolManager, PoolManager,
            },
        },
        tenants::repository::TenantsRepository,
    },
};
use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::{Error, authentication::Credentials, response::Response},
};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

pub(crate) mod dto;
pub(crate) mod error;
pub(crate) mod extractors;
pub(crate) mod macros;
pub(crate) mod model;
pub(crate) mod services;
pub(crate) mod types;

pub trait ConfigProvider: Send + Sync {
    fn config(&self) -> Arc<AppConfig>;
}

#[async_trait]
pub trait MailTransporter: ConfigProvider + Send + Sync {
    async fn send(&self, message: Message) -> Result<Response, Error>;
}

pub struct AppState<P, T>
where
    P: Send + Sync,
    T: Send + Sync,
{
    pub config: Arc<AppConfig>,
    pub default_smtp_transport: Arc<T>,
    pub pool_manager: Arc<P>,
    pub migrator: Arc<dyn DatabaseMigrator>,
    pub connection_tester: Arc<dyn ConnectionTester>,
}

pub type DefaultSmtpTransport = AsyncSmtpTransport<Tokio1Executor>;
pub type DefaultAppState = AppState<PgPoolManager, DefaultSmtpTransport>;

impl DefaultAppState {
    fn init_config() -> anyhow::Result<AppConfig> {
        Ok(AppConfig::from_env()?)
    }
    async fn init_pool_manager(config: Arc<AppConfig>) -> anyhow::Result<PgPoolManager> {
        Ok(PgPoolManager::new(config.main_database(), config.default_tenant_database()).await?)
    }
    fn init_smpt_transport(config: Arc<AppConfig>) -> anyhow::Result<DefaultSmtpTransport> {
        Ok(
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(config.mail().smtp_host())?
                .credentials(Credentials::new(
                    config.mail().smtp_user().to_owned(),
                    config.mail().smtp_passwd().to_owned(),
                ))
                .build(),
        )
    }
    pub async fn new() -> anyhow::Result<DefaultAppState> {
        let config = Arc::new(Self::init_config()?);
        let pool_manager = Arc::new(Self::init_pool_manager(config.clone()).await?);
        Ok(Self {
            config: config.clone(),
            default_smtp_transport: Arc::new(Self::init_smpt_transport(config.clone())?),
            pool_manager: pool_manager.clone(),
            migrator: pool_manager.clone(),
            connection_tester: Arc::new(PgConnectionTester),
        })
    }
    pub async fn init_tenant_pools(&self) -> anyhow::Result<()> {
        for tenant in TenantsRepository::get_all(self.pool_manager.as_ref()).await? {
            match BasicDatabaseConfig::try_from(&tenant) {
                Ok(db_config) => {
                    match self
                        .pool_manager
                        .add_tenant_pool(tenant.id, &db_config)
                        .await
                    {
                        Ok(tenant_id) => {
                            info!("Tenant pool initialization is successful: {}", &tenant_id)
                        }
                        // TODO: Notify the administrator about the failed tenant pool initialization
                        Err(e) => error!("Tenant pool initialization failed: {}", e),
                    }
                }
                Err(e) => error!("Error parsing tenant: {}", e),
            }
        }
        Ok(())
    }
}

impl<P, T> ConfigProvider for AppState<P, T>
where
    P: Send + Sync,
    T: Send + Sync,
{
    fn config(&self) -> Arc<AppConfig> {
        self.config.clone()
    }
}

#[async_trait]
impl<P> MailTransporter for AppState<P, DefaultSmtpTransport>
where
    P: Send + Sync,
{
    async fn send(&self, message: Message) -> Result<Response, Error> {
        self.default_smtp_transport.send(message).await
    }
}

#[async_trait]
impl<T> DatabaseMigrator for AppState<PgPoolManager, T>
where
    T: Send + Sync,
{
    async fn migrate_main_db(&self) -> RepositoryResult<()> {
        self.migrator.migrate_main_db().await
    }
    async fn migrate_tenant_db(&self, tenant_id: Uuid) -> RepositoryResult<()> {
        self.migrator.migrate_tenant_db(tenant_id).await
    }
    async fn migrate_all_tenant_dbs(&self) -> RepositoryResult<()> {
        self.migrator.migrate_all_tenant_dbs().await
    }
}

#[async_trait]
impl<T> PoolManager for AppState<PgPoolManager, T>
where
    T: Send + Sync,
{
    fn get_main_pool(&self) -> PgPool {
        self.pool_manager.get_main_pool()
    }
    fn get_default_tenant_pool(&self) -> PgPool {
        self.pool_manager.get_default_tenant_pool()
    }
    fn get_tenant_pool(&self, tenant_id: Uuid) -> RepositoryResult<PgPool> {
        self.pool_manager.get_tenant_pool(tenant_id)
    }
    async fn add_tenant_pool(
        &self,
        tenant_id: Uuid,
        config: &BasicDatabaseConfig,
    ) -> RepositoryResult<Uuid> {
        self.pool_manager.add_tenant_pool(tenant_id, config).await
    }
}
