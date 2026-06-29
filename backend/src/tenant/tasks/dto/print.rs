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

use crate::tenant::tasks::model::TaskResolved;
use bigdecimal::BigDecimal;
use chrono_tz::Tz;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct TaskResolvedPrint {
    pub id: Uuid,
    pub worksheet_id: Uuid,
    pub worksheet: String,
    pub service_id: Uuid,
    pub service: String,
    pub currency_code: String,
    pub quantity: Option<BigDecimal>,
    pub price: Option<BigDecimal>,
    pub tax_id: Uuid,
    pub tax: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub status: String,
    pub priority: Option<String>,
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub description: Option<String>,
}

impl TaskResolvedPrint {
    pub fn from_task_resolved(task_resolved: TaskResolved, tz: Tz) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: task_resolved.id,
            worksheet_id: task_resolved.worksheet_id,
            worksheet: task_resolved.worksheet,
            service_id: task_resolved.service_id,
            service: task_resolved.service,
            currency_code: task_resolved.currency_code,
            quantity: task_resolved.quantity,
            price: task_resolved.price,
            tax_id: task_resolved.tax_id,
            tax: task_resolved.tax,
            created_by_id: task_resolved.created_by_id,
            created_by: task_resolved.created_by,
            status: Self::map_status(&task_resolved.status),
            priority: task_resolved.priority.map(|v| Self::map_priority(&v)),
            due_date: task_resolved
                .due_date
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
            created_at: task_resolved
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: task_resolved
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            deleted_at: task_resolved
                .deleted_at
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
            description: task_resolved.description,
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
    fn map_priority(priority: &str) -> String {
        match priority {
            "low" => "Alacsony",
            "normal" => "Normál",
            "high" => "Magas",
            _ => "",
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
    fn test_from_task_resolved() {
        let task_id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let input_date: DateTime<Utc> = "2026-01-01T01:00:00Z".parse().unwrap();
        let tz: Tz = "Europe/Budapest".parse().unwrap();
        let output_date = "2026. 01. 01. 02:00:00 (Europe/Budapest)".to_string();
        let task_resolved = TaskResolved {
            id: task_id,
            worksheet_id,
            worksheet: "Test worksheet".to_string(),
            service_id,
            service: "Test service".to_string(),
            currency_code: "HUF".to_string(),
            quantity: Some("10".parse().unwrap()),
            price: Some("20".parse().unwrap()),
            tax_id,
            tax: "Test tax".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            status: "active".to_string(),
            priority: Some("normal".to_string()),
            due_date: Some(input_date),
            created_at: input_date,
            updated_at: input_date,
            deleted_at: None,
            description: Some("Test description".to_string()),
        };
        let task_resolved_print = TaskResolvedPrint::from_task_resolved(task_resolved, tz);
        let task_resolved_print_expected = TaskResolvedPrint {
            id: task_id,
            worksheet_id,
            worksheet: "Test worksheet".to_string(),
            service_id,
            service: "Test service".to_string(),
            currency_code: "HUF".to_string(),
            quantity: Some("10".parse().unwrap()),
            price: Some("20".parse().unwrap()),
            tax_id,
            tax: "Test tax".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            status: "Aktív".to_string(),
            priority: Some("Normál".to_string()),
            due_date: Some(output_date.clone()),
            created_at: output_date.clone(),
            updated_at: output_date,
            deleted_at: None,
            description: Some("Test description".to_string()),
        };

        assert_eq!(task_resolved_print, task_resolved_print_expected);
    }
}
