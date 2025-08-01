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

use super::{
    AuthModule,
    service::{try_login, try_register},
};
use crate::auth::dto::register::RegisterRequestHelper;
use crate::auth::repository::AuthRepository;
use crate::common::repository::PoolWrapper;
use crate::{
    auth::dto::{login::LoginRequest, register::RegisterRequest},
    common::error::FriendlyError,
};
use axum::{
    Json, debug_handler,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

/// Handles the inner login logic for an authentication flow.
///
/// This function is responsible for managing the login process, using the provided `auth_module`,
/// `payload`, and a repository factory to interact with the underlying authentication repository.
/// It attempts to authenticate the user based on the given login payload and returns an appropriate
/// HTTP response.
///
/// # Type Parameters
/// - `F`: A factory function that produces a future resolving to a boxed `AuthRepository` implementation.
/// - `Fut`: The future type returned by the factory function, resolving to `Box<dyn AuthRepository>`.
///
/// # Parameters
/// - `auth_module`: A shared reference-counted pointer (`Arc`) to the `AuthModule`, responsible for authentication logic.
/// - `payload`: The `LoginRequest` payload containing the login credentials and related information.
/// - `repo_factory`: A factory function producing a future that resolves to an instance of `AuthRepository`.
///    This allows the function to dynamically access the repository when needed and makes testing easier.
///
/// # Returns
/// - A `Response` representing the result of the login process. On success, it returns a `200 OK` response
///   containing the authentication response as JSON. On failure, it returns the relevant error response.
///
/// # Errors
/// - If the login process fails (e.g., invalid credentials, user not found, repository issues),
///   an appropriate error response is returned.
pub async fn login_inner<F, Fut>(
    auth_module: Arc<AuthModule>,
    payload: LoginRequest,
    repo_factory: F,
) -> Response
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = Box<dyn AuthRepository + Send + Sync>>,
{
    let repo = repo_factory().await;
    match try_login(&*repo, auth_module.clone(), payload).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(e) => e.into_response(),
    }
}

/// Handles the login process for a user in an asynchronous manner.
///
/// # Parameters
/// - `State(auth_module)`: An instance of `AuthModule` wrapped in an `Arc`, providing access
///   to authentication-related functionality such as pool management and configurations.
///   It's extracted from the application state.
/// - `Json(payload)`: A `LoginRequest` object extracted from the JSON body of the request.
///   This contains the credentials or necessary data for authentication.
///
/// # Returns
/// A `Response` object, wrapped in an `async` function, which represents the outcome of the
/// login operation. This response may contain a success token, error message, or other
/// authentication-related information depending on the login process's result.
///
/// # Functionality
/// - Delegates the core login logic to the `login_inner` function to handle authentication.
/// - The repository for authentication (`AuthRepository`) is created using a `PoolWrapper`
///   that is initialized with the main database connection pool via the `pool_manager`.
/// - The `login_inner` function handles the login operation utilizing the provided
///   repository and `payload`.
#[debug_handler]
pub async fn login(
    State(auth_module): State<Arc<AuthModule>>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    login_inner(auth_module.clone(), payload, || async {
        Box::new(PoolWrapper::new(auth_module.pool_manager.get_main_pool()))
            as Box<dyn AuthRepository + Send + Sync>
    })
    .await
}

/// Asynchronous function to handle user registration logic.
///
/// This function is an inner implementation that processes incoming registration requests,
/// validates user input, and interacts with the authentication repository to complete
/// the registration process.
///
/// # Parameters
///
/// - `auth_module`: An `Arc` reference to the `AuthModule` that provides authentication-related utilities,
///   such as the password hasher.
/// - `payload`: A `Result` containing either a deserialized `Json` request of type `RegisterRequestHelper`
///   or a `JsonRejection` indicating invalid input structure.
/// - `repo_factory`: A closure that asynchronously returns an implementation of the `AuthRepository`
///   trait. This is used to interact with the underlying user data store.
///
/// # Return
///
/// Returns a `Response` that represents the HTTP response for the registration operation:
/// - On success, responds with `201 Created` and the registered user information in JSON format.
/// - On failure, responds with detailed error messages, such as:
///   - `400 Bad Request` for invalid input structure.
///   - `422 Unprocessable Entry` Relevant error response from the internal `try_register` function
///      for other failure cases.
///
/// # Error Handling
///
/// - If the input `payload` cannot be parsed into a `RegisterRequest`, the function returns a
///   `400 Bad Request` error with a user-friendly error message.
/// - `422 Unprocessable Entry` If the `try_register` function encounters an error during the
///    registration process, the specific error details are returned in the response.
///
/// # Notes
///
/// - This function abstracts the core registration logic and can be reused or extended
///   with additional middleware for logging, security validation, etc.
pub async fn register_inner<F, Fut>(
    auth_module: Arc<AuthModule>,
    payload: Result<Json<RegisterRequestHelper>, JsonRejection>,
    repo_factory: F,
) -> Response
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = Box<dyn AuthRepository + Send + Sync>>,
{
    match payload {
        Ok(Json(payload)) => match RegisterRequest::try_from(payload) {
            Ok(user_input) => {
                let repo = repo_factory().await;
                match try_register(&*repo, auth_module.password_hasher.clone(), user_input).await {
                    Ok(resp) => (StatusCode::CREATED, Json(resp)).into_response(),
                    Err(e) => e.into_response(),
                }
            }
            Err(e) => e.into_response(),
        },
        Err(_) => FriendlyError::UserFacing(
            StatusCode::BAD_REQUEST,
            "AUTH/HANDLER/REGISTER".to_string(),
            "Hibás adatszerkezet".to_string(),
        )
        .trace(tracing::Level::DEBUG)
        .into_response(),
    }
}

/// Handles user registration requests.
///
/// This function is an HTTP handler that processes a user registration request by:
/// 1. Validating the incoming payload.
/// 2. Passing the data to the appropriate module for handling the registration logic.
///
/// # Parameters
/// - `State(auth_module)`: Provides a shared reference to the `AuthModule`,
///   which contains the necessary components for handling authentication and user
///   management. The `AuthModule` is wrapped in an `Arc` for thread-safe shared access.
/// - `payload`: The incoming user registration request payload. This is wrapped in a
///   `Result` to handle potential payload rejections due to deserialization errors.
///
/// # Returns
/// An asynchronous HTTP `Response` containing the result of the registration process. The response
/// includes the appropriate status code and/or error messages.
///
/// # Implementation Details
/// - This handler calls an internal function `register_inner`, which manages the logic for
///   processing the registration request.
/// - For repository interaction, the function dynamically constructs an instance of `PoolWrapper`
///   using the pool managed by the `AuthModule`. `PoolWrapper` implements the `AuthRepository` trait
///   to abstract database access.
///
/// # Errors
/// - Returns appropriate error responses if:
///   - The payload is invalid or rejected (e.g., malformed JSON).
///   - There is any issue during the registration process (e.g., database connectivity issues).
#[debug_handler]
pub async fn register(
    State(auth_module): State<Arc<AuthModule>>,
    payload: Result<Json<RegisterRequestHelper>, JsonRejection>,
) -> Response {
    register_inner(auth_module.clone(), payload, || async {
        Box::new(PoolWrapper::new(auth_module.pool_manager.get_main_pool()))
            as Box<dyn AuthRepository + Send + Sync>
    })
    .await
}
