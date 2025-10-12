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
pub struct UnitsOfMeasure(pub String);

impl ValueObjectable for UnitsOfMeasure {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if self.0.trim().is_empty() {
            return Err("A mező kitöltése kötelező".to_string());
        }
        if self.0.trim().len() > 50 {
            return Err("A mező maximum 50 karakter hosszú lehet".to_string());
        }
        Ok(())
    }

    /// Retrieves a reference to the value contained within the struct.
    ///
    /// # Returns
    /// A reference to the internal value of type `Self::DataType`.
    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for UnitsOfMeasure {
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

impl<'de> Deserialize<'de> for ValueObject<UnitsOfMeasure> {
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
        ValueObject::new(UnitsOfMeasure(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_unit_of_measure() {
        let uom: ValueObject<UnitsOfMeasure> = serde_json::from_str(r#""kg""#).unwrap();
        assert_eq!(uom.extract().get_value(), "kg");
    }

    #[test]
    fn test_empty_unit_of_measure() {
        let uom: Result<ValueObject<UnitsOfMeasure>, _> = serde_json::from_str(r#""""#);
        assert!(uom.is_err());
    }

    #[test]
    fn test_whitespace_only_unit_of_measure() {
        let uom: Result<ValueObject<UnitsOfMeasure>, _> = serde_json::from_str(r#"" ""#);
        assert!(uom.is_err());
    }

    #[test]
    fn test_too_long_unit_of_measure() {
        let long_str = "a".repeat(51);
        let uom: Result<ValueObject<UnitsOfMeasure>, _> =
            serde_json::from_str(&format!(r#""{}""#, long_str));
        assert!(uom.is_err());
    }

    #[test]
    fn test_max_length_unit_of_measure() {
        let max_str = "a".repeat(50);
        let uom: Result<ValueObject<UnitsOfMeasure>, _> =
            serde_json::from_str(&format!(r#""{}""#, max_str));
        assert!(uom.is_ok());
    }

    #[test]
    fn test_special_characters() {
        let cases = vec![r#""kg/m²""#, r#""°C""#, r#""m³""#, r#""μm""#];
        for case in cases {
            let uom: Result<ValueObject<UnitsOfMeasure>, _> = serde_json::from_str(case);
            assert!(uom.is_ok());
        }
    }

    #[test]
    fn test_display_format() {
        let uom = UnitsOfMeasure("kg".to_string());
        assert_eq!(format!("{}", uom), "kg");
    }

    #[test]
    fn test_deserialization_error_handling() {
        let invalid_json = r#"{"unit": "kg"}"#;
        let uom: Result<ValueObject<UnitsOfMeasure>, _> = serde_json::from_str(invalid_json);
        assert!(uom.is_err());
    }
}
