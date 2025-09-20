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
use crate::tenant::users::model::User;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn insert_from_manager(
        &self,
        user: User,
        active_tenant: Uuid,
    ) -> Result<User, RepositoryError>;
}

#[async_trait]
impl UsersRepository for PoolManagerWrapper {
    async fn insert_from_manager(
        &self,
        user: User,
        active_tenant: Uuid,
    ) -> Result<User, RepositoryError> {
        Ok(sqlx::query_as::<_, User>(
            "INSERT INTO users (
                    id,
                    email,
                    first_name,
                    last_name,
                    phone,
                    status,
                    profile_picture_url,
                    locale,
                    invited_by,
                    email_verified_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *",
        )
        .bind(user.id)
        .bind(user.email)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.phone)
        .bind(user.status)
        .bind(user.profile_picture_url)
        .bind(user.locale)
        .bind(user.invited_by)
        .bind(user.email_verified_at)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
}
