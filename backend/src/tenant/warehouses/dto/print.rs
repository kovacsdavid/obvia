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

use chrono_tz::Tz;
use serde::Serialize;
use uuid::Uuid;

use crate::tenant::warehouses::model::WarehouseResolved;

#[derive(Serialize, PartialEq, Debug)]
pub struct WarehouseResolvedPrint {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl WarehouseResolvedPrint {
    pub fn from_warehouse_resolved(warehouse_resolved: WarehouseResolved, tz: Tz) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: warehouse_resolved.id,
            name: warehouse_resolved.name,
            contact_name: warehouse_resolved.contact_name,
            contact_phone: warehouse_resolved.contact_phone,
            status: Self::map_status(&warehouse_resolved.status),
            created_by_id: warehouse_resolved.created_by_id,
            created_by: warehouse_resolved.created_by,
            created_at: warehouse_resolved
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: warehouse_resolved
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            deleted_at: warehouse_resolved
                .deleted_at
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
        }
    }
    fn map_status(status: &str) -> String {
        match status {
            "active" => "Aktív",
            "inactive" => "Inaktív",
            "maintenance" => "Karbantartás alatt",
            "closed" => "Véglegesen bezárt",
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
    fn test_from_warehouse_resolved() {
        let warehouse_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let input_date: DateTime<Utc> = "2026-01-01T01:00:00Z".parse().unwrap();
        let tz: Tz = "Europe/Budapest".parse().unwrap();
        let output_date = "2026. 01. 01. 02:00:00 (Europe/Budapest)".to_string();
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
        let warehouse_resolved_print =
            WarehouseResolvedPrint::from_warehouse_resolved(warehouse_resolved, tz);
        let warehouse_resolved_print_expected = WarehouseResolvedPrint {
            id: warehouse_id,
            name: "Test warehouse".to_string(),
            contact_name: None,
            contact_phone: None,
            status: "Aktív".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: output_date.clone(),
            updated_at: output_date,
            deleted_at: None,
        };

        assert_eq!(warehouse_resolved_print, warehouse_resolved_print_expected);
    }
}
