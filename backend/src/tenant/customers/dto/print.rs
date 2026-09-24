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
use crate::tenant::customers::model::CustomerFull;
use chrono_tz::Tz;
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize, PartialEq, Debug, Builder)]
#[builder(build_fn(error = "CommonBuilderError"))]
pub struct CustomerFullPrint {
    id: Uuid,
    name: String,
    contact_name: Option<String>,
    email: String,
    phone_number: Option<String>,
    status: String,
    customer_type: String,
    created_by_id: Uuid,
    created_by: String,
    billing_address: Option<String>,
    mailing_address: Option<String>,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl CustomerFullPrint {
    pub fn new(customer_full: CustomerFull, tz: Tz) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: customer_full.id,
            name: customer_full.name,
            contact_name: customer_full.contact_name,
            email: customer_full.email,
            phone_number: customer_full.phone_number,
            status: Self::map_status(&customer_full.status),
            customer_type: Self::map_customer_type(&customer_full.customer_type),
            created_by_id: customer_full.created_by_id,
            created_by: customer_full.created_by,
            billing_address: customer_full.billing_address.map(|v| v.to_string()),
            mailing_address: customer_full.mailing_address.map(|v| v.to_string()),
            created_at: customer_full
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: customer_full
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            deleted_at: customer_full
                .deleted_at
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
        }
    }
    fn map_status(status: &str) -> String {
        match status {
            "active" => "Aktív",
            "lead" => "Érdeklődő",
            "prospect" => "Lehetséges vevő",
            "inactive" => "Inaktív",
            _ => "Ismeretlen státusz",
        }
        .to_string()
    }
    fn map_customer_type(customer_type: &str) -> String {
        match customer_type {
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

pub fn test_customer_full_print_builder() -> CustomerFullPrintBuilder {
    let mut builder = CustomerFullPrintBuilder::default();
    builder
        .id(Uuid::new_v4())
        .name("Test Customer".to_string())
        .contact_name(None)
        .email("test.customer@example.com".to_string())
        .phone_number(Some("+36301234567".to_string()))
        .status("Aktív".to_string())
        .customer_type("Természetes személy".to_string())
        .created_by_id(Uuid::new_v4())
        .created_by("Test User".to_string())
        .billing_address(Some(
            test_address_resolved_builder().build().unwrap().to_string(),
        ))
        .mailing_address(None)
        .created_at(TEST_TIME_TZ.clone())
        .updated_at(TEST_TIME_TZ.clone())
        .deleted_at(None);

    builder
}

#[cfg(test)]
mod tests {

    use crate::{common::TEST_TZ, tenant::customers::model::tests::test_customer_full_builder};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_customer_resolved() {
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let customer_resolved = test_customer_full_builder()
            .id(customer_id)
            .created_by_id(created_by_id)
            .build()
            .unwrap();
        let customer_resolved_print = CustomerFullPrint::new(customer_resolved, *TEST_TZ);
        let customer_resolved_print_expected = CustomerFullPrint {
            id: customer_id,
            name: "Test Customer".to_string(),
            contact_name: None,
            email: "test.customer@example.com".to_string(),
            phone_number: Some("+36301234567".to_string()),
            status: "Aktív".to_string(),
            customer_type: "Természetes személy".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            billing_address: None,
            mailing_address: None,
            created_at: TEST_TIME_TZ.clone(),
            updated_at: TEST_TIME_TZ.clone(),
            deleted_at: None,
        };

        assert_eq!(customer_resolved_print, customer_resolved_print_expected);
    }
}
