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

/// Struct `Warehouse` represents a data model for storing information about a warehouse.
///
/// This struct is commonly used in database interactions (e.g., querying or inserting warehouse records)
/// and is compatible with serialization and deserialization for use in APIs or other data handling contexts.
///
/// # Attributes:
///
/// * `id` (Uuid): Unique identifier of the warehouse.
/// * `name` (String): Name of the warehouse.
/// * `contact_name` (Option<String>): Optional name of the contact person for the warehouse.
/// * `contact_phone` (Option<String>): Optional phone number of the contact person for the warehouse.
/// * `is_active` (Option<bool>): Indicates whether the warehouse is active. By default, this field is optional.
/// * `created_by` (Uuid): Unique identifier of the user or system that created the warehouse record.
/// * `created_at` (DateTime<Local>): Timestamp when the warehouse record was created.
/// * `updated_at` (DateTime<Local>): Timestamp of the last update to the warehouse record.
/// * `deleted_at` (Option<DateTime<Local>): Optional timestamp that indicates when the warehouse was deleted. If `None`, the warehouse is not deleted.
///
/// # Traits:
///
/// * `Debug`: Provides debugging capabilities for the struct.
/// * `Clone`: Allows cloning of the struct.
/// * `Serialize`, `Deserialize`: Enables easy serialization and deserialization for use in APIs or data persistence.
/// * `FromRow`: Allows parsing results from database rows into a `Warehouse` struct.
///
/// # Usage:
///
/// This struct is typically used in scenarios where warehouses are being managed, such as inventory systems,
/// logistics platforms, or other business applications that require the modeling of warehouse data.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Warehouse {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WarehouseResolved {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
