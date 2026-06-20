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

use crate::tenant::services::model::ServiceResolved;
use bigdecimal::BigDecimal;
use chrono_tz::Tz;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, PartialEq, Debug)]
pub struct ServicesResolvedPrint {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub default_price: Option<BigDecimal>,
    pub default_tax_id: Option<Uuid>,
    pub default_tax: Option<String>,
    pub currency_code: Option<String>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl ServicesResolvedPrint {
    pub fn from_service_resolved(service_resolved: ServiceResolved, tz: Tz) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: service_resolved.id,
            name: service_resolved.name,
            description: service_resolved.description,
            default_price: service_resolved.default_price,
            default_tax_id: service_resolved.default_tax_id,
            default_tax: service_resolved.default_tax,
            currency_code: service_resolved.currency_code,
            status: Self::map_status(&service_resolved.status),
            created_by_id: service_resolved.created_by_id,
            created_by: service_resolved.created_by,
            created_at: service_resolved
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: service_resolved
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            deleted_at: service_resolved
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
    use super::*;
    use chrono::{DateTime, Utc};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_service_resolved() {
        let service_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let default_tax_id = Some(Uuid::new_v4());
        let input_date: DateTime<Utc> = "2026-01-01T01:00:00Z".parse().unwrap();
        let tz: Tz = "Europe/Budapest".parse().unwrap();
        let output_date = "2026. 01. 01. 02:00:00 (Europe/Budapest)".to_string();
        let service_resolved = ServiceResolved {
            id: service_id,
            name: "Test service".to_string(),
            description: Some("Test description".to_string()),
            default_price: Some("10".parse().unwrap()),
            default_tax_id,
            default_tax: Some("Test tax".to_string()),
            currency_code: Some("HUF".to_string()),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: input_date,
            updated_at: input_date,
            deleted_at: None,
        };
        let service_resolved_print =
            ServicesResolvedPrint::from_service_resolved(service_resolved, tz);
        let service_resolved_print_expected = ServicesResolvedPrint {
            id: service_id,
            name: "Test service".to_string(),
            description: Some("Test description".to_string()),
            default_price: Some("10".parse().unwrap()),
            default_tax_id,
            default_tax: Some("Test tax".to_string()),
            currency_code: Some("HUF".to_string()),
            status: "Aktív".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: output_date.clone(),
            updated_at: output_date,
            deleted_at: None,
        };
        assert_eq!(service_resolved_print, service_resolved_print_expected);
    }
}
