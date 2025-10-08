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

use crate::common::types::value_object::{ValueObject, ValueObjectable};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct EndDate(pub String);

impl ValueObjectable for EndDate {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if self.0.trim().is_empty() {
            Ok(())
        } else {
            NaiveDateTime::parse_from_str(self.0.trim(), "%Y-%m-%d %H:%M:%S")
                .map_err(|_| String::from("Hibás dátum formátum! (2006-01-02 15:04:05)"))?;
            Ok(())
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

impl Display for EndDate {
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

impl<'de> Deserialize<'de> for ValueObject<EndDate> {
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
        ValueObject::new(EndDate(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_end_date_format() {
        let date: ValueObject<EndDate> = serde_json::from_str(r#""2025-01-01 12:00:00""#).unwrap();
        assert_eq!(date.extract().get_value(), "2025-01-01 12:00:00");
    }

    #[test]
    fn test_valid_empty_end_date() {
        let date: ValueObject<EndDate> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(date.extract().get_value(), "");
    }

    #[test]
    fn test_invalid_date_format() {
        let cases = vec![
            r#""2025/01/01 12:00:00""#,
            r#""2025-13-01 12:00:00""#,
            r#""2025-01-32 12:00:00""#,
            r#""2025-01-01 25:00:00""#,
            r#""2025-01-01 12:61:00""#,
            r#""2025-01-01""#,
            r#""not a date""#
        ];

        for case in cases {
            let date: Result<ValueObject<EndDate>, _> = serde_json::from_str(case);
            assert!(date.is_err(), "Should fail for input: {}", case);
        }
    }

    #[test]
    fn test_date_display() {
        let date = EndDate("2025-01-01 12:00:00".to_string());
        assert_eq!(format!("{}", date), "2025-01-01 12:00:00");
    }

    #[test]
    fn test_date_value_access() {
        let date = EndDate("2025-01-01 12:00:00".to_string());
        assert_eq!(date.get_value(), "2025-01-01 12:00:00");
    }

    #[test]
    fn test_date_validation() {
        let valid_date = EndDate("2025-01-01 12:00:00".to_string());
        assert!(valid_date.validate().is_ok());

        let empty_date = EndDate("".to_string());
        assert!(empty_date.validate().is_ok());

        let invalid_date = EndDate("invalid date".to_string());
        assert!(invalid_date.validate().is_err());
    }
}
