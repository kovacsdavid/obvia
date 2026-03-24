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
pub struct CurrencyCode(pub String);

impl ValueObjectData for CurrencyCode {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.trim().len() == 3 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(
                "A mező csak három karakteres pénznemformátumot tartalmazhat. Pl.: HUF",
            ))
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<CurrencyCode> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(CurrencyCode(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_currency() {
        let currency: ValueObject<CurrencyCode> = serde_json::from_str(r#""USD""#).unwrap();
        assert_eq!(currency.as_str(), "USD");
    }

    #[test]
    fn test_invalid_currency_too_short() {
        let currency: Result<ValueObject<CurrencyCode>, _> = serde_json::from_str(r#""US""#);
        assert!(currency.is_err());
    }

    #[test]
    fn test_invalid_currency_too_long() {
        let currency: Result<ValueObject<CurrencyCode>, _> = serde_json::from_str(r#""USDT""#);
        assert!(currency.is_err());
    }

    #[test]
    fn test_invalid_currency_empty() {
        let currency: Result<ValueObject<CurrencyCode>, _> = serde_json::from_str(r#""""#);
        assert!(currency.is_err());
    }

    #[test]
    fn test_display_format() {
        let currency = CurrencyCode("EUR".to_string());
        assert_eq!(format!("{}", currency), "EUR");
    }

    #[test]
    fn test_validation_with_spaces() {
        let currency = CurrencyCode(" USD ".to_string());
        assert!(currency.validate().is_ok());
    }

    #[test]
    fn test_get_value() {
        let currency = CurrencyCode("GBP".to_string());
        assert_eq!(currency.get_value(), "GBP");
    }

    #[test]
    fn test_clone() {
        let currency = CurrencyCode("JPY".to_string());
        let cloned = currency.clone();
        assert_eq!(currency, cloned);
    }

    #[test]
    fn test_debug_format() {
        let currency = CurrencyCode("CNY".to_string());
        assert_eq!(format!("{:?}", currency), r#"CurrencyCode("CNY")"#);
    }

    #[test]
    fn test_deserialization_errors() {
        let invalid_types = vec!["null", "123", "true", "[]", "{}"];

        for invalid in invalid_types {
            let result: Result<ValueObject<CurrencyCode>, _> = serde_json::from_str(invalid);
            assert!(result.is_err());
        }
    }
}
