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

use crate::tenant::inventory_reservations::model::InventoryReservationResolved;

#[derive(Serialize, PartialEq, Debug)]
pub struct InventoryReservationResolvedPrint {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub quantity: BigDecimal,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub reserved_until: Option<String>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

impl InventoryReservationResolvedPrint {
    pub fn from_inventory_reservation_resolved(
        inventory_reservation_resolved: InventoryReservationResolved,
        tz: Tz,
    ) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: inventory_reservation_resolved.id,
            inventory_id: inventory_reservation_resolved.inventory_id,
            quantity: inventory_reservation_resolved.quantity,
            reference_type: inventory_reservation_resolved
                .reference_type
                .map(|v| Self::map_reference_type(&v)),
            reference_id: inventory_reservation_resolved.reference_id,
            reserved_until: inventory_reservation_resolved
                .reserved_until
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
            status: Self::map_status(&inventory_reservation_resolved.status),
            created_by_id: inventory_reservation_resolved.created_by_id,
            created_by: inventory_reservation_resolved.created_by,
            created_at: inventory_reservation_resolved
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: inventory_reservation_resolved
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
        }
    }
    fn map_status(status: &str) -> String {
        match status {
            "active" => "Aktív",
            "fulfilled" => "Teljesített",
            "cancelled" => "Lemondott",
            "expired" => "Lejárt",
            _ => "Ismeretlen státusz",
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

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_inventory_reservation_resolved() {
        let inventory_reservation_id = Uuid::new_v4();
        let inventory_id = Uuid::new_v4();
        let reference_id = Some(Uuid::new_v4());
        let created_by_id = Uuid::new_v4();
        let input_date: DateTime<Utc> = "2026-01-01T01:00:00Z".parse().unwrap();
        let tz: Tz = "Europe/Budapest".parse().unwrap();
        let output_date = "2026. 01. 01. 02:00:00 (Europe/Budapest)".to_string();
        let inventory_reservation_resolved = InventoryReservationResolved {
            id: inventory_reservation_id,
            inventory_id,
            quantity: "10".parse().unwrap(),
            reference_type: Some("worksheets".to_string()),
            reference_id,
            reserved_until: Some(input_date),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: input_date,
            updated_at: input_date,
        };
        let inventory_resevation_resolved_print =
            InventoryReservationResolvedPrint::from_inventory_reservation_resolved(
                inventory_reservation_resolved,
                tz,
            );
        let inventory_resevation_resolved_print_expected = InventoryReservationResolvedPrint {
            id: inventory_reservation_id,
            inventory_id,
            quantity: "10".parse().unwrap(),
            reference_type: Some("Munkalap".to_string()),
            reference_id,
            reserved_until: Some(output_date.clone()),
            status: "Aktív".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: output_date.clone(),
            updated_at: output_date,
        };
        assert_eq!(
            inventory_resevation_resolved_print,
            inventory_resevation_resolved_print_expected
        );
    }
}
