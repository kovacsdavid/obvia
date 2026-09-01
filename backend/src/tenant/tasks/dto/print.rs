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
use crate::tenant::{services::dto::print::ServicesResolvedPrint, tasks::model::TaskResolved};
use bigdecimal::BigDecimal;
use chrono_tz::Tz;
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize, PartialEq, Debug, Builder)]
#[builder(build_fn(error = "CommonBuilderError"))]
pub struct TaskResolvedPrint {
    pub id: Uuid,
    pub service: ServicesResolvedPrint,
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
    pub fn new(
        task_resolved: TaskResolved,
        service_resolved_print: ServicesResolvedPrint,
        tz: Tz,
    ) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: task_resolved.id,
            service: service_resolved_print,
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

pub fn test_task_resolved_print_builder(
    service_resolved_print: ServicesResolvedPrint,
) -> TaskResolvedPrintBuilder {
    let mut builder = TaskResolvedPrintBuilder::default();
    builder
        .id(Uuid::new_v4())
        .service(service_resolved_print)
        .currency_code("HUF".to_string())
        .quantity(Some("10".parse().unwrap()))
        .price(Some("1000".parse().unwrap()))
        .tax_id(Uuid::new_v4())
        .tax("Test tax".to_string())
        .created_by_id(Uuid::new_v4())
        .created_by("Test User".to_string())
        .status("active".to_string())
        .priority(Some("normal".to_string()))
        .due_date(Some(TEST_TIME_TZ.clone()))
        .created_at(TEST_TIME_TZ.clone())
        .updated_at(TEST_TIME_TZ.clone())
        .deleted_at(None)
        .description(Some("Test description".to_string()));

    builder
}

#[cfg(test)]
mod tests {
    use crate::{
        common::TEST_TZ,
        tenant::{
            services::model::tests::test_service_resolved_builder,
            tasks::model::tests::test_task_resolved_builder,
        },
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_task_resolved_print_from_task_resolved() {
        let task_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let task_resolved = test_task_resolved_builder()
            .id(task_id)
            .tax_id(tax_id)
            .created_by_id(created_by_id)
            .build()
            .unwrap();
        let service_resolved = test_service_resolved_builder().build().unwrap();
        let service_resolved_print = ServicesResolvedPrint::new(service_resolved, *TEST_TZ);
        let task_resolved_print =
            TaskResolvedPrint::new(task_resolved, service_resolved_print.clone(), *TEST_TZ);
        let task_resolved_print_expected = TaskResolvedPrint {
            id: task_id,
            service: service_resolved_print,
            currency_code: "HUF".to_string(),
            quantity: Some("10".parse().unwrap()),
            price: Some("1000".parse().unwrap()),
            tax_id,
            tax: "Test tax".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            status: "Aktív".to_string(),
            priority: Some("Normál".to_string()),
            due_date: Some(TEST_TIME_TZ.clone()),
            created_at: TEST_TIME_TZ.clone(),
            updated_at: TEST_TIME_TZ.clone(),
            deleted_at: None,
            description: Some("Test description".to_string()),
        };

        assert_eq!(task_resolved_print, task_resolved_print_expected);
    }
}
