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
        Ok(())
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
        let contact_name: ValueObject<ContactName> = serde_json::from_str(r#""John Doe""#).unwrap();
        assert_eq!(contact_name.as_str(), "John Doe");
    }

    #[test]
    fn test_empty_contact_name() {
        let contact_name: ValueObject<ContactName> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(contact_name.as_str(), "");
    }

    #[test]
    fn test_contact_name_with_special_chars() {
        let contact_name: ValueObject<ContactName> =
            serde_json::from_str(r#""John @ Doe!""#).unwrap();
        assert_eq!(contact_name.as_str(), "John @ Doe!");
    }

    #[test]
    fn test_contact_name_with_numbers() {
        let contact_name: ValueObject<ContactName> =
            serde_json::from_str(r#""John Doe 123""#).unwrap();
        assert_eq!(contact_name.as_str(), "John Doe 123");
    }

    #[test]
    fn test_contact_name_unicode() {
        let contact_name: ValueObject<ContactName> =
            serde_json::from_str(r#""János Kovács""#).unwrap();
        assert_eq!(contact_name.as_str(), "János Kovács");
    }

    #[test]
    fn test_contact_name_display() {
        let contact_name = ContactName("John Doe".to_string());
        assert_eq!(format!("{}", contact_name), "John Doe");
    }

    #[test]
    fn test_contact_name_validate() {
        let contact_name = ContactName("John Doe".to_string());
        assert!(contact_name.validate().is_ok());
    }

    #[test]
    fn test_contact_name_clone() {
        let contact_name = ContactName("John Doe".to_string());
        let cloned = contact_name.clone();
        assert_eq!(contact_name, cloned);
    }

    #[test]
    fn test_contact_name_debug() {
        let contact_name = ContactName("John Doe".to_string());
        assert_eq!(format!("{:?}", contact_name), r#"ContactName("John Doe")"#);
    }

    #[test]
    fn test_invalid_json() {
        let result: Result<ValueObject<ContactName>, _> = serde_json::from_str("invalid");
        assert!(result.is_err());
    }
}
