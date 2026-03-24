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
pub struct Priority(pub String);

impl ValueObjectData for Priority {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.as_str() {
            "low" => Ok(()),
            "normal" => Ok(()),
            "high" => Ok(()),
            _ => Err(ValueObjectError::InvalidInput("Hibás prioritás!")),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Priority> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(Priority(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_priority_low() {
        let priority: ValueObject<Priority> = serde_json::from_str(r#""low""#).unwrap();
        assert_eq!(priority.as_str(), "low");
    }

    #[test]
    fn test_valid_priority_normal() {
        let priority: ValueObject<Priority> = serde_json::from_str(r#""normal""#).unwrap();
        assert_eq!(priority.as_str(), "normal");
    }

    #[test]
    fn test_valid_priority_high() {
        let priority: ValueObject<Priority> = serde_json::from_str(r#""high""#).unwrap();
        assert_eq!(priority.as_str(), "high");
    }

    #[test]
    fn test_invalid_priority() {
        let priority: Result<ValueObject<Priority>, _> = serde_json::from_str(r#""invalid""#);
        assert!(priority.is_err());
    }

    #[test]
    fn test_empty_priority() {
        let priority: Result<ValueObject<Priority>, _> = serde_json::from_str(r#""""#);
        assert!(priority.is_err());
    }

    #[test]
    fn test_priority_display() {
        let priority = Priority("normal".to_string());
        assert_eq!(format!("{}", priority), "normal");
    }

    #[test]
    fn test_priority_debug() {
        let priority = Priority("high".to_string());
        assert_eq!(format!("{:?}", priority), r#"Priority("high")"#);
    }

    #[test]
    fn test_priority_clone() {
        let priority = Priority("low".to_string());
        let cloned = priority.clone();
        assert_eq!(priority, cloned);
    }

    #[test]
    fn test_priority_validate() {
        let valid_priorities = vec!["low", "normal", "high"];
        for p in valid_priorities {
            let priority = Priority(p.to_string());
            assert!(priority.validate().is_ok());
        }

        let invalid_priorities = vec!["LOW", "NORMAL", "HIGH", "medium", "urgent", ""];
        for p in invalid_priorities {
            let priority = Priority(p.to_string());
            assert!(priority.validate().is_err());
        }
    }

    #[test]
    fn test_priority_get_value() {
        let priority = Priority("normal".to_string());
        assert_eq!(priority.get_value(), "normal");
    }
}
