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
use crate::common::types::quantity::Quantity;
use crate::common::types::{ValueObject, ValueObjectable};
use crate::tenant::inventory_reservations::types::{
    InventoryReferenceType, InventoryReservationsReservedUntil, InventoryReservationsStatus,
};
use crate::validate_optional_string;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InventoryReservationUserInputHelper {
    pub id: Option<String>,
    pub inventory_id: Uuid,
    pub quantity: String,
    pub reference_type: String,
    pub reference_id: String,
    pub reserved_until: String,
    pub status: String,
}

#[derive(Debug, Serialize, Default)]
pub struct InventoryReservationUserInputError {
    pub id: Option<String>,
    pub inventory_id: Option<String>,
    pub quantity: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub reserved_until: Option<String>,
    pub status: Option<String>,
}

impl InventoryReservationUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.inventory_id.is_none()
            && self.quantity.is_none()
            && self.reference_type.is_none()
            && self.reference_id.is_none()
            && self.reserved_until.is_none()
            && self.status.is_none()
    }
}

impl Display for InventoryReservationUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateInventoryReservationError: {}", json),
            Err(e) => write!(f, "CreateInventoryReservationError: {}", e),
        }
    }
}

impl FormErrorResponse for InventoryReservationUserInputError {}

impl IntoResponse for InventoryReservationUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReservationUserInput {
    pub id: Option<Uuid>,
    pub inventory_id: Uuid,
    pub quantity: ValueObject<Quantity>,
    pub reference_type: Option<ValueObject<InventoryReferenceType>>,
    pub reference_id: Option<Uuid>,
    pub reserved_until: Option<ValueObject<InventoryReservationsReservedUntil>>,
    pub status: ValueObject<InventoryReservationsStatus>,
}

impl TryFrom<InventoryReservationUserInputHelper> for InventoryReservationUserInput {
    type Error = InventoryReservationUserInputError;
    fn try_from(value: InventoryReservationUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = InventoryReservationUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|_| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let quantity = ValueObject::new(Quantity(value.quantity))
            .inspect_err(|e| error.quantity = Some(e.to_string()));

        let reference_id = Uuid::parse_str(&value.reference_id)
            .inspect_err(|_| {
                error.reference_id = Some("Hibás hivatkozás azonosító".to_string());
            })
            .ok();

        let reference_type = validate_optional_string!(
            InventoryReferenceType(value.reference_type),
            error.reference_type
        );

        let reserved_until = validate_optional_string!(
            InventoryReservationsReservedUntil(value.reserved_until),
            error.reserved_until
        );

        let status = ValueObject::new(InventoryReservationsStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        if error.is_empty() {
            Ok(InventoryReservationUserInput {
                id,
                inventory_id: value.inventory_id,
                quantity: quantity.map_err(|_| InventoryReservationUserInputError::default())?,
                reference_type,
                reference_id,
                reserved_until,
                status: status.map_err(|_| InventoryReservationUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
