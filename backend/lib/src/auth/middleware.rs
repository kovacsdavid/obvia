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
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use std::sync::Arc;

use crate::app::app_state::AppState;

use super::dto::claims::Claims;

/// Middleware function to handle authentication using JWT (JSON Web Tokens).
///
/// # Arguments
///
/// * `State(state)` - Extracts application state wrapped in an `Arc<AppState>` from the request context.
///   This contains configuration and shared resources for the application.
/// * `TypedHeader(Authorization(bearer))` - Extracts the Bearer token from the `Authorization` request header.
/// * `req` - The incoming request object that can be inspected or modified during processing.
/// * `next` - Represents the next middleware or handler in the processing pipeline.
///
/// # Returns
///
/// This function returns:
/// - `Ok(Response)` if authentication succeeds and the request proceeds to the next handler or middleware.
/// - `Err(StatusCode::UNAUTHORIZED)` if authentication fails due to an invalid token or decoding errors.
///
/// # Behavior
///
/// 1. Creates a JWT `Validation` object configured with the algorithm, expected issuer, and audience extracted
///    from the application's configuration.
/// 2. Sets required claims to ensure the token includes mandatory information like `sub`, `exp`, `iat`,
///    `nbf`, `iss`, `aud`, and `jti`.
/// 3. Decodes and validates the provided Bearer token using the decoding key extracted from the configuration.
/// 4. If validation succeeds, inserts the claims from the decoded token into the request extensions for downstream usage.
/// 5. If validation fails, responds with a 401 Unauthorized status.
///
/// This middleware ensures that only requests with valid JWT tokens are allowed to continue through the processing pipeline.
///
/// # Errors
///
/// This middleware will terminate processing and return an HTTP 401 Unauthorized status if:
/// - The token is missing, expired, or malformed.
/// - The token fails validation against the configured requirements or secret key.
pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut validator = Validation::new(Algorithm::HS256);
    validator.set_issuer(&[state.config_module.auth().jwt_issuer()]);
    validator.set_audience(&[state.config_module.auth().jwt_audience()]);
    validator.set_required_spec_claims(&["sub", "exp", "iat", "nbf", "iss", "aud", "jti"]);

    let token_data = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret(state.config_module.auth().jwt_secret().as_bytes()),
        &validator,
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(token_data.claims);

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
