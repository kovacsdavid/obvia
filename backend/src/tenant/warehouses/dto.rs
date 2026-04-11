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
use crate::common::types::UuidVO;
use crate::common::value_object::{ValueObjectError, ValueObjectOptional, ValueObjectRequired};
use crate::tenant::warehouses::types::warehouse::{
    WarehouseContactName, WarehouseContactPhone, WarehouseName, WarehouseStatus,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct WarehouseUserInputHelper {
    pub id: Option<String>,
    pub name: String,
    pub contact_name: String,
    pub contact_phone: String,
    pub status: String,
}

#[derive(Debug, Serialize, Default)]
pub struct WarehouseUserInputError {
    pub id: Option<String>,
    pub name: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub status: Option<String>,
}

impl WarehouseUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.name.is_none()
            && self.contact_name.is_none()
            && self.contact_phone.is_none()
            && self.status.is_none()
    }
}

impl Display for WarehouseUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateWarehouseError: {}", json),
            Err(e) => write!(f, "CreateWarehouseError: {}", e),
        }
    }
}

impl FormErrorResponse for WarehouseUserInputError {}

impl IntoResponse for WarehouseUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

impl From<ValueObjectError> for WarehouseUserInputError {
    fn from(_: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct WarehouseUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub name: ValueObjectRequired<WarehouseName>,
    pub contact_name: ValueObjectOptional<WarehouseContactName>,
    pub contact_phone: ValueObjectOptional<WarehouseContactPhone>,
    pub status: ValueObjectRequired<WarehouseStatus>,
}

impl TryFrom<WarehouseUserInputHelper> for WarehouseUserInput {
    type Error = WarehouseUserInputError;
    fn try_from(value: WarehouseUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = WarehouseUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let name = value
            .name
            .parse::<ValueObjectRequired<WarehouseName>>()
            .inspect_err(|e| {
                error.name = Some(e.to_string());
            });

        let contact_name = value
            .contact_name
            .parse::<ValueObjectOptional<WarehouseContactName>>()
            .inspect_err(|e| {
                error.contact_name = Some(e.to_string());
            });

        let contact_phone = value
            .contact_phone
            .parse::<ValueObjectOptional<WarehouseContactPhone>>()
            .inspect_err(|e| {
                error.contact_phone = Some(e.to_string());
            });

        let status = value
            .status
            .parse::<ValueObjectRequired<WarehouseStatus>>()
            .inspect_err(|e| {
                error.status = Some(e.to_string());
            });

        if error.is_empty() {
            Ok(WarehouseUserInput {
                id: id?,
                name: name?,
                contact_name: contact_name?,
                contact_phone: contact_phone?,
                status: status?,
            })
        } else {
            Err(error)
        }
    }
}
