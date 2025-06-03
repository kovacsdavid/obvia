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

use crate::{auth::repository::AuthRepository, common::{error::DatabaseError, repository::PostgresRepo}};
use async_trait::async_trait;
use uuid::Uuid;

use super::{dto::RegisterRequest, model::User};

#[async_trait]
impl AuthRepository for PostgresRepo {
    async fn insert_user(
        &self,
        payload: &RegisterRequest,
        password_hash: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            "INSERT INTO users (
                    id, email, password_hash, first_name, last_name, phone, locale
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
            .bind(Uuid::new_v4())
            .bind(&payload.email)
            .bind(password_hash)
            .bind(&payload.first_name)
            .bind(&payload.last_name)
            .bind(&payload.phone)
            .bind(&payload.locale)
            .execute(&self.db)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError> {
        Ok(sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
            .bind(email)
            .fetch_one(&self.db)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?)
    }
}
