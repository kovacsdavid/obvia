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

use crate::common::types::value_object::ValueObjectError;
use crate::common::types::{ValueObject, ValueObjectable};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct FilterBy(pub String);

impl ValueObjectable for FilterBy {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.trim() {
            "description" => Ok(()),
            _ => Err(ValueObjectError::InvalidInput("Hibás sorrend formátum")),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl FromStr for FilterBy {
    type Err = ValueObjectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FilterBy(s.to_string()))
    }
}

impl<'de> Deserialize<'de> for ValueObject<FilterBy> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(FilterBy(s)).map_err(serde::de::Error::custom)
    }
}

impl Display for FilterBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_filter_by() {
        let filter_by: ValueObject<FilterBy> = serde_json::from_str(r#""description""#).unwrap();
        assert_eq!(filter_by.as_str(), "description");
    }

    #[test]
    fn test_invalid_filter_by() {
        let filter_by: Result<ValueObject<FilterBy>, _> = serde_json::from_str(r#""invalid""#);
        assert!(filter_by.is_err());
    }

    #[test]
    fn test_empty_filter_by() {
        let filter_by: Result<ValueObject<FilterBy>, _> = serde_json::from_str(r#""""#);
        assert!(filter_by.is_err());
    }

    #[test]
    fn test_display_implementation() {
        let filter_by = FilterBy("description".to_string());
        assert_eq!(format!("{}", filter_by), "description");
    }

    #[test]
    fn test_clone() {
        let filter_by = FilterBy("description".to_string());
        let cloned = filter_by.clone();
        assert_eq!(filter_by, cloned);
    }

    #[test]
    fn test_debug_output() {
        let filter_by = FilterBy("description".to_string());
        assert_eq!(format!("{:?}", filter_by), r#"FilterBy("description")"#);
    }

    #[test]
    fn test_validation() {
        let valid = FilterBy("description".to_string());
        assert!(valid.validate().is_ok());

        let invalid = FilterBy("invalid".to_string());
        assert!(invalid.validate().is_err());

        let empty = FilterBy("".to_string());
        assert!(empty.validate().is_err());
    }

    #[test]
    fn test_get_value() {
        let value = "description".to_string();
        let filter_by = FilterBy(value.clone());
        assert_eq!(filter_by.get_value(), &value);
    }

    #[test]
    fn test_from_str() {
        let filter_by = FilterBy::from_str("description").unwrap();
        assert_eq!(filter_by.get_value(), "description");
    }

    #[test]
    fn test_deserialization_error_messages() {
        let invalid: Result<ValueObject<FilterBy>, _> = serde_json::from_str(r#""invalid""#);
        assert!(
            invalid
                .unwrap_err()
                .to_string()
                .contains("Hibás sorrend formátum")
        );
    }
}
