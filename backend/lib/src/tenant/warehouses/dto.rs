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

use crate::manager::common::types::value_object::ValueObject;
use crate::tenant::warehouses::types::warehouse::{
    WarehouseContactName, WarehouseContactPhone, WarehouseName,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWarehouse {
    pub name: ValueObject<WarehouseName>,
    pub contact_name: Option<ValueObject<WarehouseContactName>>,
    pub contact_phone: Option<ValueObject<WarehouseContactPhone>>,
    pub is_active: Option<bool>, // Will default to true if not provided
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWarehouse {
    pub name: ValueObject<WarehouseName>,
    pub contact_name: ValueObject<WarehouseContactName>,
    pub contact_phone: ValueObject<WarehouseContactPhone>,
    pub is_active: bool,
}
