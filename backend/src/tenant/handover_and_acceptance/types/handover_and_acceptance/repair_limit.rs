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
pub struct RepairLimit(f64);

impl RepairLimit {
    pub const PARSE_ERROR: &'static str = "Hibás maximális javítási költség formátum!";
}

impl ValueObjectData for RepairLimit {
    type DataType = f64;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.replace(",", ".").parse::<f64>().map_err(
                |_| ValueObjectError::InvalidInput(Self::PARSE_ERROR),
            )?)))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        if !self.0.is_finite() || self.0 <= 0.0 || self.0 > 9_999_999_999_999.994_f64 {
            return Err(ValueObjectError::InvalidInput(
                "A maximális javítási költség kívül esik a megengedett tartományon!",
            ));
        }
        Ok(())
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for RepairLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_repair_limit() {
        let repair_limit = "123.45"
            .parse::<ValueObjectRequired<RepairLimit>>()
            .unwrap();
        assert_eq!(repair_limit.as_f64().unwrap(), 123.45_f64);

        let repair_limit = "123,45"
            .parse::<ValueObjectRequired<RepairLimit>>()
            .unwrap();
        assert_eq!(repair_limit.as_f64().unwrap(), 123.45_f64);
    }

    #[test]
    fn test_invalid_repair_limit_format() {
        let cases = vec![
            r#""abc""#,
            r#""12.34.56""#,
            r#""12,34,56""#,
            r#""12a34""#,
            r#""$123""#,
        ];

        for case in cases {
            let repair_limit = case.parse::<ValueObjectRequired<RepairLimit>>();
            assert!(repair_limit.is_err());
        }
    }
}
