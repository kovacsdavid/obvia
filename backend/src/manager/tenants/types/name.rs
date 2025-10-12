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

/// Represents a tenant name.
///
/// The `Name` struct is a simple wrapper around a `String` that is used
/// to encapsulate the tenant name. This can
/// help ensure type safety and improve code readability by explicitly
/// conveying the purpose of the contained value.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Name(pub String);

impl ValueObjectable for Name {
    type DataType = String;

    /// Validates the string contained within the `self` object using a regular expression.
    ///
    /// # Behavior
    /// - The method checks if the string matches the regular expression `^[A-Za-z0-9]{1,255}$`.
    /// - The regular expression ensures that:
    ///   - The string contains only alphanumeric characters (uppercase A-Z, lowercase a-z, digits 0-9).
    ///   - The string's length is between 1 and 255 characters (inclusive).
    /// - If the string matches the criteria, the function returns `Ok(())`.
    /// - If the string does not match the criteria, the function returns an error with the message `"Hibás név!"` (translated as "Invalid name!").
    ///
    /// # Returns
    /// - `Ok(())` if the string is valid.
    /// - `Err(String)` with the message `"Hibás név!"` if the string is invalid.
    ///
    /// # Errors
    /// - If the `Regex::new` function fails to compile the regular expression, the method returns `Err(String)` with the message `"Hibás név!"`.
    fn validate(&self) -> Result<(), String> {
        let trimmed = self.0.trim();
        if !trimmed.is_empty() && trimmed.len() < 255 {
            Ok(())
        } else {
            Err("Hibás név!".to_string())
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
    use serde_json;

    #[test]
    fn test_valid_tenant_name() {
        let name: ValueObject<Name> = serde_json::from_str(r#""ValidTenant123""#).unwrap();
        assert_eq!(name.extract().get_value(), "ValidTenant123");
    }

    #[test]
    fn test_empty_tenant_name() {
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_whitespace_only_name() {
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(r#""   ""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_long_tenant_name() {
        let long_str = "a".repeat(255);
        let name: Result<ValueObject<Name>, _> =
            serde_json::from_str(&format!(r#""{}""#, long_str));
        assert!(name.is_err());
    }

    #[test]
    fn test_display_implementation() {
        let name = Name("TestTenant".to_string());
        assert_eq!(format!("{}", name), "TestTenant");
    }

    #[test]
    fn test_validation() {
        let name = Name("ValidName".to_string());
        assert!(name.validate().is_ok());

        let empty_name = Name("".to_string());
        assert!(empty_name.validate().is_err());

        let whitespace_name = Name("   ".to_string());
        assert!(whitespace_name.validate().is_err());

        let long_name = Name("a".repeat(255));
        assert!(long_name.validate().is_err());
    }

    #[test]
    fn test_get_value() {
        let test_str = "TestValue";
        let name = Name(test_str.to_string());
        assert_eq!(name.get_value(), test_str);
    }
}
