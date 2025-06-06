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

use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Password(String);

impl Password {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_valid_password(s: &str) -> bool {
    let len_ok = s.len() >= 8;
    let has_letter = s.chars().any(|c| c.is_alphabetic());
    let has_digit = s.chars().any(|c| c.is_ascii_digit());
    len_ok && has_letter && has_digit
}

impl FromStr for Password {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_password(s) {
            Ok(Password(s.to_owned()))
        } else {
            Err("A jelszónak legalább 8 karakter hosszúnak kell lennie és tartalmaznia kell betűket és számokat".to_string())
        }
    }
}

impl std::convert::TryFrom<String> for Password {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "********")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_password() {
        let pw: Password = serde_json::from_str(r#""abc12345""#).unwrap();
        assert_eq!(pw.as_str(), "abc12345");
        let pw: Password = serde_json::from_str(r#""Password1""#).unwrap();
        assert_eq!(pw.as_str(), "Password1");
    }

    #[test]
    fn test_invalid_password() {
        // Too short
        assert!(serde_json::from_str::<Password>(r#""a1b2c3""#).is_err());
        // No digit
        assert!(serde_json::from_str::<Password>(r#""abcdefgh""#).is_err());
        // No letter
        assert!(serde_json::from_str::<Password>(r#""12345678""#).is_err());
        // Empty
        assert!(serde_json::from_str::<Password>(r#""""#).is_err());
    }

    #[test]
    fn test_display_masks_password() {
        let pw: Password = "abc12345".parse().unwrap();
        assert_eq!(format!("{}", pw), "********");
    }
}
