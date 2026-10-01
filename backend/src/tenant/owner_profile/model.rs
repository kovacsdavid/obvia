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

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::tenant::address::model::AddressResolved;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct OwnerProfile {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub email: String,
    pub website: Option<String>,
    pub phone_number: Option<String>,
    pub owner_profile_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub billing_address: Option<Uuid>,
    pub mailing_address: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Builder)]
pub struct OwnerProfileResolved {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub email: String,
    pub website: Option<String>,
    pub phone_number: Option<String>,
    pub owner_profile_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub billing_address: Option<Uuid>,
    pub mailing_address: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
pub struct OwnerProfileFull {
    pub id: Uuid,
    pub name: String,
    pub contact_name: Option<String>,
    pub email: String,
    pub website: Option<String>,
    pub phone_number: Option<String>,
    pub owner_profile_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub billing_address: Option<AddressResolved>,
    pub mailing_address: Option<AddressResolved>,
}

impl
    From<(
        OwnerProfileResolved,
        Option<AddressResolved>,
        Option<AddressResolved>,
    )> for OwnerProfileFull
{
    fn from(
        (owner_profile_resolved, billing_address, mailing_address): (
            OwnerProfileResolved,
            Option<AddressResolved>,
            Option<AddressResolved>,
        ),
    ) -> Self {
        OwnerProfileFull {
            id: owner_profile_resolved.id,
            name: owner_profile_resolved.name,
            contact_name: owner_profile_resolved.contact_name,
            email: owner_profile_resolved.email,
            website: owner_profile_resolved.website,
            phone_number: owner_profile_resolved.phone_number,
            owner_profile_type: owner_profile_resolved.owner_profile_type,
            created_at: owner_profile_resolved.created_at,
            updated_at: owner_profile_resolved.updated_at,
            billing_address,
            mailing_address,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::common::TEST_TIME;

    use super::*;

    pub fn test_owner_profile_builder() -> OwnerProfileBuilder {
        let mut builder = OwnerProfileBuilder::default();
        builder
            .id(Uuid::now_v7())
            .name("Test Owner".to_string())
            .contact_name(None)
            .email("test.owner@example.com".to_string())
            .website(Some("https://example.com".to_string()))
            .phone_number(Some("+36301234567".to_string()))
            .owner_profile_type("natural".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .billing_address(None)
            .mailing_address(None);

        builder
    }

    pub fn test_owner_profile_resolved_builder() -> OwnerProfileResolvedBuilder {
        let mut builder = OwnerProfileResolvedBuilder::default();
        builder
            .id(Uuid::now_v7())
            .name("Test Owner".to_string())
            .contact_name(None)
            .email("test.owner@example.com".to_string())
            .website(Some("https://example.com".to_string()))
            .phone_number(Some("+36301234567".to_string()))
            .owner_profile_type("natural".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .billing_address(None)
            .mailing_address(None);

        builder
    }

    pub fn test_owner_profile_full_builder() -> OwnerProfileFullBuilder {
        let mut builder = OwnerProfileFullBuilder::default();
        builder
            .id(Uuid::now_v7())
            .name("Test Owner".to_string())
            .contact_name(None)
            .email("test.owner@example.com".to_string())
            .website(Some("https://example.com".to_string()))
            .phone_number(Some("+36301234567".to_string()))
            .owner_profile_type("natural".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .billing_address(None)
            .mailing_address(None);
        builder
    }
}
