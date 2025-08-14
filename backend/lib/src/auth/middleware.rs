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

use axum::{
    extract::{FromRequestParts, Request, State},
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use std::sync::Arc;

use crate::app::app_state::AppState;

use super::dto::claims::Claims;

/// Middleware function to enforce authentication for incoming HTTP requests.
///
/// This function checks for the presence and validity of a JWT (JSON Web Token) in the incoming
/// request headers. If the token is missing, invalid, or fails verification, the request will
/// be rejected with a `401 Unauthorized` status code.
///
/// # Arguments
///
/// * `State(state)` - The shared application state, wrapped in an `Arc`. This provides access
///   to the application's configuration, including authentication settings.
/// * `TypedHeader(Authorization(bearer))` - Extracts the `Authorization` header from the request
///   and parses it as a `Bearer` token.
/// * `mut req` - The incoming HTTP request that will be passed to the next middleware or handler
///   after authentication is verified.
/// * `next` - Represents the next middleware or handler in the processing chain.
///
/// # Returns
///
/// Returns an `Ok(Response)` if authentication succeeds and the request is passed to the next
/// stage of processing. If authentication fails, it returns an `Err(StatusCode::UNAUTHORIZED)`.
///
/// # Authentication Process
///
/// 1. Extracts the bearer token from the `Authorization` header.
/// 2. Decodes and verifies the token using the application's JWT configuration:
///     - `jwt_secret`: The secret key used to validate the token's signature.
///     - `jwt_issuer`: The expected issuer of the token.
///     - `jwt_audience`: The expected audience of the token.
/// 3. If the token is valid, its claims are inserted into the request's extensions for use
///    by subsequent middleware or handlers.
/// 4. If the token is invalid or verification fails, a `401 Unauthorized` status code is returned.
///
/// # Errors
///
/// Returns `StatusCode::UNAUTHORIZED` if:
/// - The `Authorization` header is missing or malformed.
/// - The token is invalid or fails verification (e.g., incorrect signature, expired, invalid issuer or audience).
pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    req.extensions_mut().insert(
        Claims::from_token(
            bearer.token(),
            state.config_module.auth().jwt_secret().as_bytes(),
            state.config_module.auth().jwt_issuer(),
            state.config_module.auth().jwt_audience(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?,
    );
    Ok(next.run(req).await)
}

/// Represents an authenticated user in the system.
///
/// This struct is a tuple struct that wraps around the `Claims` object, which
/// contains information extracted from a validated authentication token
/// (e.g., JWT claims). The `AuthenticatedUser` struct is commonly used to identify
/// and manage users who have successfully authenticated within the application.
///
/// # Fields
/// - `0`: A `Claims` object containing user-specific data
///
/// # Usage
/// The `AuthenticatedUser` struct can be leveraged in request handling or security
/// middleware to verify user identity and enforce access control.
///
/// # Note
/// Ensure that `Claims` is structured appropriately to include all necessary information
/// for your authentication flow.
pub struct AuthenticatedUser(pub Claims);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    /// Extracts an `AuthenticatedUser` from the request parts.
    ///
    /// This function is typically used in the context of Axum or similar web frameworks to
    /// extract user authentication claims from the request's extensions. If valid claims
    /// are available in the request parts, it wraps the claims in an `AuthenticatedUser`
    /// struct and returns it as the result. Otherwise, it rejects the request with
    /// a `401 Unauthorized` status code.
    ///
    /// # Parameters
    /// - `parts`: A mutable reference to the request parts, which contains the extensions
    ///   where the authentication claims might be stored.
    /// - `_state`: A reference to the application state. In this implementation, it's not
    ///   used but required by the trait signature.
    ///
    /// # Returns
    /// - `Ok(AuthenticatedUser)` if the `Claims` are successfully retrieved and cloned
    ///   from the request parts.
    /// - `Err(Self::Rejection)` if the `Claims` are missing, resulting in a response
    ///   with a `401 Unauthorized` status code and an error message.
    ///
    /// # Rejection
    /// This function returns a `Response` containing:
    /// - `401 Unauthorized` status code.
    /// - A message indicating the absence of authentication claims: `"Missing authentication claims"`.
    ///
    /// # Note
    /// Ensure that the `Claims` type is properly set in the extensions earlier in the
    /// request lifecycle, such as in a middleware, for this extractor to work correctly.
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(AuthenticatedUser)
            .ok_or_else(|| {
                (StatusCode::UNAUTHORIZED, "Missing authentication claims").into_response()
            })
    }
}
