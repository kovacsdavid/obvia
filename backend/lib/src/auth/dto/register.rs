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

use crate::common::dto::{ErrorBody, ErrorResponse};
use crate::common::types::value_object::{ValueObject, ValueObjectable};
use crate::common::types::{Email, FirstName, LastName, Password};
use ::serde::Serialize;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

/// This is a helper struct for registration requests.
///
/// # Fields
///
/// * `email` - The email address of the user attempting to register. This is
///   expected to be a valid email format.
/// * `first_name` - The first name of the user.
/// * `last_name` - The last name of the user.
/// * `password` - The password chosen by the user for their account. This
///   should meet the security requirements defined elsewhere in the application.
/// * `password_confirm` - A confirmation of the chosen password. This should
///   match the `password` field to ensure no typo was made during entry.
///
/// # Usage
///
/// This struct is used to deserialize user input to a more permissive struct before the more
/// restrictive validation happens to make error handling more customizable.
///
/// # Security
/// - Make sure to handle the `password` field securely and avoid logging, storing or exposing it
///   in any other ways
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RegisterRequestHelper {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub password_confirm: String,
}

/// Represents an error response for a user registration request.
///
/// This struct is designed to capture validation or processing errors
/// that may occur during a registration attempt. Each field corresponds
/// to a specific part of the input data, allowing detailed feedback on
/// invalid or missing information.
///
/// # Fields
///
/// * `email` - Optional string containing an error message associated with the email field.
/// * `first_name` - Optional string containing an error message associated with the first name field.
/// * `last_name` - Optional string containing an error message associated with the last name field.
/// * `password` - Optional string containing an error message associated with the password field.
/// * `password_confirm` - Optional string containing an error message for the password confirmation field.
///
/// # Security
/// - Make sure to handle the `password` field securely and avoid logging, storing or exposing it
///   in any other ways
/// - Make sure to handle the `password_confirm` field securely and avoid logging, storing or exposing it
///   in any other ways
#[derive(Debug, Serialize)]
pub struct RegisterRequestError {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
    pub password_confirm: Option<String>,
}
impl RegisterRequestError {
    /// Checks if the current instance of the struct is empty.
    ///
    /// This method evaluates whether all the optional fields (`email`, `first_name`,
    /// `last_name`, `password`, and `password_confirm`) in the struct are `None`.
    ///
    /// # Returns
    /// * `true` - If all fields are `None`.
    /// * `false` - If at least one field has a value.
    pub fn is_empty(&self) -> bool {
        self.email.is_none()
            && self.first_name.is_none()
            && self.last_name.is_none()
            && self.password.is_none()
            && self.password_confirm.is_none()
    }
}

