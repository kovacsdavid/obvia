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
pub struct UnitsOfMeasure(String);

impl ValueObjectData for UnitsOfMeasure {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.len() <= 50 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(
                "A mező maximum 50 karakter hosszú lehet",
            ))
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for UnitsOfMeasure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_unit_of_measure() {
        let uom = "kg".parse::<ValueObjectRequired<UnitsOfMeasure>>().unwrap();
        assert_eq!(uom.as_str().unwrap(), "kg");
    }

    #[test]
    fn test_too_long_unit_of_measure() {
        let long_str = "a".repeat(51);
        let uom = long_str.parse::<ValueObjectRequired<UnitsOfMeasure>>();
        assert!(uom.is_err());
    }

    #[test]
    fn test_max_length_unit_of_measure() {
        let max_str = "a".repeat(50);
        let uom = max_str
            .parse::<ValueObjectRequired<UnitsOfMeasure>>()
            .unwrap();
        assert_eq!(uom.as_str().unwrap(), max_str);
    }
}
