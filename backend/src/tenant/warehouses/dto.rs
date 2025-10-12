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
use crate::common::types::value_object::{ValueObject, ValueObjectable};
use crate::tenant::warehouses::types::warehouse::{
    WarehouseContactName, WarehouseContactPhone, WarehouseName, WarehouseStatus,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseUserInput {
    pub id: Option<Uuid>,
    pub name: ValueObject<WarehouseName>,
    pub contact_name: Option<ValueObject<WarehouseContactName>>,
    pub contact_phone: Option<ValueObject<WarehouseContactPhone>>,
    pub status: ValueObject<WarehouseStatus>, // Will default to true if not provided
}

impl TryFrom<WarehouseUserInputHelper> for WarehouseUserInput {
    type Error = WarehouseUserInputError;
    fn try_from(value: WarehouseUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = WarehouseUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|e| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let name = ValueObject::new(WarehouseName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });
        let contact_name = match ValueObject::new(WarehouseContactName(value.contact_name))
            .inspect_err(|e| {
                error.contact_name = Some(e.to_string());
            }) {
            Ok(val) => {
                if !val.extract().get_value().trim().is_empty() {
                    Some(val)
                } else {
                    None
                }
            }
            Err(_) => None,
        };
        let contact_phone = match ValueObject::new(WarehouseContactPhone(value.contact_phone))
            .inspect_err(|e| {
                error.contact_phone = Some(e.to_string());
            }) {
            Ok(val) => {
                if !val.extract().get_value().trim().is_empty() {
                    Some(val)
                } else {
                    None
                }
            }
            Err(_) => None,
        };
        let status = ValueObject::new(WarehouseStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        if error.is_empty() {
            Ok(WarehouseUserInput {
                id,
                name: name.map_err(|_| WarehouseUserInputError::default())?,
                contact_name,
                contact_phone,
                status: status.map_err(|_| WarehouseUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
