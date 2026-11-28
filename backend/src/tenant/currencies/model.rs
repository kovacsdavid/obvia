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

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder, PartialEq, utoipa::ToSchema)]
pub struct Currency {
    pub code: String,
    pub number: String,
    pub name: String,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_currency_builder() -> CurrencyBuilder {
        let mut builder = CurrencyBuilder::default();

        builder
            .code("HUF".to_string())
            .number(348.to_string())
            .name("Forint".to_string());

        builder
    }
}
