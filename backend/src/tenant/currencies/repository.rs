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

use crate::common::error::RepositoryResult;
use crate::common::model::SelectOption;
use crate::common::repository::PoolManagerWrapper;
use crate::tenant::currencies::model::Currency;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CurrenciesRepository: Send + Sync {
    async fn get_all_countries(&self, active_tenant: Uuid) -> RepositoryResult<Vec<Currency>>;
    async fn get_all_countries_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>>;
}

#[async_trait]
impl CurrenciesRepository for PoolManagerWrapper {
    async fn get_all_countries(&self, active_tenant: Uuid) -> RepositoryResult<Vec<Currency>> {
        Ok(sqlx::query_as::<_, Currency>(r#"SELECT * FROM currencies"#)
            .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
            .await?)
    }

    async fn get_all_countries_select_list_items(
        &self,
        active_tenant: Uuid,
    ) -> RepositoryResult<Vec<SelectOption>> {
        Ok(sqlx::query_as::<_, SelectOption>(
            r#"SELECT code as value, code || ' - ' || name as title FROM currencies"#,
        )
        .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
}
