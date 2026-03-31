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
#![allow(dead_code)]

use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};
use thiserror::Error;
use uuid::Uuid;

pub type ValueObjectResult<T> = Result<T, ValueObjectError>;

pub type ValueObjectRequired<T> = ValueObject<T, Required>;

pub type ValueObjectOptional<T> = ValueObject<T, Optional>;

#[derive(Debug, Error, PartialEq)]
pub enum ValueObjectError {
    #[error("{0}")]
    InvalidInput(&'static str),
    #[error("{0}")]
    ParseError(&'static str),
    #[error("InvalidState")]
    InvalidState,
}

pub trait ValueObjectData: Display {
    type DataType;
    fn validate(&self) -> ValueObjectResult<()>;
    fn get_data(&self) -> &Self::DataType;
}

#[derive(Debug)]
pub struct Required;
#[derive(Debug)]
pub struct Optional;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ValueObject<T, M>(Option<T>, PhantomData<M>);

impl<T> ValueObject<T, Required>
where
    T: ValueObjectData,
{
    pub fn new(
        data: Result<Option<T>, ValueObjectError>,
    ) -> ValueObjectResult<ValueObject<T, Required>> {
        let data = data?;
        match data {
            Some(d) => {
                d.validate()?;
                Ok(ValueObject(Some(d), PhantomData))
            }
            None => Err(ValueObjectError::InvalidInput("A mező kitöltése kötelező")),
        }
    }
}

impl<T> ValueObject<T, Optional>
where
    T: ValueObjectData,
{
    pub fn new(
        data: Result<Option<T>, ValueObjectError>,
    ) -> ValueObjectResult<ValueObject<T, Optional>> {
        let data = data?;
        match data {
            Some(d) => {
                d.validate()?;
                Ok(ValueObject(Some(d), PhantomData))
            }
            None => Ok(ValueObject(None, PhantomData)),
        }
    }
}

impl<T> ValueObject<T, Required>
where
    T: ValueObjectData<DataType = Uuid>,
{
    pub fn as_uuid(&self) -> ValueObjectResult<Uuid> {
        Ok(*self
            .0
            .as_ref()
            .ok_or(ValueObjectError::InvalidState)?
            .get_data())
    }
}

impl<T> ValueObject<T, Required>
where
    T: ValueObjectData<DataType = String>,
{
    pub fn as_str(&self) -> ValueObjectResult<&str> {
        Ok(self
            .0
            .as_ref()
            .ok_or(ValueObjectError::InvalidState)?
            .get_data())
    }
}

impl<T> ValueObject<T, Required>
where
    T: ValueObjectData<DataType = f64>,
{
    pub fn as_f64(&self) -> ValueObjectResult<f64> {
        Ok(*self
            .0
            .as_ref()
            .ok_or(ValueObjectError::InvalidState)?
            .get_data())
    }
}

impl<T> ValueObject<T, Required>
where
    T: ValueObjectData<DataType = i32>,
{
    pub fn as_i32(&self) -> ValueObjectResult<i32> {
        Ok(*self
            .0
            .as_ref()
            .ok_or(ValueObjectError::InvalidState)?
            .get_data())
    }
}

impl<T> ValueObject<T, Required>
where
    T: ValueObjectData<DataType = u16>,
{
    pub fn as_u16(&self) -> ValueObjectResult<u16> {
        Ok(*self
            .0
            .as_ref()
            .ok_or(ValueObjectError::InvalidState)?
            .get_data())
    }
}

impl<T, M> Display for ValueObject<T, M>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(v) => write!(f, "{}", v),
            None => write!(f, ""),
        }
    }
}

impl<T> ValueObject<T, Optional>
where
    T: ValueObjectData<DataType = Uuid>,
{
    pub fn as_uuid(&self) -> Option<Uuid> {
        self.0.as_ref().map(|v| *v.get_data())
    }
}

impl<T> ValueObject<T, Optional>
where
    T: ValueObjectData<DataType = String>,
{
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(|v| v.get_data().as_str())
    }
}

impl<T> ValueObject<T, Optional>
where
    T: ValueObjectData<DataType = f64>,
{
    pub fn as_f64(&self) -> Option<f64> {
        self.0.as_ref().map(|v| *v.get_data())
    }
}

impl<T> ValueObject<T, Optional>
where
    T: ValueObjectData<DataType = i32>,
{
    pub fn as_i32(&self) -> Option<i32> {
        self.0.as_ref().map(|v| *v.get_data())
    }
}

