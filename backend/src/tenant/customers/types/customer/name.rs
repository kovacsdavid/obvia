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
pub struct Name(String);

impl ValueObjectData for Name {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }

    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.len() <= 255 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(
                "A név nem lehet 255 karakternél hosszabb",
            ))
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        let name = "Test Name".parse::<ValueObjectRequired<Name>>().unwrap();
        assert_eq!(name.as_str().unwrap(), "Test Name");
    }

    #[test]
    fn test_empty_name() {
        let name = "".parse::<ValueObjectRequired<Name>>();
        assert!(name.is_err());
    }

    #[test]
    fn test_whitespace_only_name() {
        let name = "   ".parse::<ValueObjectRequired<Name>>();
        assert!(name.is_err());
    }

    #[test]
    fn test_too_long_name() {
        let long = "a".repeat(256);
        let name = long.parse::<ValueObjectRequired<Name>>();
        assert!(name.is_err());
    }

    #[test]
    fn test_long_name() {
        let long = "a".repeat(255);
        let name = long.parse::<ValueObjectRequired<Name>>();
        assert!(name.is_ok());
    }
}
