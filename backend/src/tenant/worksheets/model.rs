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

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Worksheet {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub customer_id: Uuid,
    pub project_id: Option<Uuid>,
    pub created_by_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct WorksheetResolved {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub customer_id: Uuid,
    pub customer: String,
    pub project_id: Option<Uuid>,
    pub project: Option<String>,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub net_material_cost: BigDecimal,
    pub gross_material_cost: BigDecimal,
    pub net_work_cost: BigDecimal,
    pub gross_work_cost: BigDecimal,
}

#[cfg(test)]
pub mod tests {
    use crate::common::TEST_TIME;

    use super::*;

    pub fn test_worksheet_resolved_builder() -> WorksheetResolvedBuilder {
        let mut builder = WorksheetResolvedBuilder::default();
        builder
            .id(Uuid::new_v4())
            .name("Test worksheet".to_string())
            .description(Some("Test description".to_string()))
            .customer_id(Uuid::new_v4())
            .customer("Test Customer".to_string())
            .project_id(None)
            .project(None)
            .created_by_id(Uuid::new_v4())
            .created_by("Test User".to_string())
            .status("active".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None)
            .net_material_cost("10".parse().unwrap())
            .gross_material_cost("20".parse().unwrap())
            .net_work_cost("30".parse().unwrap())
            .gross_work_cost("40".parse().unwrap());

        builder
    }
}
