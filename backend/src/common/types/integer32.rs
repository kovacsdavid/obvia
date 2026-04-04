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
pub struct Integer32(i32);

impl ValueObjectData for Integer32 {
    type DataType = i32;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.parse::<i32>().map_err(|_| {
                ValueObjectError::InvalidInput("Hibás formátum!")
            })?)))
        } else {
            Ok(None)
        }
    }

    fn validate(&self) -> Result<(), ValueObjectError> {
        Ok(())
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Integer32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let integer = "".parse::<ValueObjectRequired<Integer32>>().unwrap_err();
        assert_eq!(integer.to_string().as_str(), "A mező kitöltése kötelező");
    }

    #[test]
    fn test_valid_integer() {
        let integer = "123".parse::<ValueObjectRequired<Integer32>>().unwrap();
        assert_eq!(integer.as_i32().unwrap(), 123);
    }

    #[test]
    fn test_validate_negative_integer() {
        let integer = "-123".parse::<ValueObjectRequired<Integer32>>().unwrap();
        assert_eq!(integer.as_i32().unwrap(), -123);
    }

    #[test]
    fn test_validate_decimal_comma() {
        let integer = "123,456".parse::<ValueObjectRequired<Integer32>>();
        assert!(integer.is_err());
    }

    #[test]
    fn test_validate_decimal_period() {
        let integer = "123.456".parse::<ValueObjectRequired<Integer32>>();
        assert!(integer.is_err());
    }

    #[test]
    fn test_validate_non_numeric() {
        let integer = "abc".parse::<ValueObjectRequired<Integer32>>();
        assert!(integer.is_err());
    }

    #[test]
    fn test_validate_overflow() {
        let integer = "2147483648".parse::<ValueObjectRequired<Integer32>>();
        assert!(integer.is_err());
    }
}
