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
use crate::common::value_object::ValueObjectError;
use crate::common::value_object::ValueObjectOptional;
use crate::common::value_object::ValueObjectRequired;
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

impl From<ValueObjectError> for InventoryUserInputError {
    fn from(_: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct InventoryUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub product_id: ValueObjectRequired<UuidVO>,
    pub warehouse_id: ValueObjectRequired<UuidVO>,
    pub minimum_stock: ValueObjectOptional<Integer32>,
    pub maximum_stock: ValueObjectOptional<Integer32>,
    pub currency_code: ValueObjectRequired<CurrencyCode>,
    pub status: ValueObjectRequired<InventoryStatus>,
}

impl TryFrom<InventoryUserInputHelper> for InventoryUserInput {
    type Error = InventoryUserInputError;
    fn try_from(value: InventoryUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = InventoryUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let product_id = value
            .product_id
            .parse::<ValueObjectRequired<UuidVO>>()
            .inspect_err(|e| {
                error.product_id = Some(e.to_string());
            });

        let warehouse_id = value
            .warehouse_id
            .parse::<ValueObjectRequired<UuidVO>>()
            .inspect_err(|e| {
                error.warehouse_id = Some(e.to_string());
            });

        let minimum_stock = value
            .minimum_stock
            .parse::<ValueObjectOptional<Integer32>>()
            .inspect_err(|e| {
                error.minimum_stock = Some(e.to_string());
            });

        let maximum_stock = value
            .maximum_stock
            .parse::<ValueObjectOptional<Integer32>>()
            .inspect_err(|e| {
                error.maximum_stock = Some(e.to_string());
            });

        let status = value
            .status
            .parse::<ValueObjectRequired<InventoryStatus>>()
            .inspect_err(|e| {
                error.status = Some(e.to_string());
            });

        let currency_code = value
            .currency_code
            .parse::<ValueObjectRequired<CurrencyCode>>()
            .inspect_err(|e| error.currency_code = Some(e.to_string()));

        if error.is_empty() {
            Ok(InventoryUserInput {
                id: id?,
                product_id: product_id?,
                warehouse_id: warehouse_id?,
                minimum_stock: minimum_stock?,
                maximum_stock: maximum_stock?,
                currency_code: currency_code?,
                status: status?,
            })
        } else {
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn valid_inventory_user_input() {
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let iui = InventoryUserInput::try_from(InventoryUserInputHelper {
            id: None,
            product_id: product_id.to_string(),
            warehouse_id: warehouse_id.to_string(),
            minimum_stock: String::from("10"),
            maximum_stock: String::from("100"),
            currency_code: String::from("HUF"),
            status: String::from("active"),
        })
        .unwrap();

        assert_eq!(iui.product_id.as_uuid().unwrap(), product_id);
        assert_eq!(iui.warehouse_id.as_uuid().unwrap(), warehouse_id);
        assert_eq!(iui.minimum_stock.as_i32().unwrap(), 10_i32);
        assert_eq!(iui.maximum_stock.as_i32().unwrap(), 100_i32);
        assert_eq!(iui.currency_code.as_str().unwrap(), "HUF");
        assert_eq!(iui.status.as_str().unwrap(), "active");
    }

    #[test]
    fn invalid_inventory_user_input() {
        let iui = InventoryUserInput::try_from(InventoryUserInputHelper {
            id: None,
            product_id: String::from(""),
            warehouse_id: String::from("asd"),
            minimum_stock: String::from("1a"),
            maximum_stock: String::from("aaa"),
            currency_code: String::from("HUFF"),
            status: String::from("activee"),
        })
        .unwrap_err();

        assert_eq!(iui.product_id.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(iui.warehouse_id.unwrap(), UuidVO::PARSE_ERROR);
        assert_eq!(iui.minimum_stock.unwrap(), Integer32::PARSE_ERROR);
        assert_eq!(iui.maximum_stock.unwrap(), Integer32::PARSE_ERROR);
        assert_eq!(iui.currency_code.unwrap(), CurrencyCode::VALIDATION_ERROR);
        assert_eq!(iui.status.unwrap(), InventoryStatus::VALIDATION_ERROR);
    }
}
