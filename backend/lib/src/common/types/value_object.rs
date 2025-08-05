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
use serde::Deserialize;
use std::fmt::Display;

/// A trait representing a container for value object data. This trait combines functionality
/// for display and deserialization and provides methods for data validation and retrieval.
///
/// # Associated Types
/// * `DataType` - The type of the data contained within the value object.
///
/// # Required Traits
/// This trait requires the implementer to also implement the following:
/// * `Display` - Allows for formatting the value object as a string.
/// * `Deserialize<'static>` - Enables deserialization of the value object.
///
/// # Provided Methods
///
/// ## `fn validate(&self) -> Result<(), String>`
/// Validates the contained data. This method has a default implementation that returns
/// an error with the message "Not implemented".
///
/// ### Returns
/// * `Ok(())` if the data is valid.
/// * `Err(String)` containing an error message if validation fails or is not implemented.
///
/// ## `fn get_value(&self) -> &Self::DataType`
/// Retrieves a reference to the contained data.
///
/// ### Returns
/// * `&Self::DataType` - A reference to the associated type `DataType` that represents the underlying data.
pub trait ValueObjectable: Display {
    type DataType;
    /// Validates the implementation or object.
    ///
    /// This method is intended to perform validation checks on the implementing type.
    /// It returns a `Result` indicating whether the validation was successful or not.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the validation is successful.
    /// - `Err(String)`: If the validation fails, containing an error message.
    fn validate(&self) -> Result<(), String>;
    /// Retrieves a reference to the data associated with the current instance.
    ///
    /// # Returns
    /// * `&Self::DataType` - A reference to the data of the type specified by `Self::DataType`.
    ///
    /// # Notes
    /// - This method provides read-only access to the data.
    /// - Ensure that the implementing type defines the associated `DataType` for proper usage.
    #[allow(dead_code)]
    fn get_value(&self) -> &Self::DataType;
}

/// A generic `ValueObject` struct that wraps a single data type, `DataType`.
///
/// # Type Parameters
/// - `DataType`: The type of the value this struct encapsulates. It can be any type.
///
/// # Notes
/// - This struct holds only a single data item (`DataType`) and does not provide
///   any additional behavior or logic beyond encapsulation.
/// - It is often used to enforce type safety for specific domain concepts where primitive
///   types alone might not provide adequate clarity or constraint.
#[derive(Debug, Clone, PartialEq)]
pub struct ValueObject<DataType>(DataType);

impl<DataType> ValueObject<DataType>
where
    DataType: ValueObjectable,
{
    /// Creates a new `ValueObject` instance from the provided `data`.
    ///
    /// # Arguments
    /// - `data` - An instance of `DataType` that will be used to create the `ValueObject`.
    ///
    /// # Returns
    /// - `Ok(ValueObject<DataType>)` - If the provided `data` successfully passes validation.
    /// - `Err(String)` - If the `data` validation fails, an error message will be returned.
    ///
    /// # Errors
    /// - Returns an error `String` if the `validate()` method fails to validate the passed `data`.
    pub fn new(data: DataType) -> Result<ValueObject<DataType>, String> {
        data.validate()?;
        Ok(ValueObject(data))
    }
}

impl<DataType> ValueObject<DataType> {
    ///
    /// Retrieves a reference to the inner data stored within the wrapper (typically a tuple struct).
    ///
    /// # Returns
    ///
    /// A reference to the internal data of type `DataType`.
    #[allow(dead_code)]
    pub fn extract(&self) -> &DataType {
        &self.0
    }
}

impl<DataType> Display for ValueObject<DataType>
where
    DataType: Display,
{
    /// Implements the `fmt` method for the `Display` trait to control how an instance
    /// of the type is formatted when used with formatting macros, such as `println!`.
    ///
    /// # Parameters
    /// - `&self`: A reference to the instance of the type implementing the `Display` trait.
    /// - `f`: A mutable reference to a `Formatter` instance, which is used to build the
    ///   formatted output.
    ///
    /// # Returns
    /// - `std::fmt::Result`: Returns `Ok(())` if formatting is successful, or an error
    ///   (`Err`) if formatting fails.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A struct that represents a sample object
///
/// Fields:
/// - `0`: A single unnamed `String` field, stored as a tuple struct.
///
/// # Note
/// This is just an example implementation!
#[derive(Debug, Clone)]
pub struct SampleObject(String);

impl ValueObjectable for SampleObject {
    type DataType = String;
    /// Validates whether the current object meets a specific condition.
    ///
    /// This method checks if the value of the internal data (`self.0`)
    /// is equal to the string `"sample_object"`.
    /// If the condition is met, the method returns `Ok(())`, indicating successful validation.
    /// Otherwise, it returns an `Err` containing an error message.
    ///
    /// # Returns
    /// * `Ok(())` - if the internal value is `"sample_object"`.
    /// * `Err(String)` - if the internal value is not `"sample_object"`, with an error message.
    fn validate(&self) -> Result<(), String> {
        if self.0 == "sample_object" {
            Ok(())
        } else {
            Err(String::from("Invalid sample object!"))
        }
    }

    /// Retrieves a reference to the value contained within the struct.
    ///
    /// # Returns
    /// A reference to the internal value of type `Self::DataType`.
    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for SampleObject {
    /// Implements the `fmt` method from the `std::fmt::Display` or `std::fmt::Debug` trait,
    /// enabling a custom display of the struct or type.
    ///
    /// # Parameters
    /// - `&self`: A reference to the instance of the type implementing this method.
    /// - `f`: A mutable reference to a `std::fmt::Formatter` used for formatting output.
    ///
    /// # Returns
    /// - `std::fmt::Result`: Indicates whether the formatting operation was successful
    ///   (`Ok(())`) or an error occurred (`Err`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ValueObject<SampleObject> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `SampleObject` and validates it by calling `ValueObject::new`.
    /// If the validation fails, a custom deserialization error is returned.
    ///
    /// # Type Parameters
    /// - `D`: The type of the deserializer, which must implement `serde::Deserializer<'de>`.
    ///
    /// # Parameters
    /// - `deserializer`: The deserializer used to deserialize the input.
    ///
    /// # Returns
    /// - `Result<Self, D::Error>`:
    ///   - On success, returns the constructed and validated object wrapped in `Ok`.
    ///   - On failure, returns a custom error wrapped in `Err`.
    ///
    /// # Errors
    /// - Returns a deserialization error if:
    ///   - The input cannot be deserialized into a `String`.
    ///   - Validation using `ValueObject::new` fails, causing the `map_err` call to propagate an error.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(SampleObject(s)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_object_test() {
        let res = serde_json::from_str::<ValueObject<SampleObject>>(r#""test""#)
            .unwrap_err()
            .to_string();
        assert_eq!(res, "Invalid sample object!");
        let sample_object =
            serde_json::from_str::<ValueObject<SampleObject>>(r#""sample_object""#).unwrap();
        let res = sample_object.extract();
        assert_eq!(res.get_value(), "sample_object");
    }
}
