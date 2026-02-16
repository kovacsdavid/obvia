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
use crate::common::types::{ValueObject, ValueObjectable, value_object::ValueObjectError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Password(pub String);

impl ValueObjectable for Password {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        let len_ok = self.0.len() >= 8 && self.0.len() <= 128;
        let has_letter = self.0.chars().any(|c| c.is_alphabetic());
        let has_digit = self.0.chars().any(|c| c.is_ascii_digit());
        let result = len_ok && has_letter && has_digit;
        match result {
            true => Ok(()),
            false => Err(ValueObjectError::InvalidInput(
                "A jelszónak legalább 8 karakter hosszúnak kell lennie és tartalmaznia kell betűket és számokat",
            )),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ValueObject<Password> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(Password(s)).map_err(serde::de::Error::custom)
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "********")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let pwd: ValueObject<Password> = serde_json::from_str(r#""password123""#).unwrap();
        assert_eq!(pwd.as_str(), "password123");
    }

    #[test]
    fn test_invalid_password_too_short() {
        let pwd: Result<ValueObject<Password>, _> = serde_json::from_str(r#""pass1""#);
        assert!(pwd.is_err());
    }

    #[test]
    fn test_invalid_password_no_letters() {
        let pwd: Result<ValueObject<Password>, _> = serde_json::from_str(r#""12345678""#);
        assert!(pwd.is_err());
    }

    #[test]
    fn test_invalid_password_no_numbers() {
        let pwd: Result<ValueObject<Password>, _> = serde_json::from_str(r#""password""#);
        assert!(pwd.is_err());
    }

    #[test]
    fn test_password_display() {
        let pwd = Password("secret123".to_string());
        assert_eq!(format!("{}", pwd), "********");
    }
}
