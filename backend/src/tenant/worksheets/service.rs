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

use crate::common::CommonBuilderError;
use crate::common::dto::PaginatorMeta;
use crate::common::error::RepositoryError;
use crate::common::error::v2::{AppError, AppErrorVisibility};
use crate::common::model::SelectOption;
#[double]
use crate::common::pdf::PdfGenerator;
use crate::common::pdf::{PdfGenError, PdfTemplates};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::manager::auth::dto::claims::ClaimsError;
use crate::tenant::customers::dto::print::CustomerResolvedPrint;
use crate::tenant::customers::dto::print::test_customer_resolved_print_builder;
use crate::tenant::inventory::dto::print::InventoryResolvedPrint;
use crate::tenant::inventory::dto::print::test_inventory_resolved_print_builder;
use crate::tenant::inventory_movements::dto::print::InventoryMovementsResolvedPrint;
use crate::tenant::inventory_movements::dto::print::test_inventory_movement_resolved_print_builder;
use crate::tenant::products::dto::print::ProductsResolvedPrint;
use crate::tenant::products::dto::print::test_product_resolved_print_builder;
use crate::tenant::products::model::ProductResolved;
use crate::tenant::services::dto::print::ServicesResolvedPrint;
use crate::tenant::services::dto::print::test_service_resolved_print_builder;
use crate::tenant::tasks::dto::print::TaskResolvedPrint;
use crate::tenant::tasks::dto::print::test_task_resolved_print_builder;
use crate::tenant::warehouses::dto::print::WarehouseResolvedPrint;
use crate::tenant::warehouses::dto::print::test_warehouse_resolved_print_builder;
use crate::tenant::warehouses::model::WarehouseResolved;
use crate::tenant::worksheets::WorksheetsModuleInterface;
use crate::tenant::worksheets::dto::print::{
    WorksheetResolvedPrint, test_worksheet_resolved_print_builder,
};
use crate::tenant::worksheets::dto::user_input::WorksheetUserInput;
use crate::tenant::worksheets::model::{Worksheet, WorksheetResolved};
use crate::tenant::worksheets::types::worksheet::{WorksheetFilterBy, WorksheetOrderBy};
use axum::http::StatusCode;
use mockall_double::double;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum WorksheetsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("Hiba történt az adatok feldolgozása során: {0}")]
    UnprocessableEntry(&'static str),

    #[error("A lista nem létezik")]
    InvalidSelectList,

    #[error("PdfGen error: {0}")]
    PdfGenError(#[from] PdfGenError),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Claims error: {0}")]
    ClaimsError(#[from] ClaimsError),

    #[error("InvalidState error: {0}")]
    InvalidState(&'static str),

    #[error("BuilderError: {0}")]
    BuilderError(#[from] CommonBuilderError),
}

impl From<ServiceError> for WorksheetsServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => WorksheetsServiceError::Unauthorized,
        }
    }
}

impl From<WorksheetsServiceError> for AppError {
    fn from(value: WorksheetsServiceError) -> Self {
        match value {
            WorksheetsServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            WorksheetsServiceError::UnprocessableEntry(_) => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            WorksheetsServiceError::Repository(RepositoryError::Database(
                sqlx::Error::RowNotFound,
            )) => Self::new(
                Level::DEBUG,
                StatusCode::NOT_FOUND,
                file!(),
                AppErrorVisibility::UserFacing,
                json!({"message": "Nem található"}),
            ),
            _ => Self::new(
                Level::ERROR,
                StatusCode::INTERNAL_SERVER_ERROR,
                file!(),
                AppErrorVisibility::Internal,
                json!({"message": value.to_string()}),
            ),
        }
    }
}

type WorksheetsServiceResult<T> = Result<T, WorksheetsServiceError>;

pub enum WorksheetsSelectLists {
    Customers,
}

impl FromStr for WorksheetsSelectLists {
    type Err = WorksheetsServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "customers" => Ok(Self::Customers),
            _ => Err(Self::Err::InvalidSelectList),
        }
    }
}

pub trait WorksheetService {
    fn insert(
        &self,
        payload: &WorksheetUserInput,
    ) -> impl Future<Output = WorksheetsServiceResult<Worksheet>> + Send;
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = WorksheetsServiceResult<Vec<SelectOption>>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = WorksheetsServiceResult<WorksheetResolved>> + Send;
    fn get(&self, payload: Uuid)
    -> impl Future<Output = WorksheetsServiceResult<Worksheet>> + Send;
    fn update(
        &self,
        payload: &WorksheetUserInput,
    ) -> impl Future<Output = WorksheetsServiceResult<Worksheet>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = WorksheetsServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<WorksheetOrderBy, WorksheetFilterBy>,
    ) -> impl Future<Output = WorksheetsServiceResult<(PaginatorMeta, Vec<WorksheetResolved>)>> + Send;
    fn print(&self, payload: Uuid)
    -> impl Future<Output = WorksheetsServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(
        &self,
        path: &Path,
    ) -> impl Future<Output = WorksheetsServiceResult<()>> + Sync;
}

