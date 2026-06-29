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

use crate::tenant::worksheets::model::WorksheetResolved;

#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct WorksheetResolvedPrint {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub customer_id: Uuid,
    pub customer: String,
    pub project_id: Option<Uuid>,
    pub project: Option<String>,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub net_material_cost: BigDecimal,
    pub gross_material_cost: BigDecimal,
    pub net_work_cost: BigDecimal,
    pub gross_work_cost: BigDecimal,
}

impl WorksheetResolvedPrint {
    pub fn from_worksheet_resolved(worksheet_resolved: WorksheetResolved, tz: Tz) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: worksheet_resolved.id,
            name: worksheet_resolved.name,
            description: worksheet_resolved.description,
            customer_id: worksheet_resolved.customer_id,
            customer: worksheet_resolved.customer,
            project_id: worksheet_resolved.project_id,
            project: worksheet_resolved.project,
            created_by_id: worksheet_resolved.created_by_id,
            created_by: worksheet_resolved.created_by,
            status: Self::map_status(&worksheet_resolved.status),
            created_at: worksheet_resolved
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: worksheet_resolved
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            deleted_at: worksheet_resolved
                .deleted_at
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
            net_material_cost: worksheet_resolved.net_material_cost,
            gross_material_cost: worksheet_resolved.gross_material_cost,
            net_work_cost: worksheet_resolved.net_work_cost,
            gross_work_cost: worksheet_resolved.gross_work_cost,
        }
    }
    fn map_status(status: &str) -> String {
        match status {
            "active" => "Aktív",
            "inactive" => "Inaktív",
            _ => "Ismeretlen státusz!",
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
    fn test_from_worksheet_resolved() {
        let worksheet_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let project_id = Some(Uuid::new_v4());
        let created_by_id = Uuid::new_v4();
        let input_date: DateTime<Utc> = "2026-01-01T01:00:00Z".parse().unwrap();
        let tz: Tz = "Europe/Budapest".parse().unwrap();
        let output_date = "2026. 01. 01. 02:00:00 (Europe/Budapest)".to_string();
        let worksheet_resolved = WorksheetResolved {
            id: worksheet_id,
            name: "Test worksheet".to_string(),
            description: None,
            customer_id,
            customer: "Test customer".to_string(),
            project_id,
            project: Some("Test project".to_string()),
            created_by_id,
            created_by: "Test user".to_string(),
            status: "active".to_string(),
            created_at: input_date,
            updated_at: input_date,
            deleted_at: None,
            net_material_cost: "10".parse().unwrap(),
            gross_material_cost: "20".parse().unwrap(),
            net_work_cost: "30".parse().unwrap(),
            gross_work_cost: "40".parse().unwrap(),
        };
        let worksheet_resolved_print =
            WorksheetResolvedPrint::from_worksheet_resolved(worksheet_resolved, tz);
        let worksheet_resolved_print_expected = WorksheetResolvedPrint {
            id: worksheet_id,
            name: "Test worksheet".to_string(),
            description: None,
            customer_id,
            customer: "Test customer".to_string(),
            project_id,
            project: Some("Test project".to_string()),
            created_by_id,
            created_by: "Test user".to_string(),
            status: "Aktív".to_string(),
            created_at: output_date.clone(),
            updated_at: output_date,
            deleted_at: None,
            net_material_cost: "10".parse().unwrap(),
            gross_material_cost: "20".parse().unwrap(),
            net_work_cost: "30".parse().unwrap(),
            gross_work_cost: "40".parse().unwrap(),
        };
        assert_eq!(worksheet_resolved_print, worksheet_resolved_print_expected);
    }
}
