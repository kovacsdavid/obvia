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

use crate::common::types::{DbHost, DbName, DbPassword, DbPort, DbUser, ValueObject};
use crate::manager::tenants::model::Tenant;
use serde::Deserialize;
use sqlx::postgres::PgSslMode;
use std::fmt::Display;
use std::str::FromStr;

pub type BasicDatabaseConfig = DatabaseConfig<String, u16, String, String, String, u32>;

pub type TenantDatabaseConfig = DatabaseConfig<
    ValueObject<DbHost>,
    ValueObject<DbPort>,
    ValueObject<DbUser>,
    ValueObject<DbPassword>,
    ValueObject<DbName>,
    u32,
>;

pub trait DatabaseUrlProvider {
    fn url(&self) -> String;
}

pub trait DatabasePoolSizeProvider {
    type MaxPoolSizeType;

    fn max_pool_size(&self) -> Self::MaxPoolSizeType;
}

#[allow(dead_code)]
pub trait DatabasePgSslModeProvider {
    fn pg_ssl_mode(&self) -> Result<PgSslMode, String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType>
{
    pub host: HostType,
    pub port: PortType,
    pub username: UserType,
    pub password: PasswordType,
    pub database: DatabaseType,
    pub max_pool_size: Option<MaxPoolSizeType>,
    pub ssl_mode: Option<String>,
}

impl<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType>
    DatabasePgSslModeProvider
    for DatabaseConfig<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType>
{
    fn pg_ssl_mode(&self) -> Result<PgSslMode, String> {
        if let Some(ssl_mode) = &self.ssl_mode {
            Ok(PgSslMode::from_str(ssl_mode).map_err(|_| "Invalid SSL mode".to_string())?)
        } else {
            Ok(PgSslMode::VerifyFull)
        }
    }
}

impl From<TenantDatabaseConfig> for BasicDatabaseConfig {
    fn from(value: TenantDatabaseConfig) -> Self {
        Self {
            host: value.host.to_string(),
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
            host: ValueObject::new(DbHost(value.db_host.clone())).map_err(|e| e.to_string())?,
            port: ValueObject::new(DbPort(value.db_port.to_string())).map_err(|e| e.to_string())?,
            username: ValueObject::new(DbUser(value.db_user.clone())).map_err(|e| e.to_string())?,
            password: ValueObject::new(DbPassword(value.db_password.clone()))
                .map_err(|e| e.to_string())?,
            database: ValueObject::new(DbName(value.db_name.clone())).map_err(|e| e.to_string())?,
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

impl<HostType, PortType, UserType, PasswordType, DatabaseType> DatabasePoolSizeProvider
    for DatabaseConfig<HostType, PortType, UserType, PasswordType, DatabaseType, u32>
{
    type MaxPoolSizeType = u32;

    fn max_pool_size(&self) -> u32 {
        self.max_pool_size.unwrap_or(3) // TODO: read global default from cfg!
    }
}

impl<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType> DatabaseUrlProvider
    for DatabaseConfig<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType>
where
    HostType: Display,
    PortType: Display,
    UserType: Display,
    PasswordType: Display,
    DatabaseType: Display,
    MaxPoolSizeType: Display,
{
    fn url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[allow(dead_code)]
pub struct DatabaseConfigBuilder<
    HostType,
    PortType,
    UserType,
    PasswordType,
    DatabaseType,
    MaxPoolSizeType,
> {
    host: Option<HostType>,
    port: Option<PortType>,
    username: Option<UserType>,
    password: Option<PasswordType>,
    database: Option<DatabaseType>,
    max_pool_size: Option<MaxPoolSizeType>,
    ssl_mode: Option<String>,
}

#[allow(dead_code)]
impl<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType>
    DatabaseConfigBuilder<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType>
{
    pub fn new() -> Self {
        DatabaseConfigBuilder {
            host: None,
            port: None,
            username: None,
            password: None,
            database: None,
            max_pool_size: None,
            ssl_mode: None,
        }
    }
    pub fn host(mut self, host: HostType) -> Self {
        self.host = Some(host);
        self
    }
    pub fn port(mut self, port: PortType) -> Self {
        self.port = Some(port);
        self
    }
    pub fn username(mut self, username: UserType) -> Self {
        self.username = Some(username);
        self
    }
    pub fn password(mut self, password: PasswordType) -> Self {
        self.password = Some(password);
        self
    }
    pub fn database(mut self, database: DatabaseType) -> Self {
        self.database = Some(database);
        self
    }
    pub fn max_pool_size(mut self, max_pool_size: MaxPoolSizeType) -> Self {
        self.max_pool_size = Some(max_pool_size);
        self
    }
    pub fn ssl_mode(mut self, ssl_mode: String) -> Self {
        self.ssl_mode = Some(ssl_mode);
        self
    }
    pub fn build(
        self,
    ) -> Result<
        DatabaseConfig<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType>,
        String,
    > {
        Ok(DatabaseConfig {
            host: self.host.ok_or("host is required")?,
            port: self.port.ok_or("port is required")?,
            username: self.username.ok_or("username is required")?,
            password: self.password.ok_or("password is required")?,
            database: self.database.ok_or("database is required")?,
            max_pool_size: self.max_pool_size,
            ssl_mode: self.ssl_mode,
        })
    }
}

#[cfg(not(test))]
impl<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType> Default
    for DatabaseConfigBuilder<
        HostType,
        PortType,
        UserType,
        PasswordType,
        DatabaseType,
        MaxPoolSizeType,
    >
{
    fn default() -> Self {
        DatabaseConfigBuilder::new()
    }
}
