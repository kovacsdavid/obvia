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
use crate::common::types::quantity::Quantity;
use crate::common::value_object::{ValueObjectError, ValueObjectOptional, ValueObjectRequired};
use crate::tenant::inventory_reservations::types::{
    InventoryReferenceType, InventoryReservationsReservedUntil, InventoryReservationsStatus,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InventoryReservationUserInputHelper {
    pub id: Option<String>,
    pub inventory_id: String,
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

impl From<ValueObjectError> for InventoryReservationUserInputError {
    fn from(_: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct InventoryReservationUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub inventory_id: ValueObjectRequired<UuidVO>,
    pub quantity: ValueObjectRequired<Quantity>,
    pub reference_type: Option<ValueObjectRequired<InventoryReferenceType>>,
    pub reference_id: ValueObjectOptional<UuidVO>,
    pub reserved_until: ValueObjectRequired<InventoryReservationsReservedUntil>,
    pub status: ValueObjectRequired<InventoryReservationsStatus>,
}

impl TryFrom<InventoryReservationUserInputHelper> for InventoryReservationUserInput {
    type Error = InventoryReservationUserInputError;
    fn try_from(value: InventoryReservationUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = InventoryReservationUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let inventory_id = value
            .inventory_id
            .parse::<ValueObjectRequired<UuidVO>>()
            .inspect_err(|e| {
                error.inventory_id = Some(e.to_string());
            });

        let quantity = value
            .quantity
            .parse::<ValueObjectRequired<Quantity>>()
            .inspect_err(|e| error.quantity = Some(e.to_string()));

        let reference_id = value
            .reference_id
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.reference_id = Some(e.to_string());
            });

        let reference_type = if let Ok(reference_id) = &reference_id
            && reference_id.is_present()
        {
            value
                .reference_type
                .parse::<ValueObjectRequired<InventoryReferenceType>>()
                .inspect_err(|e| error.reference_type = Some(e.to_string()))
                .map(Some)
        } else {
            Ok(None)
        };

        let reserved_until = value
            .reserved_until
            .parse::<ValueObjectRequired<InventoryReservationsReservedUntil>>()
            .inspect_err(|e| {
                error.reserved_until = Some(e.to_string());
            });

        let status = value
            .status
            .parse::<ValueObjectRequired<InventoryReservationsStatus>>()
            .inspect_err(|e| {
                error.status = Some(e.to_string());
            });

        if error.is_empty() {
            Ok(InventoryReservationUserInput {
                id: id?,
                inventory_id: inventory_id?,
                quantity: quantity?,
                reference_type: reference_type?,
                reference_id: reference_id?,
                reserved_until: reserved_until?,
                status: status?,
            })
        } else {
            Err(error)
        }
    }
}

#[derive(Deserialize)]
pub struct InventoryReservationsRawQuery {
    inventory_id: Uuid,
    q: Option<String>,
}

impl InventoryReservationsRawQuery {
    pub fn inventory_id(&self) -> Uuid {
        self.inventory_id
    }
    pub fn q(&self) -> &str {
        match &self.q {
            Some(v) => v,
            None => "",
        }
    }
}
