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
use chrono::NaiveDate;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct ReceivedAt(NaiveDate);

impl ReceivedAt {
    pub const PARSE_ERROR: &'static str = "Hibás dátum formátum!";
}

impl ValueObjectData for ReceivedAt {
    type DataType = NaiveDate;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        let data_trim = data.trim();
        if !data_trim.is_empty() {
            Ok(Some(Self(data_trim.parse().map_err(|_| {
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

impl Display for ReceivedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Days, Utc};

    use super::*;

    #[test]
    fn test_tomorrow() {
        let valid_date = Utc::now()
            .checked_add_days(Days::new(1))
            .unwrap()
            .date_naive()
            .to_string();
        let date = valid_date
            .parse::<ValueObjectRequired<ReceivedAt>>()
            .unwrap();
        assert_eq!(date.as_date_naive().unwrap().to_string(), valid_date);
    }

    #[test]
    fn test_today() {
        let valid_date = Utc::now().date_naive().to_string();
        let date = valid_date
            .parse::<ValueObjectRequired<ReceivedAt>>()
            .unwrap();
        assert_eq!(date.as_date_naive().unwrap().to_string(), valid_date);
    }

    #[test]
    fn test_yesterday() {
        let valid_date = Utc::now()
            .checked_sub_days(Days::new(1))
            .unwrap()
            .date_naive()
            .to_string();
        let date = valid_date
            .parse::<ValueObjectRequired<ReceivedAt>>()
            .unwrap();
        assert_eq!(date.as_date_naive().unwrap().to_string(), valid_date);
    }

    #[test]
    fn test_invalid_received_at_formats() {
        let cases = vec![
            r#""2024/01/01""#,
            r#""2024.01.01""#,
            r#""01-01-2024""#,
            r#""2024-13-01""#,
            r#""2024-01-32""#,
            r#""2024-00-01""#,
            r#""2024-01-00""#,
            r#""abc""#,
            r#""""#,
            r#""    ""#,
        ];

        for case in cases {
            let date = case.parse::<ValueObjectRequired<ReceivedAt>>();
            assert!(date.is_err());
        }
    }
}
