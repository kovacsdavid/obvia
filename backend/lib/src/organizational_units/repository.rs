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

use crate::common::error::DatabaseError;
use crate::common::repository::PoolWrapper;
use crate::organizational_units::dto::CreateRequest;
use crate::organizational_units::model::OrganizationalUnit;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait OrganizationalUnitsRepositoryTransactional: Send + Sync + 'static {
    #[allow(dead_code)]
    async fn get_by_uuid(&mut self, uuid: &str) -> Result<OrganizationalUnit, DatabaseError>;
    #[allow(dead_code)]
    async fn insert_and_connect(&mut self, payload: CreateRequest) -> Result<(), DatabaseError>;
    #[allow(dead_code)]
    async fn get_all_by_user_uuid(
        &mut self,
        user_uuid: &str,
    ) -> Result<Vec<OrganizationalUnit>, DatabaseError>;
}

#[async_trait]
impl OrganizationalUnitsRepositoryTransactional for PoolWrapper {
    async fn get_by_uuid(&mut self, uuid: &str) -> Result<OrganizationalUnit, DatabaseError> {
        Ok(sqlx::query_as::<_, OrganizationalUnit>(
            "SELECT * FROM organizational_units WHERE uuid = $1",
        )
        .bind(uuid)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?)
    }
    async fn insert_and_connect(&mut self, _payload: CreateRequest) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn get_all_by_user_uuid(
        &mut self,
        user_uuid: &str,
    ) -> Result<Vec<OrganizationalUnit>, DatabaseError> {
        sqlx::query_as::<_, OrganizationalUnit>(
            "SELECT * FROM organizational_units WHERE user_uuid = $1",
        )
        .bind(user_uuid)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))
    }
}
