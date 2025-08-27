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
use regex::Regex;
use serde::Deserialize;
use std::fmt::Display;
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
pub struct DbUser(pub String);

impl ValueObjectable for DbUser {
    type DataType = String;

    /// Validates the format of a username using a regular expression.
    ///
    /// # Returns
    /// - `Ok(())`: If the username matches the required format.
    /// - `Err(String)`: If the username does not match the required format,
    ///   or if there is an error creating the regular expression.
    ///
    /// The valid username format follows these rules:
    /// - Must start with a letter (A-Z or a-z).
    /// - May only contain alphanumeric characters (A-Z, a-z, 0-9) and underscores (_).
    /// - Must be between 1 and 60 characters in length, inclusive.
    ///
    /// # Errors
    /// Returns an error string `"Hibás felhasználónév formátum"` if:
    /// - The username format is invalid.
    /// - There is an issue creating the regular expression.
    fn validate(&self) -> Result<(), String> {
        match Regex::new(r##"^tenant_[A-Za-z0-9_]{1,50}$"##) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err("Hibás felhasználónév formátum".to_string()),
            },
            Err(_) => Err("Hibás felhasználónév formátum".to_string()),
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

impl Display for DbUser {
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

impl<'de> Deserialize<'de> for ValueObject<DbUser> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `DbUser` and validates it by calling `ValueObject::new`.
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
        ValueObject::new(DbUser(s)).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<Uuid> for ValueObject<DbUser> {
    type Error = String;
    /// Attempts to create an instance of `Self` from a given `Uuid` value.
    ///
    /// This function converts a `Uuid` into a `DbUser`, which is then wrapped
    /// in a `ValueObject`. It leverages the `ValueObject::new` method to perform
    /// the construction. If the conversion is successful, a `Result::Ok(Self)`
    /// is returned; otherwise, a `Result::Err(Self::Error)` is returned.
    ///
    /// # Parameters
    /// - `value`: A `Uuid` instance that will be converted into the custom type.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the conversion and the creation of the `ValueObject` are successful.
    /// - `Err(Self::Error)`: If there is an error during the creation of the `ValueObject`.
    ///
    /// # Errors
    /// This function will return an error if the `ValueObject::new` fails for any reason.
    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        ValueObject::new(DbUser(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_db_user() {
        let valid_db_users = vec![r#"tenant_db_user"#];
        for db_user in valid_db_users {
            //panic!("{}", host);
            let not_a_db_user_shadow: ValueObject<DbUser> =
                serde_json::from_str(format!("\"{}\"", &db_user).as_str()).unwrap();
            assert_eq!(
                *not_a_db_user_shadow.extract().get_value(),
                db_user.to_string()
            );
        }
    }
    #[test]
    fn test_invalid_db_user() {
        let invalid_db_users = vec![r#"4db_user"#, r#"123"#, r#""#, r#" "#];
        for db_user in invalid_db_users {
            let db_user: Result<ValueObject<DbUser>, _> =
                serde_json::from_str(format!("\"{}\"", &db_user).as_str());
            assert!(db_user.is_err());
        }
    }
}
