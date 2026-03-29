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
use tracing::metadata::LevelFilter;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    bind_address: String,
    bind_port: u16,
    public_base_url: String,
    environment: String,
    #[serde(deserialize_with = "deserialize_log_level")]
    log_level: LevelFilter,
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
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub struct ServerConfigBuilder {
        host: Option<String>,
        port: Option<u16>,
        hostname: Option<String>,
        environment: Option<String>,
        log_level: Option<LevelFilter>,
    }

    impl ServerConfigBuilder {
        pub fn new() -> Self {
            Self {
                host: None,
                port: None,
                hostname: None,
                environment: None,
                log_level: None,
            }
        }
        pub fn host(mut self, host: &str) -> Self {
            self.host = Some(host.to_owned());
            self
        }
        pub fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }
        pub fn hostname(mut self, hostname: &str) -> Self {
            self.hostname = Some(hostname.to_owned());
            self
        }
        pub fn environment(mut self, environment: &str) -> Self {
            self.environment = Some(environment.to_owned());
            self
        }
        pub fn log_level(mut self, log_level: LevelFilter) -> Self {
            self.log_level = Some(log_level);
            self
        }
        pub fn build(self) -> Result<ServerConfig, String> {
            Ok(ServerConfig {
                bind_address: self.host.ok_or("host is required".to_string())?,
                bind_port: self.port.ok_or("port is required".to_string())?,
                public_base_url: self.hostname.ok_or("hostname is required".to_string())?,
                environment: self
                    .environment
                    .ok_or("environment is required".to_string())?,
                log_level: self.log_level.unwrap_or(LevelFilter::TRACE),
            })
        }
    }

    impl Default for ServerConfigBuilder {
        fn default() -> Self {
            ServerConfigBuilder::new()
                .host("127.0.0.1")
                .port(3000)
                .hostname("example.com")
                .environment("test")
                .log_level(LevelFilter::TRACE)
        }
    }
}
