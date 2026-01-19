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

use crate::common::types::{ValueObject, ValueObjectable};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Otp(pub String);

impl ValueObjectable for Otp {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        let trimmed = self.0.trim();
        if trimmed.len() != 6 {
            return Err("Hibás OTP!".to_string());
        }
        match Regex::new(r##"^[0-9]{6}$"##) {
            Ok(re) => match re.is_match(trimmed) {
                true => Ok(()),
                false => Err("Hibás OTP!".to_string()),
            },
            Err(_) => Err("Hibás OTP!".to_string()),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Otp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Otp> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(Otp(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_otp() {
        let otp: ValueObject<Otp> = serde_json::from_str(r#""123456""#).unwrap();
        assert_eq!(otp.extract().get_value(), r#"123456"#);
    }

    #[test]
    fn test_invalid_otp_too_many_characters() {
        let otp: Result<ValueObject<Otp>, _> = serde_json::from_str(r#"1234567"#);
        assert!(otp.is_err());
    }

    #[test]
    fn test_invalid_otp_too_few_characters() {
        let otp: Result<ValueObject<Otp>, _> = serde_json::from_str(r#"12345"#);
        assert!(otp.is_err());
    }

    #[test]
    fn test_invalid_otp_invalid_character() {
        let otp: Result<ValueObject<Otp>, _> = serde_json::from_str(r#"a23456"#);
        assert!(otp.is_err());
    }
}
