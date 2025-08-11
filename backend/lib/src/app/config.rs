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
use crate::common::types::value_object::{ValueObject, ValueObjectable};
use crate::tenants::model::Tenant;
use crate::tenants::types::{DbHost, DbName, DbPassword, DbPort, DbUser};
use serde::Deserialize;
use sqlx::postgres::PgSslMode;
use std::fmt::Display;
use std::str::FromStr;

/// The `AppConfig` struct is the main application configuration model used for deserializing
/// and storing the configuration details for different components of the application.
/// It provides central management for settings like server configuration, database connections,
/// and authentication.
///
/// # Attributes
///
/// * `server` - Holds the configuration details for the application server (e.g., host, port, etc.).
///   Represented by the `ServerConfig` struct.
///
/// * `main_database` - Contains the configuration for the main database, such as connection strings
///   or credentials. Represented by the `MainDatabaseConfig` struct.
///
/// * `default_tenant_database` - Specifies the configuration for the default tenant database,
///   used in multi-tenant setups. Represented by the `DefaultTenantDatabaseConfig` struct.
///
/// * `auth` - Stores the details for authentication settings, including tokens, secret keys, etc.
///   Represented by the `AuthConfig` struct.
///
/// This struct is intended to be used as the central configuration hub for initializing
/// necessary dependencies of the application.
#[cfg_attr(test, derive(Debug, Clone, Deserialize, Default))]
#[cfg_attr(not(test), derive(Debug, Clone, Deserialize))]
pub struct AppConfig {
    server: ServerConfig,
    main_database: BasicDatabaseConfig,
    default_tenant_database: BasicDatabaseConfig,
    auth: AuthConfig,
}

/// A configuration struct for defining server settings.
///
/// `ServerConfig` holds crucial information about the hostname and port
/// on which the server will run. This configuration is commonly used
/// for initializing or binding the server to the specified address.
///
/// # Fields
///
/// * `host` - A `String` representing the hostname or IP address of the server.
/// * `port` - A `u16` representing the port number that the server will listen on.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    host: String,
    port: u16,
}

pub type BasicDatabaseConfig = DatabaseConfig<String, u16, String, String, String, u32>;

pub type TenantDatabaseConfig = DatabaseConfig<
    ValueObject<DbHost>,
    ValueObject<DbPort>,
    ValueObject<DbUser>,
    ValueObject<DbPassword>,
    ValueObject<DbName>,
    u32,
>;

/// A trait that provides a database connection URL.
///
/// This trait is meant to be implemented by types that can supply a valid
/// database connection URL. The provided `url` method is used to fetch
/// the connection string, which can then be utilized to establish connections
/// to a database.
pub trait DatabaseUrlProvider {
    /// Returns a string representation of the URL.
    ///
    /// # Description
    /// This method generates and returns the URL as a `String` that
    /// represents the resource it is associated with. The specific
    /// implementation of the URL generation logic will depend on the
    /// struct or enum that implements this method.
    ///
    /// # Returns
    /// - A `String` containing the URL.
    fn url(&self) -> String;
}

/// A trait that provides the ability to obtain the maximum size configuration
/// for a database connection pool. This can be implemented by any type that
/// manages or represents settings related to database connection pooling.
pub trait DatabasePoolSizeProvider {
    type MaxPoolSizeType;

    /// Retrieves the maximum size of the connection pool.
    ///
    /// This method returns the maximum number of connections that can be maintained
    /// in the pool at any given time. The value is defined by the pool's configuration
    /// settings and reflects the upper limit of resources allocated for handling connections.
    ///
    /// # Returns
    ///
    /// * `Self::MaxPoolSizeType` - The type representing the maximum pool size, as defined
    ///   by the implementation.
    fn max_pool_size(&self) -> Self::MaxPoolSizeType;
}

