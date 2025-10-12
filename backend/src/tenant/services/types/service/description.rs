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

use crate::common::types::value_object::{ValueObject, ValueObjectable};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Description(pub String);

impl ValueObjectable for Description {
    type DataType = String;

    fn validate(&self) -> Result<(), String> {
        if self.0.len() <= 3000 {
            Ok(())
        } else {
            Err(String::from(
                "A leírás nem lehet 3 000 karakternél hosszabb!",
            ))
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Description> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(Description(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_description() {
        let desc: ValueObject<Description> =
            serde_json::from_str(r#""Valid description""#).unwrap();
        assert_eq!(desc.extract().get_value(), "Valid description");
    }

    #[test]
    fn test_empty_description() {
        let desc: ValueObject<Description> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(desc.extract().get_value(), "");
    }

    #[test]
    fn test_max_length_description() {
        let desc = "a".repeat(3000);
        let result: ValueObject<Description> =
            serde_json::from_str(&format!(r#""{}""#, desc)).unwrap();
        assert_eq!(result.extract().get_value(), &desc);
    }

    #[test]
    fn test_too_long_description() {
        let desc = "a".repeat(3001);
        let result: Result<ValueObject<Description>, _> =
            serde_json::from_str(&format!(r#""{}""#, desc));
        assert!(result.is_err());
    }

    #[test]
    fn test_display_trait() {
        let desc = Description("Test description".to_string());
        assert_eq!(format!("{}", desc), "Test description");
    }

    #[test]
    fn test_debug_trait() {
        let desc = Description("Test description".to_string());
        assert_eq!(format!("{:?}", desc), r#"Description("Test description")"#);
    }

    #[test]
    fn test_clone_trait() {
        let desc = Description("Test description".to_string());
        let cloned = desc.clone();
        assert_eq!(desc, cloned);
    }

    #[test]
    fn test_serialize() {
        let desc = Description("Test description".to_string());
        let serialized = serde_json::to_string(&desc).unwrap();
        assert_eq!(serialized, r#""Test description""#);
    }

    #[test]
    fn test_deserialize() {
        let input = r#""Test description""#;
        let deserialized: ValueObject<Description> = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized.extract().get_value(), "Test description");
    }

    #[test]
    fn test_special_characters() {
        let special = r#""Test with !@#$%^&*()_+ and unicode 你好世界""#;
        let desc: ValueObject<Description> = serde_json::from_str(special).unwrap();
        assert_eq!(
            desc.extract().get_value(),
            r#"Test with !@#$%^&*()_+ and unicode 你好世界"#
        );
    }

    #[test]
    fn test_multiline_description() {
        let multiline = r#""Line 1\nLine 2\nLine 3""#;
        let desc: ValueObject<Description> = serde_json::from_str(multiline).unwrap();
        assert_eq!(desc.extract().get_value(), "Line 1\nLine 2\nLine 3");
    }
}
