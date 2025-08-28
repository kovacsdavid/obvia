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
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DbPassword(pub String);

impl ValueObjectable for DbPassword {
    type DataType = String;

    /// Validates the database password stored in the current instance.
    ///
    /// # Details
    /// This function checks if the current value (assumed to be a string stored as `self.0`)
    /// matches the required pattern for a valid database password. The password must meet the
    /// following requirements:
    /// 1. Consist only of alphanumeric characters (A-Z, a-z, 0-9).
    /// 2. Be at least 40 characters long but no longer than 99 characters.
    ///
    /// A regular expression is used to perform this validation. If the password matches the
    /// pattern, the function returns `Ok(())`. If it does not match or if there is an issue
    /// creating the regex, the function returns a `Result::Err` with a localized error message.
    ///
    /// # Returns
    /// - `Ok(())` if the password is valid.
    /// - `Err(String)` containing the message `"Hibás adatbázis jelszó!"` (Hungarian for "Invalid database password") if:
    ///   - The password does not match the required pattern.
    ///   - The regular expression could not be created.
    fn validate(&self) -> Result<(), String> {
        match Regex::new(r##"^[A-Za-z0-9]{40,99}$"##) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err("Hibás adatbázis jelszó!".to_string()),
            },
            Err(_) => Err("Hibás adatbázis jelszó!".to_string()),
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

impl Display for DbPassword {
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

impl<'de> Deserialize<'de> for ValueObject<DbPassword> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `DbPassword` and validates it by calling `ValueObject::new`.
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
        ValueObject::new(DbPassword(s)).map_err(serde::de::Error::custom)
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
            let db_password: ValueObject<DbPassword> =
                serde_json::from_str(format!("\"{}\"", &password).as_str()).unwrap();
            assert_eq!(db_password.extract().get_value(), password);
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
            let db_password: Result<ValueObject<DbPassword>, _> =
                serde_json::from_str(format!("\"{}\"", &password).as_str());
            assert!(db_password.is_err());
        }
    }
}
