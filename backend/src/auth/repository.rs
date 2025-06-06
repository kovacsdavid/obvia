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

use async_trait::async_trait;
use crate::common::error::DatabaseError;
use crate::auth::dto::register::RegisterRequest;
use crate::users::model::User;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuthRepository: Send + Sync + 'static {
    async fn insert_user(
        &self,
        payload: &RegisterRequest,
        password_hash: &str,
    ) -> Result<(), DatabaseError>;
    async fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError>;
}
