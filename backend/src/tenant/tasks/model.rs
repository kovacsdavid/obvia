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
use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub worksheet_id: Uuid,
    pub service_id: Uuid,
    pub currency_code: String,
    pub quantity: Option<BigDecimal>,
    pub price: Option<BigDecimal>,
    pub tax_id: Uuid,
    pub created_by_id: Uuid,
    pub status: String,
    pub priority: Option<String>,
    pub due_date: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskResolved {
    pub id: Uuid,
    pub worksheet_id: Uuid,
    pub worksheet: String,
    pub service_id: Uuid,
    pub service: String,
    pub currency_code: String,
    pub quantity: Option<BigDecimal>,
    pub price: Option<BigDecimal>,
    pub tax_id: Uuid,
    pub tax: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub status: String,
    pub priority: Option<String>,
    pub due_date: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
    pub description: Option<String>,
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
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskAssignment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
