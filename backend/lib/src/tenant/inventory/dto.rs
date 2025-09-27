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
use crate::common::types::value_object::ValueObject;
use crate::common::types::value_object::ValueObjectable;
use crate::tenant::inventory::types::currency::currency::Currency;
use crate::tenant::inventory::types::inventory::quantity::Quantity;
use crate::tenant::inventory::types::inventory::{
    InventoryCost, InventoryPrice, InventoryQuantity,
};
use crate::validate_optional_string;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateInventoryHelper {
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: String,
    pub price: String,
    pub cost: String,
    pub currency_id: String,
    pub new_currency: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateInventoryError {
    pub product_id: Option<String>,
    pub warehouse_id: Option<String>,
    pub quantity: Option<String>,
    pub price: Option<String>,
    pub cost: Option<String>,
    pub currency_id: Option<String>,
    pub new_currency: Option<String>,
}

impl CreateInventoryError {
    pub fn is_empty(&self) -> bool {
        self.product_id.is_none()
            && self.warehouse_id.is_none()
            && self.quantity.is_none()
            && self.price.is_none()
            && self.cost.is_none()
            && self.currency_id.is_none()
    }
}

impl Display for CreateInventoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateInventoryError: {}", json),
            Err(e) => write!(f, "CreateInventoryError: {}", e),
        }
    }
}

impl FormErrorResponse for CreateInventoryError {}

impl IntoResponse for CreateInventoryError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInventory {
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: ValueObject<Quantity>,
    pub price: Option<ValueObject<InventoryPrice>>,
    pub cost: Option<ValueObject<InventoryCost>>,
    pub currency_id: Option<Uuid>,
    pub new_currency: Option<ValueObject<Currency>>,
}

impl TryFrom<CreateInventoryHelper> for CreateInventory {
    type Error = CreateInventoryError;
    fn try_from(value: CreateInventoryHelper) -> Result<Self, Self::Error> {
        let mut error = CreateInventoryError::default();

        let quantity = ValueObject::new(InventoryQuantity(value.quantity)).inspect_err(|e| {
            error.quantity = Some(e.to_string());
        });

        let price = validate_optional_string!(InventoryPrice(value.price), error.price);

        let cost = validate_optional_string!(InventoryCost(value.cost), error.cost);

        let currency_id = match value.currency_id.as_str() {
            "other" => None,
            _ => match Uuid::parse_str(value.currency_id.as_str()) {
                Ok(v) => Some(v),
                Err(_) => {
                    error.currency_id = Some("Hibás mértékegység".to_string());
                    None
                }
            },
        };

        let new_currency = if currency_id.is_some() {
            None
        } else {
            ValueObject::new(Currency(value.new_currency))
                .inspect_err(|e| error.new_currency = Some(e.to_string()))
                .ok()
        };

        if error.is_empty() {
            Ok(CreateInventory {
                product_id: value.product_id,
                warehouse_id: value.warehouse_id,
                quantity: quantity.map_err(|_| CreateInventoryError::default())?,
                price,
                cost,
                currency_id,
                new_currency,
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
