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
pub struct Email(pub String);

impl ValueObjectData for Email {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match Regex::new(
            r##"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"##,
        ) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err(ValueObjectError::InvalidInput(
                    "A megadott e-mail cím formátuma nem megfelelő",
                )),
            },
            Err(_) => Err(ValueObjectError::InvalidInput(
                "A megadott e-mail cím formátuma nem megfelelő",
            )),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ValueObject<Email> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(Email(s)).map_err(serde::de::Error::custom)
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::ValueObject;

    #[test]
    fn test_valid_email() {
        let email: ValueObject<Email> = serde_json::from_str(r#""test@example.com""#).unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    #[test]
    fn test_invalid_email_missing_at() {
        let email: Result<ValueObject<Email>, _> = serde_json::from_str(r#""testexample.com""#);
        assert!(email.is_err());
    }

    #[test]
    fn test_invalid_email_missing_domain() {
        let email: Result<ValueObject<Email>, _> = serde_json::from_str(r#""test@""#);
        assert!(email.is_err());
    }

    #[test]
    fn test_invalid_email_special_chars() {
        let email: Result<ValueObject<Email>, _> = serde_json::from_str(r#""test!$%@example.com""#);
        assert!(email.is_ok());
    }

    #[test]
    fn test_valid_email_with_subdomain() {
        let email: ValueObject<Email> = serde_json::from_str(r#""test@sub.example.com""#).unwrap();
        assert_eq!(email.as_str(), "test@sub.example.com");
    }

    #[test]
    fn test_display_implementation() {
        let email = Email("test@example.com".to_string());
        assert_eq!(format!("{}", email), "test@example.com");
    }
}
