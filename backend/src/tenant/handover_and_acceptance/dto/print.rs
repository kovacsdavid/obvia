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
use crate::common::utils::thousand_separated_number_bigdecimal;
use crate::tenant::customers::dto::print::CustomerFullPrint;
use crate::tenant::customers::dto::print::test_customer_full_print_builder;
use crate::tenant::handover_and_acceptance::model::HandoverAndAcceptanceFull;
use crate::tenant::owner_profile::dto::print::OwnerProfileFullPrint;
use chrono_tz::Tz;
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize, PartialEq, Debug, Builder)]
#[builder(build_fn(error = "CommonBuilderError"))]
pub struct Print {
    id: Uuid,
    display_id: String,
    owner_profile: OwnerProfileFullPrint,
    customer: CustomerFullPrint,
    item_make: Option<String>,
    item_model: Option<String>,
    item_identifier_type: String,
    item_identifier_value: String,
    reported_issue: Option<String>,
    condition_at_receipt: Option<String>,
    included_accessories: Option<Vec<String>>,
    authorized_work: Option<String>,
    repair_limit: Option<String>,
    terms_version: Option<String>,
    accepted_by: String,
    received_at: String,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl Print {
    pub fn new(
        handover_and_acceptance_full: HandoverAndAcceptanceFull,
        owner_profile_full_print: OwnerProfileFullPrint,
        tz: Tz,
    ) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        // NOTE: on user_input validation if repair_limit is set currency_code should be required.
        let repair_limit = if let Some(repair_limit) = &handover_and_acceptance_full.repair_limit
            && let Some(currency_code) = &handover_and_acceptance_full.currency_code
        {
            Some(format!(
                "{} {}",
                thousand_separated_number_bigdecimal(repair_limit, 2),
                currency_code.code
            ))
        } else {
            None
        };

        let accepted_by = if let Some(first_name) =
            handover_and_acceptance_full.accepted_by.first_name
            && let Some(last_name) = handover_and_acceptance_full.accepted_by.last_name
        {
            format!("{last_name} {first_name}")
        } else {
            "N/A".to_string()
        };
        Self {
            id: handover_and_acceptance_full.id,
            display_id: handover_and_acceptance_full.display_id,
            owner_profile: owner_profile_full_print,
            customer: CustomerFullPrint::new(handover_and_acceptance_full.customer, tz),
            item_make: handover_and_acceptance_full.item_make.map(|v| v.name),
            item_model: handover_and_acceptance_full.item_model.map(|v| v.name),
            item_identifier_type: handover_and_acceptance_full.item_identifier_type.name,
            item_identifier_value: handover_and_acceptance_full.item_identifier_value,
            reported_issue: handover_and_acceptance_full.reported_issue,
            condition_at_receipt: handover_and_acceptance_full.condition_at_receipt,
            included_accessories: handover_and_acceptance_full.included_accessories,
            authorized_work: handover_and_acceptance_full.authorized_work,
            repair_limit,
            terms_version: handover_and_acceptance_full.terms_version,
            accepted_by,
            received_at: handover_and_acceptance_full
                .received_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            created_at: handover_and_acceptance_full
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: handover_and_acceptance_full
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            deleted_at: handover_and_acceptance_full
                .deleted_at
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub fn test_handover_and_acceptance_full_print_builder(
    owner_profile_full_print: OwnerProfileFullPrint,
) -> PrintBuilder {
    let mut builder = PrintBuilder::default();
    builder
        .id(Uuid::now_v7())
        .display_id("KD-AT-2026-1".to_string())
        .owner_profile(owner_profile_full_print)
        .customer(test_customer_full_print_builder().build().unwrap())
        .item_make(Some("TestMake".to_string()))
        .item_model(Some("TestModel".to_string()))
        .item_identifier_type("serial number".to_string())
        .item_identifier_value("ASD123456789".to_string())
        .reported_issue(Some("Some reported issue".to_string()))
        .condition_at_receipt(Some("Condition at receipt".to_string()))
        .included_accessories(Some(vec![
            "chargeing cable".to_string(),
            "laptop bag".to_string(),
        ]))
        .authorized_work(Some("Some work to do.".to_string()))
        .repair_limit(Some("1 000.00 HUF".to_string()))
        .terms_version(Some("v1".to_string()))
        .accepted_by("Test User".to_string())
        .received_at(TEST_TIME_TZ.clone())
        .created_at(TEST_TIME_TZ.clone())
        .updated_at(TEST_TIME_TZ.clone())
        .deleted_at(None);

    builder
}

#[cfg(test)]
mod tests {

    use crate::{
        common::TEST_TZ,
        tenant::{
            address::model::test_address_resolved_builder,
            customers::model::tests::test_customer_full_builder,
            handover_and_acceptance::model::tests::test_handover_and_acceptance_full_builder,
            owner_profile::model::{OwnerProfileFull, tests::test_owner_profile_resolved_builder},
        },
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_handover_and_acceptance_full() {
        let handover_and_acceptance_id = Uuid::now_v7();
        let customer_id = Uuid::now_v7();
        let created_by_id = Uuid::now_v7();
        let owner_profile_id = Uuid::now_v7();
        let customer_full_print = test_customer_full_print_builder()
            .id(customer_id)
            .created_by_id(created_by_id)
            .build()
            .unwrap();
        let handover_and_acceptance_full = test_handover_and_acceptance_full_builder(
            test_customer_full_builder()
                .id(customer_id)
                .created_by_id(created_by_id)
                .billing_address(Some(test_address_resolved_builder().build().unwrap()))
                .build()
                .unwrap(),
        )
        .id(handover_and_acceptance_id)
        .build()
        .unwrap();

        let owner_profile_resolved = test_owner_profile_resolved_builder()
            .id(owner_profile_id)
            .build()
            .unwrap();

        let owner_profile_full_print = OwnerProfileFullPrint::new(
            OwnerProfileFull::from((owner_profile_resolved, None, None)),
            *TEST_TZ,
        );
        let handover_and_acceptance_resolved_print = Print::new(
            handover_and_acceptance_full,
            owner_profile_full_print.clone(),
            *TEST_TZ,
        );
        let handover_and_acceptance_resolved_print_expected = Print {
            id: handover_and_acceptance_id,
            display_id: "KD-AT-2026-1".to_string(),
            owner_profile: owner_profile_full_print,
            customer: customer_full_print,
            item_make: Some("TestMake".to_string()),
            item_model: Some("TestModel".to_string()),
            item_identifier_type: "serial number".to_string(),
            item_identifier_value: "ASD123456789".to_string(),
            reported_issue: Some("Some reported issue".to_string()),
            condition_at_receipt: Some("Condition at receipt".to_string()),
            included_accessories: Some(vec![
                "chargeing cable".to_string(),
                "laptop bag".to_string(),
            ]),
            authorized_work: Some("Some work to do.".to_string()),
            repair_limit: Some("1 000.00 HUF".to_string()),
            terms_version: Some("v1".to_string()),
            accepted_by: "Test User".to_string(),
            received_at: TEST_TIME_TZ.clone(),
            created_at: TEST_TIME_TZ.clone(),
            updated_at: TEST_TIME_TZ.clone(),
            deleted_at: None,
        };

        assert_eq!(
            handover_and_acceptance_resolved_print,
            handover_and_acceptance_resolved_print_expected
        );
    }
}
