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
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DdlParameter(pub String);

impl ValueObjectable for DdlParameter {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match Regex::new(r##"^[A-Za-z0-9]{1,255}$"##) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err(ValueObjectError::InvalidInput("Hibás DDL paraméter!")),
            },
            Err(_) => Err(ValueObjectError::InvalidInput("Hibás DDL paraméter!")),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for DdlParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<DdlParameter> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(DdlParameter(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ddl_parameter() {
        let param: ValueObject<DdlParameter> = serde_json::from_str(r#""abc123""#).unwrap();
        assert_eq!(param.as_str(), "abc123");
    }

    #[test]
    fn test_invalid_ddl_parameter_special_chars() {
        let param: Result<ValueObject<DdlParameter>, _> = serde_json::from_str(r#""abc!@#""#);
        assert!(param.is_err());
    }

    #[test]
    fn test_invalid_ddl_parameter_empty() {
        let param: Result<ValueObject<DdlParameter>, _> = serde_json::from_str(r#""""#);
        assert!(param.is_err());
    }

    #[test]
    fn test_invalid_ddl_parameter_too_long() {
        let long_str = "a".repeat(256);
        let param: Result<ValueObject<DdlParameter>, _> =
            serde_json::from_str(&format!(r#""{}""#, long_str));
        assert!(param.is_err());
    }

    #[test]
    fn test_sql_injection_basic() {
        let injection = r#""DROP TABLE users"#;
        let param: Result<ValueObject<DdlParameter>, _> = serde_json::from_str(injection);
        assert!(param.is_err());
    }

    #[test]
    fn test_sql_injection_keywords() {
        let cases = vec![
            r#""SELECT"#,
            r#""INSERT"#,
            r#""UPDATE"#,
            r#""DELETE"#,
            r#""DROP"#,
            r#""ALTER"#,
        ];
        for case in cases {
            let param: Result<ValueObject<DdlParameter>, _> = serde_json::from_str(case);
            assert!(param.is_err());
        }
    }

    #[test]
    fn test_sql_injection_multiple_statements() {
        let injection = r#""users; DROP TABLE secrets"#;
        let param: Result<ValueObject<DdlParameter>, _> = serde_json::from_str(injection);
        assert!(param.is_err());
    }

    #[test]
    fn test_sql_injection_comments() {
        let cases = vec![r#""users--"#, r#""users/*"#, r#""users*/"#, r#""users#"#];
        for case in cases {
            let param: Result<ValueObject<DdlParameter>, _> = serde_json::from_str(case);
            assert!(param.is_err());
        }
    }

    #[test]
    fn test_sql_injection_union() {
        let injection = r#""users UNION SELECT password"#;
        let param: Result<ValueObject<DdlParameter>, _> = serde_json::from_str(injection);
        assert!(param.is_err());
    }
}
