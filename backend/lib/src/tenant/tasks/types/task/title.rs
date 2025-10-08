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
pub struct Title(pub String);

impl ValueObjectable for Title {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if self.0.trim().is_empty() {
            return Err(String::from("A mező kitöltése kötelező"));
        }
        if self.0.len() <= 500 {
            Ok(())
        } else {
            Err(String::from(
                "A megnevezés nem lehet 500 karakternél hosszabb!",
            ))
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

impl Display for Title {
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

impl<'de> Deserialize<'de> for ValueObject<Title> {
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
        ValueObject::new(Title(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_title() {
        let title: ValueObject<Title> = serde_json::from_str(r#""Test Title""#).unwrap();
        assert_eq!(title.extract().get_value(), "Test Title");
    }

    #[test]
    fn test_empty_title() {
        let title: Result<ValueObject<Title>, _> = serde_json::from_str(r#""""#);
        assert!(title.is_err());
    }

    #[test]
    fn test_whitespace_only_title() {
        let title: Result<ValueObject<Title>, _> = serde_json::from_str(r#""   ""#);
        assert!(title.is_err());
    }

    #[test]
    fn test_title_too_long() {
        let long_title = "a".repeat(501);
        let title: Result<ValueObject<Title>, _> = serde_json::from_str(&format!(r#""{}""#, long_title));
        assert!(title.is_err());
    }

    #[test]
    fn test_title_max_length() {
        let max_title = "a".repeat(500);
        let title: ValueObject<Title> = serde_json::from_str(&format!(r#""{}""#, max_title)).unwrap();
        assert_eq!(title.extract().get_value().len(), 500);
    }

    #[test]
    fn test_title_display() {
        let title = Title("Test Title".to_string());
        assert_eq!(format!("{}", title), "Test Title");
    }

    #[test]
    fn test_title_validate_success() {
        let title = Title("Valid Title".to_string());
        assert!(title.validate().is_ok());
    }

    #[test]
    fn test_title_validate_failure_empty() {
        let title = Title("".to_string());
        assert!(title.validate().is_err());
    }

    #[test]
    fn test_title_validate_failure_too_long() {
        let title = Title("a".repeat(501));
        assert!(title.validate().is_err());
    }

    #[test]
    fn test_title_get_value() {
        let title = Title("Test Title".to_string());
        assert_eq!(title.get_value(), "Test Title");
    }

    #[test]
    fn test_title_clone() {
        let title = Title("Test Title".to_string());
        let cloned = title.clone();
        assert_eq!(title, cloned);
    }

    #[test]
    fn test_title_debug() {
        let title = Title("Test Title".to_string());
        assert_eq!(format!("{:?}", title), r#"Title("Test Title")"#);
    }

    #[test]
    fn test_title_serialization() {
        let title = Title("Test Title".to_string());
        let serialized = serde_json::to_string(&title).unwrap();
        assert_eq!(serialized, r#""Test Title""#);
    }

    #[test]
    fn test_title_deserialization_invalid_json() {
        let result: Result<ValueObject<Title>, _> = serde_json::from_str("invalid");
        assert!(result.is_err());
    }
}
