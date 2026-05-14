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
pub struct Inventory {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity_on_hand: BigDecimal,
    pub quantity_reserved: BigDecimal,
    pub quantity_available: BigDecimal,
    pub minimum_stock: Option<BigDecimal>,
    pub maximum_stock: Option<BigDecimal>,
    pub currency_code: String,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InventoryResolved {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product: String,
    pub warehouse_id: Uuid,
    pub warehouse: String,
    pub quantity_on_hand: BigDecimal,
    pub quantity_reserved: BigDecimal,
    pub quantity_available: BigDecimal,
    pub minimum_stock: Option<BigDecimal>,
    pub maximum_stock: Option<BigDecimal>,
    pub currency_code: String,
    pub currency: String,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

impl From<InventoryResolved> for IndexMap<String, String> {
    fn from(value: InventoryResolved) -> Self {
        let mut map = IndexMap::new();

        map.insert("id".to_string(), value.id.to_string());
        map.insert("product_id".to_string(), value.product_id.to_string());
        map.insert("product".to_string(), value.product);
        map.insert("warehouse_id".to_string(), value.warehouse_id.to_string());
        map.insert("warehouse".to_string(), value.warehouse);
        map.insert(
            "quantity_on_hand".to_string(),
            value.quantity_on_hand.to_string(),
        );
        map.insert(
            "quantity_reserved".to_string(),
            value.quantity_reserved.to_string(),
        );
        map.insert(
            "quantity_available".to_string(),
            value.quantity_available.to_string(),
        );
        map.insert(
            "minimum_stock".to_string(),
            value
                .minimum_stock
                .map(|v| v.to_string())
                .unwrap_or_default(),
        );
        map.insert(
            "maximum_stock".to_string(),
            value
                .maximum_stock
                .map(|v| v.to_string())
                .unwrap_or_default(),
        );
        map.insert("currency_code".to_string(), value.currency_code);
        map.insert("currency".to_string(), value.currency);
        map.insert("status".to_string(), value.status);
        map.insert("created_by_id".to_string(), value.created_by_id.to_string());
        map.insert("created_by".to_string(), value.created_by);
        map.insert("created_at".to_string(), value.created_at.to_rfc3339());
        map.insert("updated_at".to_string(), value.updated_at.to_rfc3339());
        map.insert(
            "deleted_at".to_string(),
            value
                .deleted_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
        );

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_resolved_into_index_map() {
        let id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let date_test = Local::now();
        let mut expected = IndexMap::new();

        expected.insert("id".to_string(), id.to_string());
        expected.insert("product_id".to_string(), product_id.to_string());
        expected.insert("product".to_string(), "test1".to_string());
        expected.insert("warehouse_id".to_string(), warehouse_id.to_string());
        expected.insert("warehouse".to_string(), "test2".to_string());
        expected.insert("quantity_on_hand".to_string(), "10".to_string());
        expected.insert("quantity_reserved".to_string(), "20".to_string());
        expected.insert("quantity_available".to_string(), "30".to_string());
        expected.insert("minimum_stock".to_string(), "40".to_string());
        expected.insert("maximum_stock".to_string(), "50".to_string());
        expected.insert("currency_code".to_string(), "test3".to_string());
        expected.insert("currency".to_string(), "test4".to_string());
        expected.insert("status".to_string(), "test5".to_string());
        expected.insert("created_by_id".to_string(), created_by_id.to_string());
        expected.insert("created_by".to_string(), "test6".to_string());
        expected.insert("created_at".to_string(), date_test.to_rfc3339());
        expected.insert("updated_at".to_string(), date_test.to_rfc3339());
        expected.insert("deleted_at".to_string(), "".to_string());

        let input: IndexMap<String, String> = InventoryResolved {
            id,
            product_id,
            product: "test1".to_string(),
            warehouse_id,
            warehouse: "test2".to_string(),
            quantity_on_hand: "10".parse().unwrap(),
            quantity_reserved: "20".parse().unwrap(),
            quantity_available: "30".parse().unwrap(),
            minimum_stock: Some("40".parse().unwrap()),
            maximum_stock: Some("50".parse().unwrap()),
            currency_code: "test3".to_string(),
            currency: "test4".to_string(),
            status: "test5".to_string(),
            created_by_id,
            created_by: "test6".to_string(),
            created_at: date_test,
            updated_at: date_test,
            deleted_at: None,
        }
        .into();

        assert_eq!(input, expected);
    }
}
