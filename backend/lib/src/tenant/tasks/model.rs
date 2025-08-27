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

/// Represents a task within the system.
///
/// This structure is used to model a task with associated metadata
/// and is designed to be serializable, deserializable, and compatible with database rows.
///
/// ## Fields
/// - `id` (*Uuid*): Unique identifier of the task.
/// - `worksheet_id` (*Uuid*): Identifier of the worksheet to which this task belongs.
/// - `title` (*String*): Title or name of the task.
/// - `description` (*Option<String>*): Optional detailed information or description of the task.
/// - `created_by` (*Uuid*): Identifier of the user who created the task.
/// - `status` (*String*): Current status of the task (e.g., "pending", "completed").
/// - `priority` (*Option<i32>*): Optional priority value of the task. Higher values may indicate higher priority.
/// - `due_date` (*Option<DateTime<Local>>*): Optional due date and time for the task completion in local time.
/// - `created_at` (*DateTime<Local>*): Timestamp when the task was created, in local time.
/// - `updated_at` (*DateTime<Local>*): Timestamp when the task was last updated, in local time.
/// - `deleted_at` (*Option<DateTime<Local>>*): Optional timestamp indicating when the task was deleted, in local time.
///
/// ## Traits
/// This struct derives the following traits:
/// - `Debug`: For formatting the task for debugging purposes.
/// - `Clone`: To allow cloning the `Task` struct.
/// - `Serialize`/`Deserialize`: For converting the struct to and from formats such as JSON.
/// - `FromRow`: To map database rows to a `Task` struct using SQLx.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub worksheet_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub status: String,
    pub priority: Option<i32>,
    pub due_date: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

/// Represents the association of a task assigned to a user, along with metadata about its creation and potential deletion.
///
/// ## Fields
///
/// - `id`: Unique identifier for the task assignment.
/// - `user_id`: Identifier of the user to whom the task is assigned.
/// - `task_id`: Identifier of the task that is assigned.
/// - `created_by`: Identifier of the user or entity that created this task assignment.
/// - `created_at`: The timestamp of when the task assignment was created.
/// - `deleted_at`: Optional timestamp for when the task assignment was deleted. If `None`, the task assignment has not been deleted.
///
/// ## Traits
///
/// - `#[derive(Debug)]`: Allows formatting for debugging purposes.
/// - `#[derive(Clone)]`: Provides the ability to clone instances of the struct.
/// - `#[derive(Serialize, Deserialize)]`: Enables serialization and deserialization for interaction with external systems or formats.
/// - `#[derive(FromRow)]`: Allows mapping of database query results to this struct.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskAssignment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_id: Uuid,
    pub created_by: Uuid,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

