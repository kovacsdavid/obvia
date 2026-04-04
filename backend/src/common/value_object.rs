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

use std::str::FromStr;
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
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
}

pub trait ValueObjectData: Display + Sized {
    type DataType;
    fn new(data: &str) -> ValueObjectResult<Option<Self>>;
    fn validate(&self) -> ValueObjectResult<()>;
    fn get_data(&self) -> &Self::DataType;
}

#[derive(Debug, Clone)]
pub struct Required;
#[derive(Debug, Clone)]
pub struct Optional;

trait ValueObjectMode<T>: Sized {
    fn new(data: ValueObjectResult<Option<T>>) -> ValueObjectResult<ValueObject<T, Self>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueObject<T, M>(Option<T>, PhantomData<M>);

impl<T> ValueObjectMode<T> for Required
where
    T: ValueObjectData,
{
    fn new(data: ValueObjectResult<Option<T>>) -> ValueObjectResult<ValueObject<T, Self>> {
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

impl<T> ValueObjectMode<T> for Optional
where
    T: ValueObjectData,
{
    fn new(data: ValueObjectResult<Option<T>>) -> ValueObjectResult<ValueObject<T, Optional>> {
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

impl<T> ValueObjectRequired<T>
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

impl<T> ValueObjectRequired<T>
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

impl<T> ValueObjectRequired<T>
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

impl<T> ValueObjectRequired<T>
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

impl<T, M> FromStr for ValueObject<T, M>
where
    T: ValueObjectData,
    M: ValueObjectMode<T>,
{
    type Err = ValueObjectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        M::new(T::new(s))
    }
}

impl<T, M> TryFrom<String> for ValueObject<T, M>
where
    T: ValueObjectData,
    M: ValueObjectMode<T>,
{
    type Error = ValueObjectError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        M::new(T::new(&value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct SampleObjectString(String);

    impl ValueObjectData for SampleObjectString {
        type DataType = String;
        fn new(data: &str) -> ValueObjectResult<Option<Self>> {
            if !data.trim().is_empty() {
                Ok(Some(Self(data.to_owned())))
            } else {
                Ok(None)
            }
        }
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

    #[derive(Debug, Clone)]
    pub struct SampleObjectUuid(Uuid);

    impl ValueObjectData for SampleObjectUuid {
        type DataType = Uuid;

        fn new(data: &str) -> ValueObjectResult<Option<Self>> {
            if !data.trim().is_empty() {
                Ok(Some(Self(data.parse::<Uuid>().map_err(|_| {
                    ValueObjectError::ParseError("Hibás UUID!")
                })?)))
            } else {
                Ok(None)
            }
        }
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
            .parse::<ValueObjectOptional<SampleObjectUuid>>()
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
