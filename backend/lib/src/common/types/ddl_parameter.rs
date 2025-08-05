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

/// Represents a Data Definition Language (DDL) parameter.
///
/// This struct is used to encapsulate a single DDL parameter as a string for security purposes
/// because in Postgres you can not bind params to DDL queries.
///
/// # Security
///
/// Always use this struct if parameter bindig is not possible to prevent SQL injection attacks!
#[derive(Debug, PartialEq, Clone)]
pub struct DdlParameter(pub String);

impl ValueObjectable for DdlParameter {
    type DataType = String;

    /// Validates the current instance of the struct.
    ///
    /// This function checks if the instance (assumed to be a string stored in `self.0`) matches a regular
    /// expression pattern. The pattern ensures that the string:
    /// - Consists only of alphanumeric characters (A-Za-z0-9).
    /// - Has a length between 1 and 255 characters.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the string matches the regular expression pattern.
    /// - `Err(String)`: If the string does not match the pattern, or if there is an error in creating
    ///   the regular expression.
    ///
    /// # Errors
    ///
    /// - Returns an error with the message `"Hibás DDL paraméter!"` if the string does not adhere to the
    ///   defined validation rules or if the regular expression fails to compile.
    ///
    /// # Note
    /// The error messages returned are in Hungarian, and the message `"Hibás DDL paraméter!"` translates to
    /// `"Invalid DDL parameter!"` in English. It is indicating that the provided string is not valid.
    fn validate(&self) -> Result<(), String> {
        match Regex::new(r##"^[A-Za-z0-9]{1,255}$"##) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err("Hibás DDL paraméter!".to_string()),
            },
            Err(_) => Err("Hibás DDL paraméter!".to_string()),
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

impl Display for DdlParameter {
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

impl<'de> Deserialize<'de> for ValueObject<DdlParameter> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `DdlParameter` and validates it by calling `ValueObject::new`.
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
        ValueObject::new(DdlParameter(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_ddl_parameter() {
        let ddl_parameter: ValueObject<DdlParameter> =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d""#).unwrap();
        assert_eq!(
            ddl_parameter.extract().get_value(),
            "bc5690796fc8414e93e32fcdaae3156d"
        );
    }

    #[test]
    fn test_invalid_ddl_parameter() {
        let ddl_parameter: Result<ValueObject<DdlParameter>, _> =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d'DROP""#);
        assert!(ddl_parameter.is_err());
        let ddl_parameter: Result<ValueObject<DdlParameter>, _> =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d;DROP""#);
        assert!(ddl_parameter.is_err());
        let ddl_parameter: Result<ValueObject<DdlParameter>, _> =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d"DROP""#);
        assert!(ddl_parameter.is_err());
        let ddl_parameter: Result<ValueObject<DdlParameter>, _> = serde_json::from_str(r#""""#);
        assert!(ddl_parameter.is_err());
        let ddl_parameter: Result<ValueObject<DdlParameter>, _> = serde_json::from_str(r#"" ""#);
        assert!(ddl_parameter.is_err());
    }
}
