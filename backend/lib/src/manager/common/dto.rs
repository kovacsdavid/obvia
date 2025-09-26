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
use crate::manager::common::types::order::Order;
use crate::manager::common::types::value_object::{ValueObject, ValueObjectable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

/// A generic response struct used to represent a successful response, containing a success flag
/// and some associated data. The data field is generic and must implement the `Serialize` trait.
///
/// # Type Parameters
/// * `T` - The type of the data being included in the response. This type must implement the `Serialize` trait.
///
/// # Fields
/// * `success` - A boolean flag indicating whether the response represents a success. Always true for this struct.
/// * `data` - The actual data payload of the response. Its type is determined by the generic parameter `T`.
#[derive(Serialize)]
pub struct OkResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
}

impl<T: Serialize> OkResponse<T> {
    /// Creates a new instance of the struct.
    ///
    /// This function initializes a new object of the struct with the given data
    /// and sets the `success` field to `true`.
    ///
    /// # Parameters
    /// - `data`: The value of type `T` to initialize the struct with.
    ///
    /// # Returns
    /// A new instance of the struct containing the specified `data` and
    /// with `success` set to `true`.
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}

/// A generic struct representing an error response, used to convey error details in API responses.
///
/// This struct is designed to be serializable for use in JSON or other formats.
///
/// # Type Parameters
/// * `T` - The type of the additional data or context associated with the error body. This type must implement the `Serialize` trait.
///
/// # Fields
/// * `success` - A boolean indicating the success status of the operation. For an error response, this is always expected to be `false`.
/// * `error` - An instance of `ErrorBody` containing detailed information about the error, along with any associated context or data.
#[derive(Serialize)]
pub struct ErrorResponse<T: Serialize> {
    pub success: bool,
    pub error: ErrorBody<T>,
}

impl<T: Serialize> ErrorResponse<T> {
    /// Constructs a new instance of the struct with the provided error information.
    ///
    /// # Parameters
    /// - `error`: An `ErrorBody<T>` that contains details about the error.
    ///
    /// # Returns
    /// A new instance of the struct with `success` set to `false` and the provided `error` value.
    pub fn new(error: ErrorBody<T>) -> Self {
        Self {
            success: false,
            error,
        }
    }
}

/// A generic struct representing the body of an error response.
///
/// This structure is designed to be serialized into formats like JSON using the `Serialize` trait.
/// It includes details about the error with optional field-specific error information.
///
/// # Type Parameters
/// - `T`: A type that implements the `Serialize` trait, representing field-specific error details.
///
/// # Fields
/// - `reference` (`String`):
///   A unique identifier referencing the specific error instance.
///   Typically used for tracking or debugging purposes.
/// - `global` (`String`):
///   A message representing the global or top-level error description.
/// - `fields` (`Option<T>`):
///   Optional field-level error details. Can be used to provide additional context for specific fields
///   when the error is related to input validation or similar cases.
#[derive(Serialize)]
pub struct ErrorBody<T: Serialize> {
    pub global: String,
    pub fields: Option<T>,
}

/// A struct representing a simple message response.
///
/// This struct is used to encapsulate a single message in string format,
/// which can be serialized (e.g., to JSON) for communication purposes.
///
/// # Attributes
/// - `message`: A `String` that contains the message content.
#[derive(Serialize)]
pub struct SimpleMessageResponse {
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QueryParam {
    pub q: Option<String>,
}

impl QueryParam {
    pub fn as_hash_map(&self) -> Option<HashMap<String, String>> {
        match &self.q {
            Some(q) => {
                let mut hmap: HashMap<String, String> = HashMap::new();
                for s1 in q.split("|").collect::<Vec<&str>>() {
                    let param_value = s1.split(":").collect::<Vec<&str>>();
                    if let Some(param) = param_value.first()
                        && let Some(value) = param_value.get(1)
                    {
                        hmap.insert(param.trim().to_string(), value.trim().to_string());
                    }
                }
                if !hmap.is_empty() { Some(hmap) } else { None }
            }
            None => None,
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum PaginatorError {
    #[allow(dead_code)]
    InvalidPage,
    #[allow(dead_code)]
    InvalidLimit,
    #[allow(dead_code)]
    MissingParams,
}

impl Display for PaginatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PaginatorError::InvalidPage => write!(f, "Invalid page!"),
            PaginatorError::InvalidLimit => write!(f, "Invalid limit!"),
            PaginatorError::MissingParams => write!(f, "Missing paginator params!"),
        }
    }
}

#[derive(Serialize)]
pub struct PagedData<T> {
    pub page: i32,
    pub limit: i32,
    pub total: i64,
    pub data: T,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, PartialEq)]
pub struct PaginatorParams {
    pub page: i32,
    pub limit: i32,
}

impl PaginatorParams {
    pub fn offset(&self) -> i32 {
        (self.page - 1) * self.limit
    }
}

impl Default for PaginatorParams {
    fn default() -> Self {
        Self { page: 1, limit: 25 }
    }
}

