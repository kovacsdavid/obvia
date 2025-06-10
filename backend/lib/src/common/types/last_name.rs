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
pub struct LastName(String);

impl LastName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_valid_last_name(s: &str) -> bool {
    let trimmed = s.trim();
    !trimmed.is_empty()
        && trimmed
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == ' ')
}

impl FromStr for LastName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_last_name(s) {
            Ok(LastName(s.trim().to_string()))
        } else {
            Err("Hibás vezetéknév formátum".to_string())
        }
    }
}

impl std::convert::TryFrom<String> for LastName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<'de> Deserialize<'de> for LastName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for LastName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_last_name() {
        let name: LastName = serde_json::from_str(r#""Kovács""#).unwrap();
        assert_eq!(name.as_str(), "Kovács");
        let name: LastName = serde_json::from_str(r#""Kovács-Kovács""#).unwrap();
        assert_eq!(name.as_str(), "Kovács-Kovács");
    }

    #[test]
    fn test_invalid_last_name() {
        let name: Result<LastName, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
        let name: Result<LastName, _> = serde_json::from_str(r#""123""#);
        assert!(name.is_err());
        let name: Result<LastName, _> = serde_json::from_str(r#""Kovács!""#);
        assert!(name.is_err());
    }
}
