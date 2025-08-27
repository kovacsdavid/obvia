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

/// Represents a worksheet entity with various properties.
///
/// This struct is meant to hold information about a worksheet, including metadata such as
/// creation and update timestamps, the associated project, and its status. It is designed
/// to be easily serializable and deserializable, making it suitable for database operations
/// or APIs.
///
/// # Fields
///
/// - `id` (`Uuid`): The unique identifier of the worksheet.
/// - `name` (`String`): The name of the worksheet.
/// - `description` (`Option<String>`): An optional field for an additional description of the worksheet. This field may contain `None` if no description is provided.
/// - `project_id` (`Uuid`): The unique identifier for the project to which this worksheet belongs.
/// - `created_by` (`Uuid`): The unique identifier of the user who created the worksheet.
/// - `status` (`String`): A string representing the current status of the worksheet (e.g., "active", "inactive").
/// - `created_at` (`DateTime<Local>`): The timestamp indicating when the worksheet was created, in the local timezone.
/// - `updated_at` (`DateTime<Local>`): The timestamp indicating when the worksheet was last updated, in the local timezone.
/// - `deleted_at` (`Option<DateTime<Local>`): An optional timestamp indicating when the worksheet was deleted. This value will be `None` if the worksheet has not been deleted.
///
/// # Derives
///
/// - `Debug`: Enables formatting for debugging purposes.
/// - `Clone`: Allows creating deep copies of a `Worksheet` instance.
/// - `Serialize`: Enables converting the struct into formats such as JSON.
/// - `Deserialize`: Enables converting formats such as JSON into the struct.
/// - `FromRow`: Allows the struct to be used in database rows, with fields
///   automatically mapped during queries.
///
/// This struct is commonly used in backend services to represent and manage worksheet-related
/// data within the context of a given project.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Worksheet {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub project_id: Uuid,
    pub created_by: Uuid,
    pub status: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
