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

pub(crate) mod auth_config;
pub(crate) mod database_config;
pub(crate) mod mail_config;
pub(crate) mod server_config;

pub(crate) use auth_config::AuthConfig;
pub(crate) use database_config::BasicDatabaseConfig;
pub(crate) use mail_config::MailConfig;
pub(crate) use server_config::ServerConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    main_database: BasicDatabaseConfig,
    default_tenant_database: BasicDatabaseConfig,
    auth: AuthConfig,
    mail: MailConfig,
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

    pub fn main_database(&self) -> &BasicDatabaseConfig {
        &self.main_database
    }

    pub fn default_tenant_database(&self) -> &BasicDatabaseConfig {
        &self.default_tenant_database
    }

    pub fn auth(&self) -> &AuthConfig {
        &self.auth
    }
    pub fn mail(&self) -> &MailConfig {
        &self.mail
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::common::config::{
        auth_config::tests::AuthConfigBuilder, database_config::tests::DatabaseConfigBuilder,
        mail_config::tests::MailConfigBuilder, server_config::tests::ServerConfigBuilder,
    };

    use super::*;

    pub struct AppConfigBuilder {
        server: Option<ServerConfig>,
        main_database: Option<BasicDatabaseConfig>,
        default_tenant_database: Option<BasicDatabaseConfig>,
        auth: Option<AuthConfig>,
        mail: Option<MailConfig>,
    }

    impl AppConfigBuilder {
        pub fn new() -> Self {
            Self {
                server: None,
                main_database: None,
                default_tenant_database: None,
                auth: None,
                mail: None,
            }
        }
        pub fn server(mut self, server: ServerConfig) -> Self {
            self.server = Some(server);
            self
        }
        pub fn main_database(mut self, main_database: BasicDatabaseConfig) -> Self {
            self.main_database = Some(main_database);
            self
        }
        pub fn default_tenant_database(
            mut self,
            default_tenant_database: BasicDatabaseConfig,
        ) -> Self {
            self.default_tenant_database = Some(default_tenant_database);
            self
        }
        pub fn auth(mut self, auth: AuthConfig) -> Self {
            self.auth = Some(auth);
            self
        }

        pub fn mail(mut self, mail: MailConfig) -> Self {
            self.mail = Some(mail);
            self
        }
        pub fn build(self) -> Result<AppConfig, String> {
            Ok(AppConfig {
                server: self.server.ok_or("server is required")?,
                main_database: self.main_database.ok_or("main_database is required")?,
                default_tenant_database: self
                    .default_tenant_database
                    .ok_or("default_tenant_database")?,
                auth: self.auth.ok_or("auth is required")?,
                mail: self.mail.ok_or("mail is required")?,
            })
        }
    }

    impl Default for AppConfigBuilder {
        fn default() -> Self {
            AppConfigBuilder::new()
                .server(ServerConfigBuilder::default().build().unwrap())
                .main_database(DatabaseConfigBuilder::default().build().unwrap())
                .default_tenant_database(DatabaseConfigBuilder::default().build().unwrap())
                .auth(AuthConfigBuilder::default().build().unwrap())
                .mail(MailConfigBuilder::default().build().unwrap())
        }
    }
}
