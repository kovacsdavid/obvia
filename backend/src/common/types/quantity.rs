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
pub struct Quantity(pub String);

impl ValueObjectable for Quantity {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if self.0.trim().is_empty() {
            Err(String::from("A mező kitöltése kötelező!"))
        } else {
            let value = self
                .0
                .trim()
                .replace(",", ".")
                .parse::<f64>()
                .map_err(|_| String::from("Hibás mennyiség formátum!"))?;
            if value >= 0_f64 {
                Ok(())
            } else {
                Err(String::from(
                    "A megadott érték csak 0 vagy annál nagyobb szám lehet!",
                ))
            }
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

impl Display for Quantity {
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

impl<'de> Deserialize<'de> for ValueObject<Quantity> {
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
        ValueObject::new(Quantity(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_string() {
        let quantity = Quantity(String::from(""));
        assert!(quantity.validate().is_err());
    }

    #[test]
    fn test_validate_valid_float() {
        let quantity = Quantity(String::from("123.456"));
        assert!(quantity.validate().is_ok());
    }

    #[test]
    fn test_validate_valid_integer() {
        let quantity = Quantity(String::from("123"));
        assert!(quantity.validate().is_ok());
    }

    #[test]
    fn test_validate_zero() {
        let quantity = Quantity(String::from("0"));
        assert!(quantity.validate().is_ok());
    }

    #[test]
    fn test_validate_negative() {
        let quantity = Quantity(String::from("-123.456"));
        assert!(quantity.validate().is_err());
    }

    #[test]
    fn test_validate_comma_decimal() {
        let quantity = Quantity(String::from("123,456"));
        assert!(quantity.validate().is_ok());
    }

    #[test]
    fn test_validate_non_numeric() {
        let quantity = Quantity(String::from("abc"));
        assert!(quantity.validate().is_err());
    }

    #[test]
    fn test_validate_whitespace() {
        let quantity = Quantity(String::from("  123.456  "));
        assert!(quantity.validate().is_ok());
    }

    #[test]
    fn test_get_value() {
        let value = String::from("123.456");
        let quantity = Quantity(value.clone());
        assert_eq!(quantity.get_value(), &value);
    }

    #[test]
    fn test_display() {
        let quantity = Quantity(String::from("123.456"));
        assert_eq!(format!("{}", quantity), "123.456");
    }

    #[test]
    fn test_deserialize_valid() {
        let json = "\"123.456\"";
        let result: Result<ValueObject<Quantity>, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deserialize_invalid() {
        let json = "\"-123.456\"";
        let result: Result<ValueObject<Quantity>, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
