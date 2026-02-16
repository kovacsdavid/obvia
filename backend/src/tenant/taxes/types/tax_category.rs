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
pub struct TaxCategory(pub String);

impl ValueObjectable for TaxCategory {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.as_str() {
            "standard" => Ok(()),
            "reduced" => Ok(()),
            "exempt" => Ok(()),
            "reverse_charge" => Ok(()),
            "small_business_exempt" => Ok(()),
            _ => Err(ValueObjectError::InvalidInput("Hibás adó kategória")),
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for TaxCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<TaxCategory> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(TaxCategory(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tax_category_standard() {
        let category: ValueObject<TaxCategory> = serde_json::from_str(r#""standard""#).unwrap();
        assert_eq!(category.as_str(), "standard");
    }

    #[test]
    fn test_valid_tax_category_reduced() {
        let category: ValueObject<TaxCategory> = serde_json::from_str(r#""reduced""#).unwrap();
        assert_eq!(category.as_str(), "reduced");
    }

    #[test]
    fn test_valid_tax_category_exempt() {
        let category: ValueObject<TaxCategory> = serde_json::from_str(r#""exempt""#).unwrap();
        assert_eq!(category.as_str(), "exempt");
    }

    #[test]
    fn test_valid_tax_category_reverse_charge() {
        let category: ValueObject<TaxCategory> =
            serde_json::from_str(r#""reverse_charge""#).unwrap();
        assert_eq!(category.as_str(), "reverse_charge");
    }

    #[test]
    fn test_valid_tax_category_small_business() {
        let category: ValueObject<TaxCategory> =
            serde_json::from_str(r#""small_business_exempt""#).unwrap();
        assert_eq!(category.as_str(), "small_business_exempt");
    }

    #[test]
    fn test_invalid_tax_category() {
        let category: Result<ValueObject<TaxCategory>, _> = serde_json::from_str(r#""invalid""#);
        assert!(category.is_err());
    }

    #[test]
    fn test_empty_tax_category() {
        let category: Result<ValueObject<TaxCategory>, _> = serde_json::from_str(r#""""#);
        assert!(category.is_err());
    }

    #[test]
    fn test_display_implementation() {
        let category = TaxCategory("standard".to_string());
        assert_eq!(format!("{}", category), "standard");
    }

    #[test]
    fn test_debug_implementation() {
        let category = TaxCategory("standard".to_string());
        assert_eq!(format!("{:?}", category), r#"TaxCategory("standard")"#);
    }

    #[test]
    fn test_validation_standard() {
        let category = TaxCategory("standard".to_string());
        assert!(category.validate().is_ok());
    }

    #[test]
    fn test_validation_reduced() {
        let category = TaxCategory("reduced".to_string());
        assert!(category.validate().is_ok());
    }

    #[test]
    fn test_validation_exempt() {
        let category = TaxCategory("exempt".to_string());
        assert!(category.validate().is_ok());
    }

    #[test]
    fn test_validation_reverse_charge() {
        let category = TaxCategory("reverse_charge".to_string());
        assert!(category.validate().is_ok());
    }

    #[test]
    fn test_validation_small_business() {
        let category = TaxCategory("small_business_exempt".to_string());
        assert!(category.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid() {
        let category = TaxCategory("invalid".to_string());
        assert!(category.validate().is_err());
    }

    #[test]
    fn test_get_value() {
        let category = TaxCategory("standard".to_string());
        assert_eq!(category.get_value(), "standard");
    }

    #[test]
    fn test_clone() {
        let category = TaxCategory("standard".to_string());
        let cloned = category.clone();
        assert_eq!(category, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let category1 = TaxCategory("standard".to_string());
        let category2 = TaxCategory("standard".to_string());
        let category3 = TaxCategory("reduced".to_string());

        assert_eq!(category1, category2);
        assert_ne!(category1, category3);
    }

    #[test]
    fn test_serialization() {
        let category = ValueObject::new(TaxCategory("standard".to_string())).unwrap();
        let serialized = serde_json::to_string(&category).unwrap();
        assert_eq!(serialized, r#""standard""#);
    }
}
