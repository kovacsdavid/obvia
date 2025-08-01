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

use regex::Regex;
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

/// A struct representing an email address as a simple wrapper around a `String`.
///
/// The `Email` struct encapsulates a single `String` value representing an email address,
/// providing additional type safety and semantic clarity in code.
#[derive(Debug, PartialEq, Clone)]
pub struct Email(String);

impl Email {
    /// Returns a string slice (`&str`) that represents the underlying content of the current instance.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validates whether a given string is a properly formatted email address.
///
/// This function takes a string slice as input and checks if it conforms to
/// the standard email format defined by a regular expression. The regex used
/// is sourced from [emailregex.com](https://emailregex.com/).
///
/// # Arguments
/// * `s` - A string slice (`&str`) representing the email address to validate.
///
/// # Returns
/// * `true` if the input string matches the email format.
/// * `false` if the input string does not match the email format or if the
///   regular expression fails to compile.
fn is_valid_email(s: &str) -> bool {
    match Regex::new(
        r##"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"##,
    ) {
        Ok(re) => re.is_match(s),
        Err(_) => false,
    }
}

impl FromStr for Email {
    type Err = String;

    /// Converts a string slice (`&str`) into an `Email` object, validating its format.
    ///
    /// # Parameters
    /// - `s`: A string slice containing the email address to validate and convert.
    ///
    /// # Returns
    /// - `Ok(Email)`: If the provided string is a valid email address, this will wrap and return
    ///   the string as an `Email` object.
    /// - `Err(String)`: If the provided string is not a valid email address, this returns an
    ///   error message as a `String`, indicating the format is incorrect.
    ///
    /// # Errors
    /// This function will return an error if the provided string does not meet the
    /// criteria for a valid email address.
    ///
    /// # Notes
    /// The email validation logic relies on the `is_valid_email` function, which should
    /// ensure compliance with the desired email address format.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_email(s) {
            Ok(Email(s.to_string()))
        } else {
            Err("A megadott e-mail cím formátuma nem megfelelő".to_string())
        }
    }
}

impl TryFrom<String> for Email {
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

impl<'de> Deserialize<'de> for Email {
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

impl fmt::Display for Email {
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
    fn test_valid_email() {
        let email: Email = serde_json::from_str(r#""user@example.com""#).unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_invalid_email() {
        let email: Result<Email, _> = serde_json::from_str(r#""not-an-email""#);
        assert!(email.is_err());
    }
}
