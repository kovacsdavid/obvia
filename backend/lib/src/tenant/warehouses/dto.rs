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
use crate::manager::common::dto::{ErrorBody, ErrorResponse};
use crate::manager::common::types::value_object::{ValueObject, ValueObjectable};
use crate::tenant::warehouses::types::warehouse::{
    WarehouseContactName, WarehouseContactPhone, WarehouseName, WarehouseStatus,
};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateWarehouseHelper {
    pub name: String,
    pub contact_name: String,
    pub contact_phone: String,
    pub status: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateWarehouseError {
    pub name: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub status: Option<String>,
}

impl CreateWarehouseError {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.contact_name.is_none()
            && self.contact_phone.is_none()
            && self.status.is_none()
    }
}

impl IntoResponse for CreateWarehouseError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                reference: String::from("WAREHOUSES/DTO/CREATE"),
                global: String::from("Kérjük, ellenőrizze a hibás mezőket"),
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWarehouse {
    pub name: ValueObject<WarehouseName>,
    pub contact_name: Option<ValueObject<WarehouseContactName>>,
    pub contact_phone: Option<ValueObject<WarehouseContactPhone>>,
    pub status: ValueObject<WarehouseStatus>, // Will default to true if not provided
}

impl TryFrom<CreateWarehouseHelper> for CreateWarehouse {
    type Error = CreateWarehouseError;
    fn try_from(value: CreateWarehouseHelper) -> Result<Self, Self::Error> {
        let mut error = CreateWarehouseError::default();

        let name = ValueObject::new(WarehouseName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });
        let contact_name = match ValueObject::new(WarehouseContactName(value.contact_name)).inspect_err(|e| {
            error.contact_name = Some(e.to_string());
        }) {
            Ok(val) => {
                if !val.extract().get_value().trim().is_empty() {
                    Some(val)
                } else {
                    None
                }
            },
            Err(_) => None
        };
        let contact_phone = match ValueObject::new(WarehouseContactPhone(value.contact_phone)).inspect_err(|e| {
            error.contact_phone = Some(e.to_string());
        }) {
            Ok(val) => {
                if !val.extract().get_value().trim().is_empty() {
                    Some(val)
                } else {
                    None
                }
            },
            Err(_) => None
        };
        let status = ValueObject::new(WarehouseStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        if error.is_empty() {
            Ok(CreateWarehouse {
                name: name.map_err(|_| CreateWarehouseError::default())?,
                contact_name,
                contact_phone,
                status: status.map_err(|_| CreateWarehouseError::default())?,
            })
        } else {
            Err(error)
        }
    }
}

pub struct UpdateWarehouseHelper {
    // TODO: fields
}

pub struct UpdateWarehouseError {
    // TODO: fields
}

impl UpdateWarehouseError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateWarehouseError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWarehouse {
    pub name: ValueObject<WarehouseName>,
    pub contact_name: ValueObject<WarehouseContactName>,
    pub contact_phone: ValueObject<WarehouseContactPhone>,
    pub is_active: bool,
}

impl TryFrom<UpdateWarehouseHelper> for UpdateWarehouse {
    type Error = UpdateWarehouseError;
    fn try_from(value: UpdateWarehouseHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}
