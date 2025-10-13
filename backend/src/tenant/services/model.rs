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

/// Represents a `Service` entity in the application.
///
/// This struct defines the model for the service data
/// and is used to interact with database records, serialize/deserialize
/// JSON, and track service details.
///
/// # Fields
/// - `id` (`Uuid`): The unique identifier for the service.
/// - `name` (`String`): The name of the service.
/// - `description` (`Option<String>`): An optional field representing the description of the service.
/// - `default_price` (`Option<BigDecimal>`): The default price for the service.
/// - `default_tax_id` (`Option<Uuid>`): An optional reference to the default tax applied to the service.
/// - `currency_code` (`Option<Uuid>`): An optional reference to the default currency for the service.
/// - `status` (`String`): The current status of the service (e.g., active, inactive).
/// - `created_by_id` (`Uuid`): The user who created the service record.
/// - `created_at` (`DateTime<Local>`): The timestamp indicating when the service record was created.
/// - `updated_at` (`DateTime<Local>`): The timestamp indicating the last update to the service record.
/// - `deleted_at` (`Option<DateTime<Local>`): An optional field to track when the service record was deleted, if applicable.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub default_price: Option<BigDecimal>,
    pub default_tax_id: Option<Uuid>,
    pub currency_code: Option<Uuid>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

/// Represents a resolved `Service` entity with additional information.
///
/// This struct extends the basic Service model with related data
/// such as the creator's name, for use in API responses.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ServiceResolved {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub default_price: Option<BigDecimal>,
    pub default_tax_id: Option<Uuid>,
    pub currency_code: Option<Uuid>,
    pub status: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
