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

use regex::Regex;

use crate::common::value_object::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Door(String);

impl Door {
    pub const VALIDATION_ERROR: &'static str = "Hibás ajtó formátum";
}

impl ValueObjectData for Door {
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
        match Regex::new(r"^[A-Za-z0-9]{0,20}$")?.is_match(&self.0) {
            true => Ok(()),
            false => Err(ValueObjectError::InvalidInput(Self::VALIDATION_ERROR)),
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Door {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_door() {
        let door = "1".parse::<ValueObjectRequired<Door>>().unwrap();
        assert_eq!(door.as_str().unwrap(), "1");
    }

    #[test]
    fn test_too_long_door() {
        let door = "1".repeat(21).parse::<ValueObjectRequired<Door>>();
        assert!(door.is_err());
    }

    #[test]
    fn test_door_with_spaces() {
        let door = "    1   ".parse::<ValueObjectRequired<Door>>().unwrap();
        assert_eq!(door.as_str().unwrap(), "1");
    }
}
