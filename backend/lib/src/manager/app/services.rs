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
use crate::common::repository::PoolManagerWrapper;
use crate::manager::app::config::BasicDatabaseConfig;
use crate::manager::app::database::{DatabaseMigrator, PgDatabaseMigrator, PgPoolManagerTrait};
use crate::manager::tenants::repository::TenantsRepository;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{error, info};

/// Executes the database migration for the main PostgreSQL database pool.
///
/// This function applies all pending migrations to the main database,
/// using the migration scripts located in the `../migrations/main` directory.
///
/// # Arguments
///
/// * `pg_pool_manager` - An `Arc` wrapped instance of `PgPoolManager` which provides access to the main database pool.
///
/// # Returns
///
/// * `Ok(())` - If the migration process completes successfully.
/// * `Err(anyhow::Error)` - If any error occurs during the migration process,
///   such as issues with the database connection or syntax errors in the migration scripts.
///
/// # Notes
///
/// * Ensure that the `../migrations/main` directory exists and contains valid migration scripts.
/// * Requires the `sqlx` runtime and feature flags for PostgreSQL.
pub async fn migrate_main_db(pg_pool_manager: Arc<dyn PgPoolManagerTrait>) -> anyhow::Result<()> {
    Ok(sqlx::migrate!("../migrations/main")
        .run(&pg_pool_manager.get_main_pool())
        .await?)
}

/// Migrates all tenant databases
///
/// This function retrieves a list of all tenants from the main database,
/// gets the corresponding tenant database pool for each tenant, and
/// attempts to run a database migration for the tenant database. It logs the result
/// of the migration for each tenant.
///
/// # Arguments
///
/// * `pg_pool_manager` - An `Arc`-wrapped `PgPoolManager` instance that provides access
///   to the main database pool and tenant-specific database pools.
///
/// # Returns
///
/// Returns `Ok(())` if tenant database migrations have completed. Returns an `Err`
/// if there's an issue retrieving the tenants or establishing a tenant-specific
/// database connection. This function will not terminate early if there are error during the
/// migration process for any tenant.
///
/// # Errors
///
/// This function will return an error if:
/// - It fails to retrieve all tenants from the main database.
///
/// Individual tenant database migration errors will not cause the function to terminate early.
/// Instead, they will be logged as errors, and migration will continue for all other tenants.
///
/// # Logging
///
/// This function uses the logging framework to log the following events:
/// - Logs an informational message for each successful tenant database migration.
/// - Logs an error message for any tenant database migration failure.
pub async fn migrate_all_tenant_dbs(
    pg_pool_manager: Arc<dyn PgPoolManagerTrait>,
) -> anyhow::Result<()> {
    let repo = PoolManagerWrapper::new(pg_pool_manager.clone());
    let tenants = <PoolManagerWrapper as TenantsRepository>::get_all(&repo).await?;
    for tenant in tenants {
        //TODO: This function should continue execution if there's an error retrieving the tenant pool
        let tenant_pool = pg_pool_manager.get_tenant_pool(tenant.id)?;
        match migrate_tenant_db(&tenant_pool).await {
            Ok(_) => info!("Tenant database migration successful: {}", &tenant.id),
            // TODO: Notify the administrator about the failed migration
            Err(e) => error!("Tenant database migration failed: {}", e),
        }
    }
    Ok(())
}

/// Runs database migrations for a tenant-specific database pool.
///
/// This function applies all pending migrations located in the `../migrations/tenant`
/// directory to the database associated with the provided `PgPool`. It ensures that
/// the tenant database schema is up-to-date with the defined migrations.
///
/// # Arguments
///
/// * `tenant_pool` - A reference to the `PgPool` instance representing the tenant's database connection pool.
///
/// # Returns
///
/// * `Ok(())` if the migrations were successfully applied.
/// * `Err(anyhow::Error)` if an error occurred during the migration process.
///
/// # Errors
///
/// This function will return an error if:
/// * The `tenant_pool` connection is not valid or accessible.
/// * There is an issue reading the migration files.
/// * The migration process encounters a failure (e.g., invalid SQL or database constraints).
///
/// # Notes
///
/// Ensure that the `../migrations/tenant` directory contains all the necessary
/// migration files in the correct format expected by `sqlx`.
pub async fn migrate_tenant_db(tenant_pool: &PgPool) -> anyhow::Result<()> {
    Ok(PgDatabaseMigrator.migrate_tenant_db(tenant_pool).await?)
}

/// Initializes tenant-specific database connection pools.
///
/// This function retrieves all tenants from the primary database pool
/// and creates individual tenant connection pools for each of them.
///
/// # Arguments
/// * `pg_pool_manager` - A thread-safe reference-counted `PgPoolManager` instance
///   used to manage database connection pools.
///
/// # Workflow
/// 1. Retrieves the primary database pool using the `PgPoolManager`.
/// 2. Fetches all tenants from the primary pool using the `TenantsRepository`.
/// 3. Iterates over each tenant and attempts to create a new connection pool
///    specific to that tenant using its ID and configuration.
/// 4. Logs the success or failure of each tenant pool initialization.
///
/// # Errors
/// Returns an `anyhow::Error` if:
/// - There's an issue fetching the tenants.
/// - Creating tenant-specific pools fails for any of the tenants (logged individually).
pub async fn init_tenant_pools(pg_pool_manager: Arc<dyn PgPoolManagerTrait>) -> anyhow::Result<()> {
    let repo = PoolManagerWrapper::new(pg_pool_manager.clone());
    let tenants = <PoolManagerWrapper as TenantsRepository>::get_all(&repo).await?;
    for tenant in tenants {
        match BasicDatabaseConfig::try_from(&tenant) {
            Ok(db_config) => {
                match pg_pool_manager.add_tenant_pool(tenant.id, &db_config).await {
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
