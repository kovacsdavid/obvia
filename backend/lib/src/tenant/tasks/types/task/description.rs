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
pub struct Description(pub String);

impl ValueObjectable for Description {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if self.0.len() <= 3000 {
            Ok(())
        } else {
            Err(String::from(
                "A leírás nem lehet 3 000 karakternél hosszabb!",
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

impl Display for Description {
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

impl<'de> Deserialize<'de> for ValueObject<Description> {
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
        ValueObject::new(Description(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_description() {
        let desc: ValueObject<Description> = serde_json::from_str(r#""Valid description""#).unwrap();
        assert_eq!(desc.extract().get_value(), "Valid description");
    }

    #[test]
    fn test_empty_description() {
        let desc: ValueObject<Description> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(desc.extract().get_value(), "");
    }

    #[test]
    fn test_max_length_description() {
        let desc = "a".repeat(3000);
        let result: ValueObject<Description> = serde_json::from_str(&format!(r#""{}""#, desc)).unwrap();
        assert_eq!(result.extract().get_value(), &desc);
    }

    #[test]
    fn test_too_long_description() {
        let desc = "a".repeat(3001);
        let result: Result<ValueObject<Description>, _> = serde_json::from_str(&format!(r#""{}""#, desc));
        assert!(result.is_err());
    }

    #[test]
    fn test_display_trait() {
        let desc = Description("Test description".to_string());
        assert_eq!(format!("{}", desc), "Test description");
    }

    #[test]
    fn test_debug_trait() {
        let desc = Description("Test description".to_string());
        assert_eq!(format!("{:?}", desc), r#"Description("Test description")"#);
    }

    #[test]
    fn test_clone_trait() {
        let desc = Description("Test description".to_string());
        let cloned = desc.clone();
        assert_eq!(desc, cloned);
    }

    #[test]
    fn test_serialize() {
        let desc = Description("Test description".to_string());
        let serialized = serde_json::to_string(&desc).unwrap();
        assert_eq!(serialized, r#""Test description""#);
    }

    #[test]
    fn test_deserialize() {
        let input = r#""Test description""#;
        let deserialized: ValueObject<Description> = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized.extract().get_value(), "Test description");
    }

    #[test]
    fn test_special_characters() {
        let special = r#""Test with !@#$%^&*()_+ and unicode 你好世界""#;
        let desc: ValueObject<Description> = serde_json::from_str(special).unwrap();
        assert_eq!(desc.extract().get_value(), r#"Test with !@#$%^&*()_+ and unicode 你好世界"#);
    }

    #[test]
    fn test_multiline_description() {
        let multiline = r#""Line 1\nLine 2\nLine 3""#;
        let desc: ValueObject<Description> = serde_json::from_str(multiline).unwrap();
        assert_eq!(desc.extract().get_value(), "Line 1\nLine 2\nLine 3");
    }
}
