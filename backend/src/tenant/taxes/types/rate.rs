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
pub struct Rate(pub String);

impl ValueObjectData for Rate {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.trim().is_empty() {
            Err(ValueObjectError::InvalidInput("A mező kitöltése kötelező!"))
        } else {
            self.0
                .trim()
                .replace(",", ".")
                .parse::<f64>()
                .map_err(|_| ValueObjectError::InvalidInput("Hibás fogyasztói ár formátum!"))?;
            Ok(())
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Rate> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(Rate(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rate() {
        let price: ValueObject<Rate> = serde_json::from_str(r#""123.45""#).unwrap();
        assert_eq!(price.as_str(), "123.45");

        let price: ValueObject<Rate> = serde_json::from_str(r#""123,45""#).unwrap();
        assert_eq!(price.as_str(), "123,45");
    }

    #[test]
    fn test_invalid_rate_format() {
        let cases = vec![
            r#""abc""#,
            r#""12.34.56""#,
            r#""12,34,56""#,
            r#""12a34""#,
            r#""$123""#,
        ];

        for case in cases {
            let price: Result<ValueObject<Rate>, _> = serde_json::from_str(case);
            assert!(price.is_err());
        }
    }

    #[test]
    fn test_display() {
        let price = Rate("123.45".to_string());
        assert_eq!(format!("{}", price), "123.45");
    }

    #[test]
    fn test_get_value() {
        let price = Rate("123.45".to_string());
        assert_eq!(price.get_value(), "123.45");
    }

    #[test]
    fn test_validation() {
        assert!(Rate("123.45".to_string()).validate().is_ok());
        assert!(Rate("123,45".to_string()).validate().is_ok());

        assert!(Rate("abc".to_string()).validate().is_err());
        assert!(Rate("12.34.56".to_string()).validate().is_err());
        assert!(Rate("12,34,56".to_string()).validate().is_err());
        assert!(Rate("$123".to_string()).validate().is_err());
    }
}
