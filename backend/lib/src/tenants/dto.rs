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
use crate::app::config::TenantDatabaseConfig;
use crate::common::dto::{ErrorBody, ErrorResponse, QueryParam};
use crate::common::types::value_object::ValueObject;
use crate::tenants::model::Tenant;
use crate::tenants::types::{DbHost, DbName, DbPassword, DbPort, DbUser, Name};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A structure that represents a helper for creating a request with optional database configuration details.
///
/// # Fields
///
/// * `name` - A mandatory field that specifies the name of the request helper. This field is represented as a `String`.
///
/// * `db_host` - An optional field that specifies the hostname or IP address of the database. This field is represented as an `Option<String>`.
///
/// * `db_port` - An optional field that specifies the port number of the database. This field is represented as an `Option<i32>`.
///
/// * `db_name` - An optional field that specifies the name of the database. This field is represented as an `Option<String>`.
///
/// * `db_user` - An optional field that specifies the username for database authentication. This field is represented as an `Option<String>`.
///
/// * `db_password` - An optional field that specifies the password for database authentication. This field is represented as an `Option<String>`.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TenantCreateRequestHelper {
    pub name: String,
    pub is_self_hosted: bool,
    pub db_host: Option<String>,
    pub db_port: Option<i32>,
    pub db_name: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
}

/// This struct contains optional fields that represent the potential issues or missing information
/// related to tenant creation
///
/// # Fields
///
/// * `name` - An optional string representing an error related to the name field of the request.
/// * `db_host` - An optional string indicating an error related to the database host.
/// * `db_port` - An optional string indicating an error related to the database port.
/// * `db_name` - An optional string representing an issue with the database name.
/// * `db_user` - An optional string indicating an error with the database user credentials or configuration.
/// * `db_password` - An optional string representing an issue with the database password credentials.
///
/// # Usage
///
/// This struct is typically used to encapsulate errors during creation requests of a tenant
/// and to provide detailed feedback about specific fields that may have encountered an issue.
#[derive(Debug, Serialize)]
pub struct TenantCreateRequestError {
    pub name: Option<String>,
    pub is_self_hosted: Option<String>,
    pub db_host: Option<String>,
    pub db_port: Option<String>,
    pub db_name: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
}

impl TenantCreateRequestError {
    /// Checks if the current instance of the struct is empty.
    ///
    ///
    /// # Returns
    /// * `true` - If all fields are `None`.
    /// * `false` - If at least one field has a value.
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.is_self_hosted.is_none()
            && self.db_host.is_none()
            && self.db_port.is_none()
            && self.db_name.is_none()
            && self.db_user.is_none()
            && self.db_password.is_none()
    }
}

impl IntoResponse for TenantCreateRequestError {
    /// Converts the given error details into an HTTP response.
    ///
    /// This function constructs a response with a status code of `422 Unprocessable Entity`
    /// and a JSON body containing error details. It utilizes the `ErrorResponse` and `ErrorBody`
    /// types to format the error information in a structured way.
    ///
    /// # Returns
    ///
    /// A `Response` object representing the error response.
    ///
    /// The response body includes:
    /// - A reference string indicating the context of the error (`"ORGANIZATIONAL_UNITS/DTO/CREATE"`).
    /// - A generic global error message (`"Kérjük, ellenőrizze a hibás mezőket"`),
    ///   which translates to "Please check the incorrect fields".
    /// - A `fields` object containing specific details of the error.
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                reference: String::from("ORGANIZATIONAL_UNITS/DTO/CREATE"),
                global: String::from("Kérjük, ellenőrizze a hibás mezőket"),
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

/// A structure representing a request for creating a tenant resource.
///
/// # Fields
/// - `name`: The name of the tenant to be created. This field is mandatory and must be provided during initialization.
/// - `db_host`: An optional field specifying the hostname or IP address of the database server.
/// - `db_port`: An optional field representing the port number for connecting to the database.
/// - `db_name`: An optional field specifying the name of the database.
/// - `db_user`: An optional field for the username required to connect to the database.
/// - `db_password`: An optional field for providing the password required for authentication when connecting to the database.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TenantCreateRequest {
    pub name: ValueObject<Name>,
    pub is_self_hosted: bool,
    pub db_host: Option<ValueObject<DbHost>>,
    pub db_port: Option<ValueObject<DbPort>>,
    pub db_name: Option<ValueObject<DbName>>,
    pub db_user: Option<ValueObject<DbUser>>,
    pub db_password: Option<ValueObject<DbPassword>>,
}

impl TenantCreateRequest {
    /// Checks if the instance is self-hosted.
    ///
    /// This method returns the value of the `is_self_hosted` field, indicating whether
    /// the instance is self-hosted or not.
    ///
    /// # Returns
    /// * `true` - If the instance is self-hosted.
    /// * `false` - If the instance is not self-hosted.
    pub fn is_self_hosted(&self) -> bool {
        self.is_self_hosted
    }
}

impl TryInto<TenantDatabaseConfig> for TenantCreateRequest {
    type Error = String;
    fn try_into(self) -> Result<TenantDatabaseConfig, Self::Error> {
        Ok(TenantDatabaseConfig {
            host: self
                .db_host
                .ok_or_else(|| "db_host is missing".to_string())?,
            port: self
                .db_port
                .ok_or_else(|| "db_port is missing".to_string())?,
            username: self
                .db_user
                .ok_or_else(|| "db_user is missing".to_string())?,
            password: self
                .db_password
                .ok_or_else(|| "db_password is missing".to_string())?,
            database: self
                .db_name
                .ok_or_else(|| "db_name is missing".to_string())?,
            max_pool_size: None,
            ssl_mode: None,
        })
    }
}

