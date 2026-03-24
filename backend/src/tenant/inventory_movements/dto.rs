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
use crate::common::types::UuidVO;
use crate::common::types::ValueObject;
use crate::common::types::quantity::Quantity;
use crate::common::types::value_object::ValueObjectError;
use crate::tenant::inventory_movements::types::{InventoryMovementType, InventoryReferenceType};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InventoryMovementUserInputHelper {
    pub id: Option<String>,
    pub inventory_id: String,
    pub movement_type: String,
    pub quantity: String,
    pub reference_type: String,
    pub reference_id: String,
    pub unit_price: String,
    pub total_price: String,
    pub tax_id: String,
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
    pub id: Option<ValueObject<UuidVO>>,
    pub inventory_id: ValueObject<UuidVO>,
    pub movement_type: ValueObject<InventoryMovementType>,
    pub quantity: ValueObject<Quantity>,
    pub reference_type: Option<ValueObject<InventoryReferenceType>>,
    pub reference_id: Option<ValueObject<UuidVO>>,
    pub unit_price: Option<ValueObject<Float64>>,
    pub total_price: Option<ValueObject<Float64>>,
    pub tax_id: ValueObject<UuidVO>,
}

impl InventoryMovementUserInput {
    pub fn quantity(&self, negate: bool) -> Result<f64, ValueObjectError> {
        if negate {
            Ok(-self.quantity.as_f64()?)
        } else {
            Ok(self.quantity.as_f64()?)
        }
    }
}

impl TryFrom<InventoryMovementUserInputHelper> for InventoryMovementUserInput {
    type Error = InventoryMovementUserInputError;
    fn try_from(value: InventoryMovementUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = InventoryMovementUserInputError::default();

        let id = if let Some(id) = value.id {
            ValueObject::new_optional(UuidVO(id)).inspect_err(|e| {
                error.id = Some(e.to_string());
            })
        } else {
            Ok(None)
        };

        let inventory_id =
            ValueObject::new_required(UuidVO(value.inventory_id)).inspect_err(|_| {
                error.inventory_id = Some("A mező kitöltése kötelező!".to_string());
            });

        let tax_id = ValueObject::new_required(UuidVO(value.tax_id)).inspect_err(|_| {
            error.tax_id = Some("A mező kitöltése kötelező!".to_string());
        });

        let movement_type = ValueObject::new_required(InventoryMovementType(value.movement_type))
            .inspect_err(|e| error.movement_type = Some(e.to_string()));

        let quantity = ValueObject::new_required(Quantity(value.quantity))
            .inspect_err(|e| error.quantity = Some(e.to_string()));

        let reference_id = ValueObject::new_optional(UuidVO(value.reference_id)).inspect_err(|e| {
            error.reference_id = Some(e.to_string());
        });

        let reference_type = if value.reference_type.trim() == "" {
            None
        } else {
            ValueObject::new_required(InventoryReferenceType(value.reference_type))
                .inspect_err(|e| error.reference_type = Some(e.to_string()))
                .ok()
        };

        let unit_price = ValueObject::new_optional(Float64(value.unit_price)).inspect_err(|e| {
            error.unit_price = Some(e.to_string());
        });

        let total_price = ValueObject::new_optional(Float64(value.total_price)).inspect_err(|e| {
            error.total_price = Some(e.to_string());
        });

        if error.is_empty() {
            Ok(InventoryMovementUserInput {
                id: id.map_err(|_| InventoryMovementUserInputError::default())?,
                inventory_id: inventory_id
                    .map_err(|_| InventoryMovementUserInputError::default())?,
                movement_type: movement_type
                    .map_err(|_| InventoryMovementUserInputError::default())?,
                quantity: quantity.map_err(|_| InventoryMovementUserInputError::default())?,
                reference_type,
                reference_id: reference_id
                    .map_err(|_| InventoryMovementUserInputError::default())?,
                unit_price: unit_price.map_err(|_| InventoryMovementUserInputError::default())?,
                total_price: total_price.map_err(|_| InventoryMovementUserInputError::default())?,
                tax_id: tax_id.map_err(|_| InventoryMovementUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}

#[derive(Deserialize)]
pub struct InventoryMovementsRawQuery {
    inventory_id: Uuid,
    q: Option<String>,
}

impl InventoryMovementsRawQuery {
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
