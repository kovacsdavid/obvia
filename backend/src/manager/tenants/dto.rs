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

use crate::common::config::database_config::TenantDatabaseConfig;
use crate::common::error::FormErrorResponse;
use crate::common::types::{DbHost, DbName, DbPassword, DbPort, DbUser};
use crate::common::value_object::{ValueObjectError, ValueObjectRequired};
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::model::Tenant;
use crate::manager::tenants::types::Name;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CreateTenantHelper {
    pub name: String,
    pub is_self_hosted: bool,
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateTenantError {
    pub name: Option<String>,
    pub is_self_hosted: Option<String>,
    pub db_host: Option<String>,
    pub db_port: Option<String>,
    pub db_name: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
}

impl CreateTenantError {
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

impl Display for CreateTenantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateTenantError: {}", json),
            Err(e) => write!(f, "CreateTenantError: {}", e),
        }
    }
}

impl FormErrorResponse for CreateTenantError {}

impl IntoResponse for CreateTenantError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

impl From<ValueObjectError> for CreateTenantError {
    fn from(value: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Error)]
pub enum DatabaseConfigError {
    #[error("{0} is missing")]
    MissingField(&'static str),

    #[error("ValueObjectError: {0}")]
    ValueObejctError(#[from] ValueObjectError),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CreateTenant {
    pub name: ValueObjectRequired<Name>,
    pub is_self_hosted: bool,
    pub db_host: Option<ValueObjectRequired<DbHost>>,
    pub db_port: Option<ValueObjectRequired<DbPort>>,
    pub db_name: Option<ValueObjectRequired<DbName>>,
    pub db_user: Option<ValueObjectRequired<DbUser>>,
    pub db_password: Option<ValueObjectRequired<DbPassword>>,
}

impl CreateTenant {
    pub fn is_self_hosted(&self) -> bool {
        self.is_self_hosted
    }
}

impl TryInto<TenantDatabaseConfig> for CreateTenant {
    type Error = DatabaseConfigError;
    fn try_into(self) -> Result<TenantDatabaseConfig, Self::Error> {
        Ok(TenantDatabaseConfig {
            host: self
                .db_host
                .ok_or(DatabaseConfigError::MissingField("db_host"))?,
            port: self
                .db_port
                .ok_or(DatabaseConfigError::MissingField("db_port"))?,
            username: self
                .db_user
                .ok_or(DatabaseConfigError::MissingField("db_user"))?,
            password: self
                .db_password
                .ok_or(DatabaseConfigError::MissingField("db_password"))?,
            database: self
                .db_name
                .ok_or(DatabaseConfigError::MissingField("db_name"))?,
            max_pool_size: None,
            ssl_mode: None,
        })
    }
}

impl TryFrom<CreateTenantHelper> for CreateTenant {
    type Error = CreateTenantError;
    fn try_from(value: CreateTenantHelper) -> Result<Self, Self::Error> {
        let mut error = CreateTenantError::default();

        let name = value
            .name
            .parse::<ValueObjectRequired<Name>>()
            .inspect_err(|e| {
                error.name = Some(e.to_string());
            });

        let mut db_host = Ok(None);
        let mut db_port = Ok(None);
        let mut db_name = Ok(None);
        let mut db_user = Ok(None);
        let mut db_password = Ok(None);

        if value.is_self_hosted {
            db_host = value
                .db_host
                .parse::<ValueObjectRequired<DbHost>>()
                .inspect_err(|e| {
                    error.db_host = Some(e.to_string());
                })
                .map(Some);
            db_port = value
                .db_port
                .parse::<ValueObjectRequired<DbPort>>()
                .inspect_err(|e| {
                    error.db_port = Some(e.to_string());
                })
                .map(Some);
            db_name = value
                .db_name
                .parse::<ValueObjectRequired<DbName>>()
                .inspect_err(|e| {
                    error.db_name = Some(e.to_string());
                })
                .map(Some);
            db_user = value
                .db_user
                .parse::<ValueObjectRequired<DbUser>>()
                .inspect_err(|e| {
                    error.db_user = Some(e.to_string());
                })
                .map(Some);
            db_password = value
                .db_password
                .parse::<ValueObjectRequired<DbPassword>>()
                .inspect_err(|e| {
                    error.db_password = Some(e.to_string());
                })
                .map(Some);
        }
        if error.is_empty() {
            Ok(CreateTenant {
                name: name?,
                is_self_hosted: value.is_self_hosted,
                db_host: db_host?,
                db_port: db_port?,
                db_name: db_name?,
                db_user: db_user?,
                db_password: db_password?,
            })
        } else {
            Err(error)
        }
    }
}

#[allow(dead_code)]
pub struct UserTenantConnect {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
    pub invited_by: Option<Uuid>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum PublicTenant {
    Managed(PublicTenantManaged),
    SelfHosted(PublicTenantSelfHosted),
}

impl From<Tenant> for PublicTenant {
    fn from(value: Tenant) -> Self {
        if value.is_self_hosted() {
            Self::SelfHosted(PublicTenantSelfHosted {
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
            })
        } else {
            Self::Managed(PublicTenantManaged {
                id: value.id,
                name: value.name,
                is_self_hosted: value.is_self_hosted,
                db_host: "[MANAGED]".to_string(),
                db_port: 0,
                db_name: "[MANAGED]".to_string(),
                db_user: "[MANAGED]".to_string(),
                db_password: "[REDACTED]".to_string(),
                db_max_pool_size: value.db_max_pool_size,
                db_ssl_mode: value.db_ssl_mode,
                created_at: value.created_at,
                updated_at: value.updated_at,
                deleted_at: value.deleted_at,
            })
        }
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct PublicTenantSelfHosted {
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

impl From<Tenant> for PublicTenantSelfHosted {
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

#[derive(Serialize, Debug, Clone, Default)]
pub struct PublicTenantManaged {
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

impl From<Tenant> for PublicTenantManaged {
    fn from(value: Tenant) -> Self {
        Self {
            id: value.id,
            name: value.name,
            is_self_hosted: value.is_self_hosted,
            db_host: "[MANAGED]".to_string(),
            db_port: 0,
            db_name: "[MANAGED]".to_string(),
            db_user: "[MANAGED]".to_string(),
            db_password: "[REDACTED]".to_string(),
            db_max_pool_size: value.db_max_pool_size,
            db_ssl_mode: value.db_ssl_mode,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TenantActivateRequest {
    pub new_tenant_id: Uuid,
}

#[derive(Serialize, Debug)]
pub struct NewTokenResponse {
    pub token: String,
    pub claims: Claims,
}
