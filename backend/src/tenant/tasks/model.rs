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
pub struct Task {
    pub id: Uuid,
    pub worksheet_id: Uuid,
    pub service_id: Uuid,
    pub currency_code: String,
    pub quantity: Option<BigDecimal>,
    pub price: Option<BigDecimal>,
    pub tax_id: Uuid,
    pub created_by_id: Uuid,
    pub status: String,
    pub priority: Option<String>,
    pub due_date: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskResolved {
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
    pub due_date: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
    pub description: Option<String>,
}

impl From<TaskResolved> for IndexMap<String, String> {
    fn from(value: TaskResolved) -> Self {
        let mut map = IndexMap::new();

        map.insert("id".to_string(), value.id.to_string());
        map.insert("worksheet_id".to_string(), value.worksheet_id.to_string());
        map.insert("worksheet".to_string(), value.worksheet);
        map.insert("service_id".to_string(), value.service_id.to_string());
        map.insert("service".to_string(), value.service);
        map.insert("currency_code".to_string(), value.currency_code);
        map.insert(
            "quantity".to_string(),
            value.quantity.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert(
            "price".to_string(),
            value.price.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("tax_id".to_string(), value.tax_id.to_string());
        map.insert("tax".to_string(), value.tax);
        map.insert("created_by_id".to_string(), value.created_by_id.to_string());
        map.insert("created_by".to_string(), value.created_by);
        map.insert("status".to_string(), value.status);
        map.insert("priority".to_string(), value.priority.unwrap_or_default());
        map.insert(
            "due_date".to_string(),
            value.due_date.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
        );
        map.insert("created_at".to_string(), value.created_at.to_rfc3339());
        map.insert("updated_at".to_string(), value.updated_at.to_rfc3339());
        map.insert(
            "deleted_at".to_string(),
            value
                .deleted_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
        );
        map.insert(
            "description".to_string(),
            value.description.unwrap_or_default(),
        );

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn task_resolved_into_index_map() {
        let id = Uuid::new_v4();
        let worksheet_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let date_test = Local::now();
        let mut expected = IndexMap::new();

        expected.insert("id".to_string(), id.to_string());
        expected.insert("worksheet_id".to_string(), worksheet_id.to_string());
        expected.insert("worksheet".to_string(), "test1".to_string());
        expected.insert("service_id".to_string(), service_id.to_string());
        expected.insert("service".to_string(), "test2".to_string());
        expected.insert("currency_code".to_string(), "test3".to_string());
        expected.insert("quantity".to_string(), "10".parse().unwrap());
        expected.insert("price".to_string(), "20".parse().unwrap());
        expected.insert("tax_id".to_string(), tax_id.to_string());
        expected.insert("tax".to_string(), "test4".to_string());
        expected.insert("created_by_id".to_string(), created_by_id.to_string());
        expected.insert("created_by".to_string(), "test5".to_string());
        expected.insert("status".to_string(), "test6".to_string());
        expected.insert("priority".to_string(), "test7".to_string());
        expected.insert("due_date".to_string(), date_test.to_rfc3339());
        expected.insert("created_at".to_string(), date_test.to_rfc3339());
        expected.insert("updated_at".to_string(), date_test.to_rfc3339());
        expected.insert("deleted_at".to_string(), "".to_string());
        expected.insert("description".to_string(), "test8".to_string());

        let input: IndexMap<String, String> = TaskResolved {
            id,
            worksheet_id,
            worksheet: "test1".to_string(),
            service_id,
            service: "test2".to_string(),
            currency_code: "test3".to_string(),
            quantity: Some("10".parse().unwrap()),
            price: Some("20".parse().unwrap()),
            tax_id,
            tax: "test4".to_string(),
            created_by_id,
            created_by: "test5".to_string(),
            status: "test6".to_string(),
            priority: Some("test7".to_string()),
            due_date: Some(date_test),
            created_at: date_test,
            updated_at: date_test,
            deleted_at: None,
            description: Some("test8".to_string()),
        }
        .into();

        assert_eq!(input, expected);
    }
}
