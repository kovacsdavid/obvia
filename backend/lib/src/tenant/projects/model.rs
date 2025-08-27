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

/// The `Project` struct represents a project entity with various metadata fields.
///
/// # Fields
/// - `id` (`Uuid`): A unique identifier for the project.
/// - `name` (`String`): The name of the project.
/// - `description` (`Option<String>`): An optional detailed description of the project.
/// - `created_by` (`Uuid`): The unique identifier of the user who created the project.
/// - `status` (`String`): The current status of the project (e.g., active, completed, etc.).
/// - `start_date` (`Option<DateTime<Local>`): An optional field representing the start date of the project in local time.
/// - `end_date` (`Option<DateTime<Local>`): An optional field representing the end date of the project in local time.
/// - `created_at` (`DateTime<Local>`): The timestamp indicating when the project was created (in local time).
/// - `updated_at` (`DateTime<Local>`): The timestamp indicating when the project was last updated (in local time).
/// - `deleted_at` (`Option<DateTime<Local>`): An optional timestamp indicating when the project was deleted (if it has been marked as deleted).
///
/// # Derives
/// This struct derives the following traits:
/// - `Debug`: Enables printing the struct using the `Debug` format.
/// - `Clone`: Allows cloning of the struct.
/// - `Serialize`: Allows serialization of the struct into formats such as JSON.
/// - `Deserialize`: Allows deserialization into the struct from formats such as JSON.
/// - `FromRow`: Allows mapping database rows directly into the struct.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub status: String,
    pub start_date: Option<DateTime<Local>>,
    pub end_date: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

/// Represents the assignment of a user to a project within the system.
///
/// This struct is used to track which user is assigned to which project, along with metadata
/// about the creation and deletion of the assignment.
///
/// # Fields
/// - `id` (`Uuid`): The unique identifier for the project assignment.
/// - `user_id` (`Uuid`): The unique identifier of the user assigned to the project.
/// - `project_id` (`Uuid`): The unique identifier of the project to which the user is assigned.
/// - `created_by` (`Uuid`): The unique identifier of the user who created the assignment.
/// - `created_at` (`DateTime<Local>`): The timestamp indicating when the assignment was created.
/// - `deleted_at` (`Option<DateTime<Local>>`): An optional timestamp indicating when the assignment
///   was deleted. If `None`, the assignment is still active.
///
/// # Derives
/// This struct derives the following traits for serialization, deserialization, and other functionalities:
/// - `Debug`: Allows printing and debugging of the struct.
/// - `Clone`: Enables creating a duplicate of the struct.
/// - `Serialize` and `Deserialize`: For serializing and deserializing the struct, e.g., to/from JSON.
/// - `FromRow`: Allows mapping the struct from a database row.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectAssignment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub created_by: Uuid,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
