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
        ValueObject::new_required(Name(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_warehouse_name() {
        let name: ValueObject<Name> = serde_json::from_str(r#""Test Warehouse""#).unwrap();
        assert_eq!(name.as_str(), "Test Warehouse");
    }

    #[test]
    fn test_valid_warehouse_name_with_spaces() {
        let name: ValueObject<Name> = serde_json::from_str(r#""  Test  Warehouse  ""#).unwrap();
        assert_eq!(name.as_str(), "  Test  Warehouse  ");
    }

    #[test]
    fn test_invalid_warehouse_name_empty() {
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_invalid_warehouse_name_whitespace() {
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(r#""   ""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_warehouse_name_display() {
        let name = Name("Test Warehouse".to_string());
        assert_eq!(format!("{}", name), "Test Warehouse");
    }

    #[test]
    fn test_warehouse_name_debug() {
        let name = Name("Test Warehouse".to_string());
        assert_eq!(format!("{:?}", name), r#"Name("Test Warehouse")"#);
    }

    #[test]
    fn test_warehouse_name_clone() {
        let name = Name("Test Warehouse".to_string());
        let cloned = name.clone();
        assert_eq!(name, cloned);
    }

    #[test]
    fn test_warehouse_name_deserialization() {
        let json = r#""Test Warehouse""#;
        let name: ValueObject<Name> = serde_json::from_str(json).unwrap();
        assert_eq!(name.as_str(), "Test Warehouse");
    }

    #[test]
    fn test_warehouse_name_serialization() {
        let name = ValueObject::new_required(Name("Test Warehouse".to_string())).unwrap();
        let json = serde_json::to_string(&name).unwrap();
        assert_eq!(json, r#""Test Warehouse""#);
    }
}
