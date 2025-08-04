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
use serde::Deserialize;
use std::fmt::Display;
use std::str::FromStr;

/// Represents a tenant name.
///
/// The `Name` struct is a simple wrapper around a `String` that is used
/// to encapsulate the tenant name. This can
/// help ensure type safety and improve code readability by explicitly
/// conveying the purpose of the contained value.
#[derive(Debug, PartialEq, Clone)]
pub struct Name(String);

impl Name {
    /// Returns a string slice (`&str`) referencing the inner string data.
    ///
    /// # Notes
    /// - This function borrows the inner string (`self.0`) as a shared reference.
    ///
    /// # Allowance
    /// The `#[allow(dead_code)]` attribute indicates that the function may not always be used and avoids warnings during compilation.
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Checks if the given string is tenant name.
///
/// A valid tenant name:
/// - Contains only alphanumeric characters (A-Z, a-z, 0-9).
/// - Has a length between 1 and 255 characters.
///
/// # Parameters
/// - `s`: A reference to the string (`&str`) to be validated.
///
/// # Returns
/// - `true` if the string matches the specified pattern.
/// - `false` if the string is invalid or there is a problem compiling the regular expression.
///
/// # Note
///
/// This may be too strict but enough for now.
///
/// # Safety
///
/// Do not let any chars here that can be used in an SQLi attack!
fn is_valid_name(s: &str) -> bool {
    match Regex::new(r##"^[A-Za-z0-9]{1,255}$"##) {
        Ok(re) => re.is_match(s),
        Err(_) => false,
    }
}

impl TryFrom<String> for Name {
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

impl FromStr for Name {
    type Err = String;

    /// Attempts to create an instance of `Name` from the given string slice.
    ///
    /// This function validates the provided string to ensure it meets the criteria
    /// for a valid tenant name. If the string is valid, it constructs a new
    /// `Name` instance and returns it wrapped in a `Result::Ok`. Otherwise,
    /// it returns a `Result::Err` containing an error message.
    ///
    /// # Parameters
    /// - `s`: A string slice representing the tenant name to be validated and used for creating a new `Name` instance.
    ///
    /// # Returns
    /// - `Ok(Name)`: If the string provided is a valid tenant name.
    /// - `Err(String)`: If the string is invalid, containing an error message.
    ///
    /// # Errors
    /// - Returns `"Hibás név!"` as the error message if validation fails.
    ///
    /// # Note
    /// The function `is_valid_name(s: &str)` is expected to perform the
    /// validation logic and must be defined elsewhere in the module.
    ///
    /// # Implements
    /// This function is a part of the `FromStr` trait implementation for the `Name` type,
    /// enabling string-to-`Name` conversions.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_name(s) {
            Ok(Name(s.to_string()))
        } else {
            Err("Hibás név!".to_string())
        }
    }
}

impl<'de> Deserialize<'de> for Name {
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
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Display for Name {
    /// Implements the `fmt` method for formatting the current type using the `Display` trait.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_name() {
        let name: Name = serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d""#).unwrap();
        assert_eq!(name.as_str(), "bc5690796fc8414e93e32fcdaae3156d");
    }

    #[test]
    fn test_invalid_name() {
        let name: Result<Name, _> =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d'DROP""#);
        assert!(name.is_err());
        let name: Result<Name, _> =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d;DROP""#);
        assert!(name.is_err());
        let name: Result<Name, _> =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d"DROP""#);
        assert!(name.is_err());
    }
}
