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

use crate::tenant::taxes::model::TaxResolved;

#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct TaxResolvedPrint {
    pub id: Uuid,
    pub rate: Option<BigDecimal>,
    pub description: String,
    pub country_code: String,
    pub country: String,
    pub tax_category: String,
    pub is_rate_applicable: bool,
    pub legal_text: Option<String>,
    pub reporting_code: Option<String>,
    pub is_default: bool,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl TaxResolvedPrint {
    pub fn from_tax_resolved(tax_resolved: TaxResolved, tz: Tz) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: tax_resolved.id,
            rate: tax_resolved.rate,
            description: tax_resolved.description,
            country_code: tax_resolved.country_code,
            country: tax_resolved.country,
            tax_category: Self::map_tax_category(&tax_resolved.tax_category),
            is_rate_applicable: tax_resolved.is_rate_applicable,
            legal_text: tax_resolved.legal_text,
            reporting_code: tax_resolved.reporting_code,
            is_default: tax_resolved.is_default,
            status: Self::map_status(&tax_resolved.status),
            created_by_id: tax_resolved.created_by_id,
            created_by: tax_resolved.created_by,
            created_at: tax_resolved
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: tax_resolved
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            deleted_at: tax_resolved
                .deleted_at
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
        }
    }
    fn map_status(status: &str) -> String {
        match status {
            "active" => "Aktív",
            "draft" => "Vázlat",
            "inactive" => "Inaktív",
            _ => "Ismeretlen státusz",
        }
        .to_string()
    }
    fn map_tax_category(tax_category: &str) -> String {
        match tax_category {
            "standard" => "Általános",
            "reduced" => "Kedvezményes",
            "zero" => "Nulla kulcsos",
            _ => "Ismeretlen adó kategória",
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
    fn test_from_tax_resolved() {
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let input_date: DateTime<Utc> = "2026-01-01T01:00:00Z".parse().unwrap();
        let tz: Tz = "Europe/Budapest".parse().unwrap();
        let output_date = "2026. 01. 01. 02:00:00 (Europe/Budapest)".to_string();
        let tax_resolved = TaxResolved {
            id: tax_id,
            rate: Some("10".parse().unwrap()),
            description: "Test tax".to_string(),
            country_code: "HUF".to_string(),
            country: "HU".to_string(),
            tax_category: "standard".to_string(),
            is_rate_applicable: true,
            legal_text: None,
            reporting_code: None,
            is_default: true,
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: input_date,
            updated_at: input_date,
            deleted_at: None,
        };
        let tax_resolved_print = TaxResolvedPrint::from_tax_resolved(tax_resolved, tz);
        let tax_resolved_print_expected = TaxResolvedPrint {
            id: tax_id,
            rate: Some("10".parse().unwrap()),
            description: "Test tax".to_string(),
            country_code: "HUF".to_string(),
            country: "HU".to_string(),
            tax_category: "Általános".to_string(),
            is_rate_applicable: true,
            legal_text: None,
            reporting_code: None,
            is_default: true,
            status: "Aktív".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: output_date.clone(),
            updated_at: output_date,
            deleted_at: None,
        };

        assert_eq!(tax_resolved_print, tax_resolved_print_expected);
    }
}
