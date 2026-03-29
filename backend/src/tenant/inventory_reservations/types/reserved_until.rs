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
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ReservedUntil(pub String);

impl ValueObjectData for ReservedUntil {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        let today = Local::now().date_naive();
        let input_date: NaiveDate = self
            .0
            .trim()
            .parse()
            .map_err(|_| ValueObjectError::InvalidInput("Hibás dátum formátum!"))?;
        if input_date > today {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(
                "Foglalás csak a mai napnál későbbi dátumra lehetséges",
            ))
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl ValueObject<ReservedUntil> {
    pub fn date_naive(&self) -> Result<NaiveDate, ValueObjectError> {
        self.as_str()
            .trim()
            .parse()
            .map_err(|_| ValueObjectError::InvalidInput("Hibás dátum formátum!"))
    }
}

impl Display for ReservedUntil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<ReservedUntil> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(ReservedUntil(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Days;

    use super::*;

    #[test]
    fn test_tomorrow() {
        let valid_date = Local::now()
            .checked_add_days(Days::new(1))
            .unwrap()
            .date_naive()
            .to_string();
        assert!(ValueObject::new_required(ReservedUntil(valid_date)).is_ok());
    }

    #[test]
    fn test_today() {
        let valid_date = Local::now().date_naive().to_string();
        assert!(ValueObject::new_required(ReservedUntil(valid_date)).is_err());
    }

    #[test]
    fn test_yesterday() {
        let valid_date = Local::now()
            .checked_sub_days(Days::new(1))
            .unwrap()
            .date_naive()
            .to_string();
        assert!(ValueObject::new_required(ReservedUntil(valid_date)).is_err());
    }

    #[test]
    fn test_invalid_due_date_formats() {
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
            let due_date: Result<ValueObject<ReservedUntil>, _> = serde_json::from_str(case);
            assert!(due_date.is_err());
        }
    }

    #[test]
    fn test_display() {
        let due_date = ReservedUntil("2024-01-01".to_string());
        assert_eq!(format!("{}", due_date), "2024-01-01");
    }

    #[test]
    fn test_get_value() {
        let due_date = ReservedUntil("2024-01-01".to_string());
        assert_eq!(due_date.get_value(), "2024-01-01");
    }
}
