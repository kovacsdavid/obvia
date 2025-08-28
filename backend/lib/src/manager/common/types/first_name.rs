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
use crate::manager::common::types::value_object::{ValueObject, ValueObjectable};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A struct representing a first name as a simple wrapper around a `String`.
///
/// The `FirstName` struct encapsulates a single `String` value representing a first name,
/// providing additional type safety and semantic clarity in code.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct FirstName(pub String);

impl ValueObjectable for FirstName {
    type DataType = String;
    /// Validates the format of a first name by ensuring it meets certain criteria:
    /// - The string is not empty after trimming whitespace.
    /// - The string contains only alphabetic characters, hyphens ('-'), or spaces (' ').
    ///
    /// # Returns
    /// * `Ok(())` - If the first name meets all the validation criteria.
    /// * `Err(String)` - If the first name fails validation, returning an error message.
    ///
    /// # Error
    /// Will return `"Hibás keresztnév formátum"` if the first name is invalid.
    fn validate(&self) -> Result<(), String> {
        let trimmed = self.0.trim();
        match !trimmed.is_empty()
            && trimmed
                .chars()
                .all(|c| c.is_alphabetic() || c == '-' || c == ' ')
        {
            true => Ok(()),
            false => Err("Hibás keresztnév formátum".to_string()),
        }
    }
    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for FirstName {
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

impl<'de> Deserialize<'de> for ValueObject<FirstName> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `FirstName` and validates it by calling `ValueObject::new`.
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
        ValueObject::new(FirstName(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_first_name() {
        let name: ValueObject<FirstName> = serde_json::from_str(r#""Dávid""#).unwrap();
        assert_eq!(name.extract().get_value(), "Dávid");
        let name: ValueObject<FirstName> = serde_json::from_str(r#""Eleonóra Tímea""#).unwrap();
        assert_eq!(name.extract().get_value(), "Eleonóra Tímea");
    }

    #[test]
    fn test_invalid_first_name() {
        let name: Result<ValueObject<FirstName>, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
        let name: Result<ValueObject<FirstName>, _> = serde_json::from_str(r#""123""#);
        assert!(name.is_err());
        let name: Result<ValueObject<FirstName>, _> = serde_json::from_str(r#""Dávid!""#);
        assert!(name.is_err());
    }
}
