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
use crate::common::types::value_object::{ValueObject, ValueObjectable};
use crate::tenant::products::types::product::{ProductDescription, ProductName, ProductStatus};
use crate::tenant::products::types::unit_of_measure::unit_of_measure::UnitsOfMeasure;
use crate::validate_optional_string;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

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
    pub id: Option<Uuid>,
    pub name: ValueObject<ProductName>,
    pub description: Option<ValueObject<ProductDescription>>,
    pub unit_of_measure_id: Option<Uuid>,
    pub new_unit_of_measure: Option<ValueObject<UnitsOfMeasure>>,
    pub status: ValueObject<ProductStatus>,
}

impl TryFrom<ProductUserInputHelper> for ProductUserInput {
    type Error = ProductUserInputError;
    fn try_from(value: ProductUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = ProductUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|e| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let name = ValueObject::new(ProductName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });

        let status = ValueObject::new(ProductStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        let description =
            validate_optional_string!(ProductDescription(value.description), error.description);

        let unit_of_measure_id = match value.unit_of_measure_id.as_str() {
            "other" => None,
            _ => match Uuid::parse_str(value.unit_of_measure_id.as_str()) {
                Ok(v) => Some(v),
                Err(_) => {
                    error.unit_of_measure_id = Some("Hibás mértékegység".to_string());
                    None
                }
            },
        };

        let new_unit_of_measure = if unit_of_measure_id.is_some() {
            None
        } else {
            ValueObject::new(UnitsOfMeasure(value.new_unit_of_measure))
                .inspect_err(|e| error.new_unit_of_measure = Some(e.to_string()))
                .ok()
        };

        if error.is_empty() {
            Ok(ProductUserInput {
                id,
                name: name.map_err(|_| ProductUserInputError::default())?,
                description,
                unit_of_measure_id,
                new_unit_of_measure,
                status: status.map_err(|_| ProductUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}

pub struct UpdateProductHelper {
    // TODO: fields
}

pub struct UpdateProductError {
    // TODO: fields
}

impl UpdateProductError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateProductError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProduct {
    pub name: ValueObject<ProductName>,
    pub description: Option<String>,
    pub unit_of_measure: Uuid,
    pub is_active: bool,
    pub created_by_id: Uuid,
}

impl TryFrom<UpdateProductHelper> for UpdateProduct {
    type Error = UpdateProductError;
    fn try_from(value: UpdateProductHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductCategoryConnect {
    pub product_id: Uuid,
    pub product_category_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductCategoryConnect {
    pub product_id: Option<Uuid>,
    pub product_category_id: Option<Uuid>,
}

pub struct CreateUnitOfMeasureHelper {
    // TODO: fields
}

pub struct CreateUnitOfMeasureError {
    // TODO: fields
}

impl CreateUnitOfMeasureError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for CreateUnitOfMeasureError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUnitOfMeasure {
    pub unit_of_measure: ValueObject<UnitsOfMeasure>,
}

impl TryFrom<CreateUnitOfMeasureHelper> for CreateUnitOfMeasure {
    type Error = CreateUnitOfMeasureError;
    fn try_from(value: CreateUnitOfMeasureHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct UpdateUnitOfMeasureHelper {
    // TODO: fields
}

pub struct UpdateUnitOfMeasureError {
    // TODO: fields
}

impl UpdateUnitOfMeasureError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateUnitOfMeasureError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUnitOfMeasure {
    pub unit_of_measure: ValueObject<UnitsOfMeasure>,
}

impl TryFrom<UpdateUnitOfMeasureHelper> for UpdateUnitOfMeasure {
    type Error = UpdateUnitOfMeasureError;
    fn try_from(value: UpdateUnitOfMeasureHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}
