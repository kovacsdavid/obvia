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
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Clone)]
pub struct ItemIdentifierValue(String);

impl ItemIdentifierValue {
    pub const VALIDATION_ERROR: &'static str =
        "Az azonosító érték nem lehet 100 karakternél hosszabb";
}

impl ValueObjectData for ItemIdentifierValue {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.graphemes(true).count() <= 100 && self.0.chars().count() <= 100 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(Self::VALIDATION_ERROR))
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for ItemIdentifierValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_item_identifier_value() {
        let item_identifier_value = "KD00001"
            .parse::<ValueObjectRequired<ItemIdentifierValue>>()
            .unwrap();
        assert_eq!(item_identifier_value.as_str().unwrap(), "KD00001");
    }

    #[test]
    fn test_empty_item_identifier_value() {
        let item_identifier_value = "".parse::<ValueObjectRequired<ItemIdentifierValue>>();
        assert!(item_identifier_value.is_err());
    }

    #[test]
    fn test_whitespace_item_identifier_value() {
        let item_identifier_value = "  ".parse::<ValueObjectRequired<ItemIdentifierValue>>();
        assert!(item_identifier_value.is_err());
    }

    #[test]
    fn test_item_identifier_invalid_value() {
        let item_identifier_value = "a"
            .repeat(101)
            .parse::<ValueObjectRequired<ItemIdentifierValue>>();
        assert!(item_identifier_value.is_err());
    }
}
