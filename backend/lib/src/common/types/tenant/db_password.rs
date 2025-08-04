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

/// Represents the database password.
///
/// The `DbPassword` struct is a simple wrapper around a `String` that is used
/// to encapsulate the database password. This can
/// help ensure type safety and improve code readability by explicitly
/// conveying the purpose of the contained value.
///
/// # Fields
///
/// * `0`: The inner `String` containing the hostname or address of the database.

#[derive(Debug, PartialEq, Clone)]
pub struct DbPassword(String);

impl DbPassword {
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
/// Validates if the given string is a valid database password.
///
/// A valid database password must adhere to the following rules:
/// 1. It should only contain uppercase letters (`A-Z`), lowercase letters (`a-z`), and digits (`0-9`).
/// 2. The length of the password must be between 40 and 99 characters (inclusive).
///
/// # Arguments
///
/// * `s` - A string slice reference representing the password to validate.
///
/// # Returns
///
/// * `true` - If the password is valid according to the above criteria.
/// * `false` - If the password is invalid or if there is an error while compiling the regex.
fn is_valid_db_password(s: &str) -> bool {
    match Regex::new(r##"^[A-Za-z0-9]{40,99}$"##) {
        Ok(re) => re.is_match(s),
        Err(_) => false,
    }
}

impl TryFrom<String> for DbPassword {
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

impl FromStr for DbPassword {
    type Err = String;

    /// Attempts to create an instance of `DbPassword` from the given string slice.
    ///
    /// This function validates the provided string to ensure it meets the criteria
    /// for a valid database password. If the string is valid, it constructs a new
    /// `DbPassword` instance and returns it wrapped in a `Result::Ok`. Otherwise,
    /// it returns a `Result::Err` containing an error message.
    ///
    /// # Parameters
    /// - `s`: A string slice representing the database password to be validated and used for creating a new `DbPassword` instance.
    ///
    /// # Returns
    /// - `Ok(DbPassword)`: If the string provided is a valid database password.
    /// - `Err(String)`: If the string is invalid, containing an error message.
    ///
    /// # Errors
    /// - Returns `"Hibás adatbázis jelszó!"` as the error message if validation fails.
    ///
    /// # Note
    /// The function `is_valid_db_password(s: &str)` is expected to perform the
    /// validation logic and must be defined elsewhere in the module.
    ///
    /// # Implements
    /// This function is a part of the `FromStr` trait implementation for the `DbPassword` type,
    /// enabling string-to-`DbPassword` conversions.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_db_password(s) {
            Ok(DbPassword(s.to_string()))
        } else {
            Err("Hibás adatbázis jelszó!".to_string())
        }
    }
}

impl<'de> Deserialize<'de> for DbPassword {
    /// A custom implementation of the `deserialize` method
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Display for DbPassword {
    /// Implements the `fmt` method for formatting the current type using the `Display` trait.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_db_password() {
        let valid_passwords = vec![r#"RpehL35tQxnG6fgST0FQUnqHhkaqVOtTgflqArsl"#];
        for password in valid_passwords {
            //panic!("{}", host);
            let db_password: DbPassword =
                serde_json::from_str(format!("\"{}\"", &password).as_str()).unwrap();
            assert_eq!(db_password.as_str(), password);
        }
    }
    #[test]
    fn test_invalid_db_password() {
        let invalid_passwords = vec![
            r#"4v4X;yk+|hJH3QLL>%7S-~]%1A2}|Fh_:`p[#QPL+!"#,
            r#"password"#,
            r#"jwP9rF3xa4hvTk1m2PLVobm99gXv5BSdmuEeTvmCxB9YgIVDwYjwP9rF3xa4hvTk1m2PLVobm99gXv5BSdmuEeTvmCxB9YgIVDwY"#,
            r#"gyoP823CB0e7GFx5FUNqscBQg76VdXwcMRD6vsP"#,
            r#""#,
            r#" "#,
        ];
        for password in invalid_passwords {
            let db_password: Result<DbPassword, _> =
                serde_json::from_str(format!("\"{}\"", &password).as_str());
            assert!(db_password.is_err());
        }
    }
}
