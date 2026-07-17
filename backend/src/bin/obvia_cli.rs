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

use clap::{Parser, Subcommand, ValueEnum};

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

fn gen_pdf_test_snapshot(module: &Modules) {
    match module {
        Modules::Customers => {
            todo!()
        }
        Modules::Warehouses => {
            todo!()
        }
        Modules::Taxes => {
            todo!()
        }
        Modules::Products => {
            todo!()
        }
        Modules::Inventory => {
            todo!()
        }
        Modules::InventoryMovements => {
            todo!()
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        CliCommands::Dev { command } => match &command {
            DevCommands::PdfTestSnapshot { module } => {
                println!("{:?}", cli);
                gen_pdf_test_snapshot(module);
            }
        },
    }
    Ok(())
}
