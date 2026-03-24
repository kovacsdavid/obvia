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

use crate::common::types::{ValueObject, ValueObjectData, value_object::ValueObjectError};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct UuidVO(pub String);

impl ValueObject<UuidVO> {
    pub fn as_uuid(&self) -> Result<Uuid, ValueObjectError> {
        Uuid::parse_str(self.as_str()).map_err(|_| ValueObjectError::InvalidInput("Hibás uuid!"))
    }
}

impl ValueObjectData for UuidVO {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        Uuid::parse_str(&self.0).map_err(|_| ValueObjectError::InvalidInput("Hibás uuid!"))?;
        Ok(())
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ValueObject<UuidVO> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(UuidVO(s)).map_err(serde::de::Error::custom)
    }
}

impl Display for UuidVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UuidVO {
    type Err = ValueObjectError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::ValueObject;

    #[test]
    fn test_valid_uuid() {
        let valid_uuids = vec![
            "b5557d08-d92b-4064-b746-5a654accd55a",
            "ab0927b6-db88-4795-a80e-7ef54afe9270",
            "b0765c7d-2ecf-4c22-bbc6-e882e5e05848",
            "27e8ed88-48a8-4175-86be-d6aed60d9671",
            "d668e9df-eef5-4b3b-9a14-8a57841b3235",
        ];
        for value in valid_uuids {
            assert_eq!(
                ValueObject::new_required(UuidVO::from_str(value).unwrap())
                    .unwrap()
                    .as_str(),
                value
            );
        }
    }

    #[test]
    fn test_invalid_uuid() {
        let valid_uuids = vec!["", "invalid string"];
        for value in valid_uuids {
            assert!(ValueObject::new_required(UuidVO::from_str(value).unwrap()).is_err());
        }
    }
}
