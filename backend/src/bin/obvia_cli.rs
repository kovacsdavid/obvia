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

use anyhow::anyhow;
use chrono::{Duration, Utc};
use chrono_tz::Tz;
use clap::{Parser, Subcommand};
use obvia::{
    common::{config::AppConfig, init::init_default_app_state, service::Service},
    manager::auth::dto::claims::Claims,
    tenant::{
        Modules, customers::service::CustomerService, inventory::service::InventoryService,
        inventory_movements::service::InventoryMovementService,
        inventory_reservations::service::InventoryReservationService,
        products::service::ProductService, services::service::ServiceService,
        tasks::service::TaskService, taxes::service::TaxService,
        warehouses::service::WarehouseService, worksheets::service::WorksheetService,
    },
};
use std::{path::Path, sync::Arc};
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
    let config = AppConfig::from_env()?;
    let claims = init_dev_claims(&config)?;
    let app_state = init_default_app_state(config).await?;
    let service = Service::new(Some(&claims), Arc::new(app_state));
    let path = format!("{folder}/{module}_test.pdf");
    let path = Path::new(&path);
    match module {
        Modules::Customers => Ok(CustomerService::print_snapshot(&service, path).await?),
        Modules::Warehouses => Ok(WarehouseService::print_snapshot(&service, path).await?),
        Modules::Taxes => Ok(TaxService::print_snapshot(&service, path).await?),
        Modules::Products => Ok(ProductService::print_snapshot(&service, path).await?),
        Modules::Inventory => Ok(InventoryService::print_snapshot(&service, path).await?),
        Modules::InventoryMovements => {
            Ok(InventoryMovementService::print_snapshot(&service, path).await?)
        }
        Modules::InventoryReservations => {
            Ok(InventoryReservationService::print_snapshot(&service, path).await?)
        }
        Modules::Services => Ok(ServiceService::print_snapshot(&service, path).await?),
        Modules::Tasks => Ok(TaskService::print_snapshot(&service, path).await?),
        Modules::Worksheets => Ok(WorksheetService::print_snapshot(&service, path).await?),
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
