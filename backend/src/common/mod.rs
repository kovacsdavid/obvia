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

use derive_builder::UninitializedFieldError;
use sqlx::PgPool;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    common::{
        config::{AppConfig, BasicDatabaseConfig},
        database::{DatabaseMigrator, PoolManager},
        error::RepositoryResult,
    },
    manager::{
        auth::repository::AuthRepository,
        tenants::{model::Tenant, repository::TenantsRepository},
        users::repository::UsersRepository as ManagerUsersRepository,
    },
    tenant::{
        activity_feed::repository::ActivityFeedRepository, address::repository::AddressRepository,
        comments::repository::CommentsRepository, currencies::repository::CurrenciesRepository,
        customers::repository::CustomersRepository,
        handover_and_acceptance::Repository as HandoverAndAcceptanceRepository,
        inventory::repository::InventoryRepository,
        inventory_movements::repository::InventoryMovementsRepository,
        inventory_reservations::repository::InventoryReservationsRepository,
        owner_profile::repository::OwnerProfileRepository,
        products::repository::ProductsRepository, services::repository::ServicesRepository,
        tasks::repository::TasksRepository, taxes::repository::TaxesRepository,
        users::repository::UsersRepository as TenantUsersRepository,
        warehouses::repository::WarehousesRepository, worksheets::repository::WorksheetsRepository,
    },
};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use lettre::{
    AsyncTransport, Message,
    message::header::{Subject, To},
    transport::smtp::{Error, response::Response},
};
use std::sync::LazyLock;
use tracing::{error, info};

pub mod config;
pub mod database;
pub(crate) mod dto;
pub(crate) mod error;
pub(crate) mod extractors;
pub(crate) mod handler;
pub mod init;
pub(crate) mod macros;
pub(crate) mod model;
pub(crate) mod pdf;
pub(crate) mod query_parser;
pub mod service;
pub(crate) mod types;
pub(crate) mod utils;
pub(crate) mod value_object;

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

pub trait ManagerRepositoryProvider: PoolManager {
    fn auth_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn AuthRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn tenants_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn TenantsRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn users_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn ManagerUsersRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
}

pub trait TenantRepositoryProvider: PoolManager {
    fn activity_feed_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn ActivityFeedRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn address_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn AddressRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn comments_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn CommentsRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn currencies_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn CurrenciesRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn customers_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn CustomersRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn handover_and_acceptance_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn HandoverAndAcceptanceRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn inventory_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn InventoryRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn inventory_movements_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn InventoryMovementsRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn inventory_reservations_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn InventoryReservationsRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn owner_profile_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn OwnerProfileRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn products_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn ProductsRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn services_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn ServicesRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn tasks_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn TasksRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn taxes_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn TaxesRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn users_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn TenantUsersRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn warehouses_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn WarehousesRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
    fn worksheets_repo(
        &self,
        tenant_id: Uuid,
    ) -> RepositoryResult<Arc<dyn WorksheetsRepository + Send + Sync>> {
        Ok(self.get_tenant_pool(tenant_id)?)
    }
}

#[derive(Debug)]
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
    fn get_main_pool(&self) -> Arc<PgPool> {
        self.pool_manager().get_main_pool()
    }
    fn get_tenant_pool(&self, tenant_id: Uuid) -> RepositoryResult<Arc<PgPool>> {
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

impl<P, T> ManagerRepositoryProvider for AppState<P, T>
where
    P: PoolManager,
    T: Send + Sync,
{
}

impl<P, T> TenantRepositoryProvider for AppState<P, T>
where
    P: PoolManager,
    T: Send + Sync,
{
}

pub trait AppStateInterface:
    ConfigProvider
    + MailTransporter
    + DatabaseMigrator
    + PoolManager
    + ManagerRepositoryProvider
    + TenantRepositoryProvider
{
}

#[derive(Debug, Error)]
pub struct CommonBuilderError(String);

impl From<UninitializedFieldError> for CommonBuilderError {
    fn from(value: UninitializedFieldError) -> Self {
        Self(value.to_string())
    }
}

impl Display for CommonBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub static TEST_TIME: LazyLock<DateTime<Utc>> =
    LazyLock::new(|| "2026-01-02T11:11:11Z".parse().unwrap());
pub static TEST_TZ: LazyLock<Tz> = LazyLock::new(|| "Europe/Budapest".parse().unwrap());
pub static TEST_TIME_TZ: LazyLock<String> = LazyLock::new(|| {
    TEST_TIME
        .with_timezone(&*TEST_TZ)
        .format(&format!("%Y. %m. %d. %H:%M:%S ({})", *TEST_TZ))
        .to_string()
});

#[cfg(test)]
pub mod tests {
    use super::*;
    use error::RepositoryError;
    use mockall::mock;

    mock!(
        pub AppState {}
        impl Debug for AppState {
            fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a> ) -> std::fmt::Result;
        }
        impl ConfigProvider for AppState {
            type Cfg = AppConfig;
            fn config(&self) -> &<Self as ConfigProvider>::Cfg;
        }
        impl MailTransporter for AppState {
            async fn send(&self, message: Message) -> Result<Option<Response>, Error>;
        }
        impl PoolManager for AppState {
            fn get_main_pool(&self) -> Arc<PgPool>;
            fn get_tenant_pool(&self, tenant_id: Uuid) -> Result<Arc<PgPool>, RepositoryError>;
            fn add_tenant_pool(
                &self,
                tenant_id: Uuid,
                config: &BasicDatabaseConfig,
            ) -> impl Future<Output = Result<Uuid, RepositoryError>> + Send;
            fn delete_tenant_pool(
                &self,
                tenant_id: Uuid,
            ) -> impl Future<Output = Result<(), RepositoryError>> + Send;
        }
        impl ManagerRepositoryProvider for AppState {
            fn auth_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn AuthRepository + Send + Sync>>;
            fn tenants_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn TenantsRepository + Send + Sync>>;
            fn users_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn ManagerUsersRepository + Send + Sync>>;
        }
        impl TenantRepositoryProvider for AppState {
            fn activity_feed_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn ActivityFeedRepository + Send + Sync>>;
            fn address_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn AddressRepository + Send + Sync>>;
            fn comments_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn CommentsRepository + Send + Sync>>;
            fn currencies_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn CurrenciesRepository + Send + Sync>>;
            fn customers_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn CustomersRepository + Send + Sync>>;
            fn handover_and_acceptance_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn HandoverAndAcceptanceRepository + Send + Sync>>;
            fn inventory_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn InventoryRepository + Send + Sync>>;
            fn inventory_movements_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn InventoryMovementsRepository + Send + Sync>>;
            fn inventory_reservations_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn InventoryReservationsRepository + Send + Sync>>;
            fn owner_profile_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn OwnerProfileRepository + Send + Sync>>;
            fn products_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn ProductsRepository + Send + Sync>>;
            fn services_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn ServicesRepository + Send + Sync>>;
            fn tasks_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn TasksRepository + Send + Sync>>;
            fn taxes_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn TaxesRepository + Send + Sync>>;
            fn users_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn TenantUsersRepository + Send + Sync>>;
            fn warehouses_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn WarehousesRepository + Send + Sync>>;
            fn worksheets_repo(
                &self,
                tenant_id: Uuid,
            ) -> RepositoryResult<Arc<dyn WorksheetsRepository + Send + Sync>>;
        }
    );
}
