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
use crate::common::types::{ValueObject, ValueObjectable};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct OrderBy(pub String);

impl ValueObjectable for OrderBy {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        match self.0.trim() {
            "name" | "created_at" | "updated_at" | "" => Ok(()),
            _ => Err("Hibás sorrend formátum".to_string()),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl FromStr for OrderBy {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OrderBy(s.to_string()))
    }
}

impl<'de> Deserialize<'de> for ValueObject<OrderBy> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(OrderBy(s)).map_err(serde::de::Error::custom)
    }
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_order_by() {
        let values = ["name", "created_at", "updated_at", ""];
        for value in values.iter() {
            let order: OrderBy = OrderBy(value.to_string());
            assert!(order.validate().is_ok());
        }
    }

    #[test]
    fn test_invalid_order_by() {
        let order = OrderBy("invalid_column".to_string());
        assert!(order.validate().is_err());
    }

    #[test]
    fn test_get_value() {
        let test_str = "name";
        let order = OrderBy(test_str.to_string());
        assert_eq!(order.get_value(), test_str);
    }

    #[test]
    fn test_from_str() {
        let test_str = "name";
        let order = OrderBy::from_str(test_str).unwrap();
        assert_eq!(order.get_value(), test_str);
    }

    #[test]
    fn test_display() {
        let test_str = "name";
        let order = OrderBy(test_str.to_string());
        assert_eq!(format!("{}", order), test_str);
    }

    #[test]
    fn test_deserialize_valid() {
        let order: ValueObject<OrderBy> = serde_json::from_str(r#""name""#).unwrap();
        assert_eq!(order.extract().get_value(), "name");
    }

    #[test]
    fn test_deserialize_invalid() {
        let result: Result<ValueObject<OrderBy>, _> = serde_json::from_str(r#""invalid_column""#);
        assert!(result.is_err());
    }
}
