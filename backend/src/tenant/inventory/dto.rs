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
use crate::common::types::Integer32;
use crate::common::types::ValueObject;
use crate::common::types::value_object::ValueObjectable;
use crate::tenant::currencies::types::CurrencyCode;
use crate::tenant::inventory::types::inventory::InventoryStatus;
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
    pub minimum_stock: String,
    pub maximum_stock: String,
    pub currency_code: String,
    pub status: String,
}

#[derive(Debug, Serialize, Default)]
pub struct InventoryUserInputError {
    pub id: Option<String>,
    pub product_id: Option<String>,
    pub warehouse_id: Option<String>,
    pub minimum_stock: Option<String>,
    pub maximum_stock: Option<String>,
    pub currency_code: Option<String>,
    pub status: Option<String>,
}

impl InventoryUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.product_id.is_none()
            && self.warehouse_id.is_none()
            && self.minimum_stock.is_none()
            && self.maximum_stock.is_none()
            && self.currency_code.is_none()
            && self.status.is_none()
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
    pub minimum_stock: Option<ValueObject<Integer32>>,
    pub maximum_stock: Option<ValueObject<Integer32>>,
    pub currency_code: ValueObject<CurrencyCode>,
    pub status: ValueObject<InventoryStatus>,
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

        let minimum_stock =
            validate_optional_string!(Integer32(value.minimum_stock), error.minimum_stock);

        let maximum_stock =
            validate_optional_string!(Integer32(value.maximum_stock), error.maximum_stock);

        let status = ValueObject::new(InventoryStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        let currency_code = ValueObject::new(CurrencyCode(value.currency_code))
            .inspect_err(|e| error.currency_code = Some(e.to_string()));

        if error.is_empty() {
            Ok(InventoryUserInput {
                id,
                product_id: value.product_id,
                warehouse_id: value.warehouse_id,
                minimum_stock,
                maximum_stock,
                currency_code: currency_code.map_err(|_| InventoryUserInputError::default())?,
                status: status.map_err(|_| InventoryUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
