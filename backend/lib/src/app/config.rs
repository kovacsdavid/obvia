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

use crate::common::types::tenant::db_host::DbHost;
use crate::common::types::tenant::db_name::DbName;
use crate::common::types::tenant::db_password::DbPassword;
use crate::common::types::tenant::db_port::DbPort;
use crate::common::types::tenant::db_user::DbUser;
use crate::organizational_units::model::OrganizationalUnit;
use serde::Deserialize;
#[cfg(test)]
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
    main_database: MainDatabaseConfig,
    default_tenant_database: DefaultTenantDatabaseConfig,
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

/// Represents the configuration settings required to connect to the main database.
///
/// This struct is intended to be deserialized from a configuration file or environment variables.
/// It provides detailed options for specifying database connection properties.
///
/// # Fields
///
/// * `host` - The hostname or IP address of the database server.
/// * `port` - The port number on which the database server is running.
/// * `username` - The username required for authentication with the database.
/// * `password` - The password required for authentication with the database.
/// * `database` - The name of the specific database to connect to.
/// * `pool_size` - The maximum size of the connection pool for managing database connections.
#[derive(Debug, Clone, Deserialize)]
pub struct MainDatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub pool_size: u32,
}

/// Represents the configuration settings for the default tenant database server.
///
/// This structure holds the necessary parameters to connect to the default tenant database,
/// including connection details and pool size settings.
///
/// # Fields
/// - `host` (`String`): The hostname or IP address of the database server.
/// - `port` (`u16`): The port on which the database server is listening.
/// - `username` (`String`): The username used to authenticate with the database.
/// - `password` (`String`): The password used to authenticate with the database.
/// - `database` (`String`): The name of the specific database to connect to.
/// - `pool_size` (`u32`): The size of the database connection pool.
#[derive(Debug, Clone, Deserialize)]
pub struct DefaultTenantDatabaseConfig {
    pub host: DbHost,
    pub port: DbPort,
    pub username: DbUser,
    pub password: DbPassword,
    pub database: DbName,
    pub pool_size: u32,
}

/// A structure representing the configuration details required to connect to a tenant-specific database.
///
/// This configuration includes details such as the database host, port, authentication credentials,
/// database name, and connection pool size.
///
/// # Fields
///
/// * `host` - The hostname of the database server (e.g., "localhost" or "db.example.com").
/// * `port` - The port on which the database is running. Typically, 5432 for PostgreSQL.
/// * `username` - The username required for authenticating with the database server.
/// * `password` - The password corresponding to the `username` for database authentication.
/// * `database` - The name of the specific database to connect to.
/// * `pool_size` - The maximum number of connections allowed in the connection pool.
#[derive(Debug, Clone)]
pub struct TenantDatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub pool_size: u32,
}

