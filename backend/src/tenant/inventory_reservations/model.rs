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
use chrono::{DateTime, Local};
use indexmap::IndexMap;
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
    pub reserved_until: Option<DateTime<Local>>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InventoryReservationResolved {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub quantity: BigDecimal,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub reserved_until: Option<DateTime<Local>>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<InventoryReservationResolved> for IndexMap<String, String> {
    fn from(value: InventoryReservationResolved) -> Self {
        let mut map = IndexMap::new();

        map.insert("id".to_string(), value.id.to_string());
        map.insert("inventory_id".to_string(), value.inventory_id.to_string());
        map.insert("quantity".to_string(), value.quantity.to_string());
        map.insert(
            "reference_type".to_string(),
            value.reference_type.unwrap_or_default(),
        );
        map.insert(
            "reference_id".to_string(),
            value
                .reference_id
                .map(|v| v.to_string())
                .unwrap_or_default(),
        );
        map.insert(
            "reserved_until".to_string(),
            value
                .reserved_until
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
        );
        map.insert("status".to_string(), value.status);
        map.insert("created_by_id".to_string(), value.created_by_id.to_string());
        map.insert("created_by".to_string(), value.created_by);
        map.insert("created_at".to_string(), value.created_at.to_rfc3339());
        map.insert("updated_at".to_string(), value.updated_at.to_rfc3339());

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn inventory_reservations_resolved_into_index_map() {
        let id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let date_test = Local::now();
        let mut expected = IndexMap::new();

        expected.insert("id".to_string(), id.to_string());
        expected.insert("inventory_id".to_string(), inventory_id.to_string());
        expected.insert("quantity".to_string(), "10".parse().unwrap());
        expected.insert("reference_type".to_string(), "test1".to_string());
        expected.insert("reference_id".to_string(), reference_id.to_string());
        expected.insert("reserved_until".to_string(), date_test.to_rfc3339());
        expected.insert("status".to_string(), "test2".to_string());
        expected.insert("created_by_id".to_string(), created_by_id.to_string());
        expected.insert("created_by".to_string(), "test3".to_string());
        expected.insert("created_at".to_string(), date_test.to_rfc3339());
        expected.insert("updated_at".to_string(), date_test.to_rfc3339());

        let input: IndexMap<String, String> = InventoryReservationResolved {
            id,
            inventory_id,
            quantity: "10".parse().unwrap(),
            reference_type: Some("test1".to_string()),
            reference_id: Some(reference_id),
            reserved_until: Some(date_test),
            status: "test2".to_string(),
            created_by_id,
            created_by: "test3".to_string(),
            created_at: date_test,
            updated_at: date_test,
        }
        .into();

        assert_eq!(input, expected);
    }
}
