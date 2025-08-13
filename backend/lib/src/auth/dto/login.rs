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

use crate::users::model::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a login request payload.
///
/// This struct is used to deserialize incoming JSON data for user authentication purposes.
/// It contains the email and password fields required for login.
///
/// # Fields
/// - `email` (String): The email address of the user trying to log in.
/// - `password` (String): The plain text password of the user trying to log in.
///
/// # Example JSON
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "securepassword123"
/// }
/// ```
///
/// # Usage
/// This struct is part of the web application's authentication flow
/// where the client sends their login credentials, and the server deserializes
/// the request into this struct for processing.
///
/// # Security
/// - Make sure to handle the `password` field securely and avoid logging, storing or exposing it
///   in any other ways
#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Represents a public view of a user with limited information.
///
/// This struct is typically used to expose user data in contexts where detailed
/// or sensitive user information should not be disclosed.
///
/// # Fields
///
/// * `id` - A unique identifier for the user, represented as a UUID.
/// * `email` - The user's email address.
/// * `first_name` - The user's first name, which is optional. This field may be `None`
///   if no first name is provided.
/// * `last_name` - The user's last name, which is optional. This field may be `None`
///   if no last name is provided.
/// * `status` - A string representing the current status of the user. The possible
///   values for this field depend on the application's specific requirements.
/// * `profile_picture_url` - An optional URL pointing to the user's profile picture.
///   This field may be `None` if no profile picture is set.
#[derive(Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: String,
    pub profile_picture_url: Option<String>,
}

impl From<User> for UserPublic {
    /// Converts a `User` instance into a corresponding instance of the current struct.
    ///
    /// # Parameters
    /// - `value`: An instance of the `User` struct to be converted.
    ///
    /// # Returns
    /// A new instance of the current struct containing fields mapped from the provided
    /// `User` instance.
    ///
    /// # Field Mapping
    /// - `id`: Copied from `value.id`.
    /// - `email`: Copied from `value.email`.
    /// - `first_name`: Copied from `value.first_name`.
    /// - `last_name`: Copied from `value.last_name`.
    /// - `status`: Copied from `value.status`.
    /// - `profile_picture_url`: Copied from `value.profile_picture_url`.
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            status: value.status,
            profile_picture_url: value.profile_picture_url,
        }
    }
}

/// The `LoginResponse` struct represents the response sent back to the client
/// after a successful login attempt. It contains user information and an
/// authentication token.
///
/// # Fields
///
/// * `user` - A `UserPublic` struct that holds the public-facing information
///   about the authenticated user.
/// * `token` - A string representing the authentication JWT token issued to the
///   user for subsequent requests.
#[derive(Serialize)]
pub struct LoginResponse {
    user: UserPublic,
    token: String,
}

impl LoginResponse {
    /// Creates a new instance of the struct with the given user and token.
    ///
    /// # Parameters
    /// - `user`: A `UserPublic` instance representing the user's public information.
    /// - `token`: A `String` containing the token associated with the user.
    ///
    /// # Returns
    /// - Returns a new instance of the struct containing the provided `user` and `token`.
    pub fn new(user: UserPublic, token: String) -> Self {
        Self { user, token }
    }
    /// Returns a reference to the `token` field of the struct.
    ///
    /// # Purpose
    /// This method provides read-only access to the `token` field within the struct.
    /// It is marked with `#[allow(dead_code)]` to suppress warnings in case the function is unused.
    ///
    /// # Returns
    /// A reference to the `String` stored in the `token` field.
    #[allow(dead_code)]
    pub fn token(&self) -> &String {
        &self.token
    }
}