impl TryFrom<TenantCreateRequestHelper> for TenantCreateRequest {
    type Error = TenantCreateRequestError;
    // TODO: new docs
    fn try_from(value: TenantCreateRequestHelper) -> Result<Self, Self::Error> {
        let mut error = TenantCreateRequestError {
            name: None,
            is_self_hosted: None,
            db_host: None,
            db_port: None,
            db_name: None,
            db_user: None,
            db_password: None,
        };

        let name = ValueObject::new(Name(value.name));
        let mut db_host: Option<ValueObject<DbHost>> = None;
        let mut db_port: Option<ValueObject<DbPort>> = None;
        let mut db_name: Option<ValueObject<DbName>> = None;
        let mut db_user: Option<ValueObject<DbUser>> = None;
        let mut db_password: Option<ValueObject<DbPassword>> = None;

        if let Err(e) = &name {
            error.name = Some(e.to_string());
        }

        if value.is_self_hosted {
            const REQUIRED_IF_SELF_HOSTED_ERROR: &str =
                "A mező kitöltése kötelező, ha saját adatbázist üzemeltet";
            match &value.db_host {
                Some(val) => {
                    db_host = match ValueObject::new(DbHost(val.clone())) {
                        Ok(db_host) => Some(db_host),
                        Err(e) => {
                            error.db_host = Some(e.to_string());
                            None
                        }
                    }
                }
                None => {
                    error.db_host = Some(String::from(REQUIRED_IF_SELF_HOSTED_ERROR));
                }
            }
            match value.db_port {
                Some(val) => {
                    db_port = match ValueObject::new(DbPort(val as i64)) {
                        Ok(db_port) => Some(db_port),
                        Err(e) => {
                            error.db_port = Some(e.to_string());
                            None
                        }
                    }
                }
                None => {
                    error.db_port = Some(String::from(REQUIRED_IF_SELF_HOSTED_ERROR));
                }
            }
            match &value.db_name {
                Some(val) => {
                    db_name = match ValueObject::new(DbName(val.clone())) {
                        Ok(db_name) => Some(db_name),
                        Err(e) => {
                            error.db_name = Some(e.to_string());
                            None
                        }
                    }
                }
                None => {
                    error.db_name = Some(String::from(REQUIRED_IF_SELF_HOSTED_ERROR));
                }
            }
            match &value.db_user {
                Some(val) => {
                    db_user = match ValueObject::new(DbUser(val.clone())) {
                        Ok(db_user) => Some(db_user),
                        Err(e) => {
                            error.db_user = Some(e.to_string());
                            None
                        }
                    }
                }
                None => {
                    error.db_user = Some(String::from(REQUIRED_IF_SELF_HOSTED_ERROR));
                }
            }
            match &value.db_password {
                Some(val) => {
                    db_password = match ValueObject::new(DbPassword(val.clone())) {
                        Ok(db_password) => Some(db_password),
                        Err(e) => {
                            error.db_password = Some(e.to_string());
                            None
                        }
                    }
                }
                None => {
                    error.db_password = Some(String::from(REQUIRED_IF_SELF_HOSTED_ERROR));
                }
            }
        }
        if error.is_empty() {
            Ok(TenantCreateRequest {
                name: name.unwrap(),
                is_self_hosted: value.is_self_hosted,
                db_host,
                db_port,
                db_name,
                db_user,
                db_password,
            })
        } else {
            Err(error)
        }
    }
}

/// This struct is for creating connection between users and tenants
///
/// Fields:
/// - `user_id` (`Uuid`): The unique identifier for the user.
/// - `tenant_id` (`Uuid`): The unique identifier for the tenant the user is connected to.
/// - `role` (`String`): The role of the user in the tenant. Examples of roles could include "admin", "member", or other custom-defined roles.
/// - `invited_by` (`Option<Uuid>`): The unique identifier of the user who invited this user to the tenant, if applicable. This field is optional and may be `None` if the user created the tenant.
#[allow(dead_code)]
pub struct UserTenantConnect {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
    pub invited_by: Option<Uuid>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct PublicTenant {
    pub id: Uuid,
    pub name: String,
    pub is_self_hosted: bool,
    pub db_host: String,
    pub db_port: i32,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_max_pool_size: i32,
    pub db_ssl_mode: String,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
    pub deleted_at: Option<chrono::DateTime<chrono::Local>>,
}

impl From<Tenant> for PublicTenant {
    fn from(value: Tenant) -> Self {
        Self {
            id: value.id,
            name: value.name,
            is_self_hosted: value.is_self_hosted,
            db_host: value.db_host,
            db_port: value.db_port,
            db_name: value.db_name,
            db_user: value.db_user,
            db_password: "[REDACTED]".to_string(),
            db_max_pool_size: value.db_max_pool_size,
            db_ssl_mode: value.db_ssl_mode,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

pub struct FilteringParams {
    pub name: Option<String>,
}

impl From<&QueryParam> for FilteringParams {
    fn from(value: &QueryParam) -> Self {
        match value.as_hash_map() {
            None => Self { name: None },
            Some(hmap) => {
                let name = match hmap.get("name").cloned() {
                    None => None,
                    Some(name) => {
                        if !name.trim().is_empty() {
                            Some(format!("%{name}%"))
                        } else {
                            None
                        }
                    }
                };
                Self { name }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TenantActivateRequest {
    pub tenant_id: Uuid,
}
