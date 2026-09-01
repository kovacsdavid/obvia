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

use crate::common::CommonBuilderError;
use crate::common::TEST_TIME_TZ;
use crate::tenant::{
    inventory::dto::print::InventoryResolvedPrint,
    inventory_movements::model::InventoryMovementResolved,
};
use bigdecimal::BigDecimal;
use chrono_tz::Tz;
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize, PartialEq, Debug, Builder)]
#[builder(build_fn(error = "CommonBuilderError"))]
pub struct InventoryMovementsResolvedPrint {
    id: Uuid,
    inventory: InventoryResolvedPrint,
    movement_type: String,
    quantity: BigDecimal,
    reference_type: Option<String>,
    reference_id: Option<Uuid>,
    unit_price: Option<BigDecimal>,
    total_price: Option<BigDecimal>,
    tax_id: Uuid,
    tax: Option<String>,
    movement_date: String,
    created_by_id: Uuid,
    created_by: String,
    created_at: String,
}

impl InventoryMovementsResolvedPrint {
    pub fn new(
        inventory_movement_resolved: InventoryMovementResolved,
        inventory_resolved_print: InventoryResolvedPrint,
        tz: Tz,
    ) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: inventory_movement_resolved.id,
            inventory: inventory_resolved_print,
            movement_type: Self::map_movement_type(&inventory_movement_resolved.movement_type),
            quantity: inventory_movement_resolved.quantity,
            reference_type: inventory_movement_resolved
                .reference_type
                .map(|v| Self::map_reference_type(&v)),
            reference_id: inventory_movement_resolved.reference_id,
            unit_price: inventory_movement_resolved.unit_price,
            total_price: inventory_movement_resolved.total_price,
            tax_id: inventory_movement_resolved.tax_id,
            tax: inventory_movement_resolved.tax,
            movement_date: inventory_movement_resolved
                .movement_date
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            created_by_id: inventory_movement_resolved.created_by_id,
            created_by: inventory_movement_resolved.created_by,
            created_at: inventory_movement_resolved
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
        }
    }
    fn map_movement_type(movement_type: &str) -> String {
        match movement_type {
            "in" => "Bevétel",
            "out" => "Kiadás",
            _ => "Ismeretlen művelet",
        }
        .to_string()
    }
    fn map_reference_type(reference_type: &str) -> String {
        match reference_type {
            "worksheets" => "Munkalap",
            _ => "Ismeretlen referencia típus",
        }
        .to_string()
    }
}

pub fn test_inventory_movement_resolved_print_builder(
    inventory_resolved_print: InventoryResolvedPrint,
) -> InventoryMovementsResolvedPrintBuilder {
    let mut builder = InventoryMovementsResolvedPrintBuilder::default();
    builder
        .id(Uuid::new_v4())
        .inventory(inventory_resolved_print)
        .movement_type("Bevétel".to_string())
        .quantity("10".parse().unwrap())
        .reference_type(Some("Munkalap".to_string()))
        .reference_id(Some(Uuid::new_v4()))
        .unit_price(Some("20".parse().unwrap()))
        .total_price(Some("30".parse().unwrap()))
        .tax_id(Uuid::new_v4())
        .tax(Some("Test tax".to_string()))
        .movement_date(TEST_TIME_TZ.clone())
        .created_by_id(Uuid::new_v4())
        .created_by("Test User".to_string())
        .created_at(TEST_TIME_TZ.clone());

    builder
}

#[cfg(test)]
mod tests {
    use crate::tenant::{
        inventory::model::InventoryResolved,
        products::{dto::print::ProductsResolvedPrint, model::ProductResolved},
        warehouses::{dto::print::WarehouseResolvedPrint, model::WarehouseResolved},
    };

    use super::*;
    use chrono::{DateTime, Utc};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_inventory_movements_resolved() {
        let inventory_movement_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Some(Uuid::new_v4());
        let tax_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let warehouse_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();
        let input_date: DateTime<Utc> = "2026-01-01T01:00:00Z".parse().unwrap();
        let tz: Tz = "Europe/Budapest".parse().unwrap();
        let output_date = "2026. 01. 01. 02:00:00 (Europe/Budapest)".to_string();

        let inventory_movement_resolved = InventoryMovementResolved {
            id: inventory_movement_id,
            inventory_id,
            movement_type: "in".to_string(),
            quantity: "10".parse().unwrap(),
            reference_type: Some("worksheets".to_string()),
            reference_id,
            unit_price: Some("20".parse().unwrap()),
            total_price: Some("30".parse().unwrap()),
            tax_id,
            tax: Some("Áfa".to_string()),
            movement_date: input_date,
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: input_date,
        };
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
        let product_resolved = ProductResolved {
            id: product_id,
            name: "Test product".to_string(),
            description: None,
            unit_of_measure_id,
            unit_of_measure: "cm".to_string(),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: input_date,
            updated_at: input_date,
            deleted_at: None,
        };
        let product_resolved_print = ProductsResolvedPrint::new(product_resolved, tz);

        let warehouse_resolved = WarehouseResolved {
            id: warehouse_id,
            name: "Test warehouse".to_string(),
            contact_name: None,
            contact_phone: None,
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: input_date,
            updated_at: input_date,
            deleted_at: None,
        };
        let warehouse_resolved_print = WarehouseResolvedPrint::new(warehouse_resolved, tz);

        let inventory_resolved_print = InventoryResolvedPrint::new(
            inventory_resolved,
            product_resolved_print.clone(),
            warehouse_resolved_print.clone(),
            tz,
        );
        let inventory_movement_resolved_print = InventoryMovementsResolvedPrint::new(
            inventory_movement_resolved,
            inventory_resolved_print.clone(),
            tz,
        );
        let inventory_movement_resolved_print_expected = InventoryMovementsResolvedPrint {
            id: inventory_movement_id,
            inventory: inventory_resolved_print,
            movement_type: "Bevétel".to_string(),
            quantity: "10".parse().unwrap(),
            reference_type: Some("Munkalap".to_string()),
            reference_id,
            unit_price: Some("20".parse().unwrap()),
            total_price: Some("30".parse().unwrap()),
            tax_id,
            tax: Some("Áfa".to_string()),
            movement_date: output_date.clone(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: output_date,
        };

        assert_eq!(
            inventory_movement_resolved_print,
            inventory_movement_resolved_print_expected
        );
    }
}
