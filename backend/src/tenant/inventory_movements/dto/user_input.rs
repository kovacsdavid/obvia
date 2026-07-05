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

use crate::common::error::v2::AppError;
use crate::common::error::v2::AppErrorVisibility;
use crate::common::types::Float64;
use crate::common::types::UuidVO;
use crate::common::types::quantity::Quantity;
use crate::common::value_object::ValueObjectError;
use crate::common::value_object::ValueObjectOptional;
use crate::common::value_object::ValueObjectRequired;
use crate::tenant::inventory_movements::types::{InventoryMovementType, InventoryReferenceType};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use tracing::Level;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InventoryMovementUserInputHelper {
    pub id: Option<String>,
    pub inventory_id: String,
    pub movement_type: String,
    pub quantity: String,
    pub reference_type: String,
    pub reference_id: String,
    pub unit_price: String,
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

impl From<InventoryMovementUserInputError> for AppError {
    fn from(value: InventoryMovementUserInputError) -> Self {
        Self::new(
            Level::DEBUG,
            StatusCode::UNPROCESSABLE_ENTITY,
            file!(),
            AppErrorVisibility::UserFacing,
            json!({
                "message": "Kérjük ellenőrizze a hibás mezőket!",
                "fields": value
            }),
        )
    }
}

impl From<ValueObjectError> for InventoryMovementUserInputError {
    fn from(_: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InventoryMovementUserInput {
    #[allow(dead_code)]
    pub id: ValueObjectOptional<UuidVO>,
    pub inventory_id: ValueObjectRequired<UuidVO>,
    pub movement_type: ValueObjectRequired<InventoryMovementType>,
    pub quantity: ValueObjectRequired<Quantity>,
    pub reference_type: Option<ValueObjectRequired<InventoryReferenceType>>,
    pub reference_id: ValueObjectOptional<UuidVO>,
    pub unit_price: ValueObjectOptional<Float64>,
    pub tax_id: ValueObjectRequired<UuidVO>,
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

        let tax_id = value
            .tax_id
            .parse::<ValueObjectRequired<UuidVO>>()
            .inspect_err(|e| {
                error.tax_id = Some(e.to_string());
            });

        let movement_type = value
            .movement_type
            .parse::<ValueObjectRequired<InventoryMovementType>>()
            .inspect_err(|e| error.movement_type = Some(e.to_string()));

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

        let unit_price = value
            .unit_price
            .parse::<ValueObjectOptional<Float64>>()
            .inspect_err(|e| {
                error.unit_price = Some(e.to_string());
            });

        if error.is_empty() {
            Ok(InventoryMovementUserInput {
                id: id?,
                inventory_id: inventory_id?,
                movement_type: movement_type?,
                quantity: quantity?,
                reference_type: reference_type?,
                reference_id: reference_id?,
                unit_price: unit_price?,
                tax_id: tax_id?,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_inventory_movements_user_input() {
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let imui = InventoryMovementUserInput::try_from(InventoryMovementUserInputHelper {
            id: None,
            inventory_id: inventory_id.to_string(),
            movement_type: String::from("out"),
            quantity: String::from("10"),
            reference_type: String::from("worksheets"),
            reference_id: reference_id.to_string(),
            unit_price: String::from("1000"),
            tax_id: tax_id.to_string(),
        })
        .unwrap();

        assert_eq!(imui.inventory_id.as_uuid().unwrap(), inventory_id);
        assert_eq!(imui.movement_type.as_str().unwrap(), "out");
        assert_eq!(imui.quantity.as_f64().unwrap(), 10_f64);
        assert_eq!(imui.reference_type.unwrap().as_str().unwrap(), "worksheets");
        assert_eq!(imui.reference_id.as_uuid().unwrap(), reference_id);
        assert_eq!(imui.unit_price.as_f64().unwrap(), 1000_f64);
        assert_eq!(imui.tax_id.as_uuid().unwrap(), tax_id);
    }

    #[test]
    fn invalid_inventory_movements_user_input() {
        let reference_id = Uuid::new_v4();
        let imui = InventoryMovementUserInput::try_from(InventoryMovementUserInputHelper {
            id: None,
            inventory_id: String::from("invalid"),
            movement_type: String::from("invalid"),
            quantity: String::from(""),
            reference_type: String::from(""),
            reference_id: reference_id.to_string(),
            unit_price: String::from("asd"),
            tax_id: String::from("asd"),
        })
        .unwrap_err();

        assert_eq!(imui.inventory_id.unwrap(), UuidVO::PARSE_ERROR);
        assert_eq!(
            imui.movement_type.unwrap(),
            InventoryMovementType::VALIDATION_ERROR
        );
        assert_eq!(imui.quantity.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(imui.reference_type.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(imui.reference_id, None);
        assert_eq!(imui.unit_price.unwrap(), Float64::PARSE_ERROR);
        assert_eq!(imui.tax_id.unwrap(), UuidVO::PARSE_ERROR);
    }
}
