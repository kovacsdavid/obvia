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
use crate::manager::app::database::{PgPoolManager, PoolManager};
use crate::manager::users::model::User;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn get_by_uuid(&self, uuid: Uuid) -> Result<User, RepositoryError>;
}

#[async_trait]
impl UsersRepository for PgPoolManager {
    async fn get_by_uuid(&self, uuid: Uuid) -> Result<User, RepositoryError> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL")
                .bind(uuid)
                .fetch_one(&self.get_main_pool())
                .await?,
        )
    }
}
