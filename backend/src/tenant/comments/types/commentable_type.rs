/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct CommentableType(String);

impl CommentableType {
    pub const VALIDATION_ERROR: &'static str = "Hibás erőforrás típus";
}

impl ValueObjectData for CommentableType {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        let data_trim = data.trim();
        if !data_trim.is_empty() {
            Ok(Some(Self(data_trim.to_owned())))
        } else {
            Ok(None)
        }
    }

    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.as_str() {
            "customers"
            | "warehouses"
            | "taxes"
            | "products"
            | "inventory"
            | "inventory_movements"
            | "inventory_reservations"
            | "services"
            | "tasks"
            | "worksheets" => Ok(()),
            _ => Err(ValueObjectError::InvalidInput(Self::VALIDATION_ERROR)),
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for CommentableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_order_by() {
        let order_by = "customers"
            .parse::<ValueObjectRequired<CommentableType>>()
            .unwrap();
        assert_eq!(order_by.as_str().unwrap(), "customers");
    }

    #[test]
    fn test_invalid_order_by() {
        let cases = vec![r#""invalid""#, r#""''""#, r#"";""#];

        for case in cases {
            let order_by = case.parse::<ValueObjectRequired<CommentableType>>();
            assert!(order_by.is_err());
        }
    }
}
