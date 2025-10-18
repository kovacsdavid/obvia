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
pub struct Name(pub String);

impl ValueObjectable for Name {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if !self.0.trim().is_empty() {
            Ok(())
        } else {
            Err(String::from("A mező kitöltése kötelező"))
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
    use serde_json;

    #[test]
    fn test_valid_name() {
        let name: ValueObject<Name> = serde_json::from_str(r#""Test Service""#).unwrap();
        assert_eq!(name.extract().get_value(), "Test Service");
    }

    #[test]
    fn test_empty_name() {
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_whitespace_name() {
        let name: Result<ValueObject<Name>, _> = serde_json::from_str(r#""   ""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_name_display() {
        let name = Name("Test Service".to_string());
        assert_eq!(format!("{}", name), "Test Service");
    }

    #[test]
    fn test_name_clone() {
        let name = Name("Test Service".to_string());
        let cloned = name.clone();
        assert_eq!(name, cloned);
    }

    #[test]
    fn test_name_debug() {
        let name = Name("Test Service".to_string());
        assert_eq!(format!("{:?}", name), r#"Name("Test Service")"#);
    }

    #[test]
    fn test_name_partial_eq() {
        let name1 = Name("Test Service".to_string());
        let name2 = Name("Test Service".to_string());
        let name3 = Name("Different Service".to_string());

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_name_validation() {
        let name = Name("Test Service".to_string());
        assert!(name.validate().is_ok());
    }

    #[test]
    fn test_name_get_value() {
        let name = Name("Test Service".to_string());
        assert_eq!(name.get_value(), "Test Service");
    }

    #[test]
    fn test_name_serialization() {
        let name = ValueObject::new(Name("Test Service".to_string())).unwrap();
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, r#""Test Service""#);
    }

    #[test]
    fn test_name_deserialization() {
        let name: ValueObject<Name> = serde_json::from_str(r#""Test Service""#).unwrap();
        assert_eq!(name.extract().get_value(), "Test Service");
    }
}
