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

use crate::common::types::{ValueObject, ValueObjectable, value_object::ValueObjectError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct UnitsOfMeasure(pub String);

impl ValueObjectable for UnitsOfMeasure {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.trim().is_empty() {
            return Err(ValueObjectError::InvalidInput("A mező kitöltése kötelező"));
        }
        if self.0.trim().len() > 50 {
            return Err(ValueObjectError::InvalidInput(
                "A mező maximum 50 karakter hosszú lehet",
            ));
        }
        Ok(())
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for UnitsOfMeasure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<UnitsOfMeasure> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(UnitsOfMeasure(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_unit_of_measure() {
        let uom: ValueObject<UnitsOfMeasure> = serde_json::from_str(r#""kg""#).unwrap();
        assert_eq!(uom.as_str(), "kg");
    }

    #[test]
    fn test_empty_unit_of_measure() {
        let uom: Result<ValueObject<UnitsOfMeasure>, _> = serde_json::from_str(r#""""#);
        assert!(uom.is_err());
    }

    #[test]
    fn test_whitespace_only_unit_of_measure() {
        let uom: Result<ValueObject<UnitsOfMeasure>, _> = serde_json::from_str(r#"" ""#);
        assert!(uom.is_err());
    }

    #[test]
    fn test_too_long_unit_of_measure() {
        let long_str = "a".repeat(51);
        let uom: Result<ValueObject<UnitsOfMeasure>, _> =
            serde_json::from_str(&format!(r#""{}""#, long_str));
        assert!(uom.is_err());
    }

    #[test]
    fn test_max_length_unit_of_measure() {
        let max_str = "a".repeat(50);
        let uom: Result<ValueObject<UnitsOfMeasure>, _> =
            serde_json::from_str(&format!(r#""{}""#, max_str));
        assert!(uom.is_ok());
    }

    #[test]
    fn test_special_characters() {
        let cases = vec![r#""kg/m²""#, r#""°C""#, r#""m³""#, r#""μm""#];
        for case in cases {
            let uom: Result<ValueObject<UnitsOfMeasure>, _> = serde_json::from_str(case);
            assert!(uom.is_ok());
        }
    }

    #[test]
    fn test_display_format() {
        let uom = UnitsOfMeasure("kg".to_string());
        assert_eq!(format!("{}", uom), "kg");
    }

    #[test]
    fn test_deserialization_error_handling() {
        let invalid_json = r#"{"unit": "kg"}"#;
        let uom: Result<ValueObject<UnitsOfMeasure>, _> = serde_json::from_str(invalid_json);
        assert!(uom.is_err());
    }
}
