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
pub struct InventoryMovement {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub movement_type: String,
    pub quantity: BigDecimal,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub unit_price: Option<BigDecimal>,
    pub total_price: Option<BigDecimal>,
    pub tax_id: Uuid,
    pub movement_date: DateTime<Local>,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InventoryMovementResolved {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub movement_type: String,
    pub quantity: BigDecimal,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub unit_price: Option<BigDecimal>,
    pub total_price: Option<BigDecimal>,
    pub tax_id: Uuid,
    pub tax: Option<String>,
    pub movement_date: DateTime<Local>,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Local>,
}

impl From<InventoryMovementResolved> for IndexMap<String, String> {
    fn from(value: InventoryMovementResolved) -> Self {
        let mut map = IndexMap::new();

        map.insert("id".to_string(), value.id.to_string());
        map.insert("inventory_id".to_string(), value.inventory_id.to_string());
        map.insert("movement_type".to_string(), value.movement_type);
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
            "unit_price".to_string(),
            value.unit_price.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert(
            "total_price".to_string(),
            value.total_price.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("tax_id".to_string(), value.tax_id.to_string());
        map.insert("tax".to_string(), value.tax.unwrap_or_default());
        map.insert(
            "movement_date".to_string(),
            value.movement_date.to_rfc3339(),
        );
        map.insert("created_by_id".to_string(), value.created_by_id.to_string());
        map.insert("created_by".to_string(), value.created_by);
        map.insert("created_at".to_string(), value.created_at.to_rfc3339());

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_movements_into_index_map() {
        let id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let date_test = Local::now();
        let mut expected = IndexMap::new();

        expected.insert("id".to_string(), id.to_string());
        expected.insert("inventory_id".to_string(), inventory_id.to_string());
        expected.insert("movement_type".to_string(), "test1".to_string());
        expected.insert("quantity".to_string(), "10".parse().unwrap());
        expected.insert("reference_type".to_string(), "test2".to_string());
        expected.insert("reference_id".to_string(), reference_id.to_string());
        expected.insert("unit_price".to_string(), "20".parse().unwrap());
        expected.insert("total_price".to_string(), "30".parse().unwrap());
        expected.insert("tax_id".to_string(), tax_id.to_string());
        expected.insert("tax".to_string(), "test3".to_string());
        expected.insert("movement_date".to_string(), date_test.to_rfc3339());
        expected.insert("created_by_id".to_string(), created_by_id.to_string());
        expected.insert("created_by".to_string(), "test4".to_string());
        expected.insert("created_at".to_string(), date_test.to_rfc3339());

        let input: IndexMap<String, String> = InventoryMovementResolved {
            id,
            inventory_id,
            movement_type: "test1".to_string(),
            quantity: "10".parse().unwrap(),
            reference_type: Some("test2".to_string()),
            reference_id: Some(reference_id),
            unit_price: Some("20".parse().unwrap()),
            total_price: Some("30".parse().unwrap()),
            tax_id,
            tax: Some("test3".to_string()),
            movement_date: date_test,
            created_by_id,
            created_by: "test4".to_string(),
            created_at: date_test,
        }
        .into();

        assert_eq!(input, expected);
    }
}
