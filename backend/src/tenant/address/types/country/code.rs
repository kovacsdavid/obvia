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

use crate::common::types::{ValueObject, ValueObjectable};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Code(pub String);

impl ValueObjectable for Code {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if self.0.trim().len() == 2 {
            Ok(())
        } else {
            Err("Hibás ország azonosító".to_string())
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

impl Display for Code {
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

impl<'de> Deserialize<'de> for ValueObject<Code> {
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
        ValueObject::new(Code(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::ValueObject;

    #[test]
    fn test_valid_postal_code() {
        let code = Code("HU".to_string());
        assert!(code.validate().is_ok());
    }

    #[test]
    fn test_invalid_postal_code_too_short() {
        let code = Code("H".to_string());
        assert!(code.validate().is_err());
    }

    #[test]
    fn test_invalid_postal_code_too_long() {
        let code = Code("HUN".to_string());
        assert!(code.validate().is_err());
    }

    #[test]
    fn test_postal_code_with_spaces() {
        let code = Code("  HU  ".to_string());
        assert!(code.validate().is_ok());
    }

    #[test]
    fn test_get_value() {
        let code = Code("HU".to_string());
        assert_eq!(code.get_value(), "HU");
    }

    #[test]
    fn test_display() {
        let code = Code("HU".to_string());
        assert_eq!(format!("{}", code), "HU");
    }

    #[test]
    fn test_value_object_creation() {
        let result = ValueObject::new(Code("HU".to_string()));
        assert!(result.is_ok());

        let result = ValueObject::new(Code("INVALID".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization() {
        let code = Code("HU".to_string());
        let serialized = serde_json::to_string(&code).unwrap();
        assert_eq!(serialized, r#""HU""#);
    }

    #[test]
    fn test_deserialization() {
        let json = r#""HU""#;
        let deserialized: ValueObject<Code> = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.extract().get_value(), "HU");

        let json = r#""INVALID""#;
        let result: Result<ValueObject<Code>, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
