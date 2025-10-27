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
use crate::common::types::Float64;
use crate::common::types::quantity::Quantity;
use crate::common::types::{ValueObject, ValueObjectable};
use crate::tenant::inventory_movements::types::{InventoryMovementType, InventoryReferenceType};
use crate::validate_optional_string;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InventoryMovementUserInputHelper {
    pub id: Option<String>,
    pub inventory_id: Uuid,
    pub movement_type: String,
    pub quantity: String,
    pub reference_type: String,
    pub reference_id: String,
    pub unit_price: String,
    pub total_price: String,
    pub tax_id: Uuid,
}

#[derive(Debug, Serialize, Default)]
pub struct InventoryMovementUserInputError {
    pub id: Option<String>,
    pub inventory_id: Option<String>,
    pub movement_type: Option<String>,
    pub quantity: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub unit_price: Option<String>,
    pub total_price: Option<String>,
    pub tax_id: Option<String>,
}

impl InventoryMovementUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.inventory_id.is_none()
            && self.movement_type.is_none()
            && self.quantity.is_none()
            && self.reference_type.is_none()
            && self.reference_id.is_none()
            && self.unit_price.is_none()
            && self.total_price.is_none()
            && self.tax_id.is_none()
    }
}

impl Display for InventoryMovementUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateInventoryMovementError: {}", json),
            Err(e) => write!(f, "CreateInventoryMovementError: {}", e),
        }
    }
}

impl FormErrorResponse for InventoryMovementUserInputError {}

impl IntoResponse for InventoryMovementUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryMovementUserInput {
    pub id: Option<Uuid>,
    pub inventory_id: Uuid,
    pub movement_type: ValueObject<InventoryMovementType>,
    pub quantity: ValueObject<Quantity>,
    pub reference_type: Option<ValueObject<InventoryReferenceType>>,
    pub reference_id: Option<Uuid>,
    pub unit_price: Option<ValueObject<Float64>>,
    pub total_price: Option<ValueObject<Float64>>,
    pub tax_id: Uuid,
}

impl InventoryMovementUserInput {
    pub fn quantity(&self, negate: bool) -> Result<i32, ParseIntError> {
        if negate {
            Ok(-self.quantity.extract().get_value().parse::<i32>()?)
        } else {
            Ok(self.quantity.extract().get_value().parse::<i32>()?)
        }
    }
}

impl TryFrom<InventoryMovementUserInputHelper> for InventoryMovementUserInput {
    type Error = InventoryMovementUserInputError;
    fn try_from(value: InventoryMovementUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = InventoryMovementUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|_| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let movement_type = ValueObject::new(InventoryMovementType(value.movement_type))
            .inspect_err(|e| error.movement_type = Some(e.to_string()));

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

        let unit_price = validate_optional_string!(Float64(value.unit_price), error.unit_price);

        let total_price = validate_optional_string!(Float64(value.total_price), error.total_price);

        if error.is_empty() {
            Ok(InventoryMovementUserInput {
                id,
                inventory_id: value.inventory_id,
                movement_type: movement_type
                    .map_err(|_| InventoryMovementUserInputError::default())?,
                quantity: quantity.map_err(|_| InventoryMovementUserInputError::default())?,
                reference_type,
                reference_id,
                unit_price,
                total_price,
                tax_id: value.tax_id,
            })
        } else {
            Err(error)
        }
    }
}
