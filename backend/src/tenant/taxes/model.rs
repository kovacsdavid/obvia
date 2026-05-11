/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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
use chrono::{DateTime, Local};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tax {
    pub id: Uuid,
    pub rate: Option<BigDecimal>,
    pub description: String,
    pub country_code: String,
    pub tax_category: String,
    pub is_rate_applicable: bool,
    pub legal_text: Option<String>,
    pub reporting_code: Option<String>,
    pub is_default: bool,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaxResolved {
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
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

impl From<TaxResolved> for IndexMap<String, String> {
    fn from(value: TaxResolved) -> Self {
        let mut map = IndexMap::new();

        map.insert("id".to_string(), value.id.to_string());
        map.insert(
            "rate".to_string(),
            value.rate.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("description".to_string(), value.description);
        map.insert("country_code".to_string(), value.country_code);
        map.insert("country".to_string(), value.country);
        map.insert("tax_category".to_string(), value.tax_category);
        map.insert(
            "is_rate_applicable".to_string(),
            value.is_rate_applicable.to_string(),
        );
        map.insert(
            "legal_text".to_string(),
            value.legal_text.unwrap_or_default(),
        );
        map.insert(
            "reporting_code".to_string(),
            value.reporting_code.unwrap_or_default(),
        );
        map.insert("is_default".to_string(), value.is_default.to_string());
        map.insert("status".to_string(), value.status);
        map.insert("created_by_id".to_string(), value.created_by_id.to_string());
        map.insert("created_by".to_string(), value.created_by);
        map.insert("created_at".to_string(), value.created_at.to_rfc3339());
        map.insert("updated_at".to_string(), value.updated_at.to_rfc3339());
        map.insert(
            "deleted_at".to_string(),
            value
                .deleted_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
        );

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tax_resolved_into_index_map() {
        let id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let date_test = Local::now();
        let mut expected = IndexMap::new();

        expected.insert("id".to_string(), id.to_string());
        expected.insert("rate".to_string(), "27".to_string());
        expected.insert("description".to_string(), "test1".to_string());
        expected.insert("country_code".to_string(), "test2".to_string());
        expected.insert("country".to_string(), "test3".to_string());
        expected.insert("tax_category".to_string(), "test4".to_string());
        expected.insert("is_rate_applicable".to_string(), "true".to_string());
        expected.insert("legal_text".to_string(), "test5".to_string());
        expected.insert("reporting_code".to_string(), "test6".to_string());
        expected.insert("is_default".to_string(), "false".to_string());
        expected.insert("status".to_string(), "test7".to_string());
        expected.insert("created_by_id".to_string(), created_by_id.to_string());
        expected.insert("created_by".to_string(), "test8".to_string());
        expected.insert("created_at".to_string(), date_test.to_rfc3339());
        expected.insert("updated_at".to_string(), date_test.to_rfc3339());
        expected.insert("deleted_at".to_string(), "".to_string());

        let input: IndexMap<String, String> = TaxResolved {
            id,
            rate: Some("27".parse().unwrap()),
            description: "test1".to_string(),
            country_code: "test2".to_string(),
            country: "test3".to_string(),
            tax_category: "test4".to_string(),
            is_rate_applicable: true,
            legal_text: Some("test5".to_string()),
            reporting_code: Some("test6".to_string()),
            is_default: false,
            status: "test7".to_string(),
            created_by_id,
            created_by: "test8".to_string(),
            created_at: date_test,
            updated_at: date_test,
            deleted_at: None,
        }
        .into();

        assert_eq!(input, expected);
    }
}
