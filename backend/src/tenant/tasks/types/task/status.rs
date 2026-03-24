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
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Status(pub String);

impl ValueObjectData for Status {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.as_str() {
            "active" => Ok(()),
            "inactive" => Ok(()),
            _ => Err(ValueObjectError::InvalidInput("Hibás termék státusz")),
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
        ValueObject::new_required(Status(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_active_status() {
        let status: ValueObject<Status> = serde_json::from_str(r#""active""#).unwrap();
        assert_eq!(status.as_str(), "active");
    }

    #[test]
    fn test_valid_inactive_status() {
        let status: ValueObject<Status> = serde_json::from_str(r#""inactive""#).unwrap();
        assert_eq!(status.as_str(), "inactive");
    }

    #[test]
    fn test_invalid_status() {
        let status: Result<ValueObject<Status>, _> = serde_json::from_str(r#""invalid""#);
        assert!(status.is_err());
    }

    #[test]
    fn test_empty_status() {
        let status: Result<ValueObject<Status>, _> = serde_json::from_str(r#""""#);
        assert!(status.is_err());
    }

    #[test]
    fn test_status_display() {
        let status = Status("active".to_string());
        assert_eq!(format!("{}", status), "active");
    }

    #[test]
    fn test_status_validate_success() {
        let status = Status("active".to_string());
        assert!(status.validate().is_ok());

        let status = Status("inactive".to_string());
        assert!(status.validate().is_ok());
    }

    #[test]
    fn test_status_validate_failure() {
        let status = Status("pending".to_string());
        assert!(status.validate().is_err());
    }

    #[test]
    fn test_status_get_value() {
        let status = Status("active".to_string());
        assert_eq!(status.get_value(), "active");
    }

    #[test]
    fn test_status_clone() {
        let status = Status("active".to_string());
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_status_debug() {
        let status = Status("active".to_string());
        assert_eq!(format!("{:?}", status), r#"Status("active")"#);
    }

    #[test]
    fn test_status_serialization() {
        let status = Status("active".to_string());
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, r#""active""#);
    }

    #[test]
    fn test_status_deserialization_invalid_json() {
        let result: Result<ValueObject<Status>, _> = serde_json::from_str("invalid");
        assert!(result.is_err());
    }
}
