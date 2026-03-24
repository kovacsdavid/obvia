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
pub struct Name(pub String);

impl ValueObjectData for Name {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        let trimmed = self.0.trim();
        if !trimmed.is_empty() && trimmed.len() < 255 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput("Hibás név!"))
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
        ValueObject::new_required(Name(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_tenant_name() {
        let name: ValueObject<Name> = serde_json::from_str(r#""ValidTenant123""#).unwrap();
        assert_eq!(name.as_str(), "ValidTenant123");
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