impl<'a, T> WorksheetService for Service<'a, T>
where
    T: WorksheetsModuleInterface,
{
    async fn insert(&self, payload: &WorksheetUserInput) -> WorksheetsServiceResult<Worksheet> {
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .insert(payload.clone(), self.claims()?.sub())
            .await?)
    }

    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> WorksheetsServiceResult<Vec<SelectOption>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(WorksheetsServiceError::Unauthorized)?;
        Ok(match WorksheetsSelectLists::from_str(select_list)? {
            WorksheetsSelectLists::Customers => {
                self.module()
                    .customers_repo(active_tenant)?
                    .get_select_list_items()
                    .await?
            }
        })
    }
    async fn get_resolved(&self, payload: Uuid) -> WorksheetsServiceResult<WorksheetResolved> {
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }

    async fn get(&self, payload: Uuid) -> WorksheetsServiceResult<Worksheet> {
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }

    async fn update(&self, payload: &WorksheetUserInput) -> WorksheetsServiceResult<Worksheet> {
        if !payload.id.is_present() {
            return Err(WorksheetsServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .update(payload.clone())
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> WorksheetsServiceResult<()> {
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<WorksheetOrderBy, WorksheetFilterBy>,
    ) -> WorksheetsServiceResult<(PaginatorMeta, Vec<WorksheetResolved>)> {
        Ok(self
            .module()
            .worksheets_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(WorksheetsServiceError::Unauthorized)?,
            )?
            .get_paged(get_query)
            .await?)
    }

    /// Creates a pdf from Worksheet.
    async fn print(&self, payload: Uuid) -> WorksheetsServiceResult<Vec<u8>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(WorksheetsServiceError::Unauthorized)?;
        let tz = self.claims()?.tz()?;

        // Loads worksheet data.
        let worksheet_resolved = self.get_resolved(payload).await?;

        let get_tasks = async || -> WorksheetsServiceResult<Vec<TaskResolvedPrint>> {
            // Loads tasks data.
            let tasks_resolved = self
                .module()
                .tasks_repo(active_tenant)?
                .get_resolved_by_worksheet_id(worksheet_resolved.id)
                .await?;

            // Batch load and create services resolved because TaskResolvedPrint needs ServiceResolvedPrint.
            // ServiceResolvedPrints are inserted into a hashmap for faster access.
            let service_ids: Vec<Uuid> = tasks_resolved.iter().map(|v| v.service_id).collect();
            let services_resolved = self
                .module()
                .services_repo(active_tenant)?
                .get_resolved_by_ids(service_ids)
                .await?;
            let mut services_resolved_print = HashMap::new();
            for service_resolved in services_resolved.into_iter() {
                services_resolved_print.insert(
                    service_resolved.id,
                    ServicesResolvedPrint::new(service_resolved, tz),
                );
            }

            // Creates TaskResolvedPrints and push them in a vector.
            let mut tasks: Vec<TaskResolvedPrint> = vec![];
            for task_resolved in tasks_resolved.into_iter() {
                let service_resolved_print = services_resolved_print
                    // NOTE: Can't use remove because it will cause bug if there are two tasks for
                    // the same service, so .clone() is needed!
                    .get(&task_resolved.service_id)
                    .ok_or_else(|| {
                        WorksheetsServiceError::InvalidState("Unexpected missing service")
                    })?;
                tasks.push(TaskResolvedPrint::new(
                    task_resolved,
                    service_resolved_print.clone(),
                    tz,
                ));
            }

            Ok(tasks)
        };
        let get_materials =
            async || -> WorksheetsServiceResult<Vec<InventoryMovementsResolvedPrint>> {
                // Collect inventory movements
                let inventory_movements_resolved = self
                    .module()
                    .inventory_movements_repo(active_tenant)?
                    .get_resolved_by_worksheet_id(worksheet_resolved.id)
                    .await?;

                // Load inventory data.
                let inventory_ids: Vec<Uuid> = inventory_movements_resolved
                    .iter()
                    .map(|v| v.inventory_id)
                    .collect();
                let inventories_resolved = self
                    .module()
                    .inventory_repo(active_tenant)?
                    .get_resolved_by_ids(inventory_ids)
                    .await?;

                // Loads product and warehouse data because InventoryResolvedPrint needs them.
                // Converts them into ProductResolvedPrint and WarehouseResolvedPrint and inserts
                // them into a HashMap for faster access.
                let product_ids: Vec<Uuid> =
                    inventories_resolved.iter().map(|v| v.product_id).collect();
                let get_products_resolved =
                    async || -> WorksheetsServiceResult<Vec<ProductResolved>> {
                        Ok(self
                            .module()
                            .products_repo(active_tenant)?
                            .get_resolved_by_ids(product_ids)
                            .await?)
                    };
                let warehouse_ids: Vec<Uuid> = inventories_resolved
                    .iter()
                    .map(|v| v.warehouse_id)
                    .collect();
                let get_warehouses_resolved =
                    async || -> WorksheetsServiceResult<Vec<WarehouseResolved>> {
                        Ok(self
                            .module()
                            .warehouses_repo(active_tenant)?
                            .get_resolved_by_ids(warehouse_ids)
                            .await?)
                    };
                let (products_resolved, warehouses_resolved) =
                    tokio::try_join!(get_products_resolved(), get_warehouses_resolved())?;
                let mut products_resolved_print = HashMap::new();
                for product_resolved in products_resolved.into_iter() {
                    products_resolved_print.insert(
                        product_resolved.id,
                        ProductsResolvedPrint::new(product_resolved, tz),
                    );
                }
                let mut warehouses_resolved_print = HashMap::new();
                for warehouse_resolved in warehouses_resolved.into_iter() {
                    warehouses_resolved_print.insert(
                        warehouse_resolved.id,
                        WarehouseResolvedPrint::new(warehouse_resolved, tz),
                    );
                }

                // Converts InventoryResolveds into InventoryResolvedPrints and inserts them to a
                // hashmap for faster access
                let mut inventories_resolved_print = HashMap::new();
                for inventory_resolved in inventories_resolved.into_iter() {
                    let product_resolved_print = products_resolved_print
                        .get(&inventory_resolved.product_id)
                        .ok_or_else(|| {
                            WorksheetsServiceError::InvalidState("Unexpected missing service")
                        })?;
                    let warehouse_resolved_print = warehouses_resolved_print
                        .get(&inventory_resolved.warehouse_id)
                        .ok_or_else(|| {
                            WorksheetsServiceError::InvalidState("Unexpected missing service")
                        })?;
                    inventories_resolved_print.insert(
                        inventory_resolved.id,
                        InventoryResolvedPrint::new(
                            inventory_resolved,
                            product_resolved_print.clone(),
                            warehouse_resolved_print.clone(),
                            tz,
                        ),
                    );
                }

                // Converts InventoryMovementResolved into InventoryMovementsResolvedPrints and
                // collects them to a vector
                let mut materials: Vec<InventoryMovementsResolvedPrint> = vec![];
                for inventory_movement_resolved in inventory_movements_resolved.into_iter() {
                    let inventory_resolved_print = inventories_resolved_print
                        .get(&inventory_movement_resolved.inventory_id)
                        .ok_or_else(|| {
                            WorksheetsServiceError::InvalidState("Unexpected missing service")
                        })?;
                    materials.push(InventoryMovementsResolvedPrint::new(
                        inventory_movement_resolved,
                        inventory_resolved_print.clone(),
                        tz,
                    ));
                }

                Ok(materials)
            };
        let (tasks, materials) = tokio::try_join!(get_tasks(), get_materials())?;

        // Loads and converts customer data into CustomerResolvedPrint because it is needed to
        // construct WorksheetResolvedPrint
        let customer_resolved_print = CustomerResolvedPrint::new(
            self.module()
                .customers_repo(active_tenant)?
                .get_resolved_by_id(worksheet_resolved.customer_id)
                .await?,
            tz,
        );

        // Finally constructs the WorksheetResolvedPrint.
        let worksheet_resolved_print = WorksheetResolvedPrint::new(
            worksheet_resolved,
            customer_resolved_print,
            tasks,
            materials,
            tz,
        );

        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::WorksheetView,
            worksheet_resolved_print,
        )?)
    }

    async fn print_snapshot(&self, path: &Path) -> WorksheetsServiceResult<()> {
        // NOTE: Values here must match the values in the tests!
        let worksheet_id = "4f321721-37c6-4e91-8e42-6281c36937bc"
            .parse()
            .map_err(|e: uuid::Error| WorksheetsServiceError::ParseError(e.to_string()))?;
        let customer_id = "fd48ade1-a817-431b-8ada-6faea8c9f9dd"
            .parse()
            .map_err(|e: uuid::Error| WorksheetsServiceError::ParseError(e.to_string()))?;

        let mut tasks = vec![];

        for task_nr in 1..=3 {
            tasks.push(
                test_task_resolved_print_builder(
                    test_service_resolved_print_builder()
                        .name(format!("Test task {task_nr}"))
                        .build()?,
                )
                .build()?,
            );
        }

        let mut materials = vec![];

        for material_nr in 1..=2 {
            materials.push(
                test_inventory_movement_resolved_print_builder(
                    test_inventory_resolved_print_builder(
                        test_product_resolved_print_builder()
                            .name(format!("Test material {material_nr}"))
                            .build()?,
                        test_warehouse_resolved_print_builder().build()?,
                    )
                    .build()?,
                )
                .build()?,
            );
        }

        let customer_resolved_print = test_customer_resolved_print_builder()
            .id(customer_id)
            .build()?;

        let worksheet_resolved_print =
            test_worksheet_resolved_print_builder(customer_resolved_print, tasks, materials)
                .id(worksheet_id)
                .build()?;

        let pdf = PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::WorksheetView,
            worksheet_resolved_print,
        )?;

        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
