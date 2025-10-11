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
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct OrderBy(pub String);

impl ValueObjectable for OrderBy {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        match self.0.trim() {
            "name" => Ok(()),
            _ => Err("Hibás sorrend formátum".to_string()),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_order_by() {
        let order_by: ValueObject<OrderBy> = serde_json::from_str(r#""name""#).unwrap();
        assert_eq!(order_by.extract().get_value(), "name");
    }

    #[test]
    fn test_invalid_order_by() {
        let cases = vec![r#""price""#, r#""quantity""#, r#""invalid_column""#];

        for case in cases {
            let order_by: Result<ValueObject<OrderBy>, _> = serde_json::from_str(case);
            assert!(order_by.is_err());
        }
    }

    #[test]
    fn test_from_str() {
        let order_by = OrderBy::from_str("name").unwrap();
        assert_eq!(order_by.get_value(), "name");
    }

    #[test]
    fn test_display() {
        let order_by = OrderBy("name".to_string());
        assert_eq!(format!("{}", order_by), "name");
    }

    #[test]
    fn test_get_value() {
        let order_by = OrderBy("name".to_string());
        assert_eq!(order_by.get_value(), "name");
    }

    #[test]
    fn test_validation() {
        assert!(OrderBy("name".to_string()).validate().is_ok());
        assert!(OrderBy("price".to_string()).validate().is_err());
        assert!(OrderBy("quantity".to_string()).validate().is_err());
        assert!(OrderBy("invalid_column".to_string()).validate().is_err());
    }
}
