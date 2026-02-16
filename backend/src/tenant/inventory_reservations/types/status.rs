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
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Status(pub String);

impl ValueObjectable for Status {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.as_str() {
            "active" => Ok(()),
            "fulfilled" => Ok(()),
            "cancelled" => Ok(()),
            "expired" => Ok(()),
            _ => Err(ValueObjectError::InvalidInput("Hibás foglalás státusz")),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Status> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(Status(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::ValueObject;

    #[test]
    fn test_validate_valid_status() {
        let valid_statuses = vec!["active", "fulfilled", "cancelled", "expired"];

        for status in valid_statuses {
            let status = Status(status.to_string());
            assert_eq!(status.validate(), Ok(()));
        }
    }

    #[test]
    fn test_validate_invalid_status() {
        let invalid_status = Status("invalid".to_string());
        assert_eq!(
            invalid_status.validate(),
            Err(ValueObjectError::InvalidInput("Hibás foglalás státusz"))
        );
    }

    #[test]
    fn test_get_value() {
        let status = Status("active".to_string());
        assert_eq!(status.get_value(), &"active".to_string());
    }

    #[test]
    fn test_display() {
        let status = Status("active".to_string());
        assert_eq!(format!("{}", status), "active");
    }

    #[test]
    fn test_deserialize_valid_status() {
        let json = r#""active""#;
        let status: ValueObject<Status> = serde_json::from_str(json).unwrap();
        assert_eq!(status.as_str(), "active");
    }

    #[test]
    fn test_deserialize_invalid_status() {
        let json = r#""invalid""#;
        let result: Result<ValueObject<Status>, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize() {
        let status = Status("active".to_string());
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, r#""active""#);
    }

    #[test]
    fn test_clone() {
        let status = Status("active".to_string());
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }
}
