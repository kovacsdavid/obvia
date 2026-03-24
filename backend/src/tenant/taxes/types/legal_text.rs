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

use crate::common::types::{ValueObject, ValueObjectData, value_object::ValueObjectError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct LegalText(pub String);

impl ValueObjectData for LegalText {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.len() <= 10000 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(
                "A jogi szöveg nem lehet 10 000 karakternél hosszabb!",
            ))
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for LegalText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<LegalText> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(LegalText(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_legal_text() {
        // Test regular text
        let text: ValueObject<LegalText> =
            serde_json::from_str(r#""This is a valid legal text.""#).unwrap();
        assert_eq!(text.as_str(), "This is a valid legal text.");

        // Test empty text
        let text: ValueObject<LegalText> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(text.as_str(), "");

        // Test text with special characters
        let text: ValueObject<LegalText> =
            serde_json::from_str(r#""Legal text with §§§ symbols and numbers 123!""#).unwrap();
        assert_eq!(
            text.as_str(),
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
