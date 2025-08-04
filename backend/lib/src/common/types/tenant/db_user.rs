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
use uuid::Uuid;

/// Represents the databse username.
///
/// The `DbUser` struct is a simple wrapper around a `String` that is used
/// to encapsulate the database username. This can
/// help ensure type safety and improve code readability by explicitly
/// conveying the purpose of the contained value.
///
/// # Fields
///
/// * `0`: The inner `String` containing theusername of the database.

#[derive(Debug, PartialEq, Clone)]
pub struct DbUser(String);

impl DbUser {
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

///
/// Validates if the given string is a valid database username.
///
/// A valid database username must adhere to the following rules:
/// 1. It should only contain uppercase letters (`A-Z`), lowercase letters (`a-z`), and digits (`0-9`).
/// 2. The length of the username must be between 40 and 99 characters (inclusive).
///
/// # Arguments
///
/// * `s` - A string slice reference representing the username to validate.
///
/// # Returns
///
/// * `true` - If the username is valid according to the above criteria.
/// * `false` - If the username is invalid or if there is an error while compiling the regex.
fn is_valid_db_user(s: &str) -> bool {
    match Regex::new(r##"^[A-Za-z][A-Za-z0-9_]{0,59}$"##) {
        Ok(re) => re.is_match(s),
        Err(_) => false,
    }
}

impl FromStr for DbUser {
    type Err = String;

    /// Attempts to create an instance of `DbUser` from the given string slice.
    ///
    /// This function validates the provided string to ensure it meets the criteria
    /// for a valid database username. If the string is valid, it constructs a new
    /// `DbUser` instance and returns it wrapped in a `Result::Ok`. Otherwise,
    /// it returns a `Result::Err` containing an error message.
    ///
    /// # Parameters
    /// - `s`: A string slice representing the database username to be validated and used for creating a new `DbUser` instance.
    ///
    /// # Returns
    /// - `Ok(DbUser)`: If the string provided is a valid database username.
    /// - `Err(String)`: If the string is invalid, containing an error message.
    ///
    /// # Errors
    /// - Returns `"Hibás felhasználónév formátum"` as the error message if validation fails.
    ///
    /// # Note
    /// The function `is_valid_db_user(s: &str)` is expected to perform the
    /// validation logic and must be defined elsewhere in the module.
    ///
    /// # Implements
    /// This function is a part of the `FromStr` trait implementation for the `DbUser` type,
    /// enabling string-to-`DbUser` conversions.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_db_user(s) {
            Ok(DbUser(s.to_string()))
        } else {
            Err("Hibás felhasználónév formátum".to_string())
        }
    }
}

// TODO: docs
impl From<Uuid> for DbUser {
    fn from(uuid: Uuid) -> Self {
        DbUser(uuid.to_string())
    }
}

impl TryFrom<String> for DbUser {
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

impl<'de> Deserialize<'de> for DbUser {
    /// A custom implementation of the `deserialize` method
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Display for DbUser {
    /// Implements the `fmt` method for formatting the current type using the `Display` trait.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_db_user() {
        let valid_db_users = vec![r#"db_user"#];
        for db_user in valid_db_users {
            //panic!("{}", host);
            let db_user: DbUser =
                serde_json::from_str(format!("\"{}\"", &db_user).as_str()).unwrap();
            assert_eq!(db_user.as_str(), db_user.as_str());
        }
    }
    #[test]
    fn test_invalid_db_user() {
        let invalid_db_users = vec![r#"4db_user"#, r#"123"#, r#""#, r#" "#];
        for db_user in invalid_db_users {
            let db_user: Result<DbUser, _> =
                serde_json::from_str(format!("\"{}\"", &db_user).as_str());
            assert!(db_user.is_err());
        }
    }
}
