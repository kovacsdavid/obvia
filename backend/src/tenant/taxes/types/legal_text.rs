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
pub struct LegalText(pub String);

impl ValueObjectable for LegalText {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if self.0.len() <= 10000 {
            Ok(())
        } else {
            Err(String::from(
                "A jogi szöveg nem lehet 10 000 karakternél hosszabb!",
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

impl Display for LegalText {
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

impl<'de> Deserialize<'de> for ValueObject<LegalText> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `LegalText` and validates it by calling `ValueObject::new`.
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
        ValueObject::new(LegalText(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_legal_text() {
        // Test regular text
        let text: ValueObject<LegalText> =
            serde_json::from_str(r#""This is a valid legal text.""#).unwrap();
        assert_eq!(text.extract().get_value(), "This is a valid legal text.");

        // Test empty text
        let text: ValueObject<LegalText> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(text.extract().get_value(), "");

        // Test text with special characters
        let text: ValueObject<LegalText> =
            serde_json::from_str(r#""Legal text with §§§ symbols and numbers 123!""#).unwrap();
        assert_eq!(
            text.extract().get_value(),
            "Legal text with §§§ symbols and numbers 123!"
        );
    }

    #[test]
    fn test_max_length_legal_text() {
        // Test text at exactly 10000 characters
        let text = "a".repeat(10000);
        let json = format!(r#""{}""#, text);
        let result: Result<ValueObject<LegalText>, _> = serde_json::from_str(&json);
        assert!(result.is_ok());

        // Test text exceeding 10000 characters
        let text = "a".repeat(10001);
        let json = format!(r#""{}""#, text);
        let result: Result<ValueObject<LegalText>, _> = serde_json::from_str(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation() {
        // Test valid cases
        assert!(LegalText("Short text".to_string()).validate().is_ok());
        assert!(LegalText("".to_string()).validate().is_ok());
        assert!(LegalText("a".repeat(10000)).validate().is_ok());

        // Test invalid cases
        assert!(LegalText("a".repeat(10001)).validate().is_err());
    }

    #[test]
    fn test_display() {
        let text = LegalText("Sample legal text".to_string());
        assert_eq!(format!("{}", text), "Sample legal text");
    }

    #[test]
    fn test_get_value() {
        let text = LegalText("Test text".to_string());
        assert_eq!(text.get_value(), "Test text");
    }

    #[test]
    fn test_deserialization_errors() {
        // Test invalid JSON format
        let result: Result<ValueObject<LegalText>, _> =
            serde_json::from_str(r#"{"text": "Invalid format"}"#);
        assert!(result.is_err());

        // Test with non-string JSON value
        let result: Result<ValueObject<LegalText>, _> = serde_json::from_str("123");
        assert!(result.is_err());
    }

    #[test]
    fn test_clone_and_equality() {
        let text1 = LegalText("Test text".to_string());
        let text2 = text1.clone();
        assert_eq!(text1, text2);

        let text3 = LegalText("Different text".to_string());
        assert_ne!(text1, text3);
    }
}
