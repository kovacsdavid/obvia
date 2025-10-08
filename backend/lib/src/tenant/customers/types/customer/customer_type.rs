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
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct CustomerType(pub String);

impl ValueObjectable for CustomerType {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if &self.0 == "natural" || &self.0 == "legal" {
            Ok(())
        } else {
            Err(String::from("Hibás vevő típus!"))
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

impl Display for CustomerType {
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

impl<'de> Deserialize<'de> for ValueObject<CustomerType> {
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
        ValueObject::new(CustomerType(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_natural_customer_type() {
        let customer_type: ValueObject<CustomerType> = serde_json::from_str(r#""natural""#).unwrap();
        assert_eq!(customer_type.extract().get_value(), "natural");
    }

    #[test]
    fn test_valid_legal_customer_type() {
        let customer_type: ValueObject<CustomerType> = serde_json::from_str(r#""legal""#).unwrap();
        assert_eq!(customer_type.extract().get_value(), "legal");
    }

    #[test]
    fn test_invalid_customer_type() {
        let customer_type: Result<ValueObject<CustomerType>, _> = serde_json::from_str(r#""invalid""#);
        assert!(customer_type.is_err());
    }

    #[test]
    fn test_empty_customer_type() {
        let customer_type: Result<ValueObject<CustomerType>, _> = serde_json::from_str(r#""""#);
        assert!(customer_type.is_err());
    }

    #[test]
    fn test_display_implementation() {
        let customer_type = CustomerType("natural".to_string());
        assert_eq!(format!("{}", customer_type), "natural");
    }

    #[test]
    fn test_value_getter() {
        let customer_type = CustomerType("legal".to_string());
        assert_eq!(customer_type.get_value(), "legal");
    }

    #[test]
    fn test_validation() {
        let valid_natural = CustomerType("natural".to_string());
        let valid_legal = CustomerType("legal".to_string());
        let invalid = CustomerType("invalid".to_string());

        assert!(valid_natural.validate().is_ok());
        assert!(valid_legal.validate().is_ok());
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_validation_error_message() {
        let invalid = CustomerType("invalid".to_string());
        let result = invalid.validate();
        assert_eq!(result.unwrap_err(), "Hibás vevő típus!");
    }
}
