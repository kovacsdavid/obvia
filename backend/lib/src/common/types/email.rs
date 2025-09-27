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
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A struct representing an email address as a simple wrapper around a `String`.
///
/// The `Email` struct encapsulates a single `String` value representing an email address,
/// providing additional type safety and semantic clarity in code.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Email(pub String);

impl ValueObjectable for Email {
    type DataType = String;

    /// Validates whether the email address stored in the `self.0` field matches
    /// the specified email format using a regular expression. The function
    /// ensures that the provided email adheres to standard email format rules,
    /// including both local part and domain structure. The regex used
    /// is sourced from [emailregex.com](https://emailregex.com/).
    ///
    /// # Returns
    /// - `Ok(())`: If the email address is valid and matches the given format.
    /// - `Err(String)`: If the email address does not match the valid format,
    ///   or if there is an error compiling the regular expression. The error
    ///   message returned in these cases is:
    ///   `"A megadott e-mail cím formátuma nem megfelelő"`.
    ///
    /// # Errors
    /// - Returns an error if the regular expression cannot be compiled.
    /// - Returns an error if the email address fails to match the regex.
    ///
    /// # Note:
    ///
    /// The regular expression used here conforms closely to RFC 5322 official
    /// kemail address specifications, allowing complex and valid email formats.
    /// However, it may still reject some valid edge cases or accept certain invalid
    /// entries in rare scenarios.
    fn validate(&self) -> Result<(), String> {
        match Regex::new(
            r##"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"##,
        ) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err("A megadott e-mail cím formátuma nem megfelelő".to_string()),
            },
            Err(_) => Err("A megadott e-mail cím formátuma nem megfelelő".to_string()),
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

impl<'de> Deserialize<'de> for ValueObject<Email> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `Email` and validates it by calling `ValueObject::new`.
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
        ValueObject::new(Email(s)).map_err(serde::de::Error::custom)
    }
}

impl Display for Email {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_email() {
        let email: ValueObject<Email> = serde_json::from_str(r#""user@example.com""#).unwrap();
        assert_eq!(email.extract().get_value(), "user@example.com");
    }

    #[test]
    fn test_invalid_email() {
        let email: Result<ValueObject<Email>, _> = serde_json::from_str(r#""not-an-email""#);
        assert!(email.is_err());
    }
}