impl IntoResponse for RegisterRequestError {
    /// Converts the `RegisterRequestError` into an HTTP response.  
    ///
    /// This function is specifically designed to handle validation errors, returning
    /// an `UNPROCESSABLE_ENTITY` (HTTP 422) status code along with a detailed JSON response
    /// that contains information about the validation issues.  
    ///
    /// # Returns
    ///
    /// A `Response` object representing the HTTP response. The body of the response is a
    /// JSON object that includes:
    /// - A `reference` indicating the source or context of the error.
    /// - A `global` message describing the general issue.
    /// - A `fields` object (if available) that maps individual field names to their respective errors.
    ///
    /// # Usage
    /// This method is typically used in scenarios where user-input validation has failed,
    /// and you want to provide detailed feedback to the client about the specific errors found.
    ///
    /// # Example Response
    /// ```json
    /// {
    ///   "reference": "AUTH/DTO/REGISTER",
    ///   "global": "Kérjük ellenőrizze a hibás mezőket",
    ///   "fields": {
    ///     "username": "A username mező nem lehet üres",
    ///     "password": "A jelszó túl rövid"
    ///   }
    /// }
    /// ```
    ///
    /// # Note
    /// The returned `fields` object may be `null` if no field-specific errors are present.
    ///
    /// # Hungarian Message
    /// The `global` message is hardcoded in Hungarian: "Kérjük ellenőrizze a hibás mezőket,"
    /// which translates to "Please check the erroneous fields."
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                reference: "AUTH/DTO/REGISTER".to_string(),
                global: "Kérjük ellenőrizze a hibás mezőket".to_string(), // TODO: i18n?
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

/// The `RegisterRequest` struct represents a request payload
/// for user registration in the system. It encapsulates the
/// necessary data required to register a new user.
///
/// # Fields
///
/// * `email` - The email address of the user, represented by the `Email` type.
/// * `first_name` - The user's first name, represented by the `FirstName` type.
/// * `last_name` - The user's last name, represented by the `LastName` type.
/// * `password` - The user's password, represented by the `Password` type.
/// * `password_confirm` - A `String` that contains the confirmation of the
///   user's password, used to verify that the password field has been entered correctly.
///
/// # Security
/// - Make sure to handle the `password` field securely and avoid logging, storing or exposing it
///   in any other ways
/// - Make sure to handle the `password_confirm` field securely and avoid logging, storing or exposing it
///   in any other ways
/// # See also
/// - `RegisterRequestHelper`
#[derive(Debug, PartialEq, Clone)]
pub struct RegisterRequest {
    pub email: ValueObject<Email>,
    pub first_name: ValueObject<FirstName>,
    pub last_name: ValueObject<LastName>,
    pub password: ValueObject<Password>,
    pub password_confirm: String,
}

impl TryFrom<RegisterRequestHelper> for RegisterRequest {
    type Error = RegisterRequestError;
    /// Attempts to create an instance of `RegisterRequest` from a given `RegisterRequestHelper`.
    ///
    /// This function validates the input fields of the `RegisterRequestHelper` and attempts to create a valid `RegisterRequest` object.
    /// If any of the field validation fails, an error containing the respective validation issues is returned.
    ///
    /// # Parameters
    /// - `value`: A `RegisterRequestHelper` object containing the raw user input that needs validation and conversion.
    ///
    /// # Returns
    /// - `Ok(RegisterRequest)`: If all fields are valid and the registration request can be successfully created.
    /// - `Err(RegisterRequestError)`: If any of the input fields are invalid, returning an error object with detailed information
    ///   about which fields failed validation.
    ///
    /// # Field Validation Process
    /// - `Email`: Converted and validated using `Email::try_from(value.email)`. Adds an error message if invalid.
    /// - `First Name`: Converted and validated using `FirstName::try_from(value.first_name)`. Adds an error message if invalid.
    /// - `Last Name`: Converted and validated using `LastName::try_from(value.last_name)`. Adds an error message if invalid.
    /// - `Password`: Converted and validated using `Password::try_from(value.password)`. Adds an error message if invalid.
    /// - `Password Confirmation`: Checked to match the validated password field. Adds an error message when mismatched.
    ///
    /// # Errors
    /// A `RegisterRequestError` object is returned if any of the following conditions are met:
    /// - The email field is invalid.
    /// - The first name field is invalid.
    /// - The last name field is invalid.
    /// - The password field is invalid.
    /// - The password and password confirmation do not match.
    ///
    /// # Notes
    /// - The `is_empty()` method on `RegisterRequestError` is used to check if there are no validation errors.
    /// - If individual field validations fail, their respective error messages are collected in the `RegisterRequestError` object.
    /// - This function assumes that `RegisterRequestError` implements functionality to store errors for each field.
    ///
    /// # Security
    /// - Make sure to handle the `password` field securely and avoid logging, storing or exposing it
    ///   in any other ways
    /// - Make sure to handle the `password_confirm` field securely and avoid logging, storing or exposing it
    ///   in any other ways
    fn try_from(value: RegisterRequestHelper) -> Result<Self, Self::Error> {
        let mut errors = RegisterRequestError {
            email: None,
            first_name: None,
            last_name: None,
            password: None,
            password_confirm: None,
        };

        let email_result = ValueObject::new(Email(value.email));
        let first_name_result = ValueObject::new(FirstName(value.first_name));
        let last_name_result = ValueObject::new(LastName(value.last_name));
        let password_result = ValueObject::new(Password(value.password));

        if let Err(e) = &email_result {
            errors.email = Some(e.to_string());
        }
        if let Err(e) = &first_name_result {
            errors.first_name = Some(e.to_string());
        }
        if let Err(e) = &last_name_result {
            errors.last_name = Some(e.to_string());
        }
        if let Err(e) = &password_result {
            errors.password = Some(e.to_string());
        }
        if let Ok(password) = &password_result {
            if password.extract().get_value().clone() != value.password_confirm.clone() {
                errors.password_confirm =
                    Some("A jelszó és a jelszó megerősítés mező nem egyezik".to_string());
            }
        }

        if errors.is_empty() {
            Ok(RegisterRequest {
                email: email_result.unwrap(),
                first_name: first_name_result.unwrap(),
                last_name: last_name_result.unwrap(),
                password: password_result.unwrap(),
                password_confirm: value.password_confirm,
            })
        } else {
            Err(errors)
        }
    }
}
