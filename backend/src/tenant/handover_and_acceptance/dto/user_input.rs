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

use crate::common::error::v2::{AppError, AppErrorVisibility};
use crate::common::types::UuidVO;
use crate::common::value_object::*;
use crate::tenant::address::dto::user_input::{
    AddressUserInputError,
};
use crate::tenant::currencies::types::CurrencyCode;
use crate::tenant::handover_and_acceptance::types;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use tracing::Level;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HandoverAndAcceptanceUserInputHelper {
    pub id: Option<String>,
    pub customer_id: String,
    pub item_make_id: String,
    pub new_item_make: String,
    pub item_model_id: String,
    pub new_item_model: String,
    pub item_identifier_type_id: String,
    pub new_item_identifier_type: String,
    pub item_identifier_value: String,
    pub reported_issue: String,
    pub condition_at_receipt: String,
    pub included_accessories: Vec<String>,
    pub authorized_work: String,
    pub repair_limit: String,
    pub currency_code: String,
    pub terms_version: String,
    pub received_at: String,
}

#[derive(Debug, Serialize, Default, PartialEq)]
pub struct HandoverAndAcceptanceUserInputError {
    pub id: Option<String>,
    pub customer_id: Option<String>,
    pub item_make_id: Option<String>,
    pub new_item_make: Option<String>,
    pub item_model_id: Option<String>,
    pub new_item_model: Option<String>,
    pub item_identifier_type_id: Option<String>,
    pub new_item_identifier_type: Option<String>,
    pub item_identifier_value: Option<String>,
    pub reported_issue: Option<String>,
    pub condition_at_receipt: Option<String>,
    pub included_accessories: Option<String>,
    pub authorized_work: Option<String>,
    pub repair_limit: Option<String>,
    pub currency_code: Option<String>,
    pub terms_version: Option<String>,
    pub received_at: Option<String>,
}

impl HandoverAndAcceptanceUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.customer_id.is_none()
            && self.item_make_id.is_none()
            && self.new_item_make.is_none()
            && self.item_model_id.is_none()
            && self.new_item_model.is_none()
            && self.item_identifier_type_id.is_none()
            && self.new_item_identifier_type.is_none()
            && self.item_identifier_value.is_none()
            && self.reported_issue.is_none()
            && self.condition_at_receipt.is_none()
            && self.included_accessories.is_none()
            && self.authorized_work.is_none()
            && self.repair_limit.is_none()
            && self.currency_code.is_none()
            && self.terms_version.is_none()
            && self.received_at.is_none()
    }
}

impl Display for HandoverAndAcceptanceUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateHandoverAndAcceptanceError: {}", json),
            Err(e) => write!(f, "CreateHandoverAndAcceptanceError: {}", e),
        }
    }
}

impl From<HandoverAndAcceptanceUserInputError> for AppError {
    fn from(value: HandoverAndAcceptanceUserInputError) -> Self {
        Self::new(
            Level::DEBUG,
            StatusCode::UNPROCESSABLE_ENTITY,
            AppErrorVisibility::UserFacing,
            json!({
                "message": "Kérjük ellenőrizze a hibás mezőket!",
                "fields": value
            }),
        )
    }
}

impl From<ValueObjectError> for HandoverAndAcceptanceUserInputError {
    fn from(_: ValueObjectError) -> Self {
        HandoverAndAcceptanceUserInputError::default()
    }
}

impl From<AddressUserInputError> for HandoverAndAcceptanceUserInputError {
    fn from(_: AddressUserInputError) -> Self {
        HandoverAndAcceptanceUserInputError::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HandoverAndAcceptanceUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub customer_id: ValueObjectRequired<UuidVO>,
    pub item_make_id: ValueObjectOptional<UuidVO>,
    pub new_item_make: ValueObjectOptional<types::item_makes::name::Name>,
    pub item_model_id: ValueObjectOptional<UuidVO>,
    pub new_item_model: ValueObjectOptional<types::item_models::name::Name>,
    pub item_identifier_type_id: Option<ValueObjectRequired<UuidVO>>,
    pub new_item_identifier_type:
        Option<ValueObjectRequired<types::item_identifier_types::name::Name>>,
    pub item_identifier_value: ValueObjectRequired<
        types::handover_and_acceptance::item_identifier_value::ItemIdentifierValue,
    >,
    pub reported_issue:
        ValueObjectOptional<types::handover_and_acceptance::reported_issue::ReportedIssue>,
    pub condition_at_receipt: ValueObjectOptional<
        types::handover_and_acceptance::condition_at_receipt::ConditionAtReceipt,
    >,
    pub included_accessories: Vec<
        ValueObjectRequired<types::handover_and_acceptance::included_accessory::IncludedAccessory>,
    >,
    pub authorized_work:
        ValueObjectOptional<types::handover_and_acceptance::authorized_work::AuthorizedWork>,
    pub repair_limit:
        ValueObjectOptional<types::handover_and_acceptance::repair_limit::RepairLimit>,
    pub currency_code: ValueObjectOptional<CurrencyCode>,
    pub terms_version:
        ValueObjectOptional<types::handover_and_acceptance::terms_version::TermsVersion>,
    pub received_at: ValueObjectRequired<types::handover_and_acceptance::received_at::ReceivedAt>,
}

impl TryFrom<HandoverAndAcceptanceUserInputHelper> for HandoverAndAcceptanceUserInput {
    type Error = HandoverAndAcceptanceUserInputError;
    fn try_from(value: HandoverAndAcceptanceUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = HandoverAndAcceptanceUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        todo!()

        if error.is_empty() {
            Ok(HandoverAndAcceptanceUserInput { id: id? })
        } else {
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn valid_handover_and_acceptance_user_input_natural() {
        todo!()
    }

    #[test]
    fn valid_handover_and_acceptance_user_input_legal() {
        todo!()
    }
    #[test]
    fn invalid_handover_and_acceptance_user_input_natural() {
        todo!()
    }

    #[test]
    fn invalid_handover_and_acceptance_user_input_legal() {}
}
