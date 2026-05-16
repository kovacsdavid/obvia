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
pub struct Worksheet {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub customer_id: Uuid,
    pub project_id: Option<Uuid>,
    pub created_by_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorksheetResolved {
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
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
    pub net_material_cost: BigDecimal,
    pub gross_material_cost: BigDecimal,
    pub net_work_cost: BigDecimal,
    pub gross_work_cost: BigDecimal,
}

impl From<WorksheetResolved> for IndexMap<String, String> {
    fn from(value: WorksheetResolved) -> Self {
        let mut map = IndexMap::new();

        map.insert("id".to_string(), value.id.to_string());
        map.insert("name".to_string(), value.name);
        map.insert(
            "description".to_string(),
            value.description.unwrap_or_default(),
        );
        map.insert("customer_id".to_string(), value.customer_id.to_string());
        map.insert("customer".to_string(), value.customer);
        map.insert(
            "project_id".to_string(),
            value.project_id.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("project".to_string(), value.project.unwrap_or_default());
        map.insert("created_by_id".to_string(), value.created_by_id.to_string());
        map.insert("created_by".to_string(), value.created_by);
        map.insert("status".to_string(), value.status);
        map.insert("created_at".to_string(), value.created_at.to_rfc3339());
        map.insert("updated_at".to_string(), value.created_at.to_rfc3339());
        map.insert(
            "deleted_at".to_string(),
            value
                .deleted_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
        );
        map.insert(
            "net_material_cost".to_string(),
            value.net_material_cost.to_string(),
        );
        map.insert(
            "gross_material_cost".to_string(),
            value.gross_material_cost.to_string(),
        );
        map.insert("net_work_cost".to_string(), value.net_work_cost.to_string());
        map.insert(
            "gross_work_cost".to_string(),
            value.gross_work_cost.to_string(),
        );

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn worksheet_resolved_into_index_map() {
        let id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let date_test = Local::now();
        let mut expected = IndexMap::new();

        expected.insert("id".to_string(), id.to_string());
        expected.insert("name".to_string(), "test1".to_string());
        expected.insert("description".to_string(), "test2".to_string());
        expected.insert("customer_id".to_string(), customer_id.to_string());
        expected.insert("customer".to_string(), "test3".to_string());
        expected.insert("project_id".to_string(), project_id.to_string());
        expected.insert("project".to_string(), "test4".to_string());
        expected.insert("created_by_id".to_string(), created_by_id.to_string());
        expected.insert("created_by".to_string(), "test5".to_string());
        expected.insert("status".to_string(), "test6".to_string());
        expected.insert("created_at".to_string(), date_test.to_rfc3339());
        expected.insert("updated_at".to_string(), date_test.to_rfc3339());
        expected.insert("deleted_at".to_string(), "".to_string());
        expected.insert("net_material_cost".to_string(), "10".to_string());
        expected.insert("gross_material_cost".to_string(), "20".to_string());
        expected.insert("net_work_cost".to_string(), "30".to_string());
        expected.insert("gross_work_cost".to_string(), "40".to_string());

        let input: IndexMap<String, String> = WorksheetResolved {
            id,
            name: "test1".to_string(),
            description: Some("test2".to_string()),
            customer_id,
            customer: "test3".to_string(),
            project_id: Some(project_id),
            project: Some("test4".to_string()),
            created_by_id,
            created_by: "test5".to_string(),
            status: "test6".to_string(),
            created_at: date_test,
            updated_at: date_test,
            deleted_at: None,
            net_material_cost: "10".parse().unwrap(),
            gross_material_cost: "20".parse().unwrap(),
            net_work_cost: "30".parse().unwrap(),
            gross_work_cost: "40".parse().unwrap(),
        }
        .into();

        assert_eq!(input, expected);
    }
}
