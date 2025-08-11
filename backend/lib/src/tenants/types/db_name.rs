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
use regex::Regex;
use serde::Deserialize;
use std::fmt::Display;
use uuid::Uuid;

/// Represents the database name.
///
/// The `DbName` struct is a simple wrapper around a `String` that is used
/// to encapsulate the name of a database server. This can
/// help ensure type safety and improve code readability by explicitly
/// conveying the purpose of the contained value.
///
/// # Fields
///
/// * `0`: The inner `String` containing the hostname or address of the database.
#[derive(Debug, PartialEq, Clone)]
pub struct DbName(pub String);

impl ValueObjectable for DbName {
    type DataType = String;

    /// Validates the database name stored in the object.
    ///
    /// # Description
    /// This method checks whether the contained string meets the following criteria:
    /// - It only contains alphanumeric characters (uppercase and lowercase letters, and digits).
    /// - Its length is between 1 and 99 characters (inclusive).
    ///
    /// The function uses a regular expression to ensure the string adheres to these rules.
    ///
    /// # Returns
    /// - `Ok(())` if the database name is valid.
    /// - `Err(String)` with an error message ("Hibás adatbázis név!") if the validation fails.
    ///
    /// # Errors
    /// - Returns an error message if the regular expression itself cannot be created (though this situation is highly unlikely since the regex pattern is constant and valid).
    fn validate(&self) -> Result<(), String> {
        match Regex::new(r##"^tenant_[A-Za-z0-9]{1,50}$"##) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err("Hibás adatbázis név!".to_string()),
            },
            Err(_) => Err("Hibás adatbázis név!".to_string()),
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

impl Display for DbName {
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

impl<'de> Deserialize<'de> for ValueObject<DbName> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `DbName` and validates it by calling `ValueObject::new`.
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
        ValueObject::new(DbName(s)).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<Uuid> for ValueObject<DbName> {
    type Error = String;
    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        ValueObject::new(DbName(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_db_name() {
        let valid_names = vec![r#"tenant_mydatabase"#];
        for name in valid_names {
            //panic!("{}", host);
            let db_name: ValueObject<DbName> =
                serde_json::from_str(format!("\"{}\"", &name).as_str()).unwrap();
            assert_eq!(db_name.extract().to_string(), name);
        }
    }
    #[test]
    fn test_invalid_db_name() {
        let invalid_names = vec![
            r#"'DROP"#,
            r#""DROP"#,
            r#";DROP"#,
            r#"-DROP"#,
            r#"--DROP"#,
            r#""#,
            r#" "#,
        ];
        for name in invalid_names {
            let db_name: Result<ValueObject<DbName>, _> =
                serde_json::from_str(format!("\"{}\"", &name).as_str());
            assert!(db_name.is_err());
        }
    }
}
