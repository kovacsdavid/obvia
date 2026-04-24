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
pub struct Rate(f64);

impl Rate {
    pub const PARSE_ERROR: &'static str = "Hibás adókulcs formátum!";
    pub const VALIDATION_ERROR: &'static str = "A adókulcs csak 0 és 100 közötti érték lehet.";
}

impl ValueObjectData for Rate {
    type DataType = f64;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.replace(",", ".").parse().map_err(
                |_| ValueObjectError::InvalidInput(Self::PARSE_ERROR),
            )?)))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0 >= 0_f64 && self.0 <= 100_f64 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(Self::VALIDATION_ERROR))
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rate() {
        let price = "23.45".parse::<ValueObjectRequired<Rate>>().unwrap();
        assert_eq!(price.as_f64().unwrap(), 23.45_f64);

        let price = "23,45".parse::<ValueObjectRequired<Rate>>().unwrap();
        assert_eq!(price.as_f64().unwrap(), 23.45_f64);
    }

    #[test]
    fn test_invalid_rate_format() {
        let cases = vec![
            r#""abc""#,
            r#""12.34.56""#,
            r#""12,34,56""#,
            r#""12a34""#,
            r#""$123""#,
        ];

        for case in cases {
            let price = case.parse::<ValueObjectRequired<Rate>>();
            assert!(price.is_err());
        }
    }
}
