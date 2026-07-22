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

use clap::ValueEnum;
use std::fmt::Display;

pub mod activity_feed;
pub mod address;
pub mod comments;
pub mod currencies;
pub mod customers;
pub mod inventory;
pub mod inventory_movements;
pub mod inventory_reservations;
pub mod products;
pub mod services;
pub mod tasks;
pub mod taxes;
pub mod users;
pub mod warehouses;
pub mod worksheets;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Modules {
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

impl Display for Modules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modules::Customers => write!(f, "customers"),
            Modules::Warehouses => write!(f, "warehouses"),
            Modules::Taxes => write!(f, "taxes"),
            Modules::Products => write!(f, "products"),
            Modules::Inventory => write!(f, "inventory"),
            Modules::InventoryMovements => write!(f, "inventory_movements"),
            Modules::InventoryReservations => write!(f, "inventory_reservations"),
            Modules::Services => write!(f, "services"),
            Modules::Tasks => write!(f, "tasks"),
            Modules::Worksheets => write!(f, "worksheets"),
        }
    }
}