/// The `DatabasePgSslModeProvider` trait defines a contract for providing the PostgreSQL SSL mode configuration.
///
/// Implementors of this trait are responsible for determining the appropriate SSL mode that should
/// be used when connecting to a PostgreSQL database. SSL mode specifies the level of security the
/// connection should have regarding encryption and certificate validation.
///
/// # Required Method
///
/// ## `pg_ssl_mode`
///
/// Returns the configured `PgSslMode` for the database connection, or a descriptive `String` error
/// if the SSL mode could not be determined.
///
/// # Returns
/// - `Ok(PgSslMode)`: The SSL mode to be used for the PostgreSQL database connection.
/// - `Err(String)`: An error message indicating why the SSL mode could not be retrieved.
pub trait DatabasePgSslModeProvider {
    /// Retrieves the SSL mode configuration for the PostgreSQL connection.
    ///
    /// # Returns
    ///
    /// - `Ok(PgSslMode)` - If the SSL mode is successfully obtained.
    /// - `Err(String)` - If there is an error determining the SSL mode,
    ///   with a message describing the issue.
    fn pg_ssl_mode(&self) -> Result<PgSslMode, String>;
}

/// A generic configuration structure for database connection settings.
///
/// This struct is designed to be flexible with its field types, allowing
/// for a range of configurations depending on the use case. The generic
/// types provide the ability to define the specific types for each
/// configuration field, such as strings, integers, or custom wrapper types.
///
/// ## Generic Parameters
/// - `HostType`: The type of the host field, commonly a `String`.
/// - `PortType`: The type of the port field, typically an integer like `u16` or `i32`.
/// - `UserType`: The type of the username field, usually a `String`.
/// - `PasswordType`: The type of the password field, often a `String` or a secure wrapper.
/// - `DatabaseType`: The type of the database name, generally a `String`.
/// - `MaxPoolSizeType`: The type of maximum pool size for managing database connections, commonly a numeric type like `u32`.
///
/// ## Fields
/// - `host`: The host address of the database server (e.g., "localhost" or an IP address).
/// - `port`: The port used to connect to the database server.
/// - `username`: The username for authentication with the database.
/// - `password`: The password for authentication with the database.
/// - `database`: The name of the specific database to connect to.
/// - `max_pool_size`: An optional parameter specifying the maximum size of the connection pool. If `None`, a default maximum size may be used depending on the implementation.
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
    /// Converts a `TenantDatabaseConfig` into another data type by extracting and transforming its values.
    ///
    /// This function takes an instance of `TenantDatabaseConfig` and constructs a new instance of the
    /// target type (`Self`) by extracting and cloning specific values from the source.
    ///
    /// # Parameters
    /// - `value`: An instance of `TenantDatabaseConfig` containing the configuration details to be converted.
    ///
    /// # Returns
    /// A new instance of the target structure (`Self`) populated with the extracted and transformed values.
    ///
    /// # Fields Mapping:
    /// - `host`: Extracted, de-referenced, and cloned from `value.host` using its internal methods.
    /// - `port`: Extracted, de-referenced, and cast to `u16` from `value.port`.
    /// - `username`: Extracted and cloned from `value.username`.
    /// - `password`: Extracted and cloned from `value.password`.
    /// - `database`: Extracted and cloned from `value.database`.
    /// - `max_pool_size`: Directly assigned from `value.max_pool_size`.
    /// - `ssl_mode`: Directly assigned from `value.ssl_mode`.
    fn from(value: TenantDatabaseConfig) -> Self {
        Self {
            host: value.host.extract().get_value().clone(),
            port: *value.port.extract().get_value() as u16,
            username: value.username.extract().get_value().clone(),
            password: value.password.extract().get_value().clone(),
            database: value.database.extract().get_value().clone(),
            max_pool_size: value.max_pool_size,
            ssl_mode: value.ssl_mode,
        }
    }
}

