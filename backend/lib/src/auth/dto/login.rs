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

use crate::users::model::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: String,
    pub profile_picture_url: Option<String>,
}

impl From<User> for UserPublic {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            status: value.status,
            profile_picture_url: value.profile_picture_url,
        }
    }
}

#[derive(Serialize)]
pub struct LoginResponse {
    user: UserPublic,
    token: String,
}

impl LoginResponse {
    pub fn new(user: UserPublic, token: String) -> Self {
        Self { user, token }
    }
    #[allow(dead_code)]
    pub fn token(&self) -> &String {
        &self.token
    }
}
