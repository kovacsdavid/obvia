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

use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

/// A struct representing a last name as a simple wrapper around a `String`.
///
/// The `LastName` struct encapsulates a single `String` value representing a last name,
/// providing additional type safety and semantic clarity in code.
#[derive(Debug, PartialEq, Clone)]
pub struct LastName(String);

impl LastName {
    /// Returns a string slice (`&str`) that represents the underlying content of the current instance.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Checks if a given string is a valid last name.
///
/// A valid last name is defined as:
/// 1. Not being empty after trimming leading and trailing whitespace.
/// 2. Only containing alphabetic characters, hyphens (`-`), or spaces (` `).
///
/// # Parameters
/// - `s`: A string slice reference (`&str`) representing the last name to validate.
///
/// # Returns
/// - `true`: If the input string meets the criteria for a valid last name.
/// - `false`: Otherwise.
fn is_valid_last_name(s: &str) -> bool {
    let trimmed = s.trim();
    !trimmed.is_empty()
        && trimmed
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == ' ')
}

impl FromStr for LastName {
    type Err = String;

    /// Attempts to create a `LastName` instance from the provided string slice.
    ///
    /// # Parameters
    /// - `s`: A string slice representing the potential last name.
    ///
    /// # Returns
    /// - `Ok(LastName)`: If the provided string is a valid last name after being trimmed.
    /// - `Err(String)`: If the provided string is deemed invalid. The error contains a descriptive message.
    ///
    /// # Errors
    /// - Returns an error with the message `"Hibás vezetéknév formátum"` (Hungarian for "Invalid last name format")
    ///   if the string does not pass the `is_valid_last_name` validation.
    ///
    /// # Validation
    /// - The function relies on the external function `is_valid_last_name` to determine the validity of the input.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_last_name(s) {
            Ok(LastName(s.trim().to_string()))
        } else {
            Err("Hibás vezetéknév formátum".to_string())
        }
    }
}

impl std::convert::TryFrom<String> for LastName {
    type Error = String;
    
    /// Attempts to create an instance of the type implementing this method from the given `String`.
    ///
    /// This function takes a `String` as input and tries to parse it into the desired type. If 
    /// parsing is successful, it returns `Ok(Self)` containing the created instance.
    /// If parsing fails, it returns a `Result::Err` containing the appropriate error.
    ///
    /// # Arguments
    ///
    /// * `value` - A `String` that represents the source value to be parsed into the target type.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the parsing is successful.
    /// * `Err(Self::Error)` - If the parsing fails, enclosing the error describing the failure.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided `String` cannot be parsed into the target type.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<'de> Deserialize<'de> for LastName {
    /// A custom implementation of the `deserialize` method for a type that can be deserialized 
    /// from a string using the Serde library.
    ///
    /// # Type Parameters:
    /// - `D`: The deserializer type implementing the `serde::Deserializer` trait.
    ///
    /// # Parameters:
    /// - `deserializer`: A deserializer instance to read and interpret the input data
    ///   and convert it into the appropriate type.
    ///
    /// # Returns:
    /// - `Result<Self, D::Error>`: Returns either:
    ///   - The successfully deserialized instance of the type (`Self`).
    ///   - An error of type `D::Error` if deserialization fails.
    ///
    /// # Behavior:
    /// 1. The function first attempts to deserialize the input data into a `String`.
    /// 2. Then, it tries to parse the deserialized string into the target type (`Self`) 
    ///    using the `parse` method.
    /// 3. If parsing fails, an error is returned using `serde::de::Error::custom` to
    ///    generate a descriptive error message.
    ///
    /// # Errors:
    /// - Returns an error if:
    ///   - The input data cannot be deserialized into a `String`.
    ///   - The parsed string cannot be converted into the type being deserialized.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for LastName {
    /// Implements the `fmt` method for formatting the current type using the `Display` trait.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_last_name() {
        let name: LastName = serde_json::from_str(r#""Kovács""#).unwrap();
        assert_eq!(name.as_str(), "Kovács");
        let name: LastName = serde_json::from_str(r#""Kovács-Kovács""#).unwrap();
        assert_eq!(name.as_str(), "Kovács-Kovács");
    }

    #[test]
    fn test_invalid_last_name() {
        let name: Result<LastName, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
        let name: Result<LastName, _> = serde_json::from_str(r#""123""#);
        assert!(name.is_err());
        let name: Result<LastName, _> = serde_json::from_str(r#""Kovács!""#);
        assert!(name.is_err());
    }
}