impl<T> ValueObject<T, Optional>
where
    T: ValueObjectData<DataType = u16>,
{
    pub fn as_u16(&self) -> Option<u16> {
        self.0.as_ref().map(|v| *v.get_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[derive(Debug, Clone)]
    pub struct SampleObjectString(String);

    impl SampleObjectString {
        fn new(data: &str) -> ValueObjectResult<Option<Self>> {
            if !data.trim().is_empty() {
                Ok(Some(Self(data.to_owned())))
            } else {
                Ok(None)
            }
        }
    }

    impl ValueObjectData for SampleObjectString {
        type DataType = String;
        fn validate(&self) -> ValueObjectResult<()> {
            if self.0 == "sample_object" {
                Ok(())
            } else {
                Err(ValueObjectError::InvalidInput("sample_object_error"))
            }
        }
        fn get_data(&self) -> &Self::DataType {
            &self.0
        }
    }

    impl Display for SampleObjectString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl FromStr for ValueObject<SampleObjectString, Required> {
        type Err = ValueObjectError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::new(SampleObjectString::new(s))
        }
    }

    impl FromStr for ValueObject<SampleObjectString, Optional> {
        type Err = ValueObjectError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::new(SampleObjectString::new(s))
        }
    }

    #[derive(Debug, Clone)]
    pub struct SampleObjectUuid(Uuid);

    impl SampleObjectUuid {
        fn new(data: &str) -> ValueObjectResult<Option<Self>> {
            if !data.trim().is_empty() {
                Ok(Some(Self(data.parse::<Uuid>().map_err(|_| {
                    ValueObjectError::ParseError("Hibás UUID!")
                })?)))
            } else {
                Ok(None)
            }
        }
    }

    impl ValueObjectData for SampleObjectUuid {
        type DataType = Uuid;
        fn validate(&self) -> ValueObjectResult<()> {
            Ok(())
        }
        fn get_data(&self) -> &Self::DataType {
            &self.0
        }
    }

    impl Display for SampleObjectUuid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl FromStr for ValueObject<SampleObjectUuid, Required> {
        type Err = ValueObjectError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::new(SampleObjectUuid::new(s))
        }
    }

    impl FromStr for ValueObject<SampleObjectUuid, Optional> {
        type Err = ValueObjectError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::new(SampleObjectUuid::new(s))
        }
    }

    #[test]
    fn value_object_required_string_valid() {
        let sample_object = "sample_object"
            .parse::<ValueObjectRequired<SampleObjectString>>()
            .unwrap();
        assert_eq!(sample_object.as_str().unwrap(), "sample_object");
    }

    #[test]
    fn value_object_required_string_invalid() {
        let sample_object = "invalid_object"
            .parse::<ValueObjectRequired<SampleObjectString>>()
            .unwrap_err();
        assert_eq!(sample_object.to_string(), "sample_object_error");
    }

    #[test]
    fn value_object_optional_string_valid() {
        let sample_object = "sample_object"
            .parse::<ValueObjectOptional<SampleObjectString>>()
            .unwrap();
        assert_eq!(sample_object.as_str().unwrap(), "sample_object");
    }

    #[test]
    fn value_object_optional_string_invalid() {
        let sample_object = "invalid_object"
            .parse::<ValueObjectOptional<SampleObjectString>>()
            .unwrap_err();
        assert_eq!(sample_object.to_string(), "sample_object_error");
    }

    #[test]
    fn value_object_optional_string_empty() {
        let sample_object = ""
            .parse::<ValueObjectOptional<SampleObjectString>>()
            .unwrap();
        assert!(sample_object.as_str().is_none());
    }

    #[test]
    fn value_object_optional_string_empty_whitespace() {
        let sample_object = "   "
            .parse::<ValueObjectOptional<SampleObjectString>>()
            .unwrap();
        assert!(sample_object.as_str().is_none());
    }

    #[test]
    fn value_object_required_uuid_valid() {
        let uuid = Uuid::new_v4();
        let sample_object = uuid
            .to_string()
            .parse::<ValueObjectRequired<SampleObjectUuid>>()
            .unwrap();
        assert_eq!(sample_object.as_uuid().unwrap(), uuid);
    }

    #[test]
    fn value_object_required_uuid_invalid() {
        let sample_object = "invalid_object"
            .parse::<ValueObjectRequired<SampleObjectUuid>>()
            .unwrap_err();
        assert_eq!(sample_object.to_string(), "Hibás UUID!");
    }

    #[test]
    fn value_object_optional_uuid_valid() {
        let uuid = Uuid::new_v4();
        let sample_object = uuid
            .to_string()
            .parse::<ValueObjectOptional<SampleObjectUuid>>()
            .unwrap();
        assert!(sample_object.as_uuid().is_some());
        assert_eq!(sample_object.as_uuid().unwrap(), uuid);
    }

    #[test]
    fn value_object_optional_uuid_invalid() {
        let sample_object = "invalid_object"
            .parse::<ValueObject<SampleObjectUuid, Optional>>()
            .unwrap_err();
        assert_eq!(sample_object.to_string(), "Hibás UUID!");
    }

    #[test]
    fn value_object_optional_uuid_empty() {
        let sample_object = "".parse::<ValueObjectOptional<SampleObjectUuid>>().unwrap();
        assert!(sample_object.as_uuid().is_none());
    }

    #[test]
    fn value_object_optional_uuid_empty_whitespace() {
        let sample_object = "   "
            .parse::<ValueObjectOptional<SampleObjectUuid>>()
            .unwrap();
        assert!(sample_object.as_uuid().is_none());
    }
}