impl TryFrom<&Tenant> for TenantDatabaseConfig {
    type Error = String;
    /// Attempts to convert an `Tenant` reference into the corresponding object of the implementing type.
    ///
    /// This function maps the fields of the `Tenant` into their respective strongly typed `ValueObject`
    /// wrappers for database configuration parameters. The function validates and constructs each field, returning
    /// an error if any field is invalid or if an intermediate operation (e.g., type conversion) fails.
    ///
    /// # Arguments
    /// * `value` - A reference to an `Tenant` object that holds the database configuration details.
    ///
    /// # Returns
    /// * `Ok(Self)` - If all fields are successfully validated and converted.
    /// * `Err(Self::Error)` - If any validation or conversion fails during the mapping process.
    ///
    /// # Errors
    /// Returns an error in the following cases:
    /// * The `DbHost`, `DbPort`, `DbUser`, `DbPassword`, or `DbName` fields cannot be constructed due to invalid values.
    /// * The conversion of `db_max_pool_size` to `u32` fails (e.g., due to the value being out of range).
    ///
    /// # Notes
    /// * This implementation uses the `ValueObject::new` function to wrap raw values into their respective types.
    /// * It's important that all fields in the `Tenant` adhere to the expected format
    ///   and constraints for successful conversion.
    fn try_from(value: &Tenant) -> Result<Self, Self::Error> {
        PgSslMode::from_str(&value.db_ssl_mode).map_err(|_| "invalid ssl_mode")?;
        Ok(Self {
            host: ValueObject::new(DbHost(value.db_host.clone()))?,
            port: ValueObject::new(DbPort(value.db_port as i64))?,
            username: ValueObject::new(DbUser(value.db_user.clone()))?,
            password: ValueObject::new(DbPassword(value.db_password.clone()))?,
            database: ValueObject::new(DbName(value.db_name.clone()))?,
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
    /// Attempts to convert an `Tenant` reference into the corresponding object of the implementing type.
    ///
    /// This function maps the fields of the `Tenant` into their respective strongly typed `ValueObject`
    /// wrappers for database configuration parameters. The function validates and constructs each field, returning
    /// an error if any field is invalid or if an intermediate operation (e.g., type conversion) fails.
    ///
    /// # Arguments
    /// * `value` - A reference to an `Tenant` object that holds the database configuration details.
    ///
    /// # Returns
    /// * `Ok(Self)` - If all fields are successfully validated and converted.
    /// * `Err(Self::Error)` - If any validation or conversion fails during the mapping process.
    ///
    /// # Errors
    /// Returns an error in the following cases:
    /// * The `DbHost`, `DbPort`, `DbUser`, `DbPassword`, or `DbName` fields cannot be constructed due to invalid values.
    /// * The conversion of `db_max_pool_size` to `u32` fails (e.g., due to the value being out of range).
    ///
    /// # Notes
    /// * This implementation uses the `ValueObject::new` function to wrap raw values into their respective types.
    /// * It's important that all fields in the `Tenant` adhere to the expected format
    ///   and constraints for successful conversion.
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

    /// Returns the maximum pool size.
    ///
    /// This method retrieves the maximum pool size for the object. If the `max_pool_size` is set,
    /// it will return its value. Otherwise, it defaults to `3`. The default value can be updated in
    /// the future to read from a global configuration, as indicated by the TODO comment.
    ///
    /// # Returns
    /// * `u32` - The maximum pool size, defaulting to `3` if not explicitly set.
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
    /// Constructs and returns a PostgreSQL connection URL as a `String`.
    ///
    /// The URL follows the format:
    /// `postgresql://<username>:<password>@<host>:<port>/<database>`
    ///
    /// # Returns
    /// * A `String` containing the PostgreSQL connection URL.
    ///
    /// # Notes
    ///
    /// This function assumes that the struct fields `username`, `password`, `host`, `port`, and
    /// `database` are properly initialized and valid. An incorrect or missing field value
    /// may result in an invalid URL.
    fn url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[cfg(test)]
impl Default for TenantDatabaseConfig {
    /// Provides a default implementation for the database configuration settings.
    ///
    /// # Returns
    /// A `Self` instance populated with default values for the following fields:
    ///
    /// - `host`: Defaults to "localhost".
    /// - `port`: Defaults to 5432.
    /// - `username`: Defaults to "user".
    /// - `password`: Defaults to "password".
    /// - `database`: Defaults to "database".
    /// - `pool_size`: Defaults to `Some(5)`.
    /// - `ssl_mode`: "prefer"
    ///
    /// Each field is wrapped in a `ValueObject` for validation and ensures safe initialization.
    /// Uses `unwrap()` to assume successful creation of `ValueObject` instances for valid inputs.
    fn default() -> Self {
        Self {
            host: ValueObject::new(DbHost("localhost".to_string())).unwrap(),
            port: ValueObject::new(DbPort(5432)).unwrap(),
            username: ValueObject::new(DbUser("user".to_string())).unwrap(),
            password: ValueObject::new(DbPassword("password".to_string())).unwrap(),
            database: ValueObject::new(DbName("database".to_string())).unwrap(),
            max_pool_size: Some(5),
            ssl_mode: Some("prefer".to_string()),
        }
    }
}

#[cfg(test)]
impl Default for BasicDatabaseConfig {
    /// Provides a default implementation for the `default` method, which initializes
    /// a new instance of the struct with predefined default configuration values.
    ///
    /// # Returns
    ///
    /// - `Self`: A new instance of the struct populated with default settings.
    ///
    /// # Default Values
    ///
    /// - `host`: `"localhost"`
    /// - `port`: `5432`
    /// - `username`: `"user"`
    /// - `password`: `"password"`
    /// - `database`: `"database"`
    /// - `pool_size`: `Some(5)`
    /// - `ssl_mode`: "prefer"
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 5432,
            username: String::from("user"),
            password: String::from("password"),
            database: String::from("database"),
            max_pool_size: Some(5),
            ssl_mode: Some("prefer".to_string()),
        }
    }
}

/// `AuthConfig` is a configuration struct that provides the necessary details for handling
/// JSON Web Token (JWT) authentication.
///
/// # Fields
///
/// * `jwt_secret` - A `String` representing the secret key used to sign and verify JWTs. This value is critical for ensuring the integrity and authenticity of the tokens.
///
/// * `jwt_issuer` - A `String` representing the issuer of the JWTs.
///
/// * `jwt_audience` - A `String` indicating the intended audience who can use the JWTs. A unique identifier associated with the clients or consumers of the service.
///
/// * `jwt_expiration_mins` - A `u64` representing the expiration time for JWTs in minutes. This determines the duration after which the token will become invalid.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    jwt_secret: String,
    jwt_issuer: String,
    jwt_audience: String,
    jwt_expiration_mins: u64,
}

impl Default for ServerConfig {
    /// Provides a default implementation for the `ServerConfig` struct.
    ///
    /// # Returns
    ///
    /// A new instance of `ServerConfig` with the following default values:
    /// - `host`: `"127.0.0.1"`
    /// - `port`: `3000`
    ///
    /// These default values are used for local development or testing scenarios.
    fn default() -> Self {
        ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}

impl Default for AuthConfig {
    /// Provides a default implementation for the `AuthConfig` struct.
    ///
    /// This implementation initializes the `AuthConfig` structure with
    /// the following default values:
    ///
    /// - `jwt_secret`: A default JWT secret string set to `"test_jwt_secret"`.
    /// - `jwt_issuer`: A default issuer URL set to `"http://localhost"`.
    /// - `jwt_audience`: A default audience URL set to `"http://localhost"`.
    /// - `jwt_expiration_mins`: The default expiration time for JWTs, set to 60 minutes.
    ///
    /// These default values are used for local development or testing scenarios.
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
    /// Loads configuration settings from the environment.
    ///
    /// This function reads configuration settings from a file named "config/default"
    /// located in the project directory. The file is expected to be a required
    /// resource, and its absence will result in an error. The settings are then
    /// deserialized into the application's configuration structure.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the configuration is successfully loaded and deserialized
    ///   into the appropriate structure.
    /// - `Err(config::ConfigError)`: If an error occurs during configuration
    ///   loading or deserialization.
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The configuration file "config/default" is missing or inaccessible.
    /// - The contents of the file cannot be parsed or deserialized into the
    ///   expected structure.
    ///
    /// Make sure to include a valid "config/default" file in your project directory.
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let builder = config::Config::builder()
            .add_source(config::File::with_name("config/default").required(true));
        builder.build()?.try_deserialize()
    }

    /// Retrieves a reference to the `ServerConfig` instance associated with the current object.
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    /// Provides access to the main database configuration.
    pub fn main_database(&self) -> &BasicDatabaseConfig {
        &self.main_database
    }

    /// Returns a reference to the `DefaultTenantDatabaseConfig` instance.
    pub fn default_tenant_database(&self) -> &BasicDatabaseConfig {
        &self.default_tenant_database
    }

    /// Retrieves a reference to the `AuthConfig`.
    pub fn auth(&self) -> &AuthConfig {
        &self.auth
    }
}

impl ServerConfig {
    /// Returns the host value.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Returns the port number.
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl AuthConfig {
    /// Returns the jwt_secret.
    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
    /// Returns the jwt_issuer.
    pub fn jwt_issuer(&self) -> &str {
        &self.jwt_issuer
    }
    /// Returns the jwt_audience.
    pub fn jwt_audience(&self) -> &str {
        &self.jwt_audience
    }
    /// Returns the jwt_expiration_mins.
    pub fn jwt_expiration_mins(&self) -> u64 {
        self.jwt_expiration_mins
    }
}
