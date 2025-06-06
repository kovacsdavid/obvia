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

use std::sync::Arc;
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use axum::{extract::{FromRequestParts, Request, State}, http::{request::Parts, StatusCode}, middleware::Next, response::{IntoResponse, Response}};

use crate::app::AppState;

use super::{dto::claims::Claims};


// ===== VERIFY =====
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

pub struct AuthenticatedUser(pub Claims);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(AuthenticatedUser)
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing authentication claims").into_response())
    }
}
