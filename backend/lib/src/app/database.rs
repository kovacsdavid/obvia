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

use crate::app::config::{
    BasicDatabaseConfig, DatabasePoolSizeProvider, DatabaseUrlProvider, TenantDatabaseConfig,
};
use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use uuid::Uuid;

/// Trait defining the behavior for managing PostgreSQL connection pools.
///
/// This trait provides a contract for interacting with different PostgreSQL
/// databases, including the main pool, the default tenant pool, and specific
/// tenant pools. It also allows adding new tenant-specific connection pools.
///
/// # Requirements
/// - The trait requires implementors to be both `Send` and `Sync` for concurrency support.
/// - The asynchronous method `add_tenant_pool` requires the use of the `#[async_trait]` macro.
///
/// # Methods
///
/// ## get_main_pool
/// Retrieves the primary PostgreSQL connection pool.
///
/// ### Returns
/// - `PgPool`: The connection pool associated with the main database.
///
/// ## get_default_tenant_pool
/// Retrieves the default tenant PostgreSQL connection pool, which provides access to the default
/// postgres instance where tenants are created
///
/// ### Returns
/// - `PgPool`: The connection pool for the default tenant database.
///
/// ## get_tenant_pool
/// Retrieves the PostgreSQL connection pool for the specified tenant.
///
/// ### Parameters
/// - `tenant_id: Uuid`: The unique identifier of the tenant whose connection
///   pool is being requested.
///
/// ### Returns
/// - `Result<Option<PgPool>>`: An optional tenant-specific connection pool if
///   it exists, wrapped in a `Result`. Errors are returned otherwise.
///
/// ## add_tenant_pool
/// Asynchronously adds a new PostgreSQL connection pool for a specific tenant.
///
/// ## remove_tenant_pool
/// Removes a tenant-specific connection pool from the system.
///
/// ### Parameters
/// - `tenant_id: Uuid`: The unique identifier of the tenant for which the
///   connection pool will be created.
/// - `config: &TenantDatabaseConfig`: Configuration details required to set up
///   the tenant's database.
///
/// ### Returns
/// - `Result<Uuid>`: The unique identifier of the tenant for whom the connection
///   pool was successfully added, wrapped in a `Result`. Errors are returned
///   otherwise.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait PgPoolManagerTrait: Send + Sync {
    /// Retrieves the main PostgreSQL connection pool.
    ///
    /// This function is defined as part of a trait or implemented in a struct and is used
    /// to provide access to the primary database connection pool. The connection pool
    /// (`PgPool`) is typically used to manage database connections efficiently in applications
    /// that interact with a PostgreSQL database.
    ///
    /// # Returns
    ///
    /// * `PgPool` - The primary PostgreSQL connection pool.
    fn get_main_pool(&self) -> PgPool;
    /// Retrieves the default tenant connection pool.
    ///
    /// This function is used to fetch the default PostgreSQL connection pool (`PgPool`)
    /// associated with the application or service. The connection pool allows
    /// efficient management and reuse of database connections for the default tenant.
    ///
    /// # Returns
    ///
    /// * `PgPool` - A configured instance of the PostgreSQL connection pool for the default tenant.
    fn get_default_tenant_pool(&self) -> PgPool;
    /// Retrieves the database connection pool associated with a specific tenant.
    ///
    /// This method fetches the tenant-specific connection pool using the provided tenant ID.
    /// It is used to manage access to database resources specific to a given tenant in a
    /// multi-tenant system. If a corresponding pool exists, it returns it wrapped in an `Option`.
    /// If no pool is found, it returns `Ok(None)`. In case of an internal error (e.g., database
    /// connectivity issues), it returns an `Err`.
    ///
    /// # Arguments
    ///
    /// * `tenant_id` - A `Uuid` representing the unique identifier of the tenant.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(PgPool))` - The tenant's database connection pool, if it exists.
    /// * `Ok(None)` - No connection pool exists for the given tenant ID.
    /// * `Err(E)` - An error occurred while attempting to retrieve the pool.
    fn get_tenant_pool(&self, tenant_id: Uuid) -> Result<Option<PgPool>>; // TODO: the return signature could be simplified
    /// Asynchronously adds a new tenant pool to the system with the specified tenant ID and configuration.
    ///
    /// # Arguments
    ///
    /// * `tenant_id` - A UUID that uniquely identifies the tenant for which the pool is being created.
    /// * `config` - A reference to a `TenantDatabaseConfig` struct containing the configuration settings for the tenant database.
    ///
    /// # Returns
    ///
    /// * `Result<Uuid>` - On success, returns the UUID of the newly created tenant pool.
    ///   On failure, returns an error wrapped in a `Result`.
    async fn add_tenant_pool(&self, tenant_id: Uuid, config: &TenantDatabaseConfig)
    -> Result<Uuid>;
}

