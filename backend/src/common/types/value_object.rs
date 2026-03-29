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

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ValueObjectError {
    #[error("{0}")]
    InvalidInput(&'static str),
    #[error("ParseError")]
    ParseError,
}

pub trait ValueObjectData: Display {
    type DataType;
    fn validate(&self) -> Result<(), ValueObjectError>;
    #[allow(dead_code)]
    fn get_value(&self) -> &Self::DataType;
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ValueObject<T>(T);

impl<T> ValueObject<T>
where
    T: ValueObjectData<DataType = String>,
{
    pub fn new_required(data: T) -> Result<ValueObject<T>, ValueObjectError> {
        data.validate()?;
        Ok(ValueObject(data))
    }
    pub fn new_optional(data: T) -> Result<Option<ValueObject<T>>, ValueObjectError> {
        if !data.get_value().trim().is_empty() {
            data.validate()?;
            Ok(Some(ValueObject(data)))
        } else {
            Ok(None)
        }
    }
}

impl<T> ValueObject<T>
where
    T: ValueObjectData<DataType = String>,
{
    #[allow(dead_code)]
    pub fn extract(&self) -> &T {
        &self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.get_value()
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.0.get_value().as_bytes()
    }
    pub fn as_f64(&self) -> Result<f64, ValueObjectError> {
        self.0
            .get_value()
            .parse::<f64>()
            .map_err(|_| ValueObjectError::ParseError)
    }
    pub fn as_i32(&self) -> Result<i32, ValueObjectError> {
        self.0
            .get_value()
            .parse::<i32>()
            .map_err(|_| ValueObjectError::ParseError)
    }
    pub fn as_u16(&self) -> Result<u16, ValueObjectError> {
        self.0
            .get_value()
            .parse::<u16>()
            .map_err(|_| ValueObjectError::ParseError)
    }
}

impl<T> Display for ValueObject<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct SampleObject(pub String);

impl ValueObjectData for SampleObject {
    type DataType = String;
    fn validate(&self) -> Result<(), ValueObjectError> {
        if self.0 == "sample_object" {
            Ok(())
        } else {
            Err(ValueObjectError::InvalidInput("Invalid sample object!"))
        }
    }

    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for SampleObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<SampleObject> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new_required(SampleObject(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_sample_object_test() {
        let sample_object =
            serde_json::from_str::<ValueObject<SampleObject>>(r#""sample_object""#).unwrap();
        let res = sample_object.as_str();
        assert_eq!(res, "sample_object");
    }
    #[test]
    fn invalid_sample_object_test() {
        let res = serde_json::from_str::<ValueObject<SampleObject>>(r#""test""#)
            .unwrap_err()
            .to_string();
        assert_eq!(res, "Invalid sample object!");
    }
}
