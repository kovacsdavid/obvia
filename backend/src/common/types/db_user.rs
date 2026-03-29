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
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DbUser(pub String);

impl ValueObjectData for DbUser {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match Regex::new(r##"^tenant_[A-Za-z0-9_]{1,50}$"##) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err(ValueObjectError::InvalidInput(
                    "Hibás felhasználónév formátum",
                )),
            },
            Err(_) => Err(ValueObjectError::InvalidInput(
                "Hibás felhasználónév formátum",
            )),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for DbUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<DbUser> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(DbUser(s)).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<Uuid> for ValueObject<DbUser> {
    type Error = ValueObjectError;
    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        ValueObject::new_required(DbUser(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_db_user() {
        let user: ValueObject<DbUser> = serde_json::from_str(r#""tenant_test123""#).unwrap();
        assert_eq!(user.as_str(), "tenant_test123");

        let user: ValueObject<DbUser> = serde_json::from_str(r#""tenant_valid_user""#).unwrap();
        assert_eq!(user.as_str(), "tenant_valid_user");
    }

    #[test]
    fn test_invalid_db_user_without_prefix() {
        let user: Result<ValueObject<DbUser>, _> = serde_json::from_str(r#""test123""#);
        assert!(user.is_err());
    }

    #[test]
    fn test_invalid_db_user_empty() {
        let user: Result<ValueObject<DbUser>, _> = serde_json::from_str(r#""""#);
        assert!(user.is_err());
    }

    #[test]
    fn test_invalid_db_user_special_chars() {
        let user: Result<ValueObject<DbUser>, _> = serde_json::from_str(r#""tenant_test!@#""#);
        assert!(user.is_err());
    }

    #[test]
    fn test_invalid_db_user_sql_injection() {
        let sql_injection_attempts = vec![
            r#""tenant_1;DROP TABLE users;--""#,
            r#""tenant_1' OR '1'='1""#,
            r#""tenant_1 UNION SELECT * FROM passwords""#,
            r#""tenant_1); DELETE FROM users; --""#,
            r#""tenant_1/**/UNION/**/SELECT/**/password/**/FROM/**/users""#,
        ];

        for attempt in sql_injection_attempts {
            let user: Result<ValueObject<DbUser>, _> = serde_json::from_str(attempt);
            assert!(user.is_err());
        }
    }

    #[test]
    fn test_invalid_db_user_too_long() {
        let long_name = format!(r#""tenant_{}" "#, "a".repeat(51));
        let user: Result<ValueObject<DbUser>, _> = serde_json::from_str(&long_name);
        assert!(user.is_err());
    }

    #[test]
    fn test_uuid_conversion() {
        let uuid = Uuid::new_v4();
        let user = ValueObject::<DbUser>::try_from(uuid);
        assert!(
            user.is_err(),
            "UUID should not be valid since it doesn't start with tenant_"
        );
    }
}
