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

use chrono::{DateTime, Local};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Warehouse {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WarehouseResolved {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

impl From<WarehouseResolved> for IndexMap<String, String> {
    fn from(value: WarehouseResolved) -> Self {
        let mut map = IndexMap::new();
        map.insert("id".to_string(), value.id.to_string());
        map.insert("name".to_string(), value.name);
        map.insert(
            "contact_name".to_string(),
            value.contact_name.unwrap_or_default(),
        );
        map.insert(
            "contact_phone".to_string(),
            value.contact_phone.unwrap_or_default(),
        );
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
    fn warehouse_resolved_into_index_map() {
        let id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let date_test = Local::now();
        let mut expected = IndexMap::new();

        expected.insert("id".to_string(), id.to_string());
        expected.insert("name".to_string(), "test1".to_string());
        expected.insert("contact_name".to_string(), "test2".to_string());
        expected.insert("contact_phone".to_string(), "test3".to_string());
        expected.insert("status".to_string(), "test4".to_string());
        expected.insert("created_by_id".to_string(), created_by_id.to_string());
        expected.insert("created_by".to_string(), "test5".to_string());
        expected.insert("created_at".to_string(), date_test.to_rfc3339());
        expected.insert("updated_at".to_string(), date_test.to_rfc3339());
        expected.insert("deleted_at".to_string(), "".to_string());

        let input: IndexMap<String, String> = WarehouseResolved {
            id,
            name: "test1".to_string(),
            contact_name: Some("test2".to_string()),
            contact_phone: Some("test3".to_string()),
            status: "test4".to_string(),
            created_by_id,
            created_by: "test5".to_string(),
            created_at: date_test,
            updated_at: date_test,
            deleted_at: None,
        }
        .into();

        assert_eq!(input, expected);
    }
}
