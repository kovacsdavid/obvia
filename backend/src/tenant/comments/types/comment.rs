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

use crate::common::types::{ValueObject, ValueObjectable, value_object::ValueObjectError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Comment(pub String);

impl ValueObjectable for Comment {
    type DataType = String;

    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0.len() <= 10_000 {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput(
                "A megjegyzés nem lehet 10 000 karakternél hosszabb!",
            ))
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<Comment> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(Comment(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_comment() {
        let desc: ValueObject<Comment> = serde_json::from_str(r#""Valid comment""#).unwrap();
        assert_eq!(desc.as_str(), "Valid comment");
    }

    #[test]
    fn test_empty_comment() {
        let desc: ValueObject<Comment> = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(desc.as_str(), "");
    }

    #[test]
    fn test_max_length_comment() {
        let desc = "a".repeat(3000);
        let result: ValueObject<Comment> = serde_json::from_str(&format!(r#""{}""#, desc)).unwrap();
        assert_eq!(result.as_str(), &desc);
    }

    #[test]
    fn test_too_long_comment() {
        let desc = "a".repeat(10001);
        let result: Result<ValueObject<Comment>, _> =
            serde_json::from_str(&format!(r#""{}""#, desc));
        assert!(result.is_err());
    }

    #[test]
    fn test_display_trait() {
        let desc = Comment("Test comment".to_string());
        assert_eq!(format!("{}", desc), "Test comment");
    }

    #[test]
    fn test_debug_trait() {
        let desc = Comment("Test comment".to_string());
        assert_eq!(format!("{:?}", desc), r#"Comment("Test comment")"#);
    }

    #[test]
    fn test_clone_trait() {
        let desc = Comment("Test comment".to_string());
        let cloned = desc.clone();
        assert_eq!(desc, cloned);
    }

    #[test]
    fn test_serialize() {
        let desc = Comment("Test comment".to_string());
        let serialized = serde_json::to_string(&desc).unwrap();
        assert_eq!(serialized, r#""Test comment""#);
    }

    #[test]
    fn test_deserialize() {
        let input = r#""Test comment""#;
        let deserialized: ValueObject<Comment> = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized.as_str(), "Test comment");
    }

    #[test]
    fn test_special_characters() {
        let special = r#""Test with !@#$%^&*()_+ and unicode 你好世界""#;
        let desc: ValueObject<Comment> = serde_json::from_str(special).unwrap();
        assert_eq!(
            desc.as_str(),
            r#"Test with !@#$%^&*()_+ and unicode 你好世界"#
        );
    }

    #[test]
    fn test_multiline_comment() {
        let multiline = r#""Line 1\nLine 2\nLine 3""#;
        let desc: ValueObject<Comment> = serde_json::from_str(multiline).unwrap();
        assert_eq!(desc.as_str(), "Line 1\nLine 2\nLine 3");
    }
}
