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
pub struct DocumentPrefix(String);

impl DocumentPrefix {
    pub const VALIDATION_ERROR: &'static str =
        "A dokumentum előtag nem lehet 10 karakternél hosszabb";
}

impl ValueObjectData for DocumentPrefix {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.graphemes(true).count() <= 10 && self.0.chars().count() <= 10 {
            true => Ok(()),
            false => Err(ValueObjectError::InvalidInput(Self::VALIDATION_ERROR)),
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for DocumentPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_document_prefix() {
        let document_prefix = "TE".parse::<ValueObjectRequired<DocumentPrefix>>().unwrap();
        assert_eq!(document_prefix.as_str().unwrap(), "TE");
    }

    #[test]
    fn test_invalid_document_prefix() {
        let document_prefix = "T"
            .repeat(11)
            .parse::<ValueObjectRequired<DocumentPrefix>>();
        assert_eq!(
            document_prefix.unwrap_err().to_string(),
            DocumentPrefix::VALIDATION_ERROR.to_string()
        );
    }
}
