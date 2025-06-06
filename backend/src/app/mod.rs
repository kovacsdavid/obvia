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
use std::sync::Arc;

use crate::{auth::AuthModule, users::UsersModule};

pub struct AppState {
    pub auth_module: Arc<AuthModule>,
    pub config_module: Arc<AppConfig>,
    pub users_module: Arc<UsersModule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    url: String,
    pool_size: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    jwt_secret: String,
    jwt_issuer: String,
    jwt_audience: String,
    jwt_expiration_mins: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            url: "postgres://user:password@localhost/db".to_string(),
            pool_size: 5,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        AuthConfig {
            jwt_secret: "test_jwt_secret".to_string(),
            jwt_issuer: "http://localhost".to_string(),
            jwt_audience: "http://localhost".to_string(),
            jwt_expiration_mins: 60,
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let builder = config::Config::builder()
            .add_source(config::File::with_name("config/default").required(true));
        builder.build()?.try_deserialize()
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn auth(&self) -> &AuthConfig {
        &self.auth
    }
}

impl ServerConfig {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl DatabaseConfig {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn pool_size(&self) -> u32 {
        self.pool_size
    }
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
    pub fn jwt_expiration_mins(&self) -> u64 {
        self.jwt_expiration_mins
    }
}
