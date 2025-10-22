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
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DueDate(pub String);

impl ValueObjectable for DueDate {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if self.0.trim().is_empty() {
            Ok(())
        } else {
            NaiveDate::parse_from_str(self.0.trim(), "%Y-%m-%d").map_err(|e| e.to_string())?;
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

impl Display for DueDate {
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

impl<'de> Deserialize<'de> for ValueObject<DueDate> {
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
        ValueObject::new(DueDate(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_due_dates() {
        let cases = vec![
            "2024-01-01",
            "2025-12-31",
            "2000-02-29", // Leap year
        ];

        for date in cases {
            let due_date: ValueObject<DueDate> =
                serde_json::from_str(&format!(r#""{}""#, date)).unwrap();
            assert_eq!(due_date.extract().get_value(), date);
        }
    }

    #[test]
    fn test_empty_due_date() {
        let due_date: ValueObject<DueDate> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(due_date.extract().get_value(), "");

        let due_date: ValueObject<DueDate> = serde_json::from_str(r#""  ""#).unwrap();
        assert_eq!(due_date.extract().get_value(), "  ");
    }

    #[test]
    fn test_invalid_due_date_formats() {
        let cases = vec![
            r#""2024/01/01""#,
            r#""2024.01.01""#,
            r#""01-01-2024""#,
            r#""2024-13-01""#,
            r#""2024-01-32""#,
            r#""2024-00-01""#,
            r#""2024-01-00""#,
            r#""abc""#,
        ];

        for case in cases {
            let due_date: Result<ValueObject<DueDate>, _> = serde_json::from_str(case);
            assert!(due_date.is_err());
        }
    }

    #[test]
    fn test_display() {
        let due_date = DueDate("2024-01-01".to_string());
        assert_eq!(format!("{}", due_date), "2024-01-01");
    }

    #[test]
    fn test_get_value() {
        let due_date = DueDate("2024-01-01".to_string());
        assert_eq!(due_date.get_value(), "2024-01-01");
    }

    #[test]
    fn test_validation() {
        assert!(DueDate("2024-01-01".to_string()).validate().is_ok());
        assert!(DueDate("".to_string()).validate().is_ok());
        assert!(DueDate("  ".to_string()).validate().is_ok());

        assert!(DueDate("2024/01/01".to_string()).validate().is_err());
        assert!(DueDate("2024.01.01".to_string()).validate().is_err());
        assert!(DueDate("01-01-2024".to_string()).validate().is_err());
        assert!(DueDate("2024-13-01".to_string()).validate().is_err());
        assert!(DueDate("2024-01-32".to_string()).validate().is_err());
        assert!(DueDate("abc".to_string()).validate().is_err());
    }
}
