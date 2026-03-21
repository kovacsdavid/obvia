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
pub struct ServerConfig {
    host: String,
    port: u16,
    hostname: String,
    environment: String,
}

impl ServerConfig {
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
    pub fn environment(&self) -> &str {
        &self.environment
    }
}

#[allow(dead_code)]
pub struct ServerConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    hostname: Option<String>,
    environment: Option<String>,
}

#[allow(dead_code)]
impl ServerConfigBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
            hostname: None,
            environment: None,
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
    pub fn build(self) -> Result<ServerConfig, String> {
        Ok(ServerConfig {
            host: self.host.ok_or("host is required".to_string())?,
            port: self.port.ok_or("port is required".to_string())?,
            hostname: self.hostname.ok_or("hostname is required".to_string())?,
            environment: self
                .environment
                .ok_or("environment is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for ServerConfigBuilder {
    fn default() -> Self {
        ServerConfigBuilder::new()
    }
}
