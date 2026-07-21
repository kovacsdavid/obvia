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

use std::{io::Write, path::Path, sync::Arc};

use anyhow::anyhow;
use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz;
use clap::{Parser, Subcommand, ValueEnum};
use obvia::{
    common::{config::AppConfig, init::init_default_app_state, service::Service},
    manager::auth::dto::claims::Claims,
    tenant::{
        customers::{
            dto::print::CustomerResolvedPrint, model::CustomerResolved, service::CustomerService,
        },
        inventory::{
            dto::print::InventoryResolvedPrint, model::InventoryResolved, service::InventoryService,
        },
        inventory_movements::{
            dto::print::InventoryMovementsResolvedPrint, model::InventoryMovementResolved,
            service::InventoryMovementService,
        },
        products::{
            dto::print::ProductsResolvedPrint, model::ProductResolved, service::ProductService,
        },
        taxes::{dto::print::TaxResolvedPrint, model::TaxResolved, service::TaxService},
        warehouses::{
            dto::print::WarehouseResolvedPrint, model::WarehouseResolved, service::WarehouseService,
        },
    },
};
use std::fs::File;
use uuid::Uuid;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand, Debug)]
enum CliCommands {
    /// Development utilities
    Dev {
        #[command(subcommand)]
        command: DevCommands,
    },
}

