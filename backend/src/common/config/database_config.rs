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

use crate::common::types::{DbHost, DbName, DbPassword, DbPort, DbUser, Password};
use crate::common::value_object::{ValueObjectError, ValueObjectRequired};
use crate::manager::tenants::model::Tenant;
use derive_builder::Builder;
use serde::Deserialize;
use sqlx::postgres::PgSslMode;
use std::fmt::{Debug, Display};
use std::str::FromStr;

pub type BasicDatabaseConfig = DatabaseConfig<String, u16, String, String, String, u32>;

pub type TenantDatabaseConfig = DatabaseConfig<
    ValueObjectRequired<DbHost>,
    ValueObjectRequired<DbPort>,
    ValueObjectRequired<DbUser>,
    ValueObjectRequired<DbPassword>,
    ValueObjectRequired<DbName>,
    u32,
>;

pub trait DatabaseUrlProvider {
    fn url(&self) -> String;
}

pub trait DatabasePoolSizeProvider {
    type MaxPoolSizeType;

    fn max_pool_size(&self) -> Self::MaxPoolSizeType;
}

#[derive(Clone, Deserialize, Builder)]
pub struct DatabaseConfig<H, P, U, Pw, D, M>
where
    H: Debug,
    P: Debug,
    U: Debug,
    Pw: Debug,
    D: Debug,
    M: Debug,
{
    pub host: H,
    pub port: P,
    pub username: U,
    pub password: Pw,
    pub database: D,
    pub max_pool_size: Option<M>,
    pub ssl_mode: Option<String>,
}

impl<H, P, U, Pw, D, M> Debug for DatabaseConfig<H, P, U, Pw, D, M>
where
    H: Debug,
    P: Debug,
    U: Debug,
    Pw: Debug,
    D: Debug,
    M: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DatabaseConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("username", &self.username)
            .field("password", &Password::HIDDEN_PASSWORD)
            .field("database", &self.database)
            .field("max_pool_size", &self.max_pool_size)
            .field("ssl_mode", &self.ssl_mode)
            .finish()
    }
}

impl From<TenantDatabaseConfig> for BasicDatabaseConfig {
    fn from(value: TenantDatabaseConfig) -> Self {
        Self {
            host: value.host.as_str().unwrap().to_owned(), // TODO: not critical, but remove this unwrap
            port: value.port.as_u16().unwrap(), // TODO: not critical, but remove this unwrap
            username: value.username.to_string(),
            password: value.password.to_string(),
            database: value.database.to_string(),
            max_pool_size: value.max_pool_size,
            ssl_mode: value.ssl_mode,
        }
    }
}

impl TryFrom<&Tenant> for TenantDatabaseConfig {
    type Error = String;
    fn try_from(value: &Tenant) -> Result<Self, Self::Error> {
        PgSslMode::from_str(&value.db_ssl_mode).map_err(|_| "invalid ssl_mode")?;
        Ok(Self {
            host: value
                .db_host
                .parse()
                .map_err(|e: ValueObjectError| e.to_string())?,
            port: value
                .db_port
                .try_into()
                .map_err(|e: ValueObjectError| e.to_string())?,
            username: value
                .db_user
                .parse()
                .map_err(|e: ValueObjectError| e.to_string())?,
            password: value
                .db_password
                .parse()
                .map_err(|e: ValueObjectError| e.to_string())?,
            database: value
                .db_name
                .parse()
                .map_err(|e: ValueObjectError| e.to_string())?,
            max_pool_size: Some(
                u32::try_from(value.db_max_pool_size)
                    .map_err(|_| "Invalid pool size".to_string())?,
            ),
            ssl_mode: Some(value.db_ssl_mode.clone()),
        })
    }
}

impl TryFrom<&Tenant> for BasicDatabaseConfig {
    type Error = String;
    fn try_from(value: &Tenant) -> Result<Self, Self::Error> {
        PgSslMode::from_str(&value.db_ssl_mode).map_err(|_| "invalid ssl_mode")?;
        Ok(Self {
            host: value.db_host.clone(),
            port: value.db_port as u16,
            username: value.db_user.clone(),
            password: value.db_password.clone(),
            database: value.db_name.clone(),
            max_pool_size: Some(
                u32::try_from(value.db_max_pool_size)
                    .map_err(|_| "Invalid pool size".to_string())?,
            ),
            ssl_mode: Some(value.db_ssl_mode.clone()),
        })
    }
}

impl<H, P, U, Pw, D> DatabasePoolSizeProvider for DatabaseConfig<H, P, U, Pw, D, u32>
where
    H: Debug,
    P: Debug,
    U: Debug,
    Pw: Debug,
    D: Debug,
{
    type MaxPoolSizeType = u32;

    fn max_pool_size(&self) -> u32 {
        self.max_pool_size.unwrap_or(3) // TODO: read global default from cfg!
    }
}

impl<H, P, U, Pw, D, M> DatabaseUrlProvider for DatabaseConfig<H, P, U, Pw, D, M>
where
    H: Display + Debug,
    P: Display + Debug,
    U: Display + Debug,
    Pw: Display + Debug,
    D: Display + Debug,
    M: Display + Debug,
{
    fn url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[cfg(test)]
pub mod tests {
    #![allow(unused)]

    use super::*;

    pub fn test_basic_database_config_builder()
    -> DatabaseConfigBuilder<String, u16, String, String, String, u32> {
        let mut builder = DatabaseConfigBuilder::default();
        builder
            .host(String::from("localhost"))
            .port(5432)
            .username(String::from("user"))
            .password(String::from("password"))
            .database(String::from("database"))
            .max_pool_size(Some(5))
            .ssl_mode(Some(String::from("prefer")));

        builder
    }
}
