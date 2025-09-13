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

use crate::manager::common::dto::{ErrorBody, ErrorResponse};
use crate::manager::common::types::value_object::{ValueObject, ValueObjectable};
use crate::tenant::products::types::currency::currency::Currency;
use crate::tenant::products::types::product::cost::Cost;
use crate::tenant::products::types::product::price::Price;
use crate::tenant::products::types::product::{
    ProductCost, ProductDescription, ProductName, ProductPrice, ProductStatus,
};
use crate::tenant::products::types::unit_of_measure::unit_of_measure::UnitsOfMeasure;
use crate::validate_optional_string;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateProductHelper {
    pub name: String,
    pub description: String,
    pub unit_of_measure: Uuid,
    pub price: String,
    pub cost: String,
    pub currency_id: Uuid,
    pub status: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateProductError {
    pub name: Option<String>,
    pub description: Option<String>,
    pub unit_of_measure: Option<String>,
    pub price: Option<String>,
    pub cost: Option<String>,
    pub currency_id: Option<String>,
    pub status: Option<String>,
}

impl CreateProductError {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.unit_of_measure.is_none()
            && self.price.is_none()
            && self.cost.is_none()
            && self.currency_id.is_none()
            && self.status.is_none()
    }
}

impl IntoResponse for CreateProductError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                reference: String::from("PRODUCTS/DTO/CREATE"),
                global: String::from("Kérjük, ellenőrizze a hibás mezőket"),
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: ValueObject<ProductName>,
    pub description: Option<ValueObject<ProductDescription>>,
    pub unit_of_measure: Uuid,
    pub price: Option<ValueObject<ProductPrice>>,
    pub cost: Option<ValueObject<ProductCost>>,
    pub currency_id: Uuid,
    pub status: ValueObject<ProductStatus>,
}

impl TryFrom<CreateProductHelper> for CreateProduct {
    type Error = CreateProductError;

    fn try_from(value: CreateProductHelper) -> Result<Self, Self::Error> {
        let mut error = CreateProductError::default();

        let name = ValueObject::new(ProductName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });

        let price = validate_optional_string!(ProductPrice(value.price), error.price);
        let cost = validate_optional_string!(ProductCost(value.cost), error.cost);

        let status = ValueObject::new(ProductStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        let description =
            validate_optional_string!(ProductDescription(value.description), error.description);

        if error.is_empty() {
            Ok(CreateProduct {
                name: name.map_err(|_| CreateProductError::default())?,
                description,
                unit_of_measure: value.unit_of_measure,
                price,
                cost,
                currency_id: value.currency_id,
                status: status.map_err(|_| CreateProductError::default())?,
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
    pub price: Option<ValueObject<Price>>,
    pub cost: Option<ValueObject<Cost>>,
    pub currency_id: Uuid,
    pub is_active: bool,
    pub created_by: Uuid,
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

pub struct CreateCurrencyHelper {
    // TODO: fields
}

pub struct CreateCurrencyError {
    // TODO: fields
}

impl CreateCurrencyError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for CreateCurrencyError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCurrency {
    pub currency: ValueObject<Currency>,
}

impl TryFrom<CreateCurrencyHelper> for CreateCurrency {
    type Error = CreateCurrencyError;
    fn try_from(value: CreateCurrencyHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct UpdateCurrencyHelper {
    // TODO: fields
}

pub struct UpdateCurrencyError {
    // TODO: fields
}

impl UpdateCurrencyError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateCurrencyError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCurrency {
    pub currency: ValueObject<Currency>,
}

impl TryFrom<UpdateCurrencyHelper> for UpdateCurrency {
    type Error = UpdateCurrencyError;
    fn try_from(value: UpdateCurrencyHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}
