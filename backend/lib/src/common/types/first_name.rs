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
pub struct FirstName(String);

impl FirstName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_valid_first_name(s: &str) -> bool {
    let trimmed = s.trim();
    !trimmed.is_empty()
        && trimmed
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == ' ')
}

impl FromStr for FirstName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_first_name(s) {
            Ok(FirstName(s.trim().to_string()))
        } else {
            Err("Hibás keresztnév formátum".to_string())
        }
    }
}

impl std::convert::TryFrom<String> for FirstName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<'de> Deserialize<'de> for FirstName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for FirstName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_first_name() {
        let name: FirstName = serde_json::from_str(r#""Dávid""#).unwrap();
        assert_eq!(name.as_str(), "Dávid");
        let name: FirstName = serde_json::from_str(r#""Eleonóra Tímea""#).unwrap();
        assert_eq!(name.as_str(), "Eleonóra Tímea");
    }

    #[test]
    fn test_invalid_first_name() {
        let name: Result<FirstName, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
        let name: Result<FirstName, _> = serde_json::from_str(r#""123""#);
        assert!(name.is_err());
        let name: Result<FirstName, _> = serde_json::from_str(r#""Dávid!""#);
        assert!(name.is_err());
    }
}
