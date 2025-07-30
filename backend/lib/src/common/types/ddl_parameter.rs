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

use regex::Regex;
use serde::Deserialize;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct DdlParameter(String);

impl DdlParameter {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_valid_ddl_parameter(s: &str) -> bool {
    match Regex::new(r##"^[A-Za-z0-9]{1,255}$"##) {
        Ok(re) => re.is_match(s),
        Err(_) => false,
    }
}

impl FromStr for DdlParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_ddl_parameter(s) {
            Ok(DdlParameter(s.to_string()))
        } else {
            Err("Hibás DDL paraméter!".to_string())
        }
    }
}

impl<'de> Deserialize<'de> for DdlParameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Display for DdlParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_ddl_parameter() {
        let ddl_parameter: DdlParameter =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d""#).unwrap();
        assert_eq!(ddl_parameter.as_str(), "bc5690796fc8414e93e32fcdaae3156d");
    }

    #[test]
    fn test_invalid_ddl_parameter() {
        let ddl_parameter: Result<DdlParameter, _> =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d'DROP""#);
        assert!(ddl_parameter.is_err());
        let ddl_parameter: Result<DdlParameter, _> =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d;DROP""#);
        assert!(ddl_parameter.is_err());
        let ddl_parameter: Result<DdlParameter, _> =
            serde_json::from_str(r#""bc5690796fc8414e93e32fcdaae3156d"DROP""#);
        assert!(ddl_parameter.is_err());
    }
}
