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

use crate::common::types::{ValueObject, ValueObjectable, value_object::ValueObjectError};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DueDate(pub String);

impl ValueObjectable for DueDate {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.trim().is_empty() {
            Ok(())
        } else {
            NaiveDate::parse_from_str(self.0.trim(), "%Y-%m-%d").map_err(|_| {
                ValueObjectError::InvalidInput("Hibás dátum formátum! (2006-01-02)")
            })?;
            Ok(())
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for DueDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<DueDate> {
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
            assert_eq!(due_date.as_str(), date);
        }
    }

    #[test]
    fn test_empty_due_date() {
        let due_date: ValueObject<DueDate> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(due_date.as_str(), "");

        let due_date: ValueObject<DueDate> = serde_json::from_str(r#""  ""#).unwrap();
        assert_eq!(due_date.as_str(), "  ");
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
