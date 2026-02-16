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
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Integer32(pub String);

impl ValueObjectable for Integer32 {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.trim().is_empty() {
            Ok(())
        } else {
            self.0
                .trim()
                .replace(",", ".")
                .parse::<i32>()
                .map_err(|_| ValueObjectError::InvalidInput("Hibás szám formátum!"))?;
            Ok(())
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Integer32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Integer32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(Integer32(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_string() {
        let integer = Integer32(String::from(""));
        assert!(integer.validate().is_ok());
    }

    #[test]
    fn test_validate_valid_integer() {
        let integer = Integer32(String::from("123"));
        assert!(integer.validate().is_ok());
    }

    #[test]
    fn test_validate_negative_integer() {
        let integer = Integer32(String::from("-123"));
        assert!(integer.validate().is_ok());
    }

    #[test]
    fn test_validate_decimal_comma() {
        let integer = Integer32(String::from("123,456"));
        assert!(integer.validate().is_err());
    }

    #[test]
    fn test_validate_decimal_period() {
        let integer = Integer32(String::from("123.456"));
        assert!(integer.validate().is_err());
    }

    #[test]
    fn test_validate_non_numeric() {
        let integer = Integer32(String::from("abc"));
        assert!(integer.validate().is_err());
    }

    #[test]
    fn test_validate_overflow() {
        let integer = Integer32(String::from("2147483648")); // Max i32 + 1
        assert!(integer.validate().is_err());
    }

    #[test]
    fn test_get_value() {
        let value = String::from("123");
        let integer = Integer32(value.clone());
        assert_eq!(integer.get_value(), &value);
    }

    #[test]
    fn test_display() {
        let integer = Integer32(String::from("123"));
        assert_eq!(format!("{}", integer), "123");
    }

    #[test]
    fn test_deserialize_valid() {
        let json = "\"123\"";
        let result: Result<ValueObject<Integer32>, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deserialize_invalid() {
        let json = "\"abc\"";
        let result: Result<ValueObject<Integer32>, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
