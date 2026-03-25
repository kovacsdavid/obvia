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

#[derive(Debug, Clone, Deserialize)]
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub struct AuthConfigBuilder {
        jwt_secret: Option<String>,
        jwt_issuer: Option<String>,
        jwt_audience: Option<String>,
        access_token_expiration_mins: Option<u64>,
        refresh_token_expiration_mins: Option<u64>,
    }

    impl AuthConfigBuilder {
        pub fn new() -> Self {
            AuthConfigBuilder {
                jwt_secret: None,
                jwt_issuer: None,
                jwt_audience: None,
                access_token_expiration_mins: None,
                refresh_token_expiration_mins: None,
            }
        }
        pub fn jwt_secret(mut self, jwt_secret: &str) -> Self {
            self.jwt_secret = Some(jwt_secret.to_owned());
            self
        }
        pub fn jwt_issuer(mut self, jwt_issuer: &str) -> Self {
            self.jwt_issuer = Some(jwt_issuer.to_owned());
            self
        }
        pub fn jwt_audience(mut self, jwt_audience: &str) -> Self {
            self.jwt_audience = Some(jwt_audience.to_owned());
            self
        }
        pub fn access_token_expiration_mins(mut self, access_token_expiration_mins: u64) -> Self {
            self.access_token_expiration_mins = Some(access_token_expiration_mins);
            self
        }
        pub fn refresh_token_expiration_mins(mut self, refresh_token_expiration_mins: u64) -> Self {
            self.refresh_token_expiration_mins = Some(refresh_token_expiration_mins);
            self
        }
        pub fn build(self) -> Result<AuthConfig, String> {
            Ok(AuthConfig {
                jwt_secret: self.jwt_secret.ok_or("jwt_secret is required")?,
                jwt_issuer: self.jwt_issuer.ok_or("jwt_issuer is required")?,
                jwt_audience: self.jwt_audience.ok_or("jwt_audience is required")?,
                access_token_expiration_mins: self
                    .access_token_expiration_mins
                    .ok_or("access_token_expiration_mins is required")?,
                refresh_token_expiration_mins: self
                    .refresh_token_expiration_mins
                    .ok_or("refresh_token_expiration_mins is required")?,
            })
        }
    }

    impl Default for AuthConfigBuilder {
        fn default() -> Self {
            AuthConfigBuilder::new()
                .jwt_secret("test_jwt_secret")
                .jwt_issuer("http://localhost")
                .jwt_audience("http://localhost")
                .access_token_expiration_mins(5)
                .refresh_token_expiration_mins(60)
        }
    }
}
