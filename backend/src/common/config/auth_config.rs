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

use derive_builder::Builder;
use serde::Deserialize;
use std::fmt::Debug;

use crate::common::types::Password;

#[derive(Clone, Deserialize, Builder)]
pub struct AuthConfig {
    jwt_secret: String,
    jwt_issuer: String,
    jwt_audience: String,
    access_token_expiration_mins: u64,
    refresh_token_expiration_mins: u64,
}

impl AuthConfig {
    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
    pub fn jwt_issuer(&self) -> &str {
        &self.jwt_issuer
    }
    pub fn jwt_audience(&self) -> &str {
        &self.jwt_audience
    }
    pub fn access_token_expiration_mins(&self) -> u64 {
        self.access_token_expiration_mins
    }
    pub fn refresh_token_expiration_mins(&self) -> u64 {
        self.refresh_token_expiration_mins
    }
}

impl Debug for AuthConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthConfig")
            .field("jwt_secret", &Password::HIDDEN_PASSWORD)
            .field("jwt_issuer", &self.jwt_issuer)
            .field("jwt_audience", &self.jwt_audience)
            .field(
                "access_token_expiration_mins",
                &self.access_token_expiration_mins,
            )
            .field(
                "refresh_token_expiration_mins",
                &self.refresh_token_expiration_mins,
            )
            .finish()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_auth_config_builder() -> AuthConfigBuilder {
        let mut builder = AuthConfigBuilder::default();
        builder
            .jwt_secret("test_jwt_secret".to_string())
            .jwt_issuer("obvia".to_string())
            .jwt_audience("obvia".to_string())
            .access_token_expiration_mins(5)
            .refresh_token_expiration_mins(60);

        builder
    }
}
