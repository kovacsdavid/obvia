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

use crate::common::types::value_object::ValueObjectError;
use crate::common::types::{ValueObject, ValueObjectData};
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DbPort(pub String);

impl ValueObjectData for DbPort {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        let value = self
            .0
            .parse::<u16>()
            .map_err(|_| ValueObjectError::InvalidInput("Hibás adatbázis port"))?;
        if value > 1024 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput("Hibás adatbázis port"))
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for DbPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
struct DbPortVisitor;

impl<'de> Visitor<'de> for DbPortVisitor {
    type Value = ValueObject<DbPort>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^63 and 2^63-1")
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ValueObject::new_required(DbPort(v.to_string())).map_err(de::Error::custom)
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ValueObject::new_required(DbPort(v.to_string())).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for ValueObject<DbPort> {
    /// A custom implementation of the `deserialize` method
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i64(DbPortVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_db_port_i64() {
        let port: ValueObject<DbPort> = serde_json::from_str("5432").unwrap();
        assert_eq!(port.as_u16().unwrap(), 5432);
    }

    #[test]
    fn test_valid_db_port_u64() {
        let port: ValueObject<DbPort> = serde_json::from_str("5432").unwrap();
        assert_eq!(port.as_u16().unwrap(), 5432);
    }

    #[test]
    fn test_invalid_db_port_too_low() {
        let port: Result<ValueObject<DbPort>, _> = serde_json::from_str("1024");
        assert!(port.is_err());
    }

    #[test]
    fn test_invalid_db_port_too_high() {
        let port: Result<ValueObject<DbPort>, _> = serde_json::from_str("65536");
        assert!(port.is_err());
    }

    #[test]
    fn test_validation() {
        let valid_port = DbPort(5432.to_string());
        assert!(valid_port.validate().is_ok());

        let invalid_port_low = DbPort(1024.to_string());
        assert!(invalid_port_low.validate().is_err());

        let invalid_port_high = DbPort(65536.to_string());
        assert!(invalid_port_high.validate().is_err());
    }

    #[test]
    fn test_display() {
        let port = DbPort(5432.to_string());
        assert_eq!(format!("{}", port), "5432");
    }

    #[test]
    fn test_get_value() {
        let port = DbPort(5432.to_string());
        assert_eq!(port.get_value(), 5432.to_string().as_str());
    }

    #[test]
    fn test_deserialization_invalid_json() {
        let port: Result<ValueObject<DbPort>, _> = serde_json::from_str("invalid");
        assert!(port.is_err());
    }

    #[test]
    fn test_cloning() {
        let port = DbPort(5432.to_string());
        let cloned = port.clone();
        assert_eq!(port, cloned);
    }
}
