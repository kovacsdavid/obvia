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
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Represents the port of a database.
///
/// The `DbPort` struct is a simple wrapper around a `String` that is used
/// to encapsulate the port of a database server. This can
/// help ensure type safety and improve code readability by explicitly
/// conveying the purpose of the contained value.
///
/// # Fields
///
/// * `0`: The inner `String` containing the port of the database.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DbPort(pub i64);

impl ValueObjectable for DbPort {
    type DataType = i64;

    /// Validates the value of the database port stored in the current instance.
    ///
    /// This function checks if the port value (assumed to be stored as `self.0`)
    /// falls within the valid range for common database ports (1025 to 65535 inclusive).
    /// If the value is within the valid range, the function returns `Ok(())`.
    /// Otherwise, it returns an `Err` with a specific error message.
    ///
    /// # Returns
    /// - `Ok(())` if the `self.0` value is valid (i.e., greater than 1024 and less than or equal 65535).
    /// - `Err(String)` containing an error message ("Hibás adatbázis port") if the value is outside the valid range.
    fn validate(&self) -> Result<(), String> {
        if self.0 > 1024 && self.0 <= 65535 {
            Ok(())
        } else {
            Err(String::from("Hibás adatbázis port"))
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

impl Display for DbPort {
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
/// A struct representing the `DbPortVisitor`.
///
/// `DbPortVisitor` is a type that can be used as a custom visitor in the context of deserialization or similar use cases.
/// Its main purpose is to customize or control the logic of handling database port values during specific operations.
///
/// This struct may be particularly useful when parsing or validating input data during deserialization,
/// ensuring that the database port adheres to certain constraints or expectations.
struct DbPortVisitor;

impl<'de> Visitor<'de> for DbPortVisitor {
    type Value = ValueObject<DbPort>;

    /// Formats an expected value description for error messages.
    ///
    /// This method is typically used in serialization/deserialization error handling to
    /// describe the expected type or value. In this case, it specifies that an integer
    /// within the valid 64-bit signed range is expected.
    ///
    /// # Arguments
    ///
    /// * `formatter` - A mutable reference to a `std::fmt::Formatter` that handles
    ///   the formatting of the output string.
    ///
    /// # Returns
    ///
    /// This method returns a `std::fmt::Result`, which is `Ok(())` if the writing
    /// operation was successful or an error otherwise.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^63 and 2^63-1")
    }
    /// A visitor method for handling `i64` deserialization into a `DbPort`.
    ///
    /// This method is typically used when implementing a custom deserializer
    /// with the serde framework. It attempts to transform an input value of
    /// type `i64` into a `DbPort`. Errors are propagated using the `de::Error`
    /// trait.
    ///
    /// # Type Parameters
    /// - `E`: The error type that implements `serde::de::Error`
    ///
    /// # Parameters
    /// - `v`: An input integer of type `i64` to be processed and converted.
    ///
    /// # Returns
    /// Returns a `Result` where:
    /// - `Ok(Self::Value)` contains the successfully converted `DbPort` value.
    /// - `Err(E)` contains an error raised during the conversion process.
    ///
    /// # Errors
    /// - If the `i64` value `v` cannot be successfully converted to a `DbPort`
    ///   via `DbPort::try_from`, an error is returned using `de::Error::custom`.
    /// - If the intermediate operations within the conversion process fail,
    ///   those errors are also propagated with `de::Error::custom`.
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ValueObject::new(DbPort(v)).map_err(de::Error::custom)
    }
    /// Visits a `u64` value during deserialization and attempts to convert it into a `DbPort`.
    ///
    /// This method is typically part of a type implementing the `Visitor` trait and is used during
    /// the deserialization process to handle `u64` values. It performs the following steps:
    /// 1. Converts the `u64` value into an `i64` using `i64::try_from`, returning a custom deserialization
    ///    error if the conversion fails.
    /// 2. Attempts to convert the resulting `i64` into a `DbPort` using `DbPort::try_from`.
    /// 3. Returns a `Result` wrapping the successfully created `DbPort` or propagates any errors as a
    ///    custom deserialization error.
    ///
    /// # Type Parameters
    /// - `E`: A type implementing the `de::Error` trait, used for reporting deserialization errors.
    ///
    /// # Parameters
    /// - `self`: The instance of the visitor.
    /// - `v`: The `u64` value to be visited and converted.
    ///
    /// # Returns
    /// - `Result<Self::Value, E>`: Returns `Ok(Self::Value)` if the conversion is successful,
    ///   or `Err(E)` if any step in the process fails.
    ///
    /// # Errors
    /// - Returns a custom deserialization error using `de::Error::custom` if:
    ///   - The conversion from `u64` to `i64` fails.
    ///   - The conversion from `i64` to `DbPort` fails.
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ValueObject::new(DbPort(i64::try_from(v).map_err(de::Error::custom)?))
            .map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for ValueObject<DbPort> {
    /// A custom implementation of the `deserialize` method
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i64(DbPortVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct Test {
        pub port: ValueObject<DbPort>,
    }
    #[test]
    fn test_valid_db_port() {
        let valid_ports = vec![1025_i64, 5432_i64, 3306_i64];
        for port in valid_ports {
            let data = json!({
                "port": port,
            });
            let test = Test::deserialize(&data).unwrap();
            assert_eq!(*test.port.extract().get_value(), port);
        }
    }
    #[test]
    fn test_invalid_db_port() {
        let invalid_ports = vec![0_i64, 1024_i64, 65536_i64];
        for port in invalid_ports {
            let data = json!({
                "port": port,
            });
            assert_eq!(
                Test::deserialize(&data).unwrap_err().to_string(),
                "Hibás adatbázis port"
            );
        }
        let data = json!({
                "port": "",
        });
        assert!(
            Test::deserialize(&data)
                .unwrap_err()
                .to_string()
                .contains("an integer between -2^63 and 2^63-1")
        );
        let data = json!({
                "port": " ",
        });
        assert!(
            Test::deserialize(&data)
                .unwrap_err()
                .to_string()
                .contains("an integer between -2^63 and 2^63-1")
        );
        let data = json!({
                "port": "asdflkj",
        });
        assert!(
            Test::deserialize(&data)
                .unwrap_err()
                .to_string()
                .contains("an integer between -2^63 and 2^63-1")
        );
        let data = json!({
                "port": null,
        });
        assert!(
            Test::deserialize(&data)
                .unwrap_err()
                .to_string()
                .contains("an integer between -2^63 and 2^63-1")
        );
    }
}
