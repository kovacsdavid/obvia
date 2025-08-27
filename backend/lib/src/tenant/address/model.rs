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

/// Represents an address entity with details about its location and associated metadata.
///
/// This struct is typically used for storing and managing address information in a database
/// or for transferring data across layers of an application.
///
/// # Fields
///
/// - `id` (`Uuid`): A unique identifier for the address.
/// - `street_address` (`String`): The primary address line (e.g., house number, street name).
/// - `city_id` (`Uuid`): A foreign key referencing the city's unique identifier.
/// - `state_id` (`Uuid`): A foreign key referencing the state's unique identifier.
/// - `country_id` (`Uuid`): A foreign key referencing the country's unique identifier.
/// - `additional_info` (`Option<String>`): Additional information about the address (e.g., apartment number, landmark).
/// - `created_at` (`DateTime<Local>`): The timestamp when the address record was created.
/// - `updated_at` (`DateTime<Local>`): The timestamp when the address record was last updated.
/// - `deleted_at` (`Option<DateTime<Local>`): The timestamp when the address record was deleted (if applicable).
///
/// # Traits
///
/// - `Debug`: Enables formatting of the struct for debugging purposes.
/// - `Clone`: Provides the ability to create duplicate instances of the struct.
/// - `Serialize`: Allows the struct to be serialized to formats like JSON.
/// - `Deserialize`: Allows the struct to be deserialized from formats like JSON.
/// - `FromRow`: Enables the struct to be used with database query results.
///
/// # Notes
///
/// - The `#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]` attribute automatically implements
///   several useful traits for the struct.
///
/// This struct is intended to integrate seamlessly with common database libraries and serialization frameworks.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Address {
    pub id: Uuid,
    pub street_address: String,
    pub city_id: Uuid,
    pub state_id: Uuid,
    pub country_id: Uuid,
    pub additional_info: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

///
/// The `Country` struct represents a country entity with associated metadata.
/// It is used to store and manipulate country-related information within the system.
///
/// # Attributes
///
/// * `id` (`Uuid`): A unique identifier for the country.
/// * `name` (`String`): The name of the country.
/// * `created_at` (`DateTime<Local>`): The timestamp indicating when the country record was created.
///
/// # Derives
///
/// * `Debug`: Allows the struct to be formatted using the `fmt::Debug` trait.
/// * `Clone`: Enables the struct to be cloned, creating copies of its values.
/// * `Serialize` and `Deserialize`: Enables the struct to be serialized and deserialized,
///   for example when working with JSON or other data formats.
/// * `FromRow`: Supports seamless mapping from database query results to the struct.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Country {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Local>,
}

/// Represents the `State` structure that holds information about a state.
///
/// The `State` struct is equipped with the following traits:
/// - `Debug`: Allows the structure to be formatted using the `{:?}` formatter.
/// - `Clone`: Enables deep cloning of the `State` instance.
/// - `Serialize` and `Deserialize`: Facilitates serialization and deserialization of the structure,
///   enabling easy data interchange (commonly used with formats like JSON).
/// - `FromRow`: Used to map database query results into this structure.
///
/// ## Fields
/// - `id` (`Uuid`): A universally unique identifier (UUID) that serves as the primary identifier
///   for the `State`.
/// - `name` (`String`): The name of the state.
/// - `created_at` (`DateTime<Local>`): The timestamp indicating when the state was created,
///   stored in the local timezone.
///
/// ## Attributes
///
/// - `#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]`: Automatically implements the
///   specified traits for the `State` struct.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct State {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Local>,
}

/// Represents a PostalCode entity in the system.
///
/// This structure is used to store information about a postal code along with its associated metadata.
///
/// # Attributes
///
/// * `id` - A unique identifier for the postal code (UUID).
/// * `postal_code` - A string representing the postal code value.
/// * `created_at` - A `DateTime` object representing the timestamp when the postal code record was created.
///
/// # Derives
///
/// * `Debug` - Enables formatting the `PostalCode` struct using the `{:?}` formatter.
/// * `Clone` - Allows the `PostalCode` struct to be cloned.
/// * `Serialize` - Provides the ability to serialize the `PostalCode` struct into a format such as JSON.
/// * `Deserialize` - Provides the ability to deserialize external data (like JSON) into a `PostalCode` struct.
/// * `FromRow` - Enables the struct to be constructed automatically from database query rows (commonly used with ORMs like SQLx).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PostalCode {
    pub id: Uuid,
    pub postal_code: String,
    pub created_at: DateTime<Local>,
}

/// Represents a City entity in the application.
///
/// The `City` struct is used to model and manage information
/// about a city, including its unique identifier, postal code,
/// name, and creation timestamp. This struct is compatible with
/// database operations and supports serialization and deserialization.
///
/// ## Fields
/// - `id` (`Uuid`): A unique identifier for the city.
/// - `postal_code` (`Uuid`): A unique identifier or code representing the postal region of the city.
/// - `name` (`String`): The name of the city.
/// - `created_at` (`DateTime<Local>`): The timestamp indicating when the city record was created.
///
/// ## Derive Attributes
/// - `Debug`: Enables formatting of `City` instances using the debug formatter.
/// - `Clone`: Allows for creating duplicate instances (deep copies) of `City`.
/// - `Serialize`: Makes the struct serializable, enabling conversion to formats like JSON.
/// - `Deserialize`: Allows deserialization of data (e.g., JSON) into a `City` struct.
/// - `FromRow`: Used for mapping database query results into an instance of `City`.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct City {
    pub id: Uuid,
    pub postal_code: Uuid,
    pub name: String,
    pub created_at: DateTime<Local>,
}

/// Represents a connection between an entity and an address in the system.
///
/// This struct stores the relationship details, including identifiers for the address
/// and the entity it is associated with, along with metadata such as timestamps for
/// creation, updates, and potential deletion.
///
/// # Fields
///
/// * `id` - A unique identifier for the `AddressConnect` record.
/// * `address_id` - A reference to the unique identifier of the related address.
/// * `addressable_id` - An optional identifier for the entity associated with the address.
///   This is typically used to connect the address to another resource, such as a user or organization.
/// * `addressable_type` - A string representing the type of the entity that the address is associated with,
///   for example, "User" or "Organization".
/// * `created_at` - An optional timestamp indicating when the `AddressConnect` record was created.
/// * `updated_at` - An optional timestamp indicating when the `AddressConnect` record was last updated.
/// * `deleted_at` - An optional timestamp indicating when the `AddressConnect` record was marked as deleted,
///   if applicable.
///
/// # Derives and Attributes
///
/// This struct derives several traits:
/// - `Debug`: Allows for formatting the struct into a human-readable string, useful for debugging.
/// - `Clone`: Provides the ability to create a duplicate of the struct.
/// - `Serialize` and `Deserialize`: Enables serialization and deserialization of the struct,
///   particularly for use with formats like JSON.
/// - `FromRow`: Allows the struct to be created directly from a database row.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AddressConnect {
    pub id: Uuid,
    pub address_id: Uuid,
    pub addressable_id: Option<i32>,
    pub addressable_type: String,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
}
