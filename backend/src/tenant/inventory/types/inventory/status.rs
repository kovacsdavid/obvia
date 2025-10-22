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
pub struct Status(pub String);

impl ValueObjectable for Status {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        match self.0.as_str() {
            "active" => Ok(()),
            "inactive" => Ok(()),
            "discontinued" => Ok(()),
            _ => Err(String::from("Hibás termék státusz")),
        }
    }

    /// Retrieves a reference to the value contained within the struct.
    ///
    /// # Returns
    /// A reference to the internal value of type `Self::DataType`.
    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Status {
    /// Implements the `fmt` method from the `std::fmt::Display` or `std::fmt::Debug` trait,
    /// enabling a custom display of the struct or type.
    ///
    /// # Parameters
    /// - `&self`: A reference to the instance of the type implementing this method.
    /// - `f`: A mutable reference to a `std::fmt::Formatter` used for formatting output.
    ///
    /// # Returns
    /// - `std::fmt::Result`: Indicates whether the formatting operation was successful
    ///   (`Ok(())`) or an error occurred (`Err`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Status> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `Name` and validates it by calling `ValueObject::new`.
    /// If the validation fails, a custom deserialization error is returned.
    ///
    /// # Type Parameters
    /// - `D`: The type of the deserializer, which must implement `serde::Deserializer<'de>`.
    ///
    /// # Parameters
    /// - `deserializer`: The deserializer used to deserialize the input.
    ///
    /// # Returns
    /// - `Result<Self, D::Error>`:
    ///   - On success, returns the constructed and validated object wrapped in `Ok`.
    ///   - On failure, returns a custom error wrapped in `Err`.
    ///
    /// # Errors
    /// - Returns a deserialization error if:
    ///   - The input cannot be deserialized into a `String`.
    ///   - Validation using `ValueObject::new` fails, causing the `map_err` call to propagate an error.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(Status(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
