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
use crate::tenant::address::dto::user_input::AddressUserInputError;
use crate::tenant::currencies::types::CurrencyCode;
use crate::tenant::handover_and_acceptance::types;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use tracing::Level;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserInputHelper {
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
pub struct UserInputError {
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
    pub included_accessories: Vec<Option<String>>,
    pub authorized_work: Option<String>,
    pub repair_limit: Option<String>,
    pub currency_code: Option<String>,
    pub terms_version: Option<String>,
    pub received_at: Option<String>,
}

impl UserInputError {
    pub fn is_empty(&self) -> bool {
        let all_incluced_accessories_valid = || -> bool {
            for included_accessory in &self.included_accessories {
                if included_accessory.is_some() {
                    return false;
                }
            }
            true
        };

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
            && all_incluced_accessories_valid()
            && self.authorized_work.is_none()
            && self.repair_limit.is_none()
            && self.currency_code.is_none()
            && self.terms_version.is_none()
            && self.received_at.is_none()
    }
}

impl Display for UserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateHandoverAndAcceptanceError: {}", json),
            Err(e) => write!(f, "CreateHandoverAndAcceptanceError: {}", e),
        }
    }
}

impl From<UserInputError> for AppError {
    fn from(value: UserInputError) -> Self {
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

impl From<ValueObjectError> for UserInputError {
    fn from(_: ValueObjectError) -> Self {
        UserInputError::default()
    }
}

impl From<AddressUserInputError> for UserInputError {
    fn from(_: AddressUserInputError) -> Self {
        UserInputError::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub customer_id: ValueObjectRequired<UuidVO>,
    pub item_make_id: ValueObjectOptional<UuidVO>,
    pub new_item_make: Option<ValueObjectRequired<types::item_makes::name::Name>>,
    pub item_model_id: ValueObjectOptional<UuidVO>,
    pub new_item_model: Option<ValueObjectRequired<types::item_models::name::Name>>,
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

impl TryFrom<UserInputHelper> for UserInput {
    type Error = UserInputError;
    fn try_from(value: UserInputHelper) -> Result<Self, Self::Error> {
        let mut error = UserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let customer_id = value
            .customer_id
            .parse::<ValueObjectRequired<UuidVO>>()
            .inspect_err(|e| {
                error.customer_id = Some(e.to_string());
            });

        let item_make_id = if value.item_make_id != "other" {
            value
                .item_make_id
                .parse::<ValueObjectOptional<UuidVO>>()
                .inspect_err(|e| {
                    error.item_make_id = Some(e.to_string());
                })
        } else {
            "".parse::<ValueObjectOptional<UuidVO>>().inspect_err(|e| {
                error.item_make_id = Some(e.to_string());
            })
        };

        let new_item_make = if value.item_make_id == "other" {
            value
                .new_item_make
                .parse::<ValueObjectRequired<types::item_makes::name::Name>>()
                .inspect_err(|e| {
                    error.new_item_make = Some(e.to_string());
                })
                .ok()
        } else {
            None
        };

        let item_model_id = if value.item_model_id != "other" {
            value
                .item_model_id
                .parse::<ValueObjectOptional<UuidVO>>()
                .inspect_err(|e| {
                    error.item_model_id = Some(e.to_string());
                })
        } else {
            "".parse::<ValueObjectOptional<UuidVO>>().inspect_err(|e| {
                error.item_model_id = Some(e.to_string());
            })
        };

        let new_item_model = if value.item_model_id == "other" {
            value
                .new_item_model
                .parse::<ValueObjectRequired<types::item_models::name::Name>>()
                .inspect_err(|e| {
                    error.new_item_model = Some(e.to_string());
                })
                .ok()
        } else {
            None
        };

        let item_identifier_type_id = if value.item_identifier_type_id == "other" {
            None
        } else {
            value
                .item_identifier_type_id
                .parse::<ValueObjectRequired<UuidVO>>()
                .inspect_err(|e| {
                    error.item_identifier_type_id = Some(e.to_string());
                })
                .ok()
        };

        let new_item_identifier_type = if item_identifier_type_id.is_some() {
            None
        } else {
            value
                .new_item_identifier_type
                .parse::<ValueObjectRequired<types::item_identifier_types::name::Name>>()
                .inspect_err(|e| {
                    error.new_item_identifier_type = Some(e.to_string());
                })
                .ok()
        };

        let item_identifier_value = value
            .item_identifier_value
            .parse::<ValueObjectRequired<
                types::handover_and_acceptance::item_identifier_value::ItemIdentifierValue,
            >>()
            .inspect_err(|e| {
                error.item_identifier_value = Some(e.to_string());
            });

        let reported_issue = value
            .reported_issue
            .parse::<ValueObjectOptional<types::handover_and_acceptance::reported_issue::ReportedIssue>>()
            .inspect_err(|e| {
                error.reported_issue = Some(e.to_string());
            });

        let included_accessories =
            if !value.included_accessories.is_empty() {
                let mut included_accessories: Vec<
                    Result<
                        ValueObjectRequired<
                            types::handover_and_acceptance::included_accessory::IncludedAccessory,
                        >,
                        ValueObjectError,
                    >,
                > = vec![];
                let mut included_accessories_errors: Vec<Option<String>> =
                    vec![None; value.included_accessories.len()];

                for (i, included_accessory) in value.included_accessories.iter().enumerate() {
                    included_accessories.push(included_accessory.parse::<ValueObjectRequired<
                    types::handover_and_acceptance::included_accessory::IncludedAccessory,
                >>().inspect_err(|e| {
                    included_accessories_errors[i] = Some(e.to_string());
                }));
                }

                error.included_accessories = included_accessories_errors;

                included_accessories
            } else {
                let included_accessories: Vec<
                    Result<
                        ValueObjectRequired<
                            types::handover_and_acceptance::included_accessory::IncludedAccessory,
                        >,
                        ValueObjectError,
                    >,
                > = vec![];
                included_accessories
            };

        let condition_at_receipt = value
            .condition_at_receipt
            .parse::<ValueObjectOptional<
                types::handover_and_acceptance::condition_at_receipt::ConditionAtReceipt,
            >>()
            .inspect_err(|e| {
                error.condition_at_receipt = Some(e.to_string());
            });

        let authorized_work = value
            .authorized_work
            .parse::<ValueObjectOptional<
                types::handover_and_acceptance::authorized_work::AuthorizedWork
            >>()
            .inspect_err(|e| {
                error.authorized_work = Some(e.to_string());
            });

        let repair_limit = value
            .repair_limit
            .parse::<
                ValueObjectOptional<types::handover_and_acceptance::repair_limit::RepairLimit>
            >()
            .inspect_err(|e| {
                error.repair_limit = Some(e.to_string());
            });

        let currency_code = value
            .currency_code
            .parse::<ValueObjectOptional<CurrencyCode>>()
            .inspect_err(|e| {
                error.currency_code = Some(e.to_string());
            });

        let terms_version = value
            .terms_version
            .parse::<
                ValueObjectOptional<types::handover_and_acceptance::terms_version::TermsVersion>
            >().inspect_err(|e| {
            error.terms_version = Some(e.to_string());
        });

        let received_at = value
            .received_at
            .parse::<ValueObjectRequired<types::handover_and_acceptance::received_at::ReceivedAt>>()
            .inspect_err(|e| {
                error.received_at = Some(e.to_string());
            });

        if error.is_empty() {
            let mut included_accessories_final: Vec<
                ValueObjectRequired<
                    types::handover_and_acceptance::included_accessory::IncludedAccessory,
                >,
            > = vec![];

            for included_accessory in included_accessories {
                included_accessories_final.push(included_accessory?);
            }

            Ok(UserInput {
                id: id?,
                customer_id: customer_id?,
                item_make_id: item_make_id?,
                new_item_make,
                item_model_id: item_model_id?,
                new_item_model,
                item_identifier_type_id,
                new_item_identifier_type,
                item_identifier_value: item_identifier_value?,
                reported_issue: reported_issue?,
                condition_at_receipt: condition_at_receipt?,
                included_accessories: included_accessories_final,
                authorized_work: authorized_work?,
                repair_limit: repair_limit?,
                currency_code: currency_code?,
                terms_version: terms_version?,
                received_at: received_at?,
            })
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
