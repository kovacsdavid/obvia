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
use crate::manager::common::types::value_object::ValueObject;
use crate::tenant::products::types::currency::currency::Currency;
use crate::tenant::products::types::product::ProductName;
use crate::tenant::products::types::product::cost::Cost;
use crate::tenant::products::types::product::price::Price;
use crate::tenant::products::types::unit_of_measure::unit_of_measure::UnitsOfMeasure;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct CreateProductHelper {
    // TODO: fields
}

pub struct CreateProductError {
    // TODO: fields
}

impl CreateProductError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for CreateProductError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: ValueObject<ProductName>,
    pub description: Option<String>,
    pub unit_of_measure: Uuid,
    pub price: Option<ValueObject<Price>>,
    pub cost: Option<ValueObject<Cost>>,
    pub currency_id: Uuid,
    pub is_active: bool,
    pub created_by: Uuid,
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
