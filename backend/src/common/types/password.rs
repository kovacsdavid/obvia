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

use crate::common::value_object::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Password(String);

impl ValueObjectData for Password {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
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

    fn get_data(&self) -> &Self::DataType {
        &self.0
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
        let pwd = "password123".parse::<ValueObjectRequired<Password>>().unwrap();
        assert_eq!(pwd.as_str().unwrap(), "password123");
    }

    #[test]
    fn test_invalid_password_too_short() {
        let pwd = "pass1".parse::<ValueObjectRequired<Password>>();
        assert!(pwd.is_err());
    }

    #[test]
    fn test_invalid_password_no_letters() {
        let pwd = "12345678".parse::<ValueObjectRequired<Password>>();
        assert!(pwd.is_err());
    }

    #[test]
    fn test_invalid_password_no_numbers() {
        let pwd = "password".parse::<ValueObjectRequired<Password>>();
        assert!(pwd.is_err());
    }

    #[test]
    fn test_password_display() {
        let pwd = "secret123".parse::<ValueObjectRequired<Password>>().unwrap();
        assert_eq!(format!("{}", pwd), "********");
    }
}
