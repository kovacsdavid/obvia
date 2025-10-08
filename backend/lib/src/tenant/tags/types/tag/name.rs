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
pub struct Name(pub String);

impl ValueObjectable for Name {
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

impl Display for Name {
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

impl<'de> Deserialize<'de> for ValueObject<Name> {
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
        ValueObject::new(Name(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::value_object::ValueObject;
    use serde_json;

    #[test]
    fn test_valid_name() {
        let name: ValueObject<Name> = serde_json::from_str(r#""Test Name""#).unwrap();
        assert_eq!(name.extract().get_value(), "Test Name");
    }

    #[test]
    fn test_invalid_name_empty() {
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_invalid_name_whitespace() {
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(r#"" ""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_display() {
        let name = Name("Test Name".to_string());
        assert_eq!(format!("{}", name), "Test Name");
    }

    #[test]
    fn test_validation() {
        let valid_name = Name("Valid Name".to_string());
        assert!(valid_name.validate().is_ok());

        let empty_name = Name("".to_string());
        assert!(empty_name.validate().is_err());

        let whitespace_name = Name(" ".to_string());
        assert!(whitespace_name.validate().is_err());
    }

    #[test]
    fn test_get_value() {
        let name = Name("Test Name".to_string());
        assert_eq!(name.get_value(), "Test Name");
    }

    #[test]
    fn test_deserialization_valid() {
        let json = r#""Test Name""#;
        let name: ValueObject<Name> = serde_json::from_str(json).unwrap();
        assert_eq!(name.extract().get_value(), "Test Name");
    }

    #[test]
    fn test_deserialization_invalid() {
        let json = r#""  ""#;
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(json);
        assert!(name.is_err());
    }
}
