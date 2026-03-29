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
pub struct ContactName(pub String);

impl ValueObjectData for ContactName {
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

impl Display for ContactName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<ContactName> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(ContactName(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_contact_name() {
        let name: ValueObject<ContactName> = serde_json::from_str(r#""John Doe""#).unwrap();
        assert_eq!(name.as_str(), "John Doe");
    }

    #[test]
    fn test_empty_contact_name() {
        let name: Result<ValueObject<ContactName>, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_whitespace_contact_name() {
        let name: Result<ValueObject<ContactName>, _> = serde_json::from_str(r#"" "#);
        assert!(name.is_err());
    }

    #[test]
    fn test_contact_name_display() {
        let name = ContactName("John Doe".to_string());
        assert_eq!(format!("{}", name), "John Doe");
    }

    #[test]
    fn test_contact_name_validation() {
        let name = ContactName("John Doe".to_string());
        assert!(name.validate().is_ok());

        let empty_name = ContactName("".to_string());
        assert!(empty_name.validate().is_err());

        let whitespace_name = ContactName(" ".to_string());
        assert!(whitespace_name.validate().is_err());
    }

    #[test]
    fn test_contact_name_get_value() {
        let name = ContactName("John Doe".to_string());
        assert_eq!(name.get_value(), "John Doe");
    }

    #[test]
    fn test_contact_name_debug() {
        let name = ContactName("John Doe".to_string());
        assert_eq!(format!("{:?}", name), "ContactName(\"John Doe\")");
    }

    #[test]
    fn test_contact_name_clone() {
        let name = ContactName("John Doe".to_string());
        let cloned = name.clone();
        assert_eq!(name, cloned);
    }

    #[test]
    fn test_contact_name_partial_eq() {
        let name1 = ContactName("John Doe".to_string());
        let name2 = ContactName("John Doe".to_string());
        let name3 = ContactName("Jane Doe".to_string());

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_contact_name_serialize() {
        let name = ContactName("John Doe".to_string());
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, r#""John Doe""#);
    }
}
