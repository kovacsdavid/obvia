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
use crate::manager::common::types::value_object::ValueObject;
use crate::tenant::inventory::types::inventory::InventoryQuantity;
use crate::tenant::inventory::types::inventory::quantity::Quantity;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateInventoryHelper {
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: String,
}

#[derive(Debug, Serialize)]
pub struct CreateInventoryError {
    pub product_id: Option<String>,
    pub warehouse_id: Option<String>,
    pub quantity: Option<String>,
}

impl CreateInventoryError {
    pub fn is_empty(&self) -> bool {
        self.product_id.is_none() && self.warehouse_id.is_none() && self.quantity.is_none()
    }
}

impl IntoResponse for CreateInventoryError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                reference: String::from("INVENTORY/DTO/CREATE"),
                global: String::from("Kérjük, ellenőrizze a hibás mezőket"),
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInventory {
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: ValueObject<Quantity>,
}

impl TryFrom<CreateInventoryHelper> for CreateInventory {
    type Error = CreateInventoryError;
    fn try_from(value: CreateInventoryHelper) -> Result<Self, Self::Error> {
        let mut error = CreateInventoryError {
            product_id: None,
            warehouse_id: None,
            quantity: None,
        };

        let quantity = ValueObject::new(InventoryQuantity(value.quantity));

        if let Err(e) = &quantity {
            error.quantity = Some(e.to_string());
        }

        if error.is_empty() {
            Ok(CreateInventory {
                product_id: value.product_id,
                warehouse_id: value.warehouse_id,
                quantity: quantity.unwrap(),
            })
        } else {
            Err(error)
        }
    }
}

pub struct UpdateInventoryHelper {
    // TODO: fields
}

pub struct UpdateInventoryError {
    // TODO: fields
}

impl UpdateInventoryError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateInventoryError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInventory {
    pub product_id: Option<Uuid>,
    pub warehouse_id: Option<Uuid>,
    pub quantity: ValueObject<Quantity>,
}

impl TryFrom<UpdateInventoryHelper> for UpdateInventory {
    type Error = UpdateInventoryError;
    fn try_from(value: UpdateInventoryHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}
