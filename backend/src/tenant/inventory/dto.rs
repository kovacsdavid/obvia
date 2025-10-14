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
use crate::tenant::currencies::types::CurrencyCode;
use crate::tenant::inventory::types::inventory::quantity::Quantity;
use crate::tenant::inventory::types::inventory::{InventoryPrice, InventoryQuantity};
use crate::validate_optional_string;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InventoryUserInputHelper {
    pub id: Option<String>,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: String,
    pub price: String,
    pub tax_id: Uuid,
    pub currency_code: String,
}

#[derive(Debug, Serialize, Default)]
pub struct InventoryUserInputError {
    pub id: Option<String>,
    pub product_id: Option<String>,
    pub warehouse_id: Option<String>,
    pub quantity: Option<String>,
    pub price: Option<String>,
    pub tax_id: Option<String>,
    pub currency_code: Option<String>,
}

impl InventoryUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.product_id.is_none()
            && self.warehouse_id.is_none()
            && self.quantity.is_none()
            && self.price.is_none()
            && self.tax_id.is_none()
            && self.currency_code.is_none()
    }
}

impl Display for InventoryUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateInventoryError: {}", json),
            Err(e) => write!(f, "CreateInventoryError: {}", e),
        }
    }
}

impl FormErrorResponse for InventoryUserInputError {}

impl IntoResponse for InventoryUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryUserInput {
    pub id: Option<Uuid>,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: ValueObject<Quantity>,
    pub price: Option<ValueObject<InventoryPrice>>,
    pub tax_id: Uuid,
    pub currency_code: ValueObject<CurrencyCode>,
}

impl TryFrom<InventoryUserInputHelper> for InventoryUserInput {
    type Error = InventoryUserInputError;
    fn try_from(value: InventoryUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = InventoryUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|e| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let quantity = ValueObject::new(InventoryQuantity(value.quantity)).inspect_err(|e| {
            error.quantity = Some(e.to_string());
        });

        let price = validate_optional_string!(InventoryPrice(value.price), error.price);

        let currency_code = ValueObject::new(CurrencyCode(value.currency_code))
            .inspect_err(|e| error.currency_code = Some(e.to_string()));

        if error.is_empty() {
            Ok(InventoryUserInput {
                id,
                product_id: value.product_id,
                warehouse_id: value.warehouse_id,
                quantity: quantity.map_err(|_| InventoryUserInputError::default())?,
                price,
                tax_id: value.tax_id,
                currency_code: currency_code.map_err(|_| InventoryUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
