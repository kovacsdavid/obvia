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

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// A struct representing a user in the system.
///
/// This struct is used to store various details about a user, such as their
/// unique identifier, personal details, contact information, status, and metadata
/// regarding their account. It includes optional fields for information that may
/// not always be provided.
///
/// # Fields
/// - `id` (*Uuid*): Unique identifier for the user.
/// - `email` (*String*): Email address of the user.
/// - `first_name` (*Option<String>*): First name of the user. Optional.
/// - `last_name` (*Option<String>*): Last name of the user. Optional.
/// - `phone` (*Option<String>*): Phone number of the user. Optional.
/// - `status` (*String*): Current status of the user (e.g., active, inactive).
/// - `profile_picture_url` (*Option<String>*): URL of the user's profile picture. Optional.
/// - `locale` (*Option<String>*): Preferred locale or language of the user. Optional.
/// - `invited_by` (*Option<Uuid>*): UUID of the user who invited this user, if applicable. Optional.
/// - `email_verified_at` (*Option<DateTime<Local>>*): Timestamp of when the user's email was verified, if verified. Optional.
/// - `created_at` (*DateTime<Local>*): Timestamp of when the user record was created.
/// - `updated_at` (*DateTime<Local>*): Timestamp of the last update to the user record.
/// - `deleted_at` (*Option<DateTime<Local>>*): Timestamp of when the user was deleted, if applicable. Optional.
///
/// # Derives
/// This struct derives the following traits for enhanced functionality:
/// - `Debug`: Allows debugging information to be printed for instances of `User`.
/// - `Clone`: Enables instances of `User` to be cloned.
/// - `Serialize` and `Deserialize`: Provides support for serializing and deserializing instances
///   of `User` (e.g., converting to/from JSON).
/// - `FromRow`: Facilitates mapping database rows to instances of `User`.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub status: String,
    pub profile_picture_url: Option<String>,
    pub locale: Option<String>,
    pub invited_by: Option<Uuid>,
    pub email_verified_at: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
