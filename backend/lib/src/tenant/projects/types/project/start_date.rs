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
pub struct StartDate(pub String);

impl ValueObjectable for StartDate {
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

impl Display for StartDate {
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

impl<'de> Deserialize<'de> for ValueObject<StartDate> {
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
        ValueObject::new(StartDate(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_start_date() {
        let date: ValueObject<StartDate> = serde_json::from_str(r#""2023-12-25 10:30:00""#).unwrap();
        assert_eq!(date.extract().get_value(), "2023-12-25 10:30:00");
    }

    #[test]
    fn test_empty_date() {
        let date: ValueObject<StartDate> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(date.extract().get_value(), "");
    }

    #[test]
    fn test_invalid_date_format() {
        let date: Result<ValueObject<StartDate>, _> = serde_json::from_str(r#""2023-12-25""#);
        assert!(date.is_err());
    }

    #[test]
    fn test_invalid_date_values() {
        let date: Result<ValueObject<StartDate>, _> = serde_json::from_str(r#""2023-13-45 25:70:99""#);
        assert!(date.is_err());
    }

    #[test]
    fn test_date_display() {
        let date = StartDate("2023-12-25 10:30:00".to_string());
        assert_eq!(format!("{}", date), "2023-12-25 10:30:00");
    }

    #[test]
    fn test_date_clone() {
        let date = StartDate("2023-12-25 10:30:00".to_string());
        let cloned = date.clone();
        assert_eq!(date, cloned);
    }

    #[test]
    fn test_date_debug() {
        let date = StartDate("2023-12-25 10:30:00".to_string());
        assert_eq!(format!("{:?}", date), r#"StartDate("2023-12-25 10:30:00")"#);
    }

    #[test]
    fn test_date_partial_eq() {
        let date1 = StartDate("2023-12-25 10:30:00".to_string());
        let date2 = StartDate("2023-12-25 10:30:00".to_string());
        let date3 = StartDate("2023-12-26 10:30:00".to_string());

        assert_eq!(date1, date2);
        assert_ne!(date1, date3);
    }

    #[test]
    fn test_date_validation() {
        let valid_date = StartDate("2023-12-25 10:30:00".to_string());
        assert!(valid_date.validate().is_ok());

        let invalid_date = StartDate("invalid date".to_string());
        assert!(invalid_date.validate().is_err());
    }

    #[test]
    fn test_date_get_value() {
        let date = StartDate("2023-12-25 10:30:00".to_string());
        assert_eq!(date.get_value(), "2023-12-25 10:30:00");
    }

    #[test]
    fn test_date_serialization() {
        let date = ValueObject::new(StartDate("2023-12-25 10:30:00".to_string())).unwrap();
        let serialized = serde_json::to_string(&date).unwrap();
        assert_eq!(serialized, r#""2023-12-25 10:30:00""#);
    }

    #[test]
    fn test_date_deserialization() {
        let date: ValueObject<StartDate> = serde_json::from_str(r#""2023-12-25 10:30:00""#).unwrap();
        assert_eq!(date.extract().get_value(), "2023-12-25 10:30:00");
    }
}
