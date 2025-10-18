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
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InventoryMovement {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub movement_type: String,
    pub quantity: i32,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub unit_price: Option<BigDecimal>,
    pub total_price: Option<BigDecimal>,
    pub tax_id: Uuid,
    pub movement_date: DateTime<Local>,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InventoryMovementResolved {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub movement_type: String,
    pub quantity: i32,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub unit_price: Option<BigDecimal>,
    pub total_price: Option<BigDecimal>,
    pub tax_id: Uuid,
    pub tax: Option<String>,
    pub movement_date: DateTime<Local>,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Local>,
}
