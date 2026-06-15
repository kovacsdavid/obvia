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

use crate::common::error::FormErrorResponse;
use crate::common::value_object::{ValueObjectError, ValueObjectRequired};
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::model::Tenant;
use crate::manager::tenants::types::Name;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CreateTenantHelper {
    pub name: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateTenantError {
    pub name: Option<String>,
}

impl CreateTenantError {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
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
    fn from(_: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CreateTenant {
    pub name: ValueObjectRequired<Name>,
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

        if error.is_empty() {
            Ok(CreateTenant { name: name? })
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
}

impl From<Tenant> for PublicTenant {
    fn from(value: Tenant) -> Self {
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
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
pub struct TenantIdRequest {
    pub uuid: Uuid,
}

#[derive(Serialize, Debug)]
pub struct NewTokenResponse {
    pub token: String,
    pub claims: Claims,
}
