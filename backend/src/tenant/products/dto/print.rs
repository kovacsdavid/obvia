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

use crate::tenant::products::model::ProductResolved;
use chrono_tz::Tz;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct ProductsResolvedPrint {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub unit_of_measure_id: Uuid,
    pub unit_of_measure: String,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl ProductsResolvedPrint {
    pub fn from_product_resolved(product_resolved: ProductResolved, tz: Tz) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: product_resolved.id,
            name: product_resolved.name,
            description: product_resolved.description,
            unit_of_measure_id: product_resolved.unit_of_measure_id,
            unit_of_measure: product_resolved.unit_of_measure,
            status: Self::map_status(&product_resolved.status),
            created_by_id: product_resolved.created_by_id,
            created_by: product_resolved.created_by,
            created_at: product_resolved
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: product_resolved
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),

            deleted_at: product_resolved
                .deleted_at
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
        }
    }
    fn map_status(status: &str) -> String {
        match status {
            "active" => "Aktív",
            "inactive" => "Inaktív",
            _ => "Ismeretlen státusz",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_from_product_resolved() {
        let product_id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let input_date: DateTime<Utc> = "2026-01-01T01:00:00Z".parse().unwrap();
        let tz: Tz = "Europe/Budapest".parse().unwrap();
        let output_date = "2026. 01. 01. 02:00:00 (Europe/Budapest)".to_string();
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
        let product_resolved_print =
            ProductsResolvedPrint::from_product_resolved(product_resolved, tz);
        let product_resolved_print_expected = ProductsResolvedPrint {
            id: product_id,
            name: "Test product".to_string(),
            description: None,
            unit_of_measure_id,
            unit_of_measure: "cm".to_string(),
            status: "Aktív".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: output_date.clone(),
            updated_at: output_date,
            deleted_at: None,
        };
        assert_eq!(product_resolved_print, product_resolved_print_expected);
    }
}
