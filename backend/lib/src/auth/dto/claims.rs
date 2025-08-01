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

use serde::{Deserialize, Serialize};

/// Represents the structure of the claims contained in a JSON Web Token (JWT).
/// These claims provide metadata and security information for the token.
///
/// # Fields:
///
/// * `sub` - The subject of the token, which represents the user's unique identifier
///   (e.g., a UUID as a string).
///
/// * `exp` - The expiration timestamp of the token in UNIX time. Indicates the time
///   after which the token is no longer valid.
///
/// * `iat` - The issued-at timestamp of the token in UNIX time. Specifies when the token
///   was created.
///
/// * `nbf` - The "not valid before" timestamp in UNIX time. Indicates the time before
///   which the token is not considered valid.
///
/// * `iss` - The issuer of the token, typically representing the domain or service name
///   that issued the token
///
/// * `aud` - The audience for the token, identifying the intended recipient(s) of the
///   token (e.g., frontend client ID or domain).
///
/// * `jti` - A unique identifier for the token, typically a UUID, which ensures that
///   each token is unique. This can help prevent token reuse or replay attacks.
///
/// # Usage:
///
/// This struct is typically used to validate, decode, or create JWTs in the application.
/// By accessing its fields, you can ensure the token's integrity, validate its audience,
/// and enforce time-based constraints.
#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    sub: String, // User's unique ID (e.g., UUID as a string)
    exp: usize,  // Expiration time (as a UNIX timestamp)
    iat: usize,  // Issued at (as a UNIX timestamp)
    nbf: usize,  // Not valid before (as a UNIX timestamp)
    iss: String, // Issuer (e.g., your service domain)
    aud: String, // Audience (e.g., your frontend client ID or domain)
    jti: String, // JWT ID (unique per token, e.g., UUID)
}

impl Claims {
    /// Creates a new instance of Claims.
    ///
    /// # Parameters
    ///
    /// - `sub` (`String`): The subject of the token, which represents the user's unique identifier (e.g., a UUID as a string).
    /// - `exp` (`usize`): The expiration timestamp of the token in UNIX time. Indicates the time after which the token is no longer valid.
    /// - `iat` (`usize`): The issued-at timestamp of the token in UNIX time. Specifies when the token was created.
    /// - `nbf` (`usize`): The "not valid before" timestamp in UNIX time. Indicates the time before which the token is not considered valid.
    /// - `iss` (`String`): The issuer of the token, typically representing the domain or service name that issued the token
    /// - `aud` (`String`): The audience for the token, identifying the intended recipient(s) of the token (e.g., frontend client ID or domain).
    /// - `jti` (`String`): A unique identifier for the token, typically a UUID, which ensures that each token is unique. This can help prevent token reuse or replay attacks.
    ///
    /// # Returns
    ///
    /// Returns a new instance populated with the provided values.
    pub fn new(
        sub: String,
        exp: usize,
        iat: usize,
        nbf: usize,
        iss: String,
        aud: String,
        jti: String,
    ) -> Self {
        Self {
            sub,
            exp,
            iat,
            nbf,
            iss,
            aud,
            jti,
        }
    }
    /// Returns a reference to the `sub` field of the object.
    /// `sub` The subject of the token, which represents the user's unique identifier (e.g., a UUID as a string).
    pub fn sub(&self) -> &String {
        &self.sub
    }
    /// Returns a reference to the `exp` field of the object.
    /// `exp` The expiration timestamp of the token in UNIX time. Indicates the time after which the token is no longer valid.
    #[allow(dead_code)]
    pub fn exp(&self) -> usize {
        self.exp
    }
    /// Returns a reference to the `iat` field of the object.
    /// `iat` The issued-at timestamp of the token in UNIX time. Specifies when the token was created.
    #[allow(dead_code)]
    pub fn iat(&self) -> usize {
        self.iat
    }
    /// Returns a reference to the `nbf` field of the object.
    /// `nbf` The "not valid before" timestamp in UNIX time. Indicates the time before which the token is not considered valid.
    #[allow(dead_code)]
    pub fn nbf(&self) -> usize {
        self.nbf
    }
    /// Returns a reference to the `iss` field of the object.
    /// `iss` The issuer of the token, typically representing the domain or service name that issued the token
    #[allow(dead_code)]
    pub fn iss(&self) -> &String {
        &self.iss
    }
    /// Returns a reference to the `aud` field of the object.
    /// `aud` The audience for the token, identifying the intended recipient(s) of the token (e.g., frontend client ID or domain).
    #[allow(dead_code)]
    pub fn aud(&self) -> &String {
        &self.aud
    }
    /// Returns a reference to the `jti` field of the object.
    /// `jti`A unique identifier for the token, typically a UUID, which ensures that each token is unique. This can help prevent token reuse or replay attacks.
    #[allow(dead_code)]
    pub fn jti(&self) -> &String {
        &self.jti
    }
}
