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
use regex::Regex;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Otp(String);

impl ValueObjectData for Otp {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        let data_trim = data.trim();
        if !data_trim.is_empty() {
            Ok(Some(Self(data_trim.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.len() == 6 && Regex::new(r##"^[0-9]{6}$"##)?.is_match(&self.0) {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput("Hibás OTP!"))
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Otp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_otp() {
        let otp = "123456".parse::<ValueObjectRequired<Otp>>().unwrap();
        assert_eq!(otp.as_str().unwrap(), r#"123456"#);
    }

    #[test]
    fn test_invalid_otp_too_many_characters() {
        let otp = "1234567".parse::<ValueObjectRequired<Otp>>();
        assert!(otp.is_err());
    }

    #[test]
    fn test_invalid_otp_too_few_characters() {
        let otp = "12345".parse::<ValueObjectRequired<Otp>>();
        assert!(otp.is_err());
    }

    #[test]
    fn test_invalid_otp_invalid_character() {
        let otp = "a23456".parse::<ValueObjectRequired<Otp>>();
        assert!(otp.is_err());
    }
}