/// `PgPoolManager` is a structure designed to manage multiple instances of PostgreSQL connection pools.
///
/// This structure provides centralized storage and access to different PostgreSQL connection pools for various use cases:
/// - A main pool for primary system-level operations.
/// - A default tenant pool for operations related to managing tenants in the default postgres instance.
/// - A collection of tenant-specific pools managed dynamically in a thread-safe manner.
///
/// # Fields
///
/// * `main_pool`:
///   A `PgPool` instance used as the main database connection pool for the application.
///   It typically handles global or system-wide database interactions.
///
/// * `default_tenant_pool`:
///   A `PgPool` instance dedicated to managing tenantss in the default postgres instance.
///
/// * `tenant_pools`:
///   An `Arc<RwLock<HashMap<String, PgPool>>>` that maintains a map of dynamically created
///   database connection pools for specific tenants. Each pool is identified by a unique string key.
///   This is wrapped in an `Arc` for shared ownership
///   and an `RwLock` to enable thread-safe read and write access.
pub struct PgPoolManager {
    main_pool: PgPool,
    default_tenant_pool: PgPool,
    tenant_pools: Arc<RwLock<HashMap<String, PgPool>>>,
}

impl PgPoolManager {
    /// Creates a new instance of `PgPoolManager` with the specified database configurations.
    ///
    /// This function initializes a primary connection pool (`main_pool`) for the main database
    /// and a default connection pool (`default_tenant_pool`)
    /// Both connection pools are configured based on their respective database configurations.
    ///
    /// # Arguments
    ///
    /// * `main_database_config` - A reference to the configuration for the main database,
    ///   which includes the database connection URL and pool size.
    /// * `default_tenant_database_config` - A reference to the configuration for the postgres instance
    ///   where the tenant data being stored by default. Main purpose of this pool is to manage
    ///   tenant creation and deletion.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that contains an instance of `PgPoolManager` on success, or an error
    /// if establishing the database connection pools fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if either the `main_pool` or `default_tenant_pool`
    /// connections fail to initialize.
    pub async fn new(
        main_database_config: &BasicDatabaseConfig,
        default_tenant_database_config: &BasicDatabaseConfig,
    ) -> Result<PgPoolManager> {
        let main_pool = PgPoolOptions::new()
            .max_connections(main_database_config.max_pool_size())
            .acquire_timeout(Duration::from_secs(3))
            .connect(&main_database_config.url())
            .await?;
        let default_tenant_pool = PgPoolOptions::new()
            .max_connections(default_tenant_database_config.max_pool_size())
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
    fn get_tenant_pool(&self, tenant_id: Uuid) -> Result<Option<PgPool>> {
        let guard = self
            .tenant_pools
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock on company pools"))?;
        Ok(guard.get(&tenant_id.to_string()).cloned())
    }
    async fn add_tenant_pool(
        &self,
        tenant_id: Uuid,
        config: &TenantDatabaseConfig,
    ) -> Result<Uuid> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_pool_size())
            .acquire_timeout(Duration::from_secs(3))
            .connect(&config.url())
            .await?;

        {
            let mut pools = self
                .tenant_pools
                .write()
                .map_err(|_| anyhow::anyhow!("Failed to acquire write lock on company pools"))?;
            pools.insert(tenant_id.to_string(), pool);
        }
        Ok(tenant_id)
    }
}
