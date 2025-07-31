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
use crate::app::database::{PgPoolManager, PgPoolManagerTrait};
use crate::common::repository::PoolWrapper;
use crate::organizational_units::repository::OrganizationalUnitsRepository;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{error, info};

pub async fn migrate_main_db(pg_pool_manager: Arc<PgPoolManager>) -> anyhow::Result<()> {
    Ok(sqlx::migrate!("../migrations/main")
        .run(&pg_pool_manager.get_main_pool())
        .await?)
}

pub async fn migrate_all_tenant_dbs(pg_pool_manager: Arc<PgPoolManager>) -> anyhow::Result<()> {
    let repo = PoolWrapper::new(pg_pool_manager.get_main_pool());
    let organizational_units =
        <PoolWrapper as OrganizationalUnitsRepository>::get_all(&repo).await?;
    for organizational_unit in organizational_units {
        if let Some(tenant_pool) = pg_pool_manager.get_tenant_pool(organizational_unit.id)? {
            match migrate_tenant_db(&tenant_pool).await {
                Ok(_) => info!(
                    "Tenant database migration successful: {}",
                    &organizational_unit.id
                ),
                Err(e) => error!("Tenant database migration failed: {}", e),
            }
        }
    }
    Ok(())
}

pub async fn migrate_tenant_db(tenant_pool: &PgPool) -> anyhow::Result<()> {
    Ok(sqlx::migrate!("../migrations/tenant")
        .run(tenant_pool)
        .await?)
}

pub async fn init_tenant_pools(pg_pool_manager: Arc<PgPoolManager>) -> anyhow::Result<()> {
    let repo = PoolWrapper::new(pg_pool_manager.get_main_pool());
    let organizational_units =
        <PoolWrapper as OrganizationalUnitsRepository>::get_all(&repo).await?;
    for organizational_unit in organizational_units {
        match pg_pool_manager
            .add_tenant_pool(organizational_unit.id, &organizational_unit.into())
            .await
        {
            Ok(organizational_unit_id) => info!(
                "Tenant pool initialization is successful: {}",
                &organizational_unit_id
            ),
            Err(e) => error!("Tenant pool initialization failed: {}", e),
        }
    }
    Ok(())
}
