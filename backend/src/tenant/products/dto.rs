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
use crate::common::types::{UuidVO, ValueObject};
use crate::tenant::products::types::product::{ProductDescription, ProductName, ProductStatus};
use crate::tenant::products::types::unit_of_measure::unit_of_measure::UnitsOfMeasure;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct ProductUserInputHelper {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub unit_of_measure_id: String,
    pub new_unit_of_measure: String,
    pub status: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ProductUserInputError {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub unit_of_measure_id: Option<String>,
    pub new_unit_of_measure: Option<String>,
    pub status: Option<String>,
}

impl ProductUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.name.is_none()
            && self.description.is_none()
            && self.unit_of_measure_id.is_none()
            && self.status.is_none()
    }
}

impl Display for ProductUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateInventoryError: {}", json),
            Err(e) => write!(f, "CreateInventoryError: {}", e),
        }
    }
}

impl FormErrorResponse for ProductUserInputError {}

impl IntoResponse for ProductUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductUserInput {
    pub id: Option<ValueObject<UuidVO>>,
    pub name: ValueObject<ProductName>,
    pub description: Option<ValueObject<ProductDescription>>,
    pub unit_of_measure_id: Option<ValueObject<UuidVO>>,
    pub new_unit_of_measure: Option<ValueObject<UnitsOfMeasure>>,
    pub status: ValueObject<ProductStatus>,
}

impl TryFrom<ProductUserInputHelper> for ProductUserInput {
    type Error = ProductUserInputError;
    fn try_from(value: ProductUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = ProductUserInputError::default();

        let id = if let Some(id) = value.id {
            ValueObject::new_optional(UuidVO(id)).inspect_err(|e| {
                error.id = Some(e.to_string());
            })
        } else {
            Ok(None)
        };

        let name = ValueObject::new_required(ProductName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });

        let status = ValueObject::new_required(ProductStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        let description = ValueObject::new_optional(ProductDescription(value.description))
            .inspect_err(|e| {
                error.description = Some(e.to_string());
            });

        let unit_of_measure_id = if value.unit_of_measure_id.as_str() != "other" {
            ValueObject::new_required(UuidVO(value.unit_of_measure_id))
                .inspect_err(|_| {
                    error.unit_of_measure_id = Some("Hibás mértékegység".to_string());
                })
                .map(Some)
        } else {
            Ok(None)
        };

        let new_unit_of_measure = if let Ok(result) = &unit_of_measure_id
            && result.is_some()
        {
            None
        } else {
            ValueObject::new_required(UnitsOfMeasure(value.new_unit_of_measure))
                .inspect_err(|e| error.new_unit_of_measure = Some(e.to_string()))
                .ok()
        };

        if error.is_empty() {
            Ok(ProductUserInput {
                id: id.map_err(|_| ProductUserInputError::default())?,
                name: name.map_err(|_| ProductUserInputError::default())?,
                description: description.map_err(|_| ProductUserInputError::default())?,
                unit_of_measure_id: unit_of_measure_id
                    .map_err(|_| ProductUserInputError::default())?,
                new_unit_of_measure,
                status: status.map_err(|_| ProductUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
