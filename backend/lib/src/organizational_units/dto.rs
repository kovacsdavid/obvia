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
use crate::common::dto::{ErrorBody, ErrorResponse};
use crate::common::types::tenant::db_host::DbHost;
use crate::common::types::tenant::db_name::DbName;
use crate::common::types::tenant::db_password::DbPassword;
use crate::common::types::tenant::db_port::DbPort;
use crate::common::types::tenant::db_user::DbUser;
use crate::common::types::tenant::name::Name;
use crate::common::types::value_object::ValueObject;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CreateRequestHelper {
    pub name: String,
    pub db_self_hosted: bool,
    pub db_host: Option<String>,
    pub db_port: Option<i32>,
    pub db_name: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
}

/// This struct contains optional fields that represent the potential issues or missing information
/// related to organizational_unit creation
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
/// This struct is typically used to encapsulate errors during creation requests of an organizational_unit
/// and to provide detailed feedback about specific fields that may have encountered an issue.
#[derive(Debug, Serialize)]
pub struct CreateRequestError {
    pub name: Option<String>,
    pub db_self_hosted: Option<String>,
    pub db_host: Option<String>,
    pub db_port: Option<String>,
    pub db_name: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
}

impl IntoResponse for CreateRequestError {
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

/// A structure representing a request for creating an organizational_unit resource.
///
/// # Fields
/// - `name`: The name of the organizational_unit to be created. This field is mandatory and must be provided during initialization.
/// - `db_host`: An optional field specifying the hostname or IP address of the database server.
/// - `db_port`: An optional field representing the port number for connecting to the database.
/// - `db_name`: An optional field specifying the name of the database.
/// - `db_user`: An optional field for the username required to connect to the database.
/// - `db_password`: An optional field for providing the password required for authentication when connecting to the database.
#[allow(dead_code)]
pub struct CreateRequest {
    pub name: Name,
    pub db_self_hosted: bool,
    pub db_host: Option<ValueObject<DbHost>>,
    pub db_port: Option<DbPort>,
    pub db_name: Option<ValueObject<DbName>>,
    pub db_user: Option<DbUser>,
    pub db_password: Option<DbPassword>,
}

impl TryFrom<CreateRequestHelper> for CreateRequest {
    type Error = CreateRequestError;
    // TODO: new docs
    fn try_from(value: CreateRequestHelper) -> Result<Self, Self::Error> {
        let mut error = CreateRequestError {
            name: None,
            db_self_hosted: None,
            db_host: None,
            db_port: None,
            db_name: None,
            db_user: None,
            db_password: None,
        };

        let name = Name::try_from(value.name);
        let mut db_host: Option<ValueObject<DbHost>> = None;
        let mut db_port: Option<DbPort> = None;
        let mut db_name: Option<ValueObject<DbName>> = None;
        let mut db_user: Option<DbUser> = None;
        let mut db_password: Option<DbPassword> = None;

        if let Err(e) = &name {
            error.name = Some(e.to_string());
        }

        if value.db_self_hosted {
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
                    db_port = match DbPort::try_from(val) {
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
                    db_user = match DbUser::from_str(val) {
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
                    db_password = match DbPassword::from_str(val) {
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

        // TODO: if err
        Ok(CreateRequest {
            name: name.unwrap(),
            db_self_hosted: value.db_self_hosted,
            db_host,
            db_port,
            db_name,
            db_user,
            db_password,
        })
    }
}

/// This struct is for creating connection between users and organizational units
///
/// Fields:
/// - `user_id` (`Uuid`): The unique identifier for the user.
/// - `organizational_unit_id` (`Uuid`): The unique identifier for the organizational unit the user is connected to.
/// - `role` (`String`): The role of the user in the organizational unit. Examples of roles could include "admin", "member", or other custom-defined roles.
/// - `invited_by` (`Option<Uuid>`): The unique identifier of the user who invited this user to the organizational unit, if applicable. This field is optional and may be `None` if the user created the organizational unit.
#[allow(dead_code)]
pub struct UserOrganizationalUnitConnect {
    pub user_id: Uuid,
    pub organizational_unit_id: Uuid,
    pub role: String,
    pub invited_by: Option<Uuid>,
}
