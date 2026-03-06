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

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub unit_of_measure_id: Uuid,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductResolved {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub unit_of_measure_id: Uuid,
    pub unit_of_measure: String,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UnitOfMeasure {
    pub id: Uuid,
    pub unit_of_measure: String,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductCategoryConnect {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_category_id: Uuid,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
