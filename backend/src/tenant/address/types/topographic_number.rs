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
pub struct TopographicNumber(String);

impl TopographicNumber {
    pub const VALIDATION_ERROR: &'static str =
        "A helyrajzi szám nem lehet 100 karakternél hosszabb";
}

impl ValueObjectData for TopographicNumber {
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
        if self.0.len() <= 100 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(Self::VALIDATION_ERROR))
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for TopographicNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_valid_topographic_number() {
        let topographic_number = "123/A"
            .parse::<ValueObjectRequired<TopographicNumber>>()
            .unwrap();
        assert_eq!(topographic_number.as_str().unwrap(), "123/A");
    }

    #[test]
    fn test_too_long_topographic_number() {
        let topographic_number = "A"
            .repeat(256)
            .parse::<ValueObjectRequired<TopographicNumber>>();
        assert!(topographic_number.is_err());
    }

    #[test]
    fn test_topographic_number_with_spaces() {
        let topographic_number = "    123/A   "
            .parse::<ValueObjectRequired<TopographicNumber>>()
            .unwrap();
        assert_eq!(topographic_number.as_str().unwrap(), "123/A");
    }
}