impl From<OrganizationalUnit> for TenantDatabaseConfig {
    /// Converts an `OrganizationalUnit` struct into a new instance of the implementing type.
    ///
    /// # Parameters
    /// - `value`: An instance of the `OrganizationalUnit` struct containing database configuration details.
    ///
    /// # Returns
    /// A new instance of the implementing type, populated using the properties of the given `OrganizationalUnit`.
    ///
    /// # Field Mappings
    /// - `host`: Mapped directly from `value.db_host`.
    /// - `port`: Converted from `value.db_port` to `u16`.
    /// - `username`: Constructed by prefixing `"tenant_"` to `value.db_user`, with all `"-"` characters removed.
    /// - `password`: Mapped directly from `value.db_password`.
    /// - `database`: Constructed by prefixing `"tenant_"` to `value.db_name`, with all `"-"` characters removed.
    /// - `pool_size`: Converted from `value.db_max_pool_size` to `u32`.
    fn from(value: OrganizationalUnit) -> Self {
        Self {
            host: value.db_host,
            port: value.db_port as u16,
            username: format!("tenant_{}", value.db_user.replace("-", "")),
            password: value.db_password,
            database: format!("tenant_{}", value.db_name.replace("-", "")),
            pool_size: value.db_max_pool_size as u32,
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

impl Default for MainDatabaseConfig {
    /// Provides a default configuration for the `MainDatabaseConfig` struct.
    ///
    /// # Returns
    /// A `MainDatabaseConfig` instance initialized with the following default values:
    /// - `host`: `"localhost"`
    /// - `port`: `5432`
    /// - `username`: `"user"`
    /// - `password`: `"password"`
    /// - `database`: `"database"`
    /// - `pool_size`: `5`
    ///
    /// These default values are used for local development or testing scenarios.
    fn default() -> Self {
        MainDatabaseConfig {
            host: String::from("localhost"),
            port: 5432,
            username: String::from("user"),
            password: String::from("password"),
            database: String::from("database"),
            pool_size: 5,
        }
    }
}

#[cfg(test)]
impl Default for DefaultTenantDatabaseConfig {
    /// Provides a default implementation for the `DefaultTenantDatabaseConfig` struct.
    ///
    /// This implementation returns a struct initialized with default values:
    ///
    /// - `host`: `"localhost"` - The default hostname of the database server.
    /// - `port`: `5432` - The default port for the database connection.
    /// - `username`: `"user"` - The default username for the database connection.
    /// - `password`: `"password"` - The default password for the database connection.
    /// - `database`: `"database"` - The default name of the database.
    /// - `pool_size`: `5` - The default size of the connection pool.
    ///
    /// These default values are used for local development or testing scenarios.
    fn default() -> Self {
        DefaultTenantDatabaseConfig {
            host: DbHost::from_str("example.com").unwrap(),
            port: DbPort::try_from(5432).unwrap(),
            username: DbUser::from_str("user").unwrap(),
            password: DbPassword::from_str("on2GRECh3DR0zDRU66pplY11hsDZ3Z53Lh43hVxD").unwrap(),
            database: DbName::from_str("database").unwrap(),
            pool_size: 5,
        }
    }
}
impl Default for TenantDatabaseConfig {
    /// Provides a default implementation for the `TenantDatabaseConfig` struct.
    ///
    /// This implementation returns a struct initialized with default values:
    ///
    /// - `host`: `"localhost"` - The default hostname of the database server.
    /// - `port`: `5432` - The default port for the database connection.
    /// - `username`: `"user"` - The default username for the database connection.
    /// - `password`: `"password"` - The default password for the database connection.
    /// - `database`: `"database"` - The default name of the database.
    /// - `pool_size`: `5` - The default size of the connection pool.
    ///
    /// These default values are used for local development or testing scenarios.
    fn default() -> Self {
        TenantDatabaseConfig {
            host: String::from("localhost"),
            port: 5432,
            username: String::from("user"),
            password: String::from("password"),
            database: String::from("database"),
            pool_size: 5,
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
    pub fn main_database(&self) -> &MainDatabaseConfig {
        &self.main_database
    }

    /// Returns a reference to the `DefaultTenantDatabaseConfig` instance.
    pub fn default_tenant_database(&self) -> &DefaultTenantDatabaseConfig {
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

impl MainDatabaseConfig {
    /// Constructs a PostgreSQL URL string using the provided connection parameters.
    ///
    /// This method formats a connection string in the following format:
    /// `postgres://username:password@host:port/database`.
    ///
    /// # Returns
    ///
    /// A `String` containing the constructed PostgreSQL database connection URL.
    ///
    /// # Note / Safety
    ///
    /// Ensure there are no invalid characters in the fields!
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
    /// Returns the pool_size.
    pub fn pool_size(&self) -> u32 {
        self.pool_size
    }
}

impl DefaultTenantDatabaseConfig {
    /// Constructs a PostgreSQL URL string using the provided connection parameters.
    ///
    /// This method formats a connection string in the following format:
    /// `postgres://username:password@host:port/database`.
    ///
    /// # Returns
    ///
    /// A `String` containing the constructed PostgreSQL database connection URL.
    ///
    /// # Note / Safety
    ///
    /// Ensure there are no invalid characters in the fields!
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    /// Returns the pool_size.
    pub fn pool_size(&self) -> u32 {
        self.pool_size
    }
}

impl TenantDatabaseConfig {
    /// Constructs a PostgreSQL URL string using the provided connection parameters.
    ///
    /// This method formats a connection string in the following format:
    /// `postgres://username:password@host:port/database`.
    ///
    /// # Returns
    ///
    /// A `String` containing the constructed PostgreSQL database connection URL.
    ///
    /// # Note / Safety
    ///
    /// Ensure there are no invalid characters in the fields!
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
    /// Returns the pool_size.
    pub fn pool_size(&self) -> u32 {
        self.pool_size
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
