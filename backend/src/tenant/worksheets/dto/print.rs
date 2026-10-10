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

use crate::{
    common::{CommonBuilderError, TEST_TIME_TZ, utils::thousand_separated_number_bigdecimal},
    tenant::owner_profile::dto::print::OwnerProfileFullPrint,
};
use chrono_tz::Tz;
use derive_builder::Builder;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

use crate::tenant::{
    customers::dto::print::CustomerFullPrint,
    inventory_movements::dto::print::InventoryMovementsResolvedPrint,
    tasks::dto::print::TaskResolvedPrint, worksheets::model::WorksheetResolved,
};

#[derive(Debug, Error, PartialEq)]
pub enum WorksheetResolvedPrintError {
    #[error("Az anyagköltségek és a szolgátatások csak egy fajta pénznemben adhatók meg")]
    CurrencyCodeError,
}

pub type WorksheetResolvedPrintResult = Result<WorksheetResolvedPrint, WorksheetResolvedPrintError>;

#[derive(Clone, Serialize, PartialEq, Debug, Builder)]
#[builder(build_fn(error = "CommonBuilderError"))]
pub struct WorksheetResolvedPrint {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub customer: CustomerFullPrint,
    pub owner_profile: OwnerProfileFullPrint,
    pub project_id: Option<Uuid>,
    pub project: Option<String>,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub net_material_cost: String,
    pub gross_material_cost: String,
    pub net_work_cost: String,
    pub gross_work_cost: String,
    pub net_total: String,
    pub gross_total: String,
    pub tasks: Vec<TaskResolvedPrint>,
    pub materials: Vec<InventoryMovementsResolvedPrint>,
}

