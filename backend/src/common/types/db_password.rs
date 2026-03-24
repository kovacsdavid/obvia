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

use crate::common::types::{ValueObject, ValueObjectData, value_object::ValueObjectError};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DbPassword(pub String);

impl ValueObjectData for DbPassword {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match Regex::new(r##"^[A-Za-z0-9]{40,99}$"##) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err(ValueObjectError::InvalidInput("Hibás adatbázis jelszó!")),
            },
            Err(_) => Err(ValueObjectError::InvalidInput("Hibás adatbázis jelszó!")),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for DbPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<DbPassword> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(DbPassword(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_db_password() {
        let valid_passwords = vec![
            r#"RpehL35tQxnG6fgST0FQUnqHhkaqVOtTgflqArsl"#,
            r#"abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLM"#,
            r#"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"#,
            r#"BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB"#,
            r#"1111111111111111111111111111111111111111"#,
        ];
        for password in valid_passwords {
            let db_password: ValueObject<DbPassword> =
                serde_json::from_str(format!("\"{}\"", &password).as_str()).unwrap();
            assert_eq!(db_password.as_str(), password);
        }
    }

    #[test]
    fn test_invalid_db_password() {
        let invalid_passwords = vec![
            // Special characters not allowed
            r#"4v4X;yk+|hJH3QLL>%7S-~]%1A2}|Fh_:`p[#QPL+!"#,
            // Too short
            r#"password"#,
            // Too long (100+ chars)
            r#"jwP9rF3xa4hvTk1m2PLVobm99gXv5BSdmuEeTvmCxB9YgIVDwYjwP9rF3xa4hvTk1m2PLVobm99gXv5BSdmuEeTvmCxB9YgIVDwY"#,
            // Under 40 chars
            r#"gyoP823CB0e7GFx5FUNqscBQg76VdXwcMRD6vsP"#,
            // Empty string
            r#""#,
            // Whitespace
            r#" "#,
            // Special chars only
            r#"!@#$%^&*()_+-=[]{}|;:,.<>?"#,
            // Mixed invalid chars
            r#"abc123!@#"#,
        ];
        for password in invalid_passwords {
            let db_password: Result<ValueObject<DbPassword>, _> =
                serde_json::from_str(format!("\"{}\"", &password).as_str());
            assert!(db_password.is_err());
        }
    }
}
