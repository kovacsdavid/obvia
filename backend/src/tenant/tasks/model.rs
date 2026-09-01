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

use crate::common::CommonBuilderError;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub worksheet_id: Uuid,
    pub service_id: Uuid,
    pub currency_code: String,
    pub quantity: Option<BigDecimal>,
    pub price: Option<BigDecimal>,
    pub tax_id: Uuid,
    pub created_by_id: Uuid,
    pub status: String,
    pub priority: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
#[builder(build_fn(error = "CommonBuilderError"))]
pub struct TaskResolved {
    pub id: Uuid,
    pub worksheet_id: Uuid,
    pub worksheet: String,
    pub service_id: Uuid,
    pub service: String,
    pub currency_code: String,
    pub quantity: Option<BigDecimal>,
    pub price: Option<BigDecimal>,
    pub tax_id: Uuid,
    pub tax: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub status: String,
    pub priority: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

#[cfg(test)]
pub mod tests {
    use crate::common::TEST_TIME;

    use super::*;

    pub fn test_task_resolved_builder() -> TaskResolvedBuilder {
        let mut builder = TaskResolvedBuilder::default();
        builder
            .id(Uuid::new_v4())
            .worksheet_id(Uuid::new_v4())
            .worksheet("Test worksheet".to_string())
            .service_id(Uuid::new_v4())
            .service("Test service".to_string())
            .currency_code("HUF".to_string())
            .quantity(Some("10".parse().unwrap()))
            .price(Some("1000".parse().unwrap()))
            .tax_id(Uuid::new_v4())
            .tax("Test tax".to_string())
            .created_by_id(Uuid::new_v4())
            .created_by("Test User".to_string())
            .status("active".to_string())
            .priority(Some("normal".to_string()))
            .due_date(Some(*TEST_TIME))
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None)
            .description(Some("Test description".to_string()));

        builder
    }
}
