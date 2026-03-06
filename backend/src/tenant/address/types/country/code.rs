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
pub struct Code(pub String);

impl ValueObjectable for Code {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.trim().len() == 2 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput("Hibás ország azonosító"))
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Code> {
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
        assert_eq!(deserialized.as_str(), "HU");

        let json = r#""INVALID""#;
        let result: Result<ValueObject<Code>, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
