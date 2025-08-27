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

/// The `Comment` struct represents a comment entity and is used to store information related to a commentable resource.
/// It includes fields for associating the comment with another entity, storing the comment text, and managing its lifecycle.
///
/// # Attributes
///
/// * `id` (`Uuid`): A unique identifier for the comment.
/// * `commentable_type` (`String`): The type of the resource the comment is associated with (e.g., post, article, etc.).
/// * `commentable_id` (`Uuid`): The unique identifier of the associated resource.
/// * `comment` (`Option<String>`): The actual content of the comment, which can be optional.
/// * `created_at` (`DateTime<Local>`): The timestamp when the comment was created.
/// * `updated_at` (`DateTime<Local>`): The timestamp when the comment was last updated.
/// * `deleted_at` (`Option<DateTime<Local>>`): The timestamp when the comment was deleted, if applicable.
///
/// # Derives
///
/// * `Debug`: Enables formatting of the `Comment` struct using the `{:?}` formatter.
/// * `Clone`: Allows for creating a cloned instance of the `Comment` struct.
/// * `Serialize`: Enables serialization of the `Comment` struct, typically used for converting the struct into JSON or other formats.
/// * `Deserialize`: Enables deserialization of the `Comment` struct, allowing it to be reconstructed from formats like JSON.
/// * `FromRow`: Allows mapping database rows into an instance of the `Comment` struct, typically used with database query results.
///
/// This struct is primarily suited for use in scenarios involving comments associated with various types of resources in an application.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub commentable_type: String,
    pub commentable_id: Uuid,
    pub comment: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
