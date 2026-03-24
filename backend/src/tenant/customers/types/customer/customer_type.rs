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
pub struct CustomerType(pub String);

impl ValueObjectData for CustomerType {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if &self.0 == "natural" || &self.0 == "legal" {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput("Hibás vevő típus!"))
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for CustomerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<CustomerType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(CustomerType(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_natural_customer_type() {
        let customer_type: ValueObject<CustomerType> =
            serde_json::from_str(r#""natural""#).unwrap();
        assert_eq!(customer_type.as_str(), "natural");
    }

    #[test]
    fn test_valid_legal_customer_type() {
        let customer_type: ValueObject<CustomerType> = serde_json::from_str(r#""legal""#).unwrap();
        assert_eq!(customer_type.as_str(), "legal");
    }

    #[test]
    fn test_invalid_customer_type() {
        let customer_type: Result<ValueObject<CustomerType>, _> =
            serde_json::from_str(r#""invalid""#);
        assert!(customer_type.is_err());
    }

    #[test]
    fn test_empty_customer_type() {
        let customer_type: Result<ValueObject<CustomerType>, _> = serde_json::from_str(r#""""#);
        assert!(customer_type.is_err());
    }

    #[test]
    fn test_display_implementation() {
        let customer_type = CustomerType("natural".to_string());
        assert_eq!(format!("{}", customer_type), "natural");
    }

    #[test]
    fn test_value_getter() {
        let customer_type = CustomerType("legal".to_string());
        assert_eq!(customer_type.get_value(), "legal");
    }

    #[test]
    fn test_validation() {
        let valid_natural = CustomerType("natural".to_string());
        let valid_legal = CustomerType("legal".to_string());
        let invalid = CustomerType("invalid".to_string());

        assert!(valid_natural.validate().is_ok());
        assert!(valid_legal.validate().is_ok());
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_validation_error_message() {
        let invalid = CustomerType("invalid".to_string());
        let result = invalid.validate();
        assert_eq!(
            result.unwrap_err(),
            ValueObjectError::InvalidInput("Hibás vevő típus!")
        );
    }
}
