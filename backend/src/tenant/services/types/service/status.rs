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

use crate::common::types::value_object::{ValueObject, ValueObjectable};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Status(pub String);

impl ValueObjectable for Status {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        match self.0.as_str() {
            "active" => Ok(()),
            "inactive" => Ok(()),
            _ => Err(String::from("Hibás szolgáltatás státusz")),
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
    use serde_json;

    #[test]
    fn test_valid_status_active() {
        let status: ValueObject<Status> = serde_json::from_str(r#""active""#).unwrap();
        assert_eq!(status.extract().get_value(), "active");
    }

    #[test]
    fn test_valid_status_inactive() {
        let status: ValueObject<Status> = serde_json::from_str(r#""inactive""#).unwrap();
        assert_eq!(status.extract().get_value(), "inactive");
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
    fn test_display_implementation() {
        let status = Status("active".to_string());
        assert_eq!(format!("{}", status), "active");
    }

    #[test]
    fn test_debug_implementation() {
        let status = Status("active".to_string());
        assert_eq!(format!("{:?}", status), r#"Status("active")"#);
    }

    #[test]
    fn test_validation_active() {
        let status = Status("active".to_string());
        assert!(status.validate().is_ok());
    }

    #[test]
    fn test_validation_inactive() {
        let status = Status("inactive".to_string());
        assert!(status.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid() {
        let status = Status("invalid".to_string());
        assert!(status.validate().is_err());
    }

    #[test]
    fn test_get_value() {
        let status = Status("active".to_string());
        assert_eq!(status.get_value(), "active");
    }

    #[test]
    fn test_clone() {
        let status = Status("active".to_string());
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let status1 = Status("active".to_string());
        let status2 = Status("active".to_string());
        let status3 = Status("inactive".to_string());

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_serialization() {
        let status = ValueObject::new(Status("active".to_string())).unwrap();
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, r#""active""#);
    }
}
