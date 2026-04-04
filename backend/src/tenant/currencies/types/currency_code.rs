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
pub struct CurrencyCode(String);

impl ValueObjectData for CurrencyCode {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        let data_trim = data.trim();
        if !data_trim.is_empty() {
            Ok(Some(Self(data_trim.to_uppercase().to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.trim().len() == 3 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(
                "A mező csak három karakteres pénznemformátumot tartalmazhat. Pl.: HUF",
            ))
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_currency() {
        let currency = "USD".parse::<ValueObjectRequired<CurrencyCode>>().unwrap();
        assert_eq!(currency.as_str().unwrap(), "USD");
    }

    #[test]
    fn test_invalid_currency_too_short() {
        let currency = "US".parse::<ValueObjectRequired<CurrencyCode>>();
        assert!(currency.is_err());
    }

    #[test]
    fn test_invalid_currency_too_long() {
        let currency = "USDT".parse::<ValueObjectRequired<CurrencyCode>>();
        assert!(currency.is_err());
    }

    #[test]
    fn test_invalid_currency_empty() {
        let currency = "".parse::<ValueObjectRequired<CurrencyCode>>();
        assert!(currency.is_err());
    }

    #[test]
    fn test_validation_with_spaces() {
        let currency = " USD "
            .parse::<ValueObjectRequired<CurrencyCode>>()
            .unwrap();
        assert_eq!(currency.as_str().unwrap(), "USD");
    }

    #[test]
    fn test_validation_with_lowercase() {
        let currency = "usd".parse::<ValueObjectRequired<CurrencyCode>>().unwrap();
        assert_eq!(currency.as_str().unwrap(), "USD");
    }
}
