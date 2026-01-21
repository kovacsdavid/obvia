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

use crate::common::error::{RepositoryError, RepositoryResult};
use crate::common::types::value_object::ValueObjectable;
use crate::manager::app::database::{PgPoolManager, PoolManager};
use crate::manager::auth::dto::register::RegisterRequest;
use crate::manager::users::model::User;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn get_by_uuid(&self, uuid: Uuid) -> Result<User, RepositoryError>;
    async fn insert_user(
        &self,
        payload: &RegisterRequest,
        password_hash: &str,
    ) -> RepositoryResult<User>;
    async fn get_user_by_email(&self, email: &str) -> RepositoryResult<User>;
    async fn get_user_by_id(&self, user_id: Uuid) -> RepositoryResult<User>;
    async fn update_user(&self, user: User) -> RepositoryResult<User>;
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
    async fn insert_user(
        &self,
        payload: &RegisterRequest,
        password_hash: &str,
    ) -> RepositoryResult<User> {
        Ok(sqlx::query_as::<_, User>(
            "INSERT INTO users (
                    id, email, password_hash, first_name, last_name, status
            ) VALUES ($1, $2, $3, $4, $5, 'unchecked_email') RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(payload.email.extract().get_value())
        .bind(password_hash)
        .bind(payload.first_name.extract().get_value())
        .bind(payload.last_name.extract().get_value())
        .fetch_one(&self.get_main_pool())
        .await?)
    }

    async fn get_user_by_email(&self, email: &str) -> RepositoryResult<User> {
        Ok(
            sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL",
            )
            .bind(email)
            .fetch_one(&self.get_main_pool())
            .await?,
        )
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> RepositoryResult<User> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL")
                .bind(user_id)
                .fetch_one(&self.get_main_pool())
                .await?,
        )
    }

    async fn update_user(&self, user: User) -> RepositoryResult<User> {
        Ok(sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET email = $1,
                password_hash = $2,
                first_name = $3,
                last_name = $4,
                phone = $5,
                status = $6,
                last_login_at = $7,
                profile_picture_url = $8,
                locale = $9,
                invited_by = $10,
                email_verified_at = $11,
                is_mfa_enabled = $12,
                mfa_secret = $13
            WHERE id = $14
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(user.email)
        .bind(user.password_hash)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.phone)
        .bind(user.status)
        .bind(user.last_login_at)
        .bind(user.profile_picture_url)
        .bind(user.locale)
        .bind(user.invited_by)
        .bind(user.email_verified_at)
        .bind(user.is_mfa_enabled)
        .bind(user.mfa_secret)
        .bind(user.id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
}
