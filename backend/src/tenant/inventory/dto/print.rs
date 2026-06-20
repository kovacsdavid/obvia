/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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
use chrono_tz::Tz;
use serde::Serialize;
use uuid::Uuid;

use crate::tenant::inventory::model::InventoryResolved;

#[derive(Serialize, PartialEq, Debug)]
pub struct InventoryResolvedPrint {
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
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl InventoryResolvedPrint {
    pub fn from_inventory_resolved(inventory_resolved: InventoryResolved, tz: Tz) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: inventory_resolved.id,
            product_id: inventory_resolved.product_id,
            product: inventory_resolved.product,
            warehouse_id: inventory_resolved.warehouse_id,
            warehouse: inventory_resolved.warehouse,
            quantity_on_hand: inventory_resolved.quantity_on_hand,
            quantity_reserved: inventory_resolved.quantity_reserved,
            quantity_available: inventory_resolved.quantity_available,
            minimum_stock: inventory_resolved.minimum_stock,
            maximum_stock: inventory_resolved.maximum_stock,
            currency_code: inventory_resolved.currency_code,
            currency: inventory_resolved.currency,
            status: Self::map_status(&inventory_resolved.status),
            created_by_id: inventory_resolved.created_by_id,
            created_by: inventory_resolved.created_by,
            created_at: inventory_resolved
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: inventory_resolved
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            deleted_at: inventory_resolved
                .deleted_at
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
        }
    }
    fn map_status(status: &str) -> String {
        match status {
            "active" => "Aktív",
            "discontinued" => "Kivezetett",
            "inactive" => "Inaktív",
            _ => "Ismeretlen státusz",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_inventory_resolved() {
        let inventory_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let input_date: DateTime<Utc> = "2026-01-01T01:00:00Z".parse().unwrap();
        let tz: Tz = "Europe/Budapest".parse().unwrap();
        let output_date = "2026. 01. 01. 02:00:00 (Europe/Budapest)".to_string();
        let inventory_resolved = InventoryResolved {
            id: inventory_id,
            product_id,
            product: "Teszt termék".to_string(),
            warehouse_id,
            warehouse: "Teszt raktár".to_string(),
            quantity_on_hand: "10".parse().unwrap(),
            quantity_reserved: "20".parse().unwrap(),
            quantity_available: "30".parse().unwrap(),
            minimum_stock: Some("40".parse().unwrap()),
            maximum_stock: Some("50".parse().unwrap()),
            currency_code: "HUF".to_string(),
            currency: "Forint".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: input_date,
            updated_at: input_date,
            deleted_at: None,
        };
        let inventory_resolved_print =
            InventoryResolvedPrint::from_inventory_resolved(inventory_resolved, tz);
        let inventory_resolved_print_expected = InventoryResolvedPrint {
            id: inventory_id,
            product_id,
            product: "Teszt termék".to_string(),
            warehouse_id,
            warehouse: "Teszt raktár".to_string(),
            quantity_on_hand: "10".parse().unwrap(),
            quantity_reserved: "20".parse().unwrap(),
            quantity_available: "30".parse().unwrap(),
            minimum_stock: Some("40".parse().unwrap()),
            maximum_stock: Some("50".parse().unwrap()),
            currency_code: "HUF".to_string(),
            currency: "Forint".to_string(),
            status: "Aktív".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: output_date.clone(),
            updated_at: output_date.clone(),
            deleted_at: None,
        };

        assert_eq!(inventory_resolved_print, inventory_resolved_print_expected);
    }
}