impl TryFrom<&QueryParam> for PaginatorParams {
    type Error = PaginatorError;
    fn try_from(value: &QueryParam) -> Result<Self, Self::Error> {
        match value.as_hash_map() {
            Some(hmap) => Ok(PaginatorParams {
                page: hmap
                    .get("page")
                    .ok_or(PaginatorError::MissingParams)?
                    .parse()
                    .map_err(|_| PaginatorError::InvalidPage)?,
                limit: hmap
                    .get("limit")
                    .ok_or(PaginatorError::MissingParams)?
                    .parse()
                    .map_err(|_| PaginatorError::InvalidLimit)?,
            }),
            None => Err(PaginatorError::MissingParams),
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum OrderingError {
    #[error("Invalid order")]
    InvalidOrder,
    #[error("Invalid order_by")]
    InvalidOrderBy,
    #[error("Missing parameter")]
    MissingParams,
}

pub struct OrderingParams<T>
where
    T: ValueObjectable<DataType = String>,
{
    pub order_by: ValueObject<T>,
    pub order: ValueObject<Order>,
}

impl<T> TryFrom<&QueryParam> for OrderingParams<T>
where
    T: ValueObjectable<DataType = String> + FromStr,
{
    type Error = OrderingError;
    fn try_from(value: &QueryParam) -> Result<Self, Self::Error> {
        match value.as_hash_map() {
            Some(hmap) => {
                let order_by = ValueObject::new(
                    T::from_str(
                        hmap.get("order_by")
                            .ok_or(OrderingError::MissingParams)?
                            .trim(),
                    )
                    .map_err(|_| OrderingError::InvalidOrderBy)?,
                )
                .map_err(|_| OrderingError::InvalidOrderBy)?;
                let order = ValueObject::new(
                    Order::from_str(
                        hmap.get("order")
                            .ok_or(OrderingError::MissingParams)?
                            .trim(),
                    )
                    .map_err(|_| OrderingError::InvalidOrder)?,
                )
                .map_err(|_| OrderingError::InvalidOrder)?;
                Ok(OrderingParams { order_by, order })
            }
            None => Err(OrderingError::MissingParams),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_param() {
        let query_param = QueryParam {
            q: Some(String::from("")),
        };
        let expected = None;
        assert_eq!(query_param.as_hash_map(), expected);

        let query_param = QueryParam {
            q: Some(String::from("page:1|limit:25|name:éáűú")),
        };
        let mut expected = HashMap::new();
        expected.insert(String::from("page"), String::from("1"));
        expected.insert(String::from("limit"), String::from("25"));
        expected.insert(String::from("name"), String::from("éáűú"));
        assert_eq!(query_param.as_hash_map().unwrap(), expected);

        let query_param = QueryParam {
            q: Some(String::from("limit:25|name:éáűú|page:1")),
        };
        let mut expected = HashMap::new();
        expected.insert(String::from("page"), String::from("1"));
        expected.insert(String::from("limit"), String::from("25"));
        expected.insert(String::from("name"), String::from("éáűú"));
        assert_eq!(query_param.as_hash_map().unwrap(), expected);

        let query_param = QueryParam {
            q: Some(String::from("limit:25|page:1")),
        };
        let mut expected = HashMap::new();
        expected.insert(String::from("page"), String::from("1"));
        expected.insert(String::from("limit"), String::from("25"));
        assert_eq!(query_param.as_hash_map().unwrap(), expected);

        let query_param = QueryParam {
            q: Some(String::from("page:1")),
        };
        let mut expected = HashMap::new();
        expected.insert(String::from("page"), String::from("1"));
        assert_eq!(query_param.as_hash_map().unwrap(), expected);

        let query_param = QueryParam {
            q: Some(String::from("limit:")),
        };
        let mut expected = HashMap::new();
        expected.insert(String::from("limit"), String::from(""));
        assert_eq!(query_param.as_hash_map().unwrap(), expected);

        let query_param = QueryParam {
            q: Some(String::from("   limit   :   ")),
        };
        let mut expected = HashMap::new();
        expected.insert(String::from("limit"), String::from(""));
        assert_eq!(query_param.as_hash_map().unwrap(), expected);
    }

    #[test]
    fn test_paginator() {
        let paginator = PaginatorParams::try_from(&QueryParam {
            q: Some(String::from("page:1|limit:25")),
        })
        .unwrap();
        let expected = PaginatorParams { page: 1, limit: 25 };
        assert_eq!(paginator, expected);

        let paginator = PaginatorParams::try_from(&QueryParam {
            q: Some(String::from("name:éáűú")),
        })
        .unwrap_err();
        let expected = PaginatorError::MissingParams;
        assert_eq!(paginator, expected);

        let paginator = PaginatorParams::try_from(&QueryParam {
            q: Some(String::from("page:1")),
        })
        .unwrap_err();
        let expected = PaginatorError::MissingParams;
        assert_eq!(paginator, expected);

        let paginator = PaginatorParams::try_from(&QueryParam {
            q: Some(String::from("limit:25")),
        })
        .unwrap_err();
        let expected = PaginatorError::MissingParams;
        assert_eq!(paginator, expected);
    }
}
