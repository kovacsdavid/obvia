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

use crate::common::types::{ValueObject, ValueObjectData, value_object::ValueObjectError};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ContactPhone(pub String);

impl ValueObjectData for ContactPhone {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.trim().is_empty() {
            Ok(())
        } else {
            match Regex::new(r##"^\+[1-9]\d{4,15}$"##) {
                Ok(re) => match re.is_match(&self.0) {
                    true => Ok(()),
                    false => Err(ValueObjectError::InvalidInput("Hibás telefonszám formátum")),
                },
                Err(_) => Err(ValueObjectError::InvalidInput("Hibás telefonszám formátum")),
            }
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for ContactPhone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<ContactPhone> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(ContactPhone(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phone_number() {
        let phone: ValueObject<ContactPhone> = serde_json::from_str(r#""+36301234567""#).unwrap();
        assert_eq!(phone.as_str(), "+36301234567");
    }

    #[test]
    fn test_valid_empty_phone() {
        let phone: ValueObject<ContactPhone> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(phone.as_str(), "");
    }

    #[test]
    fn test_invalid_phone_without_plus() {
        let phone: Result<ValueObject<ContactPhone>, _> = serde_json::from_str(r#""36301234567""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_invalid_phone_too_short() {
        let phone: Result<ValueObject<ContactPhone>, _> = serde_json::from_str(r#""+3612""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_invalid_phone_too_long() {
        let phone: Result<ValueObject<ContactPhone>, _> =
            serde_json::from_str(r#""+361234567890123456789""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_invalid_phone_with_letters() {
        let phone: Result<ValueObject<ContactPhone>, _> = serde_json::from_str(r#""+3630abc1234""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_invalid_phone_with_spaces() {
        let phone: Result<ValueObject<ContactPhone>, _> =
            serde_json::from_str(r#""+36 30 123 4567""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_invalid_phone_with_special_chars() {
        let phone: Result<ValueObject<ContactPhone>, _> =
            serde_json::from_str(r#""+36-30-123-4567""#);
        assert!(phone.is_err());
    }

    #[test]
    fn test_display_formatting() {
        let phone = ContactPhone(String::from("+36301234567"));
        assert_eq!(format!("{}", phone), "+36301234567");
    }

    #[test]
    fn test_validation_error_message() {
        let phone = ContactPhone(String::from("invalid"));
        assert_eq!(
            phone.validate().err().unwrap(),
            ValueObjectError::InvalidInput("Hibás telefonszám formátum")
        );
    }
}
