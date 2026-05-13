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
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub unit_of_measure_id: Uuid,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_at: Option<DateTime<Local>>, // NOTE: possibly this should not be option
    pub updated_at: Option<DateTime<Local>>, // NOTE: possibly this should not be option
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductResolved {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub unit_of_measure_id: Uuid,
    pub unit_of_measure: String,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: Option<DateTime<Local>>, // NOTE: possibly this should not be option
    pub updated_at: Option<DateTime<Local>>, // NOTE: possibly this should not be option
    pub deleted_at: Option<DateTime<Local>>,
}

impl From<ProductResolved> for IndexMap<String, String> {
    fn from(value: ProductResolved) -> Self {
        let mut map = IndexMap::new();

        map.insert("id".to_string(), value.id.to_string());
        map.insert("name".to_string(), value.name);
        map.insert(
            "description".to_string(),
            value.description.unwrap_or_default(),
        );
        map.insert(
            "unit_of_measure_id".to_string(),
            value.unit_of_measure_id.to_string(),
        );
        map.insert("unit_of_measure".to_string(), value.unit_of_measure);
        map.insert("status".to_string(), value.status);
        map.insert("created_by_id".to_string(), value.created_by_id.to_string());
        map.insert("created_by".to_string(), value.created_by);
        map.insert(
            "created_at".to_string(),
            value
                .created_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
        );
        map.insert(
            "updated_at".to_string(),
            value
                .updated_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
        );
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

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UnitOfMeasure {
    pub id: Uuid,
    pub unit_of_measure: String,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn products_resolved_into_index_map() {
        let id = Uuid::new_v4();
        let unit_of_measure_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let date_test = Local::now();
        let mut exptected = IndexMap::new();

        exptected.insert("id".to_string(), id.to_string());
        exptected.insert("name".to_string(), "test1".to_string());
        exptected.insert("description".to_string(), "test2".to_string());
        exptected.insert(
            "unit_of_measure_id".to_string(),
            unit_of_measure_id.to_string(),
        );
        exptected.insert("unit_of_measure".to_string(), "test3".to_string());
        exptected.insert("status".to_string(), "test4".to_string());
        exptected.insert("created_by_id".to_string(), created_by_id.to_string());
        exptected.insert("created_by".to_string(), "test5".to_string());
        exptected.insert("created_at".to_string(), date_test.to_rfc3339());
        exptected.insert("updated_at".to_string(), date_test.to_rfc3339());
        exptected.insert("deleted_at".to_string(), "".to_string());

        let input: IndexMap<String, String> = ProductResolved {
            id,
            name: "test1".to_string(),
            description: Some("test2".to_string()),
            unit_of_measure_id,
            unit_of_measure: "test3".to_string(),
            status: "test4".to_string(),
            created_by_id,
            created_by: "test5".to_string(),
            created_at: Some(date_test),
            updated_at: Some(date_test),
            deleted_at: None,
        }
        .into();

        assert_eq!(input, exptected);
    }
}
