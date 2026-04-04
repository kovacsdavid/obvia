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

use crate::common::value_object::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct CustomerType(String);

impl ValueObjectData for CustomerType {
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
            "natural" | "legal" => Ok(()),
            _ => Err(ValueObjectError::InvalidInput("Hibás vevő típus!")),
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for CustomerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_natural_customer_type() {
        let customer_type = "natural"
            .parse::<ValueObjectRequired<CustomerType>>()
            .unwrap();
        assert_eq!(customer_type.as_str().unwrap(), "natural");
    }

    #[test]
    fn test_valid_legal_customer_type() {
        let customer_type = "legal"
            .parse::<ValueObjectRequired<CustomerType>>()
            .unwrap();
        assert_eq!(customer_type.as_str().unwrap(), "legal");
    }

    #[test]
    fn test_invalid_customer_type() {
        let customer_type = "invalid".parse::<ValueObjectRequired<CustomerType>>();
        assert!(customer_type.is_err());
    }

    #[test]
    fn test_empty_customer_type() {
        let customer_type = "".parse::<ValueObjectRequired<CustomerType>>();
        assert!(customer_type.is_err());
    }
}
