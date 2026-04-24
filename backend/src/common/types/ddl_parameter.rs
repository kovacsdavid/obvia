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
pub struct DdlParameter(String);

impl ValueObjectData for DdlParameter {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        match Regex::new(r##"^[A-Za-z0-9]{1,255}$"##) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err(ValueObjectError::InvalidInput("Hibás DDL paraméter!")),
            },
            Err(_) => Err(ValueObjectError::InvalidInput("Hibás DDL paraméter!")),
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
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

    #[test]
    fn test_valid_ddl_parameter() {
        let param = "abc123"
            .parse::<ValueObjectRequired<DdlParameter>>()
            .unwrap();
        assert_eq!(param.as_str().unwrap(), "abc123");
    }

    #[test]
    fn test_invalid_ddl_parameter_special_chars() {
        let param = r#""abc!@#""#.parse::<ValueObjectRequired<DdlParameter>>();
        assert!(param.is_err());
    }

    #[test]
    fn test_invalid_ddl_parameter_empty() {
        let param = "".parse::<ValueObjectRequired<DdlParameter>>();
        assert!(param.is_err());
    }

    #[test]
    fn test_invalid_ddl_parameter_too_long() {
        let long_str = "a".repeat(256);
        let param = long_str.parse::<ValueObjectRequired<DdlParameter>>();
        assert!(param.is_err());
    }

    #[test]
    fn test_sql_injection_basic() {
        let param = r#""DROP TABLE users"#.parse::<ValueObjectRequired<DdlParameter>>();
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
            let param = case.parse::<ValueObjectRequired<DdlParameter>>();
            assert!(param.is_err());
        }
    }

    #[test]
    fn test_sql_injection_multiple_statements() {
        let param = r#""users; DROP TABLE secrets"#.parse::<ValueObjectRequired<DdlParameter>>();
        assert!(param.is_err());
    }

    #[test]
    fn test_sql_injection_comments() {
        let cases = vec![r#""users--"#, r#""users/*"#, r#""users*/"#, r#""users#"#];
        for case in cases {
            let param = case.parse::<ValueObjectRequired<DdlParameter>>();
            assert!(param.is_err());
        }
    }

    #[test]
    fn test_sql_injection_union() {
        let param = r#""users UNION SELECT password"#.parse::<ValueObjectRequired<DdlParameter>>();
        assert!(param.is_err());
    }
}
