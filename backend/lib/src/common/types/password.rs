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

/// A struct representing a password as a simple wrapper around a `String`.
///
/// The `Password` struct encapsulates a single `String` value representing a password,
/// providing additional type safety and semantic clarity in code.
#[derive(Debug, PartialEq, Clone)]
pub struct Password(String);

impl Password {
    /// Returns a string slice (`&str`) that represents the underlying content of the current instance.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Checks if a given password string meets specific validity criteria.
///
/// A password is considered valid if it satisfies the following requirements:
/// 1. It is at least 8 characters long.
/// 2. It contains at least one alphabetic character (a-z or A-Z).
/// 3. It contains at least one numeric digit (0-9).
///
/// # Arguments
///
/// * `s` - A string slice representing the password to be checked.
///
/// # Returns
///
/// * `true` if the password meets all the criteria.
/// * `false` otherwise.
fn is_valid_password(s: &str) -> bool {
    let len_ok = s.len() >= 8;
    let has_letter = s.chars().any(|c| c.is_alphabetic());
    let has_digit = s.chars().any(|c| c.is_ascii_digit());
    len_ok && has_letter && has_digit
}

impl FromStr for Password {
    type Err = String;
    
    /// Attempts to create a `Password` instance from the provided string slice.
    ///
    /// # Parameters
    /// - `s`: A string slice representing the potential password.
    ///
    /// # Returns
    /// - `Ok(Password)`: If the provided string is a valid password
    /// - `Err(String)`: If the provided string is deemed invalid. The error contains a descriptive message.
    ///
    /// # Errors
    /// - Returns an error with a message in Hungarian explaining that the password must be at least 8 characters long
    ///   and contain both letters and numbers if the string does not pass the `is_valid_password` validation.
    ///
    /// # Validation
    /// - The function relies on the external function `is_valid_password` to determine the validity of the input.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_password(s) {
            Ok(Password(s.to_owned()))
        } else {
            Err("A jelszónak legalább 8 karakter hosszúnak kell lennie és tartalmaznia kell betűket és számokat".to_string())
        }
    }
}

impl std::convert::TryFrom<String> for Password {
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

impl<'de> Deserialize<'de> for Password {
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

impl fmt::Display for Password {
    /// Implements the `fmt` method for formatting the current type using the `Display` trait.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "********")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_password() {
        let pw: Password = serde_json::from_str(r#""abc12345""#).unwrap();
        assert_eq!(pw.as_str(), "abc12345");
        let pw: Password = serde_json::from_str(r#""Password1""#).unwrap();
        assert_eq!(pw.as_str(), "Password1");
    }

    #[test]
    fn test_invalid_password() {
        // Too short
        assert!(serde_json::from_str::<Password>(r#""a1b2c3""#).is_err());
        // No digit
        assert!(serde_json::from_str::<Password>(r#""abcdefgh""#).is_err());
        // No letter
        assert!(serde_json::from_str::<Password>(r#""12345678""#).is_err());
        // Empty
        assert!(serde_json::from_str::<Password>(r#""""#).is_err());
    }

    #[test]
    fn test_display_masks_password() {
        let pw: Password = "abc12345".parse().unwrap();
        assert_eq!(format!("{}", pw), "********");
    }
}
