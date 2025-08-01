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

use serde::Serialize;

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
    pub reference: String,
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
