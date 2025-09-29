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

/// Represents a Tag entity in the system.
///
/// This struct is used to model a tag, which may be associated with other
/// entities in the application. Each tag has a unique identifier, a name,
/// and a timestamp indicating when it was created.
///
/// # Fields
/// - `id` (`Uuid`): The unique identifier for the tag.
/// - `name` (`String`): The name of the tag.
/// - `created_at` (`DateTime<Local>`): The timestamp representing when the tag was created.
///
/// # Traits
/// - `Debug`: Enables formatting of the struct for debugging purposes.
/// - `Clone`: Indicates that the struct can be cloned, creating a deep copy.
/// - `Serialize`: Allows the struct to be serialized into formats like JSON.
/// - `Deserialize`: Allows the struct to be deserialized from formats like JSON.
/// - `FromRow`: Enables extraction of the struct from a database query row.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TagResolved {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

/// Represents a connection between a tag and a taggable entity in the system.
///
/// The `TagConnect` struct is used to associate tags with different types of entities
/// (referred to as "taggable"). Each connection includes metadata such as the ID of the connection,
/// the associated taggable entity, and timestamps for creation and deletion.
///
/// # Fields
/// - `id` (`Uuid`): Unique identifier for this tag connection.
/// - `taggable_id` (`Uuid`): The unique identifier of the associated entity (taggable).
/// - `taggable_type` (`String`): The type of the taggable entity (e.g., "Post", "Comment").
/// - `tag_id` (`Option<Uuid>`): Optional identifier for the associated tag, if applicable.
/// - `created_at` (`DateTime<Local>`): Timestamp indicating when the tag connection was created.
/// - `deleted_at` (`Option<DateTime<Local>`): Optional timestamp indicating when the tag connection was deleted, if applicable.
///
/// # Derives
/// - `Debug`: Enables formatting of the struct using the `{:?}` formatter.
/// - `Clone`: Allows for creating an exact copy of a `TagConnect` instance.
/// - `Serialize`: Allows the `TagConnect` struct to be serialized into supported formats (e.g., JSON).
/// - `Deserialize`: Allows the `TagConnect` struct to be deserialized from supported formats (e.g., JSON).
/// - `FromRow`: Enables database row mapping to populate a `TagConnect` instance.
///
/// This struct is typically used in contexts where tags need to be associated
/// with entities for categorization, filtering, or other tagging-related operations.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TagConnect {
    pub id: Uuid,
    pub taggable_id: Uuid,
    pub taggable_type: String,
    pub tag_id: Option<Uuid>,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
