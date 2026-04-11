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
use regex::Regex;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Email(String);

impl ValueObjectData for Email {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        if !data.trim().is_empty() {
            Ok(Some(Self(data.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        match Regex::new(
            r##"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"##,
        ) {
            Ok(re) => match re.is_match(&self.0) {
                true => Ok(()),
                false => Err(ValueObjectError::InvalidInput(
                    "A megadott e-mail cím formátuma nem megfelelő",
                )),
            },
            Err(_) => Err(ValueObjectError::InvalidInput(
                "A megadott e-mail cím formátuma nem megfelelő",
            )),
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = r#"test@example.com"#.parse::<ValueObjectRequired<Email>>().unwrap();
        assert_eq!(email.as_str().unwrap(), "test@example.com");
    }

    #[test]
    fn test_invalid_email_missing_at() {
        let email = r#"testexample.com"#.parse::<ValueObjectRequired<Email>>();
        assert!(email.is_err());
    }

    #[test]
    fn test_invalid_email_missing_domain() {
        let email = r#"test@"#.parse::<ValueObjectRequired<Email>>();
        assert!(email.is_err());
    }

    #[test]
    fn test_invalid_email_special_chars() {
        let email = r#"test!$%@example.com"#.parse::<ValueObjectRequired<Email>>();
        assert!(email.is_ok());
    }

    #[test]
    fn test_valid_email_with_subdomain() {
        let email = "test@sub.example.com"
            .parse::<ValueObjectRequired<Email>>()
            .unwrap();
        assert_eq!(email.as_str().unwrap(), "test@sub.example.com");
    }
}
