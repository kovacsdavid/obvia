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

use crate::common::error::RepositoryError;
use crate::manager::common::repository::PoolManagerWrapper;
use crate::manager::common::types::value_object::ValueObjectable;
use crate::tenant::worksheets::dto::CreateWorksheet;
use crate::tenant::worksheets::model::Worksheet;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WorksheetsRepository: Send + Sync {
    async fn get_all(&self, active_tenant: Uuid) -> Result<Vec<Worksheet>, RepositoryError>;
    async fn insert(
        &self,
        worksheet: CreateWorksheet,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Worksheet, RepositoryError>;
}

#[async_trait]
impl WorksheetsRepository for PoolManagerWrapper {
    async fn get_all(&self, active_tenant: Uuid) -> Result<Vec<Worksheet>, RepositoryError> {
        Ok(sqlx::query_as::<_, Worksheet>(
            "SELECT * FROM worksheets WHERE deleted_at IS NULL ORDER BY name",
        )
        .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn insert(
        &self,
        worksheet: CreateWorksheet,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Worksheet, RepositoryError> {
        Ok(sqlx::query_as::<_, Worksheet>(
            "INSERT INTO worksheets (name, description, project_id, created_by, status)\
             VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(worksheet.name.extract().get_value())
        .bind(
            worksheet
                .description
                .map(|v| v.extract().get_value().clone()),
        )
        .bind(worksheet.project_id)
        .bind(sub)
        .bind(worksheet.status.extract().get_value())
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
}
