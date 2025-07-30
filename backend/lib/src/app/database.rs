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

use crate::app::config::{DefaultTenantDatabaseConfig, MainDatabaseConfig, TenantDatabaseConfig};
use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PgPoolManagerTrait: Send + Sync {
    fn get_main_pool(&self) -> PgPool;
    fn get_default_tenant_pool(&self) -> PgPool;
    fn get_tenant_pool(&self, company_id: &str) -> Result<Option<PgPool>>;
    async fn add_tenant_pool(
        &self,
        company_id: String,
        config: &TenantDatabaseConfig,
    ) -> Result<()>;
}

pub struct PgPoolManager {
    main_pool: PgPool,
    default_tenant_pool: PgPool,
    tenant_pools: Arc<RwLock<HashMap<String, PgPool>>>,
}

impl PgPoolManager {
    pub async fn new(
        main_database_config: &MainDatabaseConfig,
        default_tenant_database_config: &DefaultTenantDatabaseConfig,
    ) -> Result<PgPoolManager> {
        let main_pool = PgPoolOptions::new()
            .max_connections(main_database_config.pool_size)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&main_database_config.url())
            .await?;
        let default_tenant_pool = PgPoolOptions::new()
            .max_connections(default_tenant_database_config.pool_size)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&default_tenant_database_config.url())
            .await?;
        Ok(Self {
            main_pool,
            default_tenant_pool,
            tenant_pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}

#[async_trait]
impl PgPoolManagerTrait for PgPoolManager {
    fn get_main_pool(&self) -> PgPool {
        self.main_pool.clone()
    }
    fn get_default_tenant_pool(&self) -> PgPool {
        self.default_tenant_pool.clone()
    }
    fn get_tenant_pool(&self, company_id: &str) -> Result<Option<PgPool>> {
        let guard = self
            .tenant_pools
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock on company pools"))?;
        Ok(guard.get(company_id).cloned())
    }
    async fn add_tenant_pool(
        &self,
        company_id: String,
        config: &TenantDatabaseConfig,
    ) -> Result<()> {
        let pool = PgPoolOptions::new()
            .max_connections(config.pool_size)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&config.url())
            .await?;

        {
            let mut pools = self
                .tenant_pools
                .write()
                .map_err(|_| anyhow::anyhow!("Failed to acquire write lock on company pools"))?;
            pools.insert(company_id, pool);
        }
        Ok(())
    }
}
