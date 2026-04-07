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
pub struct LegalText(String);

impl ValueObjectData for LegalText {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.len() <= 10000 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(
                "A jogi szöveg nem lehet 10 000 karakternél hosszabb!",
            ))
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for LegalText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_description() {
        let desc = "Valid legal text"
            .parse::<ValueObjectRequired<LegalText>>()
            .unwrap();
        assert_eq!(desc.as_str().unwrap(), "Valid legal text");
    }

    #[test]
    fn test_max_length_description() {
        let desc = "a".repeat(10000);
        let result = desc.parse::<ValueObjectRequired<LegalText>>().unwrap();
        assert_eq!(result.as_str().unwrap(), desc);
    }

    #[test]
    fn test_too_long_description() {
        let desc = "a".repeat(10001);
        let result = desc.parse::<ValueObjectRequired<LegalText>>();
        assert!(result.is_err());
    }
}
