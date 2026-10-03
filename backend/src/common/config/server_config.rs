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
use tracing::metadata::LevelFilter;

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct ServerConfig {
    bind_address: String,
    bind_port: u16,
    public_base_url: String,
    environment: String,
    #[serde(deserialize_with = "deserialize_log_level")]
    log_level: LevelFilter,
    log_directory: String,
}

fn deserialize_log_level<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: serde::Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<LevelFilter>()
        .map_err(serde::de::Error::custom)
}

impl ServerConfig {
    pub fn bind_address(&self) -> &str {
        &self.bind_address
    }
    pub fn bind_port(&self) -> u16 {
        self.bind_port
    }
    pub fn public_base_url(&self) -> &str {
        &self.public_base_url
    }
    pub fn environment(&self) -> &str {
        &self.environment
    }
    pub fn log_level(&self) -> LevelFilter {
        self.log_level
    }
    pub fn log_directory(&self) -> &str {
        &self.log_directory
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_server_config_builder() -> ServerConfigBuilder {
        let mut builder = ServerConfigBuilder::default();
        builder
            .bind_address("127.0.0.1".to_string())
            .bind_port(3000)
            .public_base_url("example.com".to_string())
            .environment("test".to_string())
            .log_level(LevelFilter::TRACE)
            .log_directory("/var/log/obvia".to_string());
        builder
    }
}
