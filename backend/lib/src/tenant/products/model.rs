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
use sqlx::types::BigDecimal;
use sqlx::FromRow;
use uuid::Uuid;


/// Represents a product entity in the system.
///
/// This struct is used to define the properties of a product, including
/// its identifiers, metadata, pricing details, and timestamps for tracking
/// creation, updates, and deletions.
///
/// ## Fields:
/// - `id` (`Uuid`): Unique identifier for the product.
/// - `name` (`String`): Name of the product.
/// - `description` (`Option<String>`): An optional description of the product.
/// - `unit_of_measure` (`Uuid`): Identifier for the unit of measure, referencing related measurement data.
/// - `price` (`Option<BigDecimal>`): Optional price of the product.
/// - `cost` (`Option<BigDecimal>`): Optional cost associated with the product.
/// - `currency_id` (`Uuid`): Identifier representing the currency used for product pricing.
/// - `is_active` (`Option<bool>`): Optional flag indicating if the product is active.
/// - `created_by` (`Uuid`): Identifier of the user who created the product.
/// - `created_at` (`Option<DateTime<Local>`): Optional timestamp indicating when the product was created.
/// - `updated_at` (`Option<DateTime<Local>`): Optional timestamp indicating when the product was last updated.
/// - `deleted_at` (`Option<DateTime<Local>`): Optional timestamp indicating when the product was deleted (if applicable).
///
/// ## Attributes:
/// - This struct is decorated with serialization, deserialization, and
///   database row mapping attributes:
///   - `#[derive(Debug)]`: Allows easy debugging output of struct instances.
///   - `#[derive(Clone)]`: Enables cloning of struct instances.
///   - `#[derive(Serialize)]`: Allows instance serialization (e.g., to JSON).
///   - `#[derive(Deserialize)]`: Enables deserialization (e.g., from JSON).
///   - `#[derive(FromRow)]`: Allows mapping of database rows to this struct.
///
/// ## Notes:
/// - Ensure the `Uuid`, `BigDecimal`, and `DateTime<Local>` types are compatible with external libraries such as `uuid` and `chrono`.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub unit_of_measure: Uuid,
    pub price: Option<BigDecimal>,
    pub cost: Option<BigDecimal>,
    pub currency_id: Uuid,
    pub is_active: Option<bool>,
    pub created_by: Uuid,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
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
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Currency {
    pub id: Uuid,
    pub currency: String,
    pub created_at: DateTime<Local>,
}

/// The `UnitOfMeasure` struct represents a unit of measurement for items, typically used
/// in inventory, logistics, or other systems that involve quantifiable units.
///
/// # Attributes
/// - `id` (`Uuid`): A unique identifier for the unit of measure.
/// - `unit_of_measure` (`String`): The name or description of the unit of measure, e.g., "kilogram",
///   "liter", or "piece".
/// - `created_at` (`DateTime<Local>`): A timestamp indicating when the unit of measure was created.
///
/// # Derives
/// - `Debug`: Automatically provides functionality to format the struct for debugging purposes.
/// - `Clone`: Allows for the struct to be cloned, creating an identical copy in memory.
/// - `Serialize`: Enables the struct to be serialized, typically to formats like JSON or XML.
/// - `Deserialize`: Enables the struct to be deserialized from serialized formats like JSON or XML.
/// - `FromRow`: A trait used by certain database ORM libraries (e.g., SQLx) to map database rows
///   to this struct.
///
/// # Attributes
/// - `#[allow(dead_code)]`: Suppresses compiler warnings if the struct or its fields are unused.
///
/// This struct is designed for extensibility and integration with database systems and
/// serialization mechanisms.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UnitOfMeasure {
    pub id: Uuid,
    pub unit_of_measure: String,
    pub created_at: DateTime<Local>,
}

/// Represents a product category within a system. Each category is uniquely identified
/// and may have a parent category, allowing hierarchical relationships.
///
/// # Fields
///
/// * `id` (`Uuid`): - Unique identifier for the product category.
/// * `name` (`String`): - The name of the category.
/// * `description` (`Option<String>`): - An optional description providing additional details about the category.
/// * `parent_id` (`Option<Uuid>`): - The unique identifier of the parent category, if this category is a subcategory. `None` if the category does not have a parent.
/// * `created_at` (`DateTime<Local>`): - The timestamp indicating when the category was created.
///
/// # Derives
///
/// * `Debug`: Enables formatting of the struct for debugging purposes.
/// * `Clone`: Allows the struct to be cloned.
/// * `Serialize`: Enables serialization of the struct to formats like JSON.
/// * `Deserialize`: Enables deserialization of the struct from formats like JSON.
/// * `FromRow`: Allows database rows to be mapped directly into this struct (e.g., with SQL queries).
///
/// # Attributes
///
/// * `#[allow(dead_code)]`:
///     - Suppresses compiler warnings for unused code related to this struct.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Local>,
}

/// Represents a connection between a product and a product category in the system.
///
/// This struct is designed to map the relationship between products and
/// their respective categories. It contains details about the identifiers
/// for the product and category, along with timestamps for when the connection
/// was created and (optionally) deleted.
///
/// # Fields
///
/// * `id` - A unique identifier for the connection.
/// * `product_id` - The unique identifier of the product.
/// * `product_category_id` - The unique identifier of the product category.
/// * `created_at` - The timestamp indicating when the connection was created.
/// * `deleted_at` - An optional timestamp indicating when the connection was deleted, if applicable.
///
/// # Derives
///
/// * `Debug` - Enables formatting and printing of the struct for debugging purposes.
/// * `Clone` - Allows creating a duplicate instance of the struct.
/// * `Serialize` - Enables serialization of the struct into formats such as JSON.
/// * `Deserialize` - Enables deserialization of the struct from formats such as JSON.
/// * `FromRow` - Facilitates mapping database rows to instances of this struct.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductCategoryConnect {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_category_id: Uuid,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}



