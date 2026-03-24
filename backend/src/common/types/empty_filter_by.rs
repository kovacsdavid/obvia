/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

use crate::common::types::value_object::ValueObjectError;
use crate::common::types::{ValueObject, ValueObjectData};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct EmptyFilterBy(pub String);

impl ValueObjectData for EmptyFilterBy {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.trim() {
            "" => Ok(()),
            _ => Err(ValueObjectError::InvalidInput("Hibás sorrend formátum")),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl FromStr for EmptyFilterBy {
    type Err = ValueObjectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EmptyFilterBy(s.to_string()))
    }
}

impl<'de> Deserialize<'de> for ValueObject<EmptyFilterBy> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(EmptyFilterBy(s)).map_err(serde::de::Error::custom)
    }
}

impl Display for EmptyFilterBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_filter_by() {
        let filter_by: ValueObject<EmptyFilterBy> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(filter_by.as_str(), "");
    }

    #[test]
    fn test_invalid_filter_by() {
        let filter_by: Result<ValueObject<EmptyFilterBy>, _> = serde_json::from_str(r#""any""#);
        assert!(filter_by.is_err());
    }

    #[test]
    fn test_empty_filter_by() {
        let filter_by: Result<ValueObject<EmptyFilterBy>, _> = serde_json::from_str(r#""""#);
        assert!(filter_by.is_ok());
    }

    #[test]
    fn test_display_implementation() {
        let filter_by = EmptyFilterBy("name".to_string());
        assert_eq!(format!("{}", filter_by), "name");
    }

    #[test]
    fn test_clone() {
        let filter_by = EmptyFilterBy("name".to_string());
        let cloned = filter_by.clone();
        assert_eq!(filter_by, cloned);
    }

    #[test]
    fn test_debug_output() {
        let filter_by = EmptyFilterBy("name".to_string());
        assert_eq!(format!("{:?}", filter_by), r#"EmptyFilterBy("name")"#);
    }

    #[test]
    fn test_validation() {
        let valid = EmptyFilterBy("".to_string());
        assert!(valid.validate().is_ok());

        let invalid = EmptyFilterBy("invalid".to_string());
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_get_value() {
        let value = "name".to_string();
        let filter_by = EmptyFilterBy(value.clone());
        assert_eq!(filter_by.get_value(), &value);
    }

    #[test]
    fn test_from_str() {
        let filter_by = EmptyFilterBy::from_str("name").unwrap();
        assert_eq!(filter_by.get_value(), "name");
    }

    #[test]
    fn test_deserialization_error_messages() {
        let invalid: Result<ValueObject<EmptyFilterBy>, _> = serde_json::from_str(r#""invalid""#);
        assert!(
            invalid
                .unwrap_err()
                .to_string()
                .contains("Hibás sorrend formátum")
        );
    }
}
