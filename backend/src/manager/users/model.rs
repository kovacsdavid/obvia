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

use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

/// Represents a user entity with various attributes typically associated with user models.
///
/// # Attributes
/// - `id` (`Uuid`): Unique identifier for the user.
/// - `email` (`String`): The email address of the user.
/// - `password_hash` (`String`): The hashed value of the user's password.
/// - `first_name` (`Option<String>`): The first name of the user. Optional.
/// - `last_name` (`Option<String>`): The last name of the user. Optional.
/// - `phone` (`Option<String>`): The phone number of the user. Optional.
/// - `status` (`String`): The current status of the user (e.g., active, inactive).
/// - `last_login_at` (`Option<chrono::DateTime<chrono::Utc>>`): The timestamp of the user's most recent login. Optional.
/// - `profile_picture_url` (`Option<String>`): URL to the user's profile picture. Optional.
/// - `locale` (`Option<String>`): The locale (language/culture preference) of the user. Optional.
/// - `invited_by` (`Option<Uuid>`): The ID of the user who invited this user, if applicable. Optional.
/// - `email_verified_at` (`Option<chrono::DateTime<chrono::Utc>>`): The timestamp of when the user's email was verified. Optional.
/// - `created_at` (`chrono::DateTime<chrono::Utc>`): The timestamp of when the user account was created.
/// - `updated_at` (`chrono::DateTime<chrono::Utc>`): The timestamp of the last update to the user's record.
/// - `deleted_at` (`Option<chrono::DateTime<chrono::Utc>>`): The timestamp of when the user's account was deleted, if applicable. Optional.
#[derive(Serialize, FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub status: String,
    pub last_login_at: Option<chrono::DateTime<chrono::Local>>,
    pub profile_picture_url: Option<String>,
    pub locale: Option<String>,
    pub invited_by: Option<Uuid>,
    pub email_verified_at: Option<chrono::DateTime<chrono::Local>>,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
    pub deleted_at: Option<chrono::DateTime<chrono::Local>>,
}

impl User {
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }
}
