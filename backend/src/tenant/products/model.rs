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

use chrono::{DateTime, Utc};
use derive_builder::Builder;
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct ProductResolved {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub unit_of_measure_id: Uuid,
    pub unit_of_measure: String,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UnitOfMeasure {
    pub id: Uuid,
    pub unit_of_measure: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
pub mod tests {
    use crate::common::TEST_TIME;

    use super::*;

    pub fn test_product_resolved_builder() -> ProductResolvedBuilder {
        let mut builder = ProductResolvedBuilder::default();

        builder
            .id(Uuid::new_v4())
            .name("Test product".to_string())
            .description(Some("Test description".to_string()))
            .unit_of_measure_id(Uuid::new_v4())
            .unit_of_measure("cm".to_string())
            .status("active".to_string())
            .created_by_id(Uuid::new_v4())
            .created_by("Test User".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None);

        builder
    }
}
