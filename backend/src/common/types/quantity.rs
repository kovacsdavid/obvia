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
pub struct Quantity(f64);

impl ValueObjectData for Quantity {
    type DataType = f64;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.replace(",", ".").parse::<f64>().map_err(
                |_| ValueObjectError::InvalidInput("Hibás mennyiség formátum!"),
            )?)))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0 >= 0_f64 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(
                "A megadott érték csak 0 vagy annál nagyobb szám lehet!",
            ))
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_string() {
        let quantity = "".parse::<ValueObjectRequired<Quantity>>();
        assert!(quantity.is_err());
    }

    #[test]
    fn test_validate_valid_float() {
        let quantity = "123.456".parse::<ValueObjectRequired<Quantity>>().unwrap();
        assert_eq!(quantity.as_f64().unwrap(), 123.456_f64);
    }

    #[test]
    fn test_validate_valid_integer() {
        let quantity = "123".parse::<ValueObjectRequired<Quantity>>().unwrap();
        assert_eq!(quantity.as_f64().unwrap(), 123_f64);
    }

    #[test]
    fn test_validate_zero() {
        let quantity = "0".parse::<ValueObjectRequired<Quantity>>().unwrap();
        assert_eq!(quantity.as_f64().unwrap(), 0_f64);
    }

    #[test]
    fn test_validate_negative() {
        let quantity = "-123.456".parse::<ValueObjectRequired<Quantity>>();
        assert!(quantity.is_err());
    }

    #[test]
    fn test_validate_comma_decimal() {
        let quantity = "123,456".parse::<ValueObjectRequired<Quantity>>().unwrap();
        assert_eq!(quantity.as_f64().unwrap(), 123.456_f64);
    }

    #[test]
    fn test_validate_non_numeric() {
        let quantity = "abc".parse::<ValueObjectRequired<Quantity>>();
        assert!(quantity.is_err());
    }

    #[test]
    fn test_validate_whitespace() {
        let quantity = "  123.456  ".parse::<ValueObjectRequired<Quantity>>();
        assert!(quantity.is_err());
    }
}
