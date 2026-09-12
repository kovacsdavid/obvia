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
pub struct TypeOfPublicSpace(String);

impl TypeOfPublicSpace {
    pub const VALIDATION_ERROR: &'static str =
        "A közterület jellege nem lehet hosszabb 100 karakternél";
}

impl ValueObjectData for TypeOfPublicSpace {
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

impl Display for TypeOfPublicSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_valid_type_of_public_space() {
        let type_of_public_space = "utca"
            .parse::<ValueObjectRequired<TypeOfPublicSpace>>()
            .unwrap();
        assert_eq!(type_of_public_space.as_str().unwrap(), "utca");
    }

    #[test]
    fn test_too_long_type_of_public_space() {
        let type_of_public_space = "A"
            .repeat(101)
            .parse::<ValueObjectRequired<TypeOfPublicSpace>>();
        assert!(type_of_public_space.is_err());
    }

    #[test]
    fn test_type_of_public_space_with_spaces() {
        let type_of_public_space = "    utca   "
            .parse::<ValueObjectRequired<TypeOfPublicSpace>>()
            .unwrap();
        assert_eq!(type_of_public_space.as_str().unwrap(), "utca");
    }
}
