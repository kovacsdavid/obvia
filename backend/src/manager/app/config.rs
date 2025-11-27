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
use crate::common::types::{ValueObject, ValueObjectable};
use crate::manager::tenants::model::Tenant;
use crate::manager::tenants::types::{DbHost, DbName, DbPassword, DbPort, DbUser};
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
/// * `mail` - Stores the details for mailing settings like smtp server, user, password etc.
///   Represented by the `MailConfig` struct.
///
/// This struct is intended to be used as the central configuration hub for initializing
/// necessary dependencies of the application.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    main_database: BasicDatabaseConfig,
    default_tenant_database: BasicDatabaseConfig,
    auth: AuthConfig,
    mail: MailConfig,
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
    pub fn mail(&self) -> &MailConfig {
        &self.mail
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

#[derive(Debug, Clone, Deserialize)]
pub struct MailConfig {
    smtp_host: String,
    smtp_user: String,
    smtp_passwd: String,
    default_from: String,
    default_from_name: String,
    default_notification_email: String,
}

impl MailConfig {
    pub fn smtp_host(&self) -> &str {
        &self.smtp_host
    }
    pub fn smtp_user(&self) -> &str {
        &self.smtp_user
    }
    pub fn smtp_passwd(&self) -> &str {
        &self.smtp_passwd
    }
    pub fn default_from(&self) -> &str {
        &self.default_from
    }
    pub fn default_from_name(&self) -> &str {
        &self.default_from_name
    }
    pub fn default_notification_email(&self) -> &str {
        &self.default_notification_email
    }
}

pub struct MailConfigBuilder {
    smtp_host: Option<String>,
    smtp_user: Option<String>,
    smtp_passwd: Option<String>,
    default_from: Option<String>,
    default_from_name: Option<String>,
    default_notification_email: Option<String>,
}

impl MailConfigBuilder {
    pub fn new() -> Self {
        MailConfigBuilder {
            smtp_host: None,
            smtp_user: None,
            smtp_passwd: None,
            default_from: None,
            default_from_name: None,
            default_notification_email: None,
        }
    }
    pub fn smtp_host(mut self, smtp_host: String) -> Self {
        self.smtp_host = Some(smtp_host);
        self
    }
    pub fn smtp_user(mut self, smtp_user: String) -> Self {
        self.smtp_user = Some(smtp_user);
        self
    }
    pub fn smtp_passwd(mut self, smtp_passwd: String) -> Self {
        self.smtp_passwd = Some(smtp_passwd);
        self
    }
    pub fn default_from(mut self, default_from: String) -> Self {
        self.default_from = Some(default_from);
        self
    }
    pub fn default_from_name(mut self, default_from_name: String) -> Self {
        self.default_from_name = Some(default_from_name);
        self
    }
    pub fn default_notification_email(mut self, default_notification_email: String) -> Self {
        self.default_notification_email = Some(default_notification_email);
        self
    }
    pub fn build(self) -> Result<MailConfig, String> {
        Ok(MailConfig {
            smtp_host: self.smtp_host.ok_or("smtp_host is required")?,
            smtp_user: self.smtp_user.ok_or("smtp_user is required")?,
            smtp_passwd: self.smtp_passwd.ok_or("smtp_passwd is required")?,
            default_from: self.default_from.ok_or("default_from is required")?,
            default_from_name: self
                .default_from_name
                .ok_or("default_from_name is required")?,
            default_notification_email: self
                .default_notification_email
                .ok_or("default_notification_email is required")?,
        })
    }
}

#[cfg(not(test))]
impl Default for MailConfigBuilder {
    fn default() -> Self {
        MailConfigBuilder::new()
    }
}

/// `AppConfigBuilder` is a builder struct used to configure and construct
/// an application configuration with various components such as server,
/// databases, and authentication.
///
/// # Fields
/// - `server`:
///   Contains an optional `ServerConfig` that holds the server configuration
///   details, such as host and port settings for the application.
///
/// - `main_database`:
///   Holds an optional `BasicDatabaseConfig` representing the configuration
///   for the primary database required by the application, such as connection
///   details or database credentials.
///
/// - `default_tenant_database`:
///   Contains an optional `BasicDatabaseConfig` that provides the configuration
///   for the default tenant database, helpful in multi-tenancy setups.
///
/// - `auth`:
///   Includes an optional `AuthConfig` to provide authentication settings,
///   such as token keys, protocols, or third-party integration.
///
/// This struct provides a flexible way to gradually assemble and configure
/// an `AppConfig` by setting only the required components and leaving others
/// as `None`.
pub struct AppConfigBuilder {
    server: Option<ServerConfig>,
    main_database: Option<BasicDatabaseConfig>,
    default_tenant_database: Option<BasicDatabaseConfig>,
    auth: Option<AuthConfig>,
    mail: Option<MailConfig>,
}

impl AppConfigBuilder {
    /// Creates a new instance of the struct with default values.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` where all fields are initialized to `None`.
    pub fn new() -> Self {
        Self {
            server: None,
            main_database: None,
            default_tenant_database: None,
            auth: None,
            mail: None,
        }
    }
    ///
    /// Sets the server configuration for the current instance.
    ///
    /// This method accepts a `ServerConfig` object and assigns it to the instance's `server` field.
    /// It uses a builder pattern, allowing method chaining after setting the server configuration.
    ///
    /// # Arguments
    ///
    /// * `server` - A `ServerConfig` object containing the configuration details for the server.
    ///
    /// # Returns
    ///
    /// Returns the modified instance of the struct with the server configuration updated.
    pub fn server(mut self, server: ServerConfig) -> Self {
        self.server = Some(server);
        self
    }
    /// Sets the main database configuration for the current instance.
    ///
    /// This method takes in a `BasicDatabaseConfig` object, representing
    /// the configuration details for the main database, and assigns it
    /// to the instance. It modifies the current instance by setting the
    /// `main_database` field to the provided configuration.
    ///
    /// # Parameters
    /// - `main_database`: A `BasicDatabaseConfig` object containing the configuration
    ///   for the main database.
    ///
    /// # Returns
    /// Returns `Self`, allowing for method chaining.
    pub fn main_database(mut self, main_database: BasicDatabaseConfig) -> Self {
        self.main_database = Some(main_database);
        self
    }
    /// Sets the default tenant database configuration for the instance.
    ///
    /// This method takes a `BasicDatabaseConfig` and assigns it to the `default_tenant_database` field,
    /// wrapping it in a `Some`. It allows for chaining by returning `self`.
    ///
    /// # Parameters
    /// - `default_tenant_database`: A `BasicDatabaseConfig` instance representing the default database configuration associated with a tenant.
    ///
    /// # Returns
    /// - `Self`: Returns an updated instance of the struct with the specified default tenant database set.
    pub fn default_tenant_database(mut self, default_tenant_database: BasicDatabaseConfig) -> Self {
        self.default_tenant_database = Some(default_tenant_database);
        self
    }
    /// Sets the authentication configuration for the current object.
    ///
    /// This method takes an `AuthConfig` structure as input and assigns it
    /// to the `auth` field of the current object. It returns the updated
    /// object, allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `auth` - An instance of `AuthConfig` containing the authentication configuration to be applied.
    ///
    /// # Returns
    ///
    /// Returns the updated object with the specified authentication configuration applied,
    /// allowing for further chained method calls.
    pub fn auth(mut self, auth: AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn mail(mut self, mail: MailConfig) -> Self {
        self.mail = Some(mail);
        self
    }
    /// Builds the `AppConfig` instance using the provided values in the builder object.
    ///
    /// This method finalizes the configuration setup and ensures all required fields are present.
    /// If any required configuration field is missing, it returns an error with a descriptive message.
    ///
    /// # Returns
    ///
    /// * `Ok(AppConfig)` - If all required fields (`server`, `main_database`, `default_tenant_database`, `auth`)
    ///   are properly set, it returns an instance of `AppConfig`.
    /// * `Err(String)` - If any required field is missing, it returns an error string indicating
    ///   which field is missing.
    ///
    /// # Errors
    ///
    /// This method checks for the presence of the following required configurations:
    /// - `server`: Should be set before calling `build`.
    /// - `main_database`: Should be set before calling `build`.
    /// - `default_tenant_database`: Should be set before calling `build`.
    /// - `auth`: Should be set before calling `build`.
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

#[cfg(not(test))]
impl Default for AppConfigBuilder {
    fn default() -> Self {
        AppConfigBuilder::new()
    }
}

/// `ServerConfigBuilder` is a builder struct for configuring and creating
/// a server's configuration. It provides an easy way to set optional properties
/// for the server such as `host` and `port`.
///
/// # Fields
/// - `host`: An `Option<String>` that represents the hostname or IP address
///   the server will bind to. It is optional and defaults to `None`.
/// - `port`: An `Option<u16>` that represents the port number the server will
///   listen on. It is optional and defaults to `None`.
pub struct ServerConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
}

impl ServerConfigBuilder {
    /// Creates and returns a new instance of the struct with its fields initialized to default values.
    ///
    /// # Returns
    /// * `Self` - A new instance of the struct with `host` and `port` fields set to `None`.
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
        }
    }
    /// Sets the host for the configuration.
    ///
    /// This function allows you to specify the hostname or IP address
    /// that will be used in the configuration. The provided `host`
    /// will overwrite any previously set value.
    ///
    /// # Arguments
    ///
    /// * `host` - A `String` representing the hostname or IP address to be set.
    ///
    /// # Returns
    ///
    /// Returns an updated instance of `Self` with the `host` value set.
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    /// Sets the port for the current configuration.
    ///
    /// This method allows you to specify a specific port for the instance. It takes
    /// a `u16` value representing the desired port and assigns it to the instance's
    /// `port` field. Returns the modified instance to allow method chaining.
    ///
    /// # Arguments
    ///
    /// * `port` - A `u16` value representing the port number to be set.
    ///
    /// # Returns
    ///
    /// Returns `Self` (the modified instance) to enable the chaining of further
    /// configuration methods.
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    /// Builds a `ServerConfig` instance from the current configuration in the builder.
    ///
    /// This method attempts to create a `ServerConfig` object using the values
    /// provided during the configuration process. It verifies that all required
    /// fields (`host` and `port`) are present. If any required fields are missing,
    /// it returns an error with a descriptive message.
    ///
    /// # Returns
    /// - `Ok(ServerConfig)`: If all required fields are set, returns a successfully
    ///   built `ServerConfig` instance.
    /// - `Err(String)`: If a required field (e.g., `host` or `port`) is missing,
    ///   returns an error with a relevant message.
    ///
    /// # Errors
    /// - Returns an error with the message `"host is required"` if the `host` field
    ///   is not set.
    /// - Returns an error with the message `"port is required"` if the `port` field
    ///   is not set.
    pub fn build(self) -> Result<ServerConfig, String> {
        Ok(ServerConfig {
            host: self.host.ok_or("host is required".to_string())?,
            port: self.port.ok_or("port is required".to_string())?,
        })
    }
}

