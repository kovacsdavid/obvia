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
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inventory {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity_on_hand: BigDecimal,
    pub quantity_reserved: BigDecimal,
    pub quantity_available: BigDecimal,
    pub minimum_stock: Option<BigDecimal>,
    pub maximum_stock: Option<BigDecimal>,
    pub currency_code: String,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct InventoryResolved {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product: String,
    pub warehouse_id: Uuid,
    pub warehouse: String,
    pub quantity_on_hand: BigDecimal,
    pub quantity_reserved: BigDecimal,
    pub quantity_available: BigDecimal,
    pub minimum_stock: Option<BigDecimal>,
    pub maximum_stock: Option<BigDecimal>,
    pub currency_code: String,
    pub currency: String,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
pub mod tests {
    use crate::common::TEST_TIME;

    use super::*;

    pub fn test_inventory_resolved_builder() -> InventoryResolvedBuilder {
        let mut builder = InventoryResolvedBuilder::default();
        builder
            .id(Uuid::new_v4())
            .product_id(Uuid::new_v4())
            .product("Test product".to_string())
            .warehouse_id(Uuid::new_v4())
            .warehouse("Test warehouse".to_string())
            .quantity_on_hand("10".parse().unwrap())
            .quantity_reserved("20".parse().unwrap())
            .quantity_available("30".parse().unwrap())
            .maximum_stock(None)
            .minimum_stock(None)
            .currency_code("HUF".to_string())
            .currency("Forint".to_string())
            .status("active".to_string())
            .created_by_id(Uuid::new_v4())
            .created_by("Test User".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None);

        builder
    }
}
