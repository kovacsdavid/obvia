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

/// The `Inventory` struct represents the inventory record for a product in a specific warehouse.
///
/// # Fields
///
/// * `id` - A unique identifier for the inventory record, represented as a `Uuid`.
/// * `product_id` - The unique identifier of the product associated with this inventory, represented as a `Uuid`.
/// * `warehouse_id` - The unique identifier of the warehouse where the inventory is located, represented as a `Uuid`.
/// * `quantity` - The quantity of the product available in the inventory, represented as an `i32`.
/// * `created_by` - The unique identifier of the user who created this inventory record, represented as a `Uuid`.
/// * `created_at` - The timestamp indicating when the inventory record was created, represented as a `DateTime<Local>`.
/// * `updated_at` - The timestamp indicating when the inventory record was last updated, represented as a `DateTime<Local>`.
/// * `deleted_at` - An optional timestamp indicating when the inventory record was deleted, represented as an `Option<DateTime<Local>>`.
///
/// # Traits
///
/// The `Inventory` struct derives the following traits:
///
/// * `Debug` - Allows the struct to be formatted using the `{:?}` formatter.
/// * `Clone` - Enables the struct to be cloned.
/// * `Serialize` - Enables the struct to be serialized, typically for use with formats such as JSON.
/// * `Deserialize` - Enables the struct to be deserialized, typically from formats such as JSON.
/// * `FromRow` - Allows the struct to be constructed from a database row, typically when using ORM libraries.
///
/// # Usage
///
/// This struct is typically used to track and manage inventory records in an application
/// that deals with warehouse and product management. It supports serialization and
/// deserialization to facilitate data storage and retrieval in various formats.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inventory {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: i32,
    pub price: Option<BigDecimal>,
    pub cost: Option<BigDecimal>,
    pub currency_id: Uuid,
    pub created_by: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResolvedInventory {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product: String,
    pub warehouse_id: Uuid,
    pub warehouse: String,
    pub quantity: i32,
    pub price: Option<BigDecimal>,
    pub cost: Option<BigDecimal>,
    pub currency_id: Uuid,
    pub currency: String,
    pub created_by: Uuid,
    pub created_by_resolved: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

/// A struct representing a currency entity.
///
/// The `Currency` struct is used to store and manipulate information about a currency, including
/// a unique identifier, the currency name, and the timestamp when it was created.
///
/// This structure derives several traits to enhance its functionality:
/// - `Debug`: Allows for formatting the struct using the `{:?}` formatter.
/// - `Clone`: Enables creating deep copies of `Currency` instances.
/// - `Serialize` and `Deserialize`: Provides serialization and deserialization support, typically for JSON or other formats.
/// - `FromRow`: Facilitates conversion from a database row to a `Currency` instance, commonly used in database operations.
///
/// Fields:
/// - `id` (`Uuid`): A unique identifier for the currency. It is represented as a UUID.
/// - `currency` (`String`): The name or code of the currency (e.g., "USD", "EUR").
/// - `created_at` (`DateTime<Local>`): The timestamp indicating when the currency entry was created.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Currency {
    pub id: Uuid,
    pub currency: String,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
