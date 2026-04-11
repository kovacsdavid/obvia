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
pub struct Comment(String);

impl ValueObjectData for Comment {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.len() <= 10_000 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(
                "A megjegyzés nem lehet 10 000 karakternél hosszabb!",
            ))
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_comment() {
        let comment = "Valid comment"
            .parse::<ValueObjectRequired<Comment>>()
            .unwrap();
        assert_eq!(comment.as_str().unwrap(), "Valid comment");
    }

    #[test]
    fn test_max_length_comment() {
        let long_string = "a".repeat(10000);
        let comment = long_string.parse::<ValueObjectRequired<Comment>>().unwrap();
        assert_eq!(comment.as_str().unwrap(), long_string);
    }

    #[test]
    fn test_too_long_comment() {
        let long_string = "a".repeat(10001);
        let comment = long_string.parse::<ValueObjectRequired<Comment>>();
        assert!(comment.is_err());
    }
}
