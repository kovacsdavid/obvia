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
pub struct DbName(String);

impl ValueObjectData for DbName {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        match Regex::new(r##"^tenant_[A-Za-z0-9]{1,50}$"##) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err(ValueObjectError::InvalidInput("Hibás adatbázis név!")),
            },
            Err(_) => Err(ValueObjectError::InvalidInput("Hibás adatbázis név!")),
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for DbName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_db_name() {
        let valid_names = vec![
            "tenant_mydatabase",
            "tenant_test123",
            "tenant_proddb",
            "tenant_dev",
            "tenant_a1b2c3",
        ];

        for name in valid_names {
            let db_name = name.parse::<ValueObjectRequired<DbName>>().unwrap();
            assert_eq!(db_name.as_str().unwrap(), name);
        }
    }

    #[test]
    fn test_invalid_db_name() {
        let too_long = "tenant_".to_owned() + &"a".repeat(51);
        let invalid_names = vec![
            // Basic SQL injection attempts
            "'DROP",
            "\"DROP",
            ";DROP",
            "-DROP",
            "--DROP",
            "tenant_'",
            "tenant_\"",
            "tenant_;",
            "tenant_--",
            // Advanced SQL injection attempts
            "tenant_table';DROP TABLE users;--",
            "tenant_db' UNION SELECT * FROM information_schema.tables;--",
            "tenant_' OR '1'='1",
            "tenant_') OR ('1'='1",
            "tenant_; DELETE FROM users; --",
            "tenant_/**/UNION/**/SELECT/**/password/**/FROM/**/users",
            // Multi-line SQL injection attempts
            "tenant_db'\n DROP TABLE users;--",
            "tenant_test\n\rDROP DATABASE master;--",
            // Concatenated SQL injection attempts
            "tenant_'||'DROP",
            "tenant_'+'DROP",
            "tenant_' 'DROP",
            // Invalid formats
            "",
            " ",
            "tenant",
            "tenant_",
            "_mydatabase",
            "tenant-db",
            "tenant_my#db",
            "tenant_db!",
            "TENANT_db",
            too_long.as_str(),     // Too long
            "tenant_с_кириллицей", // Non-ASCII chars
        ];

        for name in invalid_names {
            let db_name = name.parse::<ValueObjectRequired<DbName>>();
            assert!(db_name.is_err());
        }
    }
}
