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
pub struct FirstName(String);

impl ValueObjectData for FirstName {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        let trimmed = self.0.trim();
        match !trimmed.is_empty()
            && trimmed
                .chars()
                .all(|c| c.is_alphabetic() || c == '-' || c == ' ')
        {
            true => Ok(()),
            false => Err(ValueObjectError::InvalidInput("Hibás keresztnév formátum")),
        }
    }
    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for FirstName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_first_name() {
        let name = "John".parse::<ValueObjectRequired<FirstName>>().unwrap();
        assert_eq!(name.as_str().unwrap(), "John");
    }

    #[test]
    fn test_valid_first_name_with_hyphen() {
        let name = "Mary-Jane"
            .parse::<ValueObjectRequired<FirstName>>()
            .unwrap();
        assert_eq!(name.as_str().unwrap(), "Mary-Jane");
    }

    #[test]
    fn test_valid_first_name_with_space() {
        let name = "Mary Jane"
            .parse::<ValueObjectRequired<FirstName>>()
            .unwrap();
        assert_eq!(name.as_str().unwrap(), "Mary Jane");
    }

    #[test]
    fn test_invalid_first_name_empty() {
        let name = "".parse::<ValueObjectRequired<FirstName>>();
        assert!(name.is_err());
    }

    #[test]
    fn test_invalid_first_name_special_chars() {
        let name = "John123!".parse::<ValueObjectRequired<FirstName>>();
        assert!(name.is_err());
    }
}
