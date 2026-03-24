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
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Order(pub String);

impl ValueObjectData for Order {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.trim() {
            "asc" | "desc" => Ok(()),
            _ => Err(ValueObjectError::InvalidInput("Hibás sorrend formátum")),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl FromStr for Order {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Order(s.to_string()))
    }
}

impl<'de> Deserialize<'de> for ValueObject<Order> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(Order(s)).map_err(serde::de::Error::custom)
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_asc_order() {
        let order: ValueObject<Order> = serde_json::from_str(r#""asc""#).unwrap();
        assert_eq!(order.as_str(), "asc");
    }

    #[test]
    fn test_valid_desc_order() {
        let order: ValueObject<Order> = serde_json::from_str(r#""desc""#).unwrap();
        assert_eq!(order.as_str(), "desc");
    }

    #[test]
    fn test_invalid_order() {
        let order: Result<ValueObject<Order>, _> = serde_json::from_str(r#""invalid""#);
        assert!(order.is_err());
    }

    #[test]
    fn test_empty_order() {
        let order: Result<ValueObject<Order>, _> = serde_json::from_str(r#""""#);
        assert!(order.is_err());
    }

    #[test]
    fn test_to_string() {
        let order = Order("asc".to_string());
        assert_eq!(order.to_string(), "asc");
    }
}
