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
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Order(pub String);

impl ValueObjectable for Order {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        match self.0.trim() {
            "asc" | "desc" => Ok(()),
            _ => Err("Hibás sorrend formátum".to_string()),
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

impl FromStr for Order {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Order(s.to_string()))
    }
}

impl<'de> Deserialize<'de> for ValueObject<Order> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `Email` and validates it by calling `ValueObject::new`.
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
        ValueObject::new(Order(s)).map_err(serde::de::Error::custom)
    }
}

impl Display for Order {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_asc_order() {
        let order: ValueObject<Order> = serde_json::from_str(r#""asc""#).unwrap();
        assert_eq!(order.extract().get_value(), "asc");
    }

    #[test]
    fn test_valid_desc_order() {
        let order: ValueObject<Order> = serde_json::from_str(r#""desc""#).unwrap();
        assert_eq!(order.extract().get_value(), "desc");
    }

    #[test]
    fn test_invalid_order() {
        let order: Result<ValueObject<Order>, _> = serde_json::from_str(r#""invalid""#);
        assert!(order.is_err());
    }

    #[test]
    fn test_empty_order() {
        let order: Result<ValueObject<Order>, _> = serde_json::from_str(r#""""#);
        assert!(order.is_err());
    }

    #[test]
    fn test_to_string() {
        let order = Order("asc".to_string());
        assert_eq!(order.to_string(), "asc");
    }
}
