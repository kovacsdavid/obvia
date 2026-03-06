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

use crate::common::types::{ValueObject, ValueObjectable, value_object::ValueObjectError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Name(pub String);

impl ValueObjectable for Name {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if !self.0.trim().is_empty() {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput("A mező kitöltése kötelező"))
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Name> {
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

    #[test]
    fn test_valid_name() {
        let name: ValueObject<Name> = serde_json::from_str(r#""Test Product""#).unwrap();
        assert_eq!(name.as_str(), "Test Product");
    }

    #[test]
    fn test_invalid_name_empty() {
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_invalid_name_whitespace() {
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(r#"" "#);
        assert!(name.is_err());
    }

    #[test]
    fn test_display_implementation() {
        let name = Name("Test Product".to_string());
        assert_eq!(format!("{}", name), "Test Product");
    }

    #[test]
    fn test_debug_implementation() {
        let name = Name("Test Product".to_string());
        assert_eq!(format!("{:?}", name), r#"Name("Test Product")"#);
    }

    #[test]
    fn test_validation_non_empty() {
        let name = Name("Valid Product".to_string());
        assert!(name.validate().is_ok());
    }

    #[test]
    fn test_validation_empty() {
        let name = Name("".to_string());
        assert!(name.validate().is_err());
    }

    #[test]
    fn test_validation_whitespace() {
        let name = Name(" ".to_string());
        assert!(name.validate().is_err());
    }

    #[test]
    fn test_get_value() {
        let name = Name("Test Product".to_string());
        assert_eq!(name.get_value(), "Test Product");
    }

    #[test]
    fn test_clone() {
        let name = Name("Test Product".to_string());
        let cloned = name.clone();
        assert_eq!(name, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let name1 = Name("Test Product".to_string());
        let name2 = Name("Test Product".to_string());
        let name3 = Name("Different Product".to_string());

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_serialization() {
        let name = ValueObject::new(Name("Test Product".to_string())).unwrap();
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, r#""Test Product""#);
    }
}