#[derive(Subcommand, Debug)]
enum DevCommands {
    PdfTestSnapshot {
        #[arg(value_enum)]
        module: Modules,

        #[arg(default_value = "testing/pdf/snapshots/")]
        folder: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Modules {
    Customers,
    Warehouses,
    Taxes,
    Products,
    Inventory,
    InventoryMovements,
    InventoryReservations,
    Services,
    Tasks,
    Worksheets,
}

fn gen_exp(expiration_mins: u64) -> anyhow::Result<usize> {
    (Utc::now()
        + Duration::minutes(
            expiration_mins.try_into().map_err(|_| {
                anyhow!("refresh_token_expiration_mins can not be converted to i64")
            })?,
        ))
    .timestamp()
    .try_into()
    .map_err(|_| anyhow!("exp can not be converted to usize"))
}

fn init_dev_claims(config: &AppConfig) -> anyhow::Result<Claims> {
    let now = Utc::now();
    let tz: Tz = "Europe/Budapest".parse()?;
    Ok(Claims::new(
        Uuid::new_v4(),
        gen_exp(config.auth().access_token_expiration_mins())?,
        now.timestamp() as usize,
        now.timestamp() as usize,
        "obvia".to_string(),
        "obvia".to_string(),
        Uuid::new_v4(),
        "hu-HU".to_string(),
        tz,
        None,
        None,
    ))
}

async fn gen_pdf_test_snapshot(module: &Modules, folder: &str) -> anyhow::Result<()> {
    let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z".parse()?;
    let config = AppConfig::from_env()?;
    let claims = init_dev_claims(&config)?;
    let app_state = init_default_app_state(config).await?;
    let service = Service::new(Some(&claims), Arc::new(app_state));
    let tz: Tz = "Europe/Budapest".parse()?;
    match module {
        Modules::Customers => {
            let path = format!("{folder}/customers_test.pdf");
            let path = Path::new(&path);
            let tz: Tz = "Europe/Budapest".parse()?;
            let customer_resolved = CustomerResolved {
                id: "4f321721-37c6-4e91-8e42-6281c36937bc".parse()?,
                name: "Test Customer".to_string(),
                contact_name: None,
                email: "test.customer@example.com".to_string(),
                phone_number: Some("+36301234567".to_string()),
                status: "active".to_string(),
                customer_type: "natural".to_string(),
                created_by_id: "97054cdb-781c-4f40-a489-b43373d75bf0".parse()?,
                created_by: "Test User".to_string(),
                created_at: test_time,
                updated_at: test_time,
                deleted_at: None,
            };
            let customer_resolved_print =
                CustomerResolvedPrint::from_customer_revolved(customer_resolved, tz);
            let pdf = CustomerService::print(&service, &[customer_resolved_print]).await?;
            let mut file = File::create(path)?;
            file.write_all(&pdf)?;
            Ok(())
        }
        Modules::Warehouses => {
            let path = format!("{folder}/warehouses_test.pdf");
            let path = Path::new(&path);
            let warehouse_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse()?;
            let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0".parse()?;
            let warehouse_resolved = WarehouseResolved {
                id: warehouse_id,
                name: "Test Warehouse".to_string(),
                contact_name: Some("Test Contact".to_string()),
                contact_phone: Some("+36301234567".to_string()),
                status: "active".to_string(),
                created_by_id,
                created_by: "Test User".to_string(),
                created_at: test_time,
                updated_at: test_time,
                deleted_at: None,
            };
            let warehouse_resolved_print =
                WarehouseResolvedPrint::from_warehouse_resolved(warehouse_resolved, tz);
            let pdf = WarehouseService::print(&service, &[warehouse_resolved_print]).await?;
            let mut file = File::create(path)?;
            file.write_all(&pdf)?;
            Ok(())
        }
        Modules::Taxes => {
            let path = format!("{folder}/taxes_test.pdf");
            let path = Path::new(&path);
            let tax_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse()?;
            let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0".parse()?;
            let tax_resolved = TaxResolved {
                id: tax_id,
                rate: Some("10".parse().unwrap()),
                description: "Test tax".to_string(),
                country_code: "HU".to_string(),
                country: "Magyarország".to_string(),
                tax_category: "standard".to_string(),
                is_rate_applicable: true,
                legal_text: None,
                reporting_code: None,
                is_default: true,
                status: "active".to_string(),
                created_by_id,
                created_by: "Test User".to_string(),
                created_at: test_time,
                updated_at: test_time,
                deleted_at: None,
            };
            let taxes_resolved_print = TaxResolvedPrint::from_tax_resolved(tax_resolved, tz);
            let pdf = TaxService::print(&service, &[taxes_resolved_print]).await?;
            let mut file = File::create(path)?;
            file.write_all(&pdf)?;
            Ok(())
        }
        Modules::Products => {
            let path = format!("{folder}/products_test.pdf");
            let path = Path::new(&path);
            let product_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse()?;
            let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0".parse()?;
            let unit_of_measure_id = "0237354a-21ab-46f4-a4ca-b21cb08561d7".parse()?;
            let product_resolved = ProductResolved {
                id: product_id,
                name: "Test product".to_string(),
                description: None,
                unit_of_measure_id,
                unit_of_measure: "cm".to_string(),
                status: "active".to_string(),
                created_by_id,
                created_by: "Test User".to_string(),
                created_at: test_time,
                updated_at: test_time,
                deleted_at: None,
            };
            let products_resolved_print =
                ProductsResolvedPrint::from_product_resolved(product_resolved, tz);
            let pdf = ProductService::print(&service, &[products_resolved_print]).await?;
            let mut file = File::create(path)?;
            file.write_all(&pdf)?;
            Ok(())
        }
        Modules::Inventory => {
            let path = format!("{folder}/inventory_test.pdf");
            let path = Path::new(&path);
            let inventory_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse()?;
            let product_id = "0237354a-21ab-46f4-a4ca-b21cb08561d7".parse()?;
            let warehouse_id = "521f9728-f59f-435d-8656-69ba4273254c".parse()?;
            let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0".parse()?;
            let inventory_resolved = InventoryResolved {
                id: inventory_id,
                product_id,
                product: "Test product".to_string(),
                warehouse_id,
                warehouse: "Test warehouse".to_string(),
                quantity_on_hand: "10".parse().unwrap(),
                quantity_reserved: "20".parse().unwrap(),
                quantity_available: "30".parse().unwrap(),
                minimum_stock: None,
                maximum_stock: None,
                currency_code: "HUF".to_string(),
                currency: "Forint".to_string(),
                status: "active".to_string(),
                created_by_id,
                created_by: "Test User".to_string(),
                created_at: test_time,
                updated_at: test_time,
                deleted_at: None,
            };
            let inventory_resolved_print =
                InventoryResolvedPrint::from_inventory_resolved(inventory_resolved, tz);
            let pdf = InventoryService::print(&service, &[inventory_resolved_print]).await?;
            let mut file = File::create(path)?;
            file.write_all(&pdf)?;
            Ok(())
        }
        Modules::InventoryMovements => {
            let path = format!("{folder}/inventory_movements_test.pdf");
            let path = Path::new(&path);
            let inventory_movement_id = "4f321721-37c6-4e91-8e42-6281c36937bc".parse()?;
            let inventory_id = "ac55ca9c-2cd1-4cdf-8b44-ed4df798c750".parse()?;
            let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0".parse()?;
            let reference_id = "fd48ade1-a817-431b-8ada-6faea8c9f9dd".parse()?;
            let tax_id = "86097a0b-3f05-42f4-a98d-fd8a4669f02b".parse()?;
            let inventory_movement_resolved = InventoryMovementResolved {
                id: inventory_movement_id,
                inventory_id,
                movement_type: "in".to_string(),
                quantity: "10".parse().unwrap(),
                reference_type: Some("worksheets".to_string()),
                reference_id: Some(reference_id),
                unit_price: Some("20".parse().unwrap()),
                total_price: Some("30".parse().unwrap()),
                tax_id,
                tax: Some("Test Tax".to_string()),
                movement_date: test_time,
                created_by_id,
                created_by: "Test User".to_string(),
                created_at: test_time,
            };
            let inventory_movement_resolved_print =
                InventoryMovementsResolvedPrint::from_inventory_movements_resolved(
                    inventory_movement_resolved,
                    tz,
                );
            let pdf =
                InventoryMovementService::print(&service, &[inventory_movement_resolved_print])
                    .await?;
            let mut file = File::create(path)?;
            file.write_all(&pdf)?;
            Ok(())
        }
        Modules::InventoryReservations => {
            todo!()
        }
        Modules::Services => {
            todo!()
        }
        Modules::Tasks => {
            todo!()
        }
        Modules::Worksheets => {
            todo!()
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        CliCommands::Dev { command } => match &command {
            DevCommands::PdfTestSnapshot { module, folder } => {
                gen_pdf_test_snapshot(module, folder).await?;
            }
        },
    }
    Ok(())
}
