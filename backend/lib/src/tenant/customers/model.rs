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

/// Represents a `Customer` entity in the application.
///
/// This struct defines the model for the customer data
/// and is used to interact with database records, serialize/deserialize
/// JSON, and track customer details.
///
/// # Fields
/// - `id` (`Uuid`): The unique identifier for the customer.
/// - `name` (`String`): The name of the customer.
/// - `contact_name` (`Option<String>`): An optional field representing the name of the contact person for the customer.
/// - `email` (`String`): The email address of the customer.
/// - `phone_number` (`Option<String>`): An optional field representing the customer's phone number.
/// - `customer_type` (`Option<String>`): An optional field specifying the type of customer (e.g., individual, corporate).
/// - `status` (`Option<String>`): An optional field representing the customer's current status (e.g., active, inactive).
/// - `created_at` (`DateTime<Local>`): The timestamp indicating when the customer record was created.
/// - `updated_at` (`DateTime<Local>`): The timestamp indicating the last update to the customer record.
/// - `deleted_at` (`Option<DateTime<Local>`): An optional field to track when the customer record was deleted, if applicable.
///
/// # Derives
/// - `Debug`: Enables the struct to be formatted using the `{:?}` formatter.
/// - `Clone`: Allows the struct to be cloned.
/// - `Serialize` and `Deserialize`: Enables JSON serialization and deserialization of the struct.
/// - `FromRow`: Allows mapping query rows from a database into this struct.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub email: String,
    pub phone_number: Option<String>,
    pub status: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
