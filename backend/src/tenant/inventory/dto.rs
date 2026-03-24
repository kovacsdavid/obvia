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
use crate::common::types::UuidVO;
use crate::common::types::ValueObject;
use crate::tenant::currencies::types::CurrencyCode;
use crate::tenant::inventory::types::inventory::InventoryStatus;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct InventoryUserInputHelper {
    pub id: Option<String>,
    pub product_id: String,
    pub warehouse_id: String,
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
    pub id: Option<ValueObject<UuidVO>>,
    pub product_id: ValueObject<UuidVO>,
    pub warehouse_id: ValueObject<UuidVO>,
    pub minimum_stock: Option<ValueObject<Integer32>>,
    pub maximum_stock: Option<ValueObject<Integer32>>,
    pub currency_code: ValueObject<CurrencyCode>,
    pub status: ValueObject<InventoryStatus>,
}

impl TryFrom<InventoryUserInputHelper> for InventoryUserInput {
    type Error = InventoryUserInputError;
    fn try_from(value: InventoryUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = InventoryUserInputError::default();

        let id = if let Some(id) = value.id {
            ValueObject::new_optional(UuidVO(id)).inspect_err(|e| {
                error.id = Some(e.to_string());
            })
        } else {
            Ok(None)
        };

        let product_id = ValueObject::new_required(UuidVO(value.product_id)).inspect_err(|e| {
            error.product_id = Some(e.to_string());
        });

        let warehouse_id = ValueObject::new_required(UuidVO(value.warehouse_id)).inspect_err(|e| {
            error.warehouse_id = Some(e.to_string());
        });

        let minimum_stock =
            ValueObject::new_optional(Integer32(value.minimum_stock)).inspect_err(|e| {
                error.minimum_stock = Some(e.to_string());
            });

        let maximum_stock =
            ValueObject::new_optional(Integer32(value.maximum_stock)).inspect_err(|e| {
                error.maximum_stock = Some(e.to_string());
            });

        let status = ValueObject::new_required(InventoryStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        let currency_code = ValueObject::new_required(CurrencyCode(value.currency_code))
            .inspect_err(|e| error.currency_code = Some(e.to_string()));

        if error.is_empty() {
            Ok(InventoryUserInput {
                id: id.map_err(|_| InventoryUserInputError::default())?,
                product_id: product_id.map_err(|_| InventoryUserInputError::default())?,
                warehouse_id: warehouse_id.map_err(|_| InventoryUserInputError::default())?,
                minimum_stock: minimum_stock.map_err(|_| InventoryUserInputError::default())?,
                maximum_stock: maximum_stock.map_err(|_| InventoryUserInputError::default())?,
                currency_code: currency_code.map_err(|_| InventoryUserInputError::default())?,
                status: status.map_err(|_| InventoryUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
