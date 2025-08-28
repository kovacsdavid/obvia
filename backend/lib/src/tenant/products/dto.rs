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
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUnitOfMeasure {
    pub unit_of_measure: ValueObject<UnitsOfMeasure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUnitOfMeasure {
    pub unit_of_measure: ValueObject<UnitsOfMeasure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCurrency {
    pub currency: ValueObject<Currency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCurrency {
    pub currency: ValueObject<Currency>,
}