#[cfg(not(test))]
impl Default for ServerConfigBuilder {
    fn default() -> Self {
        ServerConfigBuilder::new()
    }
}

/// A builder struct for configuring database connection parameters. It allows for setting
/// various database connection attributes such as host, port, username, password, database
/// name, maximum connection pool size, and SSL mode.
///
/// Each generic type parameter corresponds to a configuration field, allowing flexible typing
/// for the builder's components.
pub struct DatabaseConfigBuilder<
    HostType,
    PortType,
    UserType,
    PasswordType,
    DatabaseType,
    MaxPoolSizeType,
> {
    pub host: Option<HostType>,
    pub port: Option<PortType>,
    pub username: Option<UserType>,
    pub password: Option<PasswordType>,
    pub database: Option<DatabaseType>,
    pub max_pool_size: Option<MaxPoolSizeType>,
    pub ssl_mode: Option<String>,
}

impl<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType>
    DatabaseConfigBuilder<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType>
{
    /// Creates a new instance of `DatabaseConfigBuilder` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A `DatabaseConfigBuilder` struct with the following fields initialized to `None`:
    /// - `host`: Represents the database host (e.g., IP address or hostname).
    /// - `port`: Represents the database port.
    /// - `username`: Represents the username for database authentication.
    /// - `password`: Represents the password for database authentication.
    /// - `database`: Represents the name of the database to connect to.
    /// - `max_pool_size`: Represents the maximum size of the connection pool.
    /// - `ssl_mode`: Represents the SSL (Secure Sockets Layer) mode for the database connection.
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
    /// Sets the host for the current configuration, applying the specified `HostType`.
    ///
    /// # Parameters
    /// - `host`: The `HostType` to set for the current instance. This value represents
    ///   the desired host configuration.
    ///
    /// # Returns
    /// - `Self`: Returns the updated instance of the struct with the specified host set.
    pub fn host(mut self, host: HostType) -> Self {
        self.host = Some(host);
        self
    }
    /// Sets the port for the current instance.
    ///
    /// This method allows you to specify a `PortType` for the instance.
    /// It modifies the `self` object by setting the `port` field to the provided value
    /// and then returns the updated instance.
    ///
    /// # Parameters
    /// - `port`: The port of type `PortType` to be assigned to the instance.
    ///
    /// # Returns
    /// Returns the updated instance of the type implementing this method.
    pub fn port(mut self, port: PortType) -> Self {
        self.port = Some(port);
        self
    }
    /// Sets the username for the current instance.
    ///
    /// This method allows you to assign a username of type `UserType` to the current instance.
    /// It takes ownership of `self`, sets the provided username, and then returns the modified instance,
    /// enabling a chained method call approach.
    ///
    /// # Parameters
    /// - `username`: The username to be assigned, of type `UserType`.
    ///
    /// # Returns
    /// The updated instance of the struct with the `username` field set to the provided value.
    pub fn username(mut self, username: UserType) -> Self {
        self.username = Some(username);
        self
    }
    /// Sets the password field for the current instance.
    ///
    /// This method takes a `PasswordType` as its parameter and assigns it
    /// to the `password` field of the instance. The method returns the
    /// modified instance to support method chaining.
    ///
    /// # Parameters
    /// - `password`: The password value of type `PasswordType` to be set.
    ///
    /// # Returns
    /// Returns the updated instance of `Self` with the `password` field set.
    pub fn password(mut self, password: PasswordType) -> Self {
        self.password = Some(password);
        self
    }
    /// Sets the `DatabaseType` for the current configuration.
    ///
    /// This method allows you to specify the type of database to be used in the configuration.
    /// It takes a `DatabaseType` enum value as an argument and updates the `database` field
    /// of the configuration object. The method consumes `self`, modifies the `database` field,
    /// and returns the updated `Self` to allow method chaining.
    ///
    /// # Arguments
    ///
    /// * `database` - A `DatabaseType` value representing the type of database to be configured.
    ///
    /// # Returns
    ///
    /// Returns the updated instance of `Self` with the new `database` value set.
    pub fn database(mut self, database: DatabaseType) -> Self {
        self.database = Some(database);
        self
    }
    /// Sets the maximum pool size for the resource pool.
    ///
    /// This method allows you to configure the maximum number of resources
    /// that can exist in the pool. It takes a parameter of type `MaxPoolSizeType`
    /// and assigns it to the `max_pool_size` field. The method then returns
    /// the modified instance of the struct to allow for method chaining.
    ///
    /// # Parameters
    ///
    /// * `max_pool_size` - The maximum number of resources allowed in the pool.
    ///
    /// # Returns
    ///
    /// Returns `Self`, allowing for method chaining to configure the instance further.
    pub fn max_pool_size(mut self, max_pool_size: MaxPoolSizeType) -> Self {
        self.max_pool_size = Some(max_pool_size);
        self
    }
    /// Sets the SSL mode for the current configuration.
    ///
    /// This method allows you to specify the SSL mode for the connection by passing
    /// a string representing the desired mode (e.g., "require", "prefer", "disable").
    ///
    /// # Parameters
    /// - `ssl_mode`: A `String` that represents the SSL mode to be applied.
    ///
    /// # Returns
    /// - `Self`: The updated instance of the builder or configuration struct, enabling method chaining.
    pub fn ssl_mode(mut self, ssl_mode: String) -> Self {
        self.ssl_mode = Some(ssl_mode);
        self
    }
    /// Builds and returns a `DatabaseConfig` instance if all required fields are set.
    ///
    /// This function constructs an instance of `DatabaseConfig` using the builder pattern.
    /// It ensures that all mandatory fields (`host`, `port`, `username`, `password`,
    /// and `database`) are properly set, returning an error message if any of them are missing.
    ///
    /// # Type Parameters
    /// - `HostType`: The type of the `host` field.
    /// - `PortType`: The type of the `port` field.
    /// - `UserType`: The type of the `username` field.
    /// - `PasswordType`: The type of the `password` field.
    /// - `DatabaseType`: The type of the `database` field.
    /// - `MaxPoolSizeType`: The type of the `max_pool_size` field.
    ///
    /// # Returns
    /// - `Ok(DatabaseConfig<HostType, PortType, UserType, PasswordType, DatabaseType, MaxPoolSizeType>)`:
    ///   If all required fields are properly set.
    /// - `Err(String)`: An error message indicating which required field is missing, if any.
    ///
    /// # Errors
    /// Returns an error if any of the following fields are not set:
    /// - `host`
    /// - `port`
    /// - `username`
    /// - `password`
    /// - `database`
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

/// A builder for constructing an `AuthConfig` with customizable properties.
///
/// The `AuthConfigBuilder` struct allows users to set various authentication-related
/// configuration options, such as JWT secret, issuer, audience, and expiration time.
///
/// # Fields
///
/// * `jwt_secret` - An optional `String` that represents the secret key used for signing JWTs. This is a critical component in ensuring the security of the authentication process.
///
/// * `jwt_issuer` - An optional `String` that represents the issuer of the JWT. This field is often used to identify the entity responsible for generating the token.
///
/// * `jwt_audience` - An optional `String` that specifies the intended audience of the JWT. The audience is typically used to restrict the scope of the token to specific consumers.
///
/// * `jwt_expiration_mins` - An optional `u64` that determines the expiration time of the token in minutes. This helps ensure that tokens have a limited validity period and adds an additional security layer.
///
/// This builder follows a step-by-step configuration approach, allowing users to customize only the fields they need, leaving the rest as default (if a default implementation exists).
pub struct AuthConfigBuilder {
    jwt_secret: Option<String>,
    jwt_issuer: Option<String>,
    jwt_audience: Option<String>,
    jwt_expiration_mins: Option<u64>,
}

impl AuthConfigBuilder {
    /// Creates a new instance of the `AuthConfigBuilder`.
    ///
    /// The `new` function initializes an `AuthConfigBuilder` with all its fields set to `None`.
    /// This allows the builder pattern to be used to configure the necessary fields before
    /// constructing an `AuthConfig` object.
    ///
    /// # Returns
    ///
    /// * A new `AuthConfigBuilder` instance with the following fields:
    ///   * `jwt_secret`: `None`
    ///   * `jwt_issuer`: `None`
    ///   * `jwt_audience`: `None`
    ///   * `jwt_expiration_mins`: `None`
    pub fn new() -> Self {
        AuthConfigBuilder {
            jwt_secret: None,
            jwt_issuer: None,
            jwt_audience: None,
            jwt_expiration_mins: None,
        }
    }
    /// Sets the JWT secret for the configuration.
    ///
    /// This method takes a `String` representing the secret key used for signing and verifying JWTs
    /// (JSON Web Tokens) and assigns it to the internal `jwt_secret` field of the struct.
    ///
    /// # Parameters
    /// - `jwt_secret`: A `String` containing the secret key for JWT operations.
    ///
    /// # Returns
    /// The updated instance of the struct, allowing method chaining.
    pub fn jwt_secret(mut self, jwt_secret: String) -> Self {
        self.jwt_secret = Some(jwt_secret);
        self
    }
    /// Sets the JWT (JSON Web Token) issuer for the current instance.
    ///
    /// This method allows specifying a custom issuer value, which will be used
    /// in the JWT payload to identify the principal that issued the token.
    ///
    /// # Arguments
    ///
    /// * `jwt_issuer` - A `String` representing the issuer of the JWT. It typically
    ///   identifies the service or entity generating the token.
    ///
    /// # Returns
    ///
    /// Returns the modified instance of `Self` after setting the `jwt_issuer` value.
    pub fn jwt_issuer(mut self, jwt_issuer: String) -> Self {
        self.jwt_issuer = Some(jwt_issuer);
        self
    }
    /// Sets the JWT audience value for the current instance.
    ///
    /// This method sets the `jwt_audience` field of the instance to the given value.
    /// It allows method chaining by returning the modified instance.
    ///
    /// # Parameters
    /// - `jwt_audience`: A `String` containing the JWT audience value to be set.
    ///
    /// # Returns
    /// Returns the modified instance of the struct implementing this method,
    /// allowing for method chaining.
    pub fn jwt_audience(mut self, jwt_audience: String) -> Self {
        self.jwt_audience = Some(jwt_audience);
        self
    }
    /// Sets the expiration time for JSON Web Tokens (JWT) in minutes.
    ///
    /// # Parameters
    /// - `jwt_expiration_mins`: The expiration time in minutes for the JWT.
    ///
    /// # Returns
    /// - `Self`: Returns the modified instance of the struct, allowing for method chaining.
    pub fn jwt_expiration_mins(mut self, jwt_expiration_mins: u64) -> Self {
        self.jwt_expiration_mins = Some(jwt_expiration_mins);
        self
    }
    /// Builds an `AuthConfig` instance from the current state of the builder.
    ///
    /// This method validates that all required fields have been set and constructs an `AuthConfig`
    /// struct from the provided configuration data. If any required field is missing, the method
    /// will return an error indicating which field is missing.
    ///
    /// # Returns
    /// An `AuthConfig` instance populated with the values provided to the builder.
    ///
    /// # Errors
    /// Returns an error for each of the following missing fields:
    /// - `"jwt_secret is required"`: If the `jwt_secret` field was not set.
    /// - `"jwt_issuer is required"`: If the `jwt_issuer` field was not set.
    /// - `"jwt_audience is required"`: If the `jwt_audience` field was not set.
    /// - `"jwt_expiration_mins is required"`: If the `jwt_expiration_mins` field was not set.
    pub fn build(self) -> Result<AuthConfig, String> {
        Ok(AuthConfig {
            jwt_secret: self.jwt_secret.ok_or("jwt_secret is required")?,
            jwt_issuer: self.jwt_issuer.ok_or("jwt_issuer is required")?,
            jwt_audience: self.jwt_audience.ok_or("jwt_audience is required")?,
            jwt_expiration_mins: self
                .jwt_expiration_mins
                .ok_or("jwt_expiration_mins is required")?,
        })
    }
}

#[cfg(not(test))]
impl Default for AuthConfigBuilder {
    fn default() -> Self {
        AuthConfigBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Default for AuthConfigBuilder {
        /// Provides a default implementation for the `AuthConfigBuilder` struct.
        ///
        /// This implementation initializes the `AuthConfigBuilder` structure with
        /// the following default values:
        ///
        /// - `jwt_secret`: A default JWT secret string set to `"test_jwt_secret"`.
        /// - `jwt_issuer`: A default issuer URL set to `"http://localhost"`.
        /// - `jwt_audience`: A default audience URL set to `"http://localhost"`.
        /// - `jwt_expiration_mins`: The default expiration time for JWTs, set to 60 minutes.
        ///
        /// These default values are used for local development or testing scenarios.
        fn default() -> Self {
            AuthConfigBuilder {
                jwt_secret: Some("test_jwt_secret".to_string()),
                jwt_issuer: Some("http://localhost".to_string()),
                jwt_audience: Some("http://localhost".to_string()),
                jwt_expiration_mins: Some(60),
            }
        }
    }

    impl Default for DatabaseConfigBuilder<String, u16, String, String, String, u32> {
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
                host: Some(String::from("localhost")),
                port: Some(5432),
                username: Some(String::from("user")),
                password: Some(String::from("password")),
                database: Some(String::from("database")),
                max_pool_size: Some(5),
                ssl_mode: Some("prefer".to_string()),
            }
        }
    }

    impl Default for ServerConfigBuilder {
        /// Provides a default implementation for the `ServerConfigBuilder` struct.
        ///
        /// # Returns
        ///
        /// A new instance of `ServerConfigBuilder` with the following default values:
        /// - `host`: `"127.0.0.1"`
        /// - `port`: `3000`
        ///
        /// These default values are used for local development or testing scenarios.
        fn default() -> Self {
            ServerConfigBuilder {
                host: Some("127.0.0.1".to_string()),
                port: Some(3000),
            }
        }
    }

    impl Default for MailConfigBuilder {
        fn default() -> Self {
            MailConfigBuilder {
                smtp_host: Some(String::from("localhost")),
                smtp_user: Some(String::from("noreply@example.com")),
                smtp_passwd: Some(String::from("secret")),
                default_from: Some(String::from("noreply@example.com")),
                default_from_name: Some(String::from("Example")),
                default_notification_email: Some(String::from("admin@example.com")),
            }
        }
    }

    impl Default for AppConfigBuilder {
        fn default() -> Self {
            AppConfigBuilder {
                server: Some(ServerConfigBuilder::default().build().unwrap()),
                main_database: Some(DatabaseConfigBuilder::default().build().unwrap()),
                default_tenant_database: Some(DatabaseConfigBuilder::default().build().unwrap()),
                auth: Some(AuthConfigBuilder::default().build().unwrap()),
                mail: Some(MailConfigBuilder::default().build().unwrap()),
            }
        }
    }
}
