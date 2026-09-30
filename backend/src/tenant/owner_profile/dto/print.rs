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

use crate::common::CommonBuilderError;
use crate::common::TEST_TIME_TZ;
use crate::tenant::address::model::test_address_resolved_builder;
use crate::tenant::owner_profile::model::OwnerProfileFull;
use chrono_tz::Tz;
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize, PartialEq, Debug, Builder)]
#[builder(build_fn(error = "CommonBuilderError"))]
pub struct OwnerProfileFullPrint {
    id: Uuid,
    name: String,
    contact_name: Option<String>,
    email: String,
    website: Option<String>,
    phone_number: Option<String>,
    owner_profile_type: String,
    billing_address: Option<String>,
    mailing_address: Option<String>,
    created_at: String,
    updated_at: String,
}

impl OwnerProfileFullPrint {
    pub fn new(owner_profile_full: OwnerProfileFull, tz: Tz) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: owner_profile_full.id,
            name: owner_profile_full.name,
            contact_name: owner_profile_full.contact_name,
            email: owner_profile_full.email,
            website: owner_profile_full.website,
            phone_number: owner_profile_full.phone_number,
            owner_profile_type: Self::map_owner_profile_type(
                &owner_profile_full.owner_profile_type,
            ),
            billing_address: owner_profile_full.billing_address.map(|v| v.to_string()),
            mailing_address: owner_profile_full.mailing_address.map(|v| v.to_string()),
            created_at: owner_profile_full
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: owner_profile_full
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
        }
    }
    fn map_owner_profile_type(owner_profile_type: &str) -> String {
        match owner_profile_type {
            "natural" => "Természetes személy",
            "legal" => "Jogi személy",
            _ => "Ismeretlen típus",
        }
        .to_string()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub fn test_owner_profile_full_print_builder() -> OwnerProfileFullPrintBuilder {
    let mut builder = OwnerProfileFullPrintBuilder::default();
    builder
        .id(Uuid::now_v7())
        .name("Test Owner".to_string())
        .contact_name(None)
        .email("test.owner@example.com".to_string())
        .website(Some("https://example.com".to_string()))
        .phone_number(Some("+36301234567".to_string()))
        .owner_profile_type("Természetes személy".to_string())
        .billing_address(Some(
            test_address_resolved_builder().build().unwrap().to_string(),
        ))
        .mailing_address(None)
        .created_at(TEST_TIME_TZ.clone())
        .updated_at(TEST_TIME_TZ.clone());

    builder
}

#[cfg(test)]
mod tests {

    use crate::{
        common::TEST_TZ, tenant::owner_profile::model::tests::test_owner_profile_full_builder,
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_owner_profile_resolved() {
        let owner_profile_id = Uuid::now_v7();
        let owner_profile_resolved = test_owner_profile_full_builder()
            .id(owner_profile_id)
            .build()
            .unwrap();
        let owner_profile_resolved_print =
            OwnerProfileFullPrint::new(owner_profile_resolved, *TEST_TZ);
        let owner_profile_resolved_print_expected = OwnerProfileFullPrint {
            id: owner_profile_id,
            name: "Test Owner".to_string(),
            contact_name: None,
            email: "test.owner@example.com".to_string(),
            website: Some("https://example.com".to_string()),
            phone_number: Some("+36301234567".to_string()),
            owner_profile_type: "Természetes személy".to_string(),
            billing_address: None,
            mailing_address: None,
            created_at: TEST_TIME_TZ.clone(),
            updated_at: TEST_TIME_TZ.clone(),
        };

        assert_eq!(
            owner_profile_resolved_print,
            owner_profile_resolved_print_expected
        );
    }
}
