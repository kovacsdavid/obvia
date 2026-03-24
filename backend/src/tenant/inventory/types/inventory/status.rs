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
pub struct Status(pub String);

impl ValueObjectData for Status {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.as_str() {
            "active" => Ok(()),
            "inactive" => Ok(()),
            "discontinued" => Ok(()),
            _ => Err(ValueObjectError::InvalidInput("Hibás termék státusz")),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Status> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(Status(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_status_values() {
        let valid_statuses = vec!["active", "inactive", "discontinued"];

        for status_str in valid_statuses {
            let status = Status(status_str.to_string());
            assert!(
                status.validate().is_ok(),
                "Status '{}' should be valid",
                status_str
            );
        }
    }

    #[test]
    fn test_invalid_status_values() {
        let invalid_statuses = vec!["pending", "deleted", "archived", "", "ACTIVE", "INACTIVE"];

        for status_str in invalid_statuses {
            let status = Status(status_str.to_string());
            assert!(
                status.validate().is_err(),
                "Status '{}' should be invalid",
                status_str
            );
        }
    }

    #[test]
    fn test_status_deserialization_valid() {
        let valid_cases = vec![r#""active""#, r#""inactive""#, r#""discontinued""#];

        for case in valid_cases {
            let status: Result<ValueObject<Status>, _> = serde_json::from_str(case);
            assert!(
                status.is_ok(),
                "Failed to deserialize valid status: {}",
                case
            );
            if let Ok(status) = status {
                assert!(status.extract().validate().is_ok());
            }
        }
    }

    #[test]
    fn test_status_deserialization_invalid() {
        let invalid_cases = vec![
            r#""pending""#,
            r#""ACTIVE""#,
            r#""deleted""#,
            r#""""#,
            r#"null"#,
        ];

        for case in invalid_cases {
            let status: Result<ValueObject<Status>, _> = serde_json::from_str(case);
            assert!(
                status.is_err(),
                "Should fail to deserialize invalid status: {}",
                case
            );
        }
    }

    #[test]
    fn test_status_display() {
        let test_cases = vec![
            ("active", "active"),
            ("inactive", "inactive"),
            ("discontinued", "discontinued"),
        ];

        for (input, expected) in test_cases {
            let status = Status(input.to_string());
            assert_eq!(format!("{}", status), expected);
        }
    }

    #[test]
    fn test_get_value() {
        let status = Status("active".to_string());
        assert_eq!(status.get_value(), "active");

        let status = Status("inactive".to_string());
        assert_eq!(status.get_value(), "inactive");

        let status = Status("discontinued".to_string());
        assert_eq!(status.get_value(), "discontinued");
    }

    #[test]
    fn test_clone() {
        let original = Status("active".to_string());
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.get_value(), cloned.get_value());
    }

    #[test]
    fn test_partial_eq() {
        let status1 = Status("active".to_string());
        let status2 = Status("active".to_string());
        let status3 = Status("inactive".to_string());

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_debug_output() {
        let status = Status("active".to_string());
        assert_eq!(format!("{:?}", status), r#"Status("active")"#);
    }
}
