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

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub email: String,
    pub phone_number: Option<String>,
    pub status: String,
    pub customer_type: String,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub billing_address: Option<Uuid>,
    pub mailing_address: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Builder)]
pub struct CustomerResolved {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub email: String,
    pub phone_number: Option<String>,
    pub status: String,
    pub customer_type: String,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub billing_address: Option<Uuid>,
    pub mailing_address: Option<Uuid>,
}

#[cfg(test)]
pub mod tests {
    use crate::common::TEST_TIME;

    use super::*;

    pub fn test_customer_builder() -> CustomerBuilder {
        let mut builder = CustomerBuilder::default();
        builder
            .id(Uuid::new_v4())
            .name("Test Customer".to_string())
            .contact_name(None)
            .email("test.customer@example.com".to_string())
            .phone_number(Some("+36301234567".to_string()))
            .status("active".to_string())
            .customer_type("natural".to_string())
            .created_by_id(Uuid::new_v4())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None)
            .billing_address(None)
            .mailing_address(None);

        builder
    }

    pub fn test_customer_resolved_builder() -> CustomerResolvedBuilder {
        let mut builder = CustomerResolvedBuilder::default();
        builder
            .id(Uuid::new_v4())
            .name("Test Customer".to_string())
            .contact_name(None)
            .email("test.customer@example.com".to_string())
            .phone_number(Some("+36301234567".to_string()))
            .status("active".to_string())
            .customer_type("natural".to_string())
            .created_by_id(Uuid::new_v4())
            .created_by("Test User".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None)
            .billing_address(None)
            .mailing_address(None);

        builder
    }
}