impl WorksheetResolvedPrint {
    pub fn new(
        worksheet_resolved: WorksheetResolved,
        customer_resolved_print: CustomerFullPrint,
        owner_profile_print: OwnerProfileFullPrint,
        tasks: Vec<TaskResolvedPrint>,
        materials: Vec<InventoryMovementsResolvedPrint>,
        tz: Tz,
    ) -> WorksheetResolvedPrintResult {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        let net_total = &worksheet_resolved.net_material_cost + &worksheet_resolved.net_work_cost;
        let gross_total =
            &worksheet_resolved.gross_material_cost + &worksheet_resolved.gross_work_cost;

        let mut currency_code = "N/A";

        if !tasks.is_empty() {
            currency_code = &tasks[0].currency_code;
        } else if !materials.is_empty() {
            currency_code = &materials[0].inventory.currency_code;
        }

        for task in &tasks {
            if task.currency_code != currency_code {
                return Err(WorksheetResolvedPrintError::CurrencyCodeError);
            }
        }

        for material in &materials {
            if material.inventory.currency_code != currency_code {
                return Err(WorksheetResolvedPrintError::CurrencyCodeError);
            }
        }

        let net_material_cost = format!(
            "{} {currency_code}",
            thousand_separated_number_bigdecimal(&worksheet_resolved.net_material_cost, 2)
        );
        let gross_material_cost = format!(
            "{} {currency_code}",
            thousand_separated_number_bigdecimal(&worksheet_resolved.gross_material_cost, 2)
        );
        let net_work_cost = format!(
            "{} {currency_code}",
            thousand_separated_number_bigdecimal(&worksheet_resolved.net_work_cost, 2)
        );
        let gross_work_cost = format!(
            "{} {currency_code}",
            thousand_separated_number_bigdecimal(&worksheet_resolved.gross_work_cost, 2)
        );

        let net_total = format!(
            "{} {currency_code}",
            thousand_separated_number_bigdecimal(&net_total, 2)
        );
        let gross_total = format!(
            "{} {currency_code}",
            thousand_separated_number_bigdecimal(&gross_total, 2)
        );

        Ok(Self {
            id: worksheet_resolved.id,
            name: worksheet_resolved.name,
            description: worksheet_resolved.description,
            customer: customer_resolved_print,
            owner_profile: owner_profile_print,
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
            net_material_cost,
            gross_material_cost,
            net_work_cost,
            gross_work_cost,
            net_total,
            gross_total,
            tasks,
            materials,
        })
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

pub fn test_worksheet_resolved_print_builder(
    customer_full_print: CustomerFullPrint,
    owner_profile_full_print: OwnerProfileFullPrint,
    tasks: Vec<TaskResolvedPrint>,
    materials: Vec<InventoryMovementsResolvedPrint>,
) -> WorksheetResolvedPrintBuilder {
    let mut builder = WorksheetResolvedPrintBuilder::default();
    builder
        .id(Uuid::now_v7())
        .name("Test worksheet".to_string())
        .description(Some("Test description".to_string()))
        .customer(customer_full_print)
        .owner_profile(owner_profile_full_print)
        .project_id(Some(Uuid::now_v7()))
        .project(Some("Test project".to_string()))
        .created_by_id(Uuid::now_v7())
        .created_by("Test User".to_string())
        .status("Aktív".to_string())
        .created_at(TEST_TIME_TZ.clone())
        .updated_at(TEST_TIME_TZ.clone())
        .deleted_at(None)
        .net_material_cost("10.00 HUF".parse().unwrap())
        .gross_material_cost("20.00 HUF".parse().unwrap())
        .net_work_cost("30.00 HUF".parse().unwrap())
        .gross_work_cost("40.00 HUF".parse().unwrap())
        .net_total("40.00 HUF".parse().unwrap())
        .gross_total("60.00 HUF".parse().unwrap())
        .tasks(tasks)
        .materials(materials);

    builder
}

#[cfg(test)]
mod tests {
    use crate::{
        common::TEST_TZ,
        tenant::{
            customers::model::tests::test_customer_resolved_builder,
            inventory::dto::print::test_inventory_resolved_print_builder,
            inventory_movements::dto::print::test_inventory_movement_resolved_print_builder,
            owner_profile::model::{OwnerProfileFull, tests::test_owner_profile_resolved_builder},
            products::dto::print::test_product_resolved_print_builder,
            services::dto::print::test_service_resolved_print_builder,
            tasks::dto::print::test_task_resolved_print_builder,
            warehouses::dto::print::test_warehouse_resolved_print_builder,
            worksheets::model::tests::test_worksheet_resolved_builder,
        },
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_worksheet_resolved() {
        let worksheet_id = Uuid::now_v7();
        let customer_id = Uuid::now_v7();
        let owner_profile_id = Uuid::now_v7();
        let created_by_id = Uuid::now_v7();

        let worksheet_resolved = test_worksheet_resolved_builder()
            .id(worksheet_id)
            .created_by_id(created_by_id)
            .build()
            .unwrap();

        let customer_resolved = test_customer_resolved_builder()
            .id(customer_id)
            .build()
            .unwrap();

        let owner_profile_resolved = test_owner_profile_resolved_builder()
            .id(owner_profile_id)
            .build()
            .unwrap();

        let customer_full_print =
            CustomerFullPrint::new(customer_resolved.into_full(None, None), *TEST_TZ);

        let owner_profile_full_print = OwnerProfileFullPrint::new(
            OwnerProfileFull::from((owner_profile_resolved, None, None)),
            *TEST_TZ,
        );

        let tasks = vec![
            test_task_resolved_print_builder(
                test_service_resolved_print_builder()
                    .name("Test service 1".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
            test_task_resolved_print_builder(
                test_service_resolved_print_builder()
                    .name("Test service 2".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
            test_task_resolved_print_builder(
                test_service_resolved_print_builder()
                    .name("Test service 3".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
        ];

        let materials = vec![
            test_inventory_movement_resolved_print_builder(
                test_inventory_resolved_print_builder(
                    test_product_resolved_print_builder()
                        .name("Test product 1".to_string())
                        .build()
                        .unwrap(),
                    test_warehouse_resolved_print_builder().build().unwrap(),
                )
                .build()
                .unwrap(),
            )
            .build()
            .unwrap(),
            test_inventory_movement_resolved_print_builder(
                test_inventory_resolved_print_builder(
                    test_product_resolved_print_builder()
                        .name("Test product 2".to_string())
                        .build()
                        .unwrap(),
                    test_warehouse_resolved_print_builder().build().unwrap(),
                )
                .build()
                .unwrap(),
            )
            .build()
            .unwrap(),
        ];

        let worksheet_resolved_print = WorksheetResolvedPrint::new(
            worksheet_resolved,
            customer_full_print.clone(),
            owner_profile_full_print.clone(),
            tasks.clone(),
            materials.clone(),
            *TEST_TZ,
        );
        let worksheet_resolved_print_expected = WorksheetResolvedPrint {
            id: worksheet_id,
            name: "Test worksheet".to_string(),
            description: Some("Test description".to_string()),
            customer: customer_full_print,
            owner_profile: owner_profile_full_print,
            project_id: None,
            project: None,
            created_by_id,
            created_by: "Test User".to_string(),
            status: "Aktív".to_string(),
            created_at: TEST_TIME_TZ.clone(),
            updated_at: TEST_TIME_TZ.clone(),
            deleted_at: None,
            net_material_cost: "10.00 HUF".parse().unwrap(),
            gross_material_cost: "20.00 HUF".parse().unwrap(),
            net_work_cost: "30.00 HUF".parse().unwrap(),
            gross_work_cost: "40.00 HUF".parse().unwrap(),
            net_total: "40.00 HUF".parse().unwrap(),
            gross_total: "60.00 HUF".parse().unwrap(),
            tasks,
            materials,
        };
        assert_eq!(
            worksheet_resolved_print.unwrap(),
            worksheet_resolved_print_expected
        );
    }

    #[test]
    fn test_from_worksheet_resolved_currency_error() {
        let worksheet_id = Uuid::now_v7();
        let customer_id = Uuid::now_v7();
        let owner_profile_id = Uuid::now_v7();
        let created_by_id = Uuid::now_v7();

        let worksheet_resolved = test_worksheet_resolved_builder()
            .id(worksheet_id)
            .created_by_id(created_by_id)
            .build()
            .unwrap();

        let customer_resolved = test_customer_resolved_builder()
            .id(customer_id)
            .build()
            .unwrap();

        let owner_profile_resolved = test_owner_profile_resolved_builder()
            .id(owner_profile_id)
            .build()
            .unwrap();

        let customer_full_print =
            CustomerFullPrint::new(customer_resolved.into_full(None, None), *TEST_TZ);

        let owner_profile_full_print = OwnerProfileFullPrint::new(
            OwnerProfileFull::from((owner_profile_resolved, None, None)),
            *TEST_TZ,
        );

        let tasks = vec![
            test_task_resolved_print_builder(
                test_service_resolved_print_builder()
                    .name("Test service 1".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
            test_task_resolved_print_builder(
                test_service_resolved_print_builder()
                    .name("Test service 2".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
            test_task_resolved_print_builder(
                test_service_resolved_print_builder()
                    .name("Test service 3".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
        ];

        let materials = vec![
            test_inventory_movement_resolved_print_builder(
                test_inventory_resolved_print_builder(
                    test_product_resolved_print_builder()
                        .name("Test product 1".to_string())
                        .build()
                        .unwrap(),
                    test_warehouse_resolved_print_builder().build().unwrap(),
                )
                .build()
                .unwrap(),
            )
            .build()
            .unwrap(),
            test_inventory_movement_resolved_print_builder(
                test_inventory_resolved_print_builder(
                    test_product_resolved_print_builder()
                        .name("Test product 2".to_string())
                        .build()
                        .unwrap(),
                    test_warehouse_resolved_print_builder().build().unwrap(),
                )
                .currency_code("EUR".to_string())
                .build()
                .unwrap(),
            )
            .build()
            .unwrap(),
        ];

        let worksheet_resolved_print = WorksheetResolvedPrint::new(
            worksheet_resolved,
            customer_full_print.clone(),
            owner_profile_full_print.clone(),
            tasks.clone(),
            materials.clone(),
            *TEST_TZ,
        );

        assert!(worksheet_resolved_print.is_err());
        assert_eq!(
            WorksheetResolvedPrintError::CurrencyCodeError,
            worksheet_resolved_print.unwrap_err()
        );
    }
}
