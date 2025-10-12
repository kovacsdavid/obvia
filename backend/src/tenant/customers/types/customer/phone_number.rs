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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PhoneNumber(pub String);

impl ValueObjectable for PhoneNumber {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        match Regex::new(r##"^\+[1-9]\d{4,15}$"##) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err(String::from("Hibás telefonszám formátum")),
            },
            Err(_) => Err(String::from("Hibás telefonszám formátum")),
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

impl Display for PhoneNumber {
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

impl<'de> Deserialize<'de> for ValueObject<PhoneNumber> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `Name` and validates it by calling `ValueObject::new`.
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
        ValueObject::new(PhoneNumber(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_phone_number() {
        let phone: ValueObject<PhoneNumber> = serde_json::from_str(r#""+36301234567""#).unwrap();
        assert_eq!(phone.extract().get_value(), "+36301234567");
    }

    #[test]
    fn test_invalid_phone_number_no_plus() {
        let phone: Result<ValueObject<PhoneNumber>, _> = serde_json::from_str(r#""36301234567""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_invalid_phone_number_too_short() {
        let phone: Result<ValueObject<PhoneNumber>, _> = serde_json::from_str(r#""+3612""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_invalid_phone_number_too_long() {
        let phone: Result<ValueObject<PhoneNumber>, _> =
            serde_json::from_str(r#""+361234567890123456""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_invalid_phone_number_special_chars() {
        let phone: Result<ValueObject<PhoneNumber>, _> =
            serde_json::from_str(r#""+36-30-123-4567""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_invalid_phone_number_letters() {
        let phone: Result<ValueObject<PhoneNumber>, _> = serde_json::from_str(r#""+36abcd1234""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_display_implementation() {
        let phone = PhoneNumber("+36301234567".to_string());
        assert_eq!(format!("{}", phone), "+36301234567");
    }

    #[test]
    fn test_clone() {
        let phone = PhoneNumber("+36301234567".to_string());
        let cloned = phone.clone();
        assert_eq!(phone, cloned);
    }

    #[test]
    fn test_debug_output() {
        let phone = PhoneNumber("+36301234567".to_string());
        assert_eq!(format!("{:?}", phone), r#"PhoneNumber("+36301234567")"#);
    }

    #[test]
    fn test_validation() {
        let valid_phone = PhoneNumber("+36301234567".to_string());
        assert!(valid_phone.validate().is_ok());

        let invalid_phone = PhoneNumber("36301234567".to_string());
        assert!(invalid_phone.validate().is_err());
    }

    #[test]
    fn test_get_value() {
        let value = "+36301234567".to_string();
        let phone = PhoneNumber(value.clone());
        assert_eq!(phone.get_value(), &value);
    }

    #[test]
    fn test_deserialization_error_messages() {
        let invalid: Result<ValueObject<PhoneNumber>, _> = serde_json::from_str(r#""invalid""#);
        assert!(invalid.unwrap_err().to_string().contains("formátum"));
    }
}
