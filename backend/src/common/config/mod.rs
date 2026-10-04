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

pub(crate) mod auth_config;
pub(crate) mod database_config;
pub(crate) mod mail_config;
pub(crate) mod server_config;

pub(crate) use auth_config::AuthConfig;
pub(crate) use database_config::BasicDatabaseConfig;
pub(crate) use mail_config::MailConfig;
pub(crate) use server_config::ServerConfig;

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct AppConfig {
    server: ServerConfig,
    main_database: BasicDatabaseConfig,
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

    pub fn auth(&self) -> &AuthConfig {
        &self.auth
    }
    pub fn mail(&self) -> &MailConfig {
        &self.mail
    }
}

#[cfg(test)]
pub mod tests {
    use crate::common::config::{
        auth_config::tests::test_auth_config_builder,
        database_config::tests::test_basic_database_config_builder,
        mail_config::tests::test_mail_config_builder,
        server_config::tests::test_server_config_builder,
    };

    use super::*;

    pub fn test_app_config_builder() -> AppConfigBuilder {
        let mut builder = AppConfigBuilder::default();
        builder
            .server(test_server_config_builder().build().unwrap())
            .main_database(test_basic_database_config_builder().build().unwrap())
            .auth(test_auth_config_builder().build().unwrap())
            .mail(test_mail_config_builder().build().unwrap());
        builder
    }
}
