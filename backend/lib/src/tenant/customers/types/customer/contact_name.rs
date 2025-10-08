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

use crate::common::types::value_object::{ValueObject, ValueObjectable};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ContactName(pub String);

impl ValueObjectable for ContactName {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if !self.0.trim().is_empty() {
            Ok(())
        } else {
            Err(String::from("A mező kitöltése kötelező"))
        }
    }

    /// Retrieves a reference to the value contained within the struct.
    ///
    /// # Returns
    /// A reference to the internal value of type `Self::DataType`.
    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for ContactName {
    /// Implements the `fmt` method from the `std::fmt::Display` or `std::fmt::Debug` trait,
    /// enabling a custom display of the struct or type.
    ///
    /// # Parameters
    /// - `&self`: A reference to the instance of the type implementing this method.
    /// - `f`: A mutable reference to a `std::fmt::Formatter` used for formatting output.
    ///
    /// # Returns
    /// - `std::fmt::Result`: Indicates whether the formatting operation was successful
    ///   (`Ok(())`) or an error occurred (`Err`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<ContactName> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `Name` and validates it by calling `ValueObject::new`.
    /// If the validation fails, a custom deserialization error is returned.
    ///
    /// # Type Parameters
    /// - `D`: The type of the deserializer, which must implement `serde::Deserializer<'de>`.
    ///
    /// # Parameters
    /// - `deserializer`: The deserializer used to deserialize the input.
    ///
    /// # Returns
    /// - `Result<Self, D::Error>`:
    ///   - On success, returns the constructed and validated object wrapped in `Ok`.
    ///   - On failure, returns a custom error wrapped in `Err`.
    ///
    /// # Errors
    /// - Returns a deserialization error if:
    ///   - The input cannot be deserialized into a `String`.
    ///   - Validation using `ValueObject::new` fails, causing the `map_err` call to propagate an error.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(ContactName(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_contact_name() {
        let name: ValueObject<ContactName> = serde_json::from_str(r#""John Doe""#).unwrap();
        assert_eq!(name.extract().get_value(), "John Doe");
    }

    #[test]
    fn test_empty_contact_name() {
        let name: Result<ValueObject<ContactName>, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_whitespace_contact_name() {
        let name: Result<ValueObject<ContactName>, _> = serde_json::from_str(r#"" "#);
        assert!(name.is_err());
    }

    #[test]
    fn test_contact_name_display() {
        let name = ContactName("John Doe".to_string());
        assert_eq!(format!("{}", name), "John Doe");
    }

    #[test]
    fn test_contact_name_validation() {
        let name = ContactName("John Doe".to_string());
        assert!(name.validate().is_ok());

        let empty_name = ContactName("".to_string());
        assert!(empty_name.validate().is_err());

        let whitespace_name = ContactName(" ".to_string());
        assert!(whitespace_name.validate().is_err());
    }

    #[test]
    fn test_contact_name_get_value() {
        let name = ContactName("John Doe".to_string());
        assert_eq!(name.get_value(), "John Doe");
    }

    #[test]
    fn test_contact_name_debug() {
        let name = ContactName("John Doe".to_string());
        assert_eq!(format!("{:?}", name), "ContactName(\"John Doe\")");
    }

    #[test]
    fn test_contact_name_clone() {
        let name = ContactName("John Doe".to_string());
        let cloned = name.clone();
        assert_eq!(name, cloned);
    }

    #[test]
    fn test_contact_name_partial_eq() {
        let name1 = ContactName("John Doe".to_string());
        let name2 = ContactName("John Doe".to_string());
        let name3 = ContactName("Jane Doe".to_string());

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_contact_name_serialize() {
        let name = ContactName("John Doe".to_string());
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, r#""John Doe""#);
    }
}
