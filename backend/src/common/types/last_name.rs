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

use crate::common::types::{ValueObject, ValueObjectData, value_object::ValueObjectError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct LastName(pub String);

impl ValueObjectData for LastName {
    type DataType = String;
    fn validate(&self) -> Result<(), ValueObjectError> {
        let trimmed = self.0.trim();
        match !trimmed.is_empty()
            && trimmed
                .chars()
                .all(|c| c.is_alphabetic() || c == '-' || c == ' ')
        {
            true => Ok(()),
            false => Err(ValueObjectError::InvalidInput("Hibás vezetéknév formátum")),
        }
    }
    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for LastName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<LastName> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(LastName(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_last_name() {
        let name: ValueObject<LastName> = serde_json::from_str(r#""Smith""#).unwrap();
        assert_eq!(name.as_str(), "Smith");
    }

    #[test]
    fn test_valid_last_name_with_hyphen() {
        let name: ValueObject<LastName> = serde_json::from_str(r#""Smith-Jones""#).unwrap();
        assert_eq!(name.as_str(), "Smith-Jones");
    }

    #[test]
    fn test_valid_last_name_with_space() {
        let name: ValueObject<LastName> = serde_json::from_str(r#""Smith Smith""#).unwrap();
        assert_eq!(name.as_str(), "Smith Smith");
    }

    #[test]
    fn test_invalid_last_name_empty() {
        let name: Result<ValueObject<LastName>, _> = serde_json::from_str(r#""""#);
        assert!(name.is_err());
    }

    #[test]
    fn test_invalid_last_name_special_chars() {
        let name: Result<ValueObject<LastName>, _> = serde_json::from_str(r#""Smith123""#);
        assert!(name.is_err());
    }
}
