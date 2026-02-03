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
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct OrderBy(pub String);

impl ValueObjectable for OrderBy {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        match self.0.trim() {
            "name" | "unit_of_measure" | "status" | "created_at" | "updated_at" => Ok(()),
            _ => Err("Hibás sorrend formátum".to_string()),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl FromStr for OrderBy {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OrderBy(s.to_string()))
    }
}

impl<'de> Deserialize<'de> for ValueObject<OrderBy> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(OrderBy(s)).map_err(serde::de::Error::custom)
    }
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_order_by() {
        let order_by: ValueObject<OrderBy> = serde_json::from_str(r#""name""#).unwrap();
        assert_eq!(order_by.extract().get_value(), "name");
    }

    #[test]
    fn test_invalid_order_by() {
        let cases = vec![r#""id""#, r#""description""#];

        for case in cases {
            let order_by: Result<ValueObject<OrderBy>, _> = serde_json::from_str(case);
            assert!(order_by.is_err());
        }
    }

    #[test]
    fn test_empty_order_by() {
        let order_by: Result<ValueObject<OrderBy>, _> = serde_json::from_str(r#""""#);
        assert!(order_by.is_err());
    }

    #[test]
    fn test_whitespace_order_by() {
        let order_by: Result<ValueObject<OrderBy>, _> = serde_json::from_str(r#"" ""#);
        assert!(order_by.is_err());
    }

    #[test]
    fn test_case_sensitivity() {
        let cases = vec![r#""NAME""#, r#""Name""#, r#""nAmE""#];

        for case in cases {
            let order_by: Result<ValueObject<OrderBy>, _> = serde_json::from_str(case);
            assert!(order_by.is_err());
        }
    }

    #[test]
    fn test_sql_injection_attempts() {
        let cases = vec![
            r#""name; DROP TABLE products"#,
            r#""name OR 1=1"#,
            r#""name--"#,
            r#""name/*comment*/"#,
            r#""name' OR '1'='1"#,
        ];

        for case in cases {
            let order_by: Result<ValueObject<OrderBy>, _> = serde_json::from_str(case);
            assert!(order_by.is_err());
        }
    }

    #[test]
    fn test_special_characters() {
        let cases = vec![
            r#""name@"#,
            r#""name!"#,
            r#""name#"#,
            r#""name$"#,
            r#""name%"#,
        ];

        for case in cases {
            let order_by: Result<ValueObject<OrderBy>, _> = serde_json::from_str(case);
            assert!(order_by.is_err());
        }
    }

    #[test]
    fn test_display_implementation() {
        let order_by = OrderBy("name".to_string());
        assert_eq!(format!("{}", order_by), "name");
    }

    #[test]
    fn test_debug_format() {
        let order_by = OrderBy("name".to_string());
        assert_eq!(format!("{:?}", order_by), r#"OrderBy("name")"#);
    }

    #[test]
    fn test_value_object_clone() {
        let order_by = OrderBy("name".to_string());
        let cloned = order_by.clone();
        assert_eq!(order_by, cloned);
    }

    #[test]
    fn test_from_str_implementation() {
        let order_by = OrderBy::from_str("name").unwrap();
        assert_eq!(order_by.get_value(), "name");
    }

    #[test]
    fn test_value_object_serialization() {
        let order_by = ValueObject::new(OrderBy("name".to_string())).unwrap();
        let serialized = serde_json::to_string(&order_by).unwrap();
        assert_eq!(serialized, r#""name""#);
    }

    #[test]
    fn test_value_object_deserialization() {
        let json = r#""name""#;
        let deserialized: ValueObject<OrderBy> = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.extract().get_value(), "name");
    }

    #[test]
    fn test_partial_eq() {
        let order_by1 = OrderBy("name".to_string());
        let order_by2 = OrderBy("name".to_string());
        let order_by3 = OrderBy("different".to_string());

        assert_eq!(order_by1, order_by2);
        assert_ne!(order_by1, order_by3);
    }

    #[test]
    fn test_get_value() {
        let order_by = OrderBy("name".to_string());
        assert_eq!(order_by.get_value(), "name");
    }

    #[test]
    fn test_validation() {
        let valid = OrderBy("name".to_string());
        let invalid = OrderBy("invalid".to_string());

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_validation_error_message() {
        let invalid = OrderBy("invalid".to_string());
        let result = invalid.validate();

        assert_eq!(result.unwrap_err(), "Hibás sorrend formátum");
    }
}
