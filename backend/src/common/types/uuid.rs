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
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct UuidVO(Uuid);

impl UuidVO {
    pub const PARSE_ERROR: &'static str = "Hibás uuid!";
}

impl ValueObjectData for UuidVO {
    type DataType = Uuid;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.parse::<Uuid>().map_err(|_| {
                ValueObjectError::InvalidInput(Self::PARSE_ERROR)
            })?)))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        Ok(())
    }
    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for UuidVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

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
                value
                    .parse::<ValueObjectRequired<UuidVO>>()
                    .unwrap()
                    .as_uuid()
                    .unwrap(),
                Uuid::from_str(value).unwrap()
            );
        }
    }

    #[test]
    fn test_invalid_uuid() {
        let valid_uuids = vec!["", "invalid string"];
        for value in valid_uuids {
            assert!(value.parse::<ValueObjectRequired<UuidVO>>().is_err());
        }
    }
}
