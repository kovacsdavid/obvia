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
use regex::Regex;
use std::fmt::Display;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Clone)]
pub struct Website(String);

impl Website {
    pub const VALIDATION_ERROR: &'static str = "Hibás weboldal formátum";
}

impl ValueObjectData for Website {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.graphemes(true).count() <= 255 &&
            Regex::new(
            r#"^https?://(?:[A-Za-z0-9-]+\.)+[A-Za-z]{2,}(?::\d{1,5})?(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?$"#
        )?.is_match(&self.0) {
            true => Ok(()),
            false => Err(ValueObjectError::InvalidInput(Self::VALIDATION_ERROR)),
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Website {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_website_1() {
        let website = "https://exmaple.com"
            .parse::<ValueObjectRequired<Website>>()
            .unwrap();
        assert_eq!(website.as_str().unwrap(), "https://exmaple.com");
    }

    #[test]
    fn test_valid_website_2() {
        let website = "https://exmaple.com/teszt?t=123&a=1"
            .parse::<ValueObjectRequired<Website>>()
            .unwrap();
        assert_eq!(
            website.as_str().unwrap(),
            "https://exmaple.com/teszt?t=123&a=1"
        );
    }

    #[test]
    fn test_valid_website_3() {
        let website = "http://exmaple.com"
            .parse::<ValueObjectRequired<Website>>()
            .unwrap();
        assert_eq!(website.as_str().unwrap(), "http://exmaple.com");
    }

    #[test]
    fn test_invalid_website_1() {
        let website = "http//exmaple.com".parse::<ValueObjectRequired<Website>>();
        assert_eq!(
            website.unwrap_err().to_string(),
            Website::VALIDATION_ERROR.to_string()
        );
    }

    #[test]
    fn test_invalid_website_2() {
        let website = "http:/exmaple.com".parse::<ValueObjectRequired<Website>>();
        assert_eq!(
            website.unwrap_err().to_string(),
            Website::VALIDATION_ERROR.to_string()
        );
    }

    #[test]
    fn test_invalid_website_3() {
        let website = "exmaple.com".parse::<ValueObjectRequired<Website>>();
        assert_eq!(
            website.unwrap_err().to_string(),
            Website::VALIDATION_ERROR.to_string()
        );
    }

    #[test]
    fn test_invalid_website_4() {
        let website = "https://exmaple".parse::<ValueObjectRequired<Website>>();
        assert_eq!(
            website.unwrap_err().to_string(),
            Website::VALIDATION_ERROR.to_string()
        );
    }
}
