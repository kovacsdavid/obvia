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
pub struct InventoryReservation {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub quantity: BigDecimal,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub reserved_until: Option<DateTime<Utc>>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct InventoryReservationResolved {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub quantity: BigDecimal,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub reserved_until: Option<DateTime<Utc>>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
pub mod tests {
    use crate::common::TEST_TIME;

    use super::*;

    pub fn test_inventory_reservation_resolved_builder() -> InventoryReservationResolvedBuilder {
        let mut builder = InventoryReservationResolvedBuilder::default();
        builder
            .id(Uuid::new_v4())
            .inventory_id(Uuid::new_v4())
            .quantity("10".parse().unwrap())
            .reference_type(Some("worksheets".to_string()))
            .reference_id(Some(Uuid::new_v4()))
            .reserved_until(Some(*TEST_TIME))
            .status("active".to_string())
            .created_by_id(Uuid::new_v4())
            .created_by("Test User".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME);

        builder
    }
}
