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
use crate::common::types::{Email, UuidVO};
use crate::common::value_object::*;
use crate::tenant::address::dto::user_input::{
    AddressUserInput, AddressUserInputError, AddressUserInputHelper,
};
use crate::tenant::owner_profile::types::owner_profile::*;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use tracing::Level;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OwnerProfileUserInputHelper {
    pub id: Option<String>,
    pub name: String,
    pub contact_name: String,
    pub email: String,
    pub phone_number: String,
    pub owner_profile_type: String,
    pub billing_address: Option<AddressUserInputHelper>,
    pub mailing_address: Option<AddressUserInputHelper>,
}

#[derive(Debug, Serialize, Default, PartialEq)]
pub struct OwnerProfileUserInputError {
    pub id: Option<String>,
    pub name: Option<String>,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub owner_profile_type: Option<String>,
    pub billing_address: AddressUserInputError,
    pub mailing_address: AddressUserInputError,
}

impl OwnerProfileUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.name.is_none()
            && self.contact_name.is_none()
            && self.email.is_none()
            && self.phone_number.is_none()
            && self.owner_profile_type.is_none()
            && self.billing_address.is_empty()
            && self.mailing_address.is_empty()
    }
}

impl Display for OwnerProfileUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateOwnerProfileError: {}", json),
            Err(e) => write!(f, "CreateOwnerProfileError: {}", e),
        }
    }
}

impl From<OwnerProfileUserInputError> for AppError {
    fn from(value: OwnerProfileUserInputError) -> Self {
        Self::new(
            Level::DEBUG,
            StatusCode::UNPROCESSABLE_ENTITY,
            file!(),
            AppErrorVisibility::UserFacing,
            json!({
                "message": "Kérjük ellenőrizze a hibás mezőket!",
                "fields": value
            }),
        )
    }
}

impl From<ValueObjectError> for OwnerProfileUserInputError {
    fn from(_: ValueObjectError) -> Self {
        OwnerProfileUserInputError::default()
    }
}

impl From<AddressUserInputError> for OwnerProfileUserInputError {
    fn from(_: AddressUserInputError) -> Self {
        OwnerProfileUserInputError::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OwnerProfileUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub name: ValueObjectRequired<OwnerProfileName>,
    pub contact_name: Option<ValueObjectRequired<OwnerProfileContactName>>,
    pub email: ValueObjectRequired<Email>,
    pub phone_number: ValueObjectOptional<OwnerProfilePhoneNumber>,
    pub owner_profile_type: ValueObjectRequired<OwnerProfileType>,
    pub billing_address: Option<AddressUserInput>,
    pub mailing_address: Option<AddressUserInput>,
}

impl TryFrom<OwnerProfileUserInputHelper> for OwnerProfileUserInput {
    type Error = OwnerProfileUserInputError;
    fn try_from(value: OwnerProfileUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = OwnerProfileUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let name = value
            .name
            .parse::<ValueObjectRequired<OwnerProfileName>>()
            .inspect_err(|e| {
                error.name = Some(e.to_string());
            });

        let email = value
            .email
            .parse::<ValueObjectRequired<Email>>()
            .inspect_err(|e| {
                error.email = Some(e.to_string());
            });

        let phone_number = value
            .phone_number
            .parse::<ValueObjectOptional<OwnerProfilePhoneNumber>>()
            .inspect_err(|e| {
                error.phone_number = Some(e.to_string());
            });

        let owner_profile_type = value
            .owner_profile_type
            .parse::<ValueObjectRequired<OwnerProfileType>>()
            .inspect_err(|e| {
                error.owner_profile_type = Some(e.to_string());
            });

        let contact_name = if let Ok(owner_profile_type) = &owner_profile_type
            && let Ok(owner_profile_type) = owner_profile_type.as_str()
            && owner_profile_type == "legal"
        {
            value
                .contact_name
                .parse::<ValueObjectRequired<OwnerProfileContactName>>()
                .inspect_err(|e| {
                    error.contact_name = Some(e.to_string());
                })
                .map(Some)
        } else {
            Ok(None)
        };

        let billing_address = if let Some(billing_address) = value.billing_address {
            AddressUserInput::try_from(billing_address)
                .inspect_err(|e| {
                    error.billing_address = e.clone();
                })
                .ok()
        } else {
            None
        };

        let mailing_address = if let Some(mailing_address) = value.mailing_address {
            AddressUserInput::try_from(mailing_address)
                .inspect_err(|e| error.mailing_address = e.clone())
                .ok()
        } else {
            None
        };

        if error.is_empty() {
            Ok(OwnerProfileUserInput {
                id: id?,
                name: name?,
                contact_name: contact_name?,
                email: email?,
                phone_number: phone_number?,
                owner_profile_type: owner_profile_type?,
                billing_address,
                mailing_address,
            })
        } else {
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use crate::tenant::address::{
        dto::user_input::tests::{
            test_address_user_input_builder, test_address_user_input_helper_builder,
        },
        types::CountryCode,
    };

    use super::*;

    #[test]
    fn valid_owner_profile_user_input_natural() {
        let owner_profile_user_input =
            OwnerProfileUserInput::try_from(OwnerProfileUserInputHelper {
                id: None,
                name: String::from("Teszt Elek"),
                contact_name: String::from(""),
                email: String::from("teszt.elek@example.com"),
                phone_number: String::from("+36301234567"),
                owner_profile_type: String::from("natural"),
                billing_address: Some(test_address_user_input_helper_builder().build().unwrap()),
                mailing_address: None,
            });
        let expected_owner_profile_user_input = OwnerProfileUserInput {
            id: "".parse().unwrap(),
            name: "Teszt Elek".parse().unwrap(),
            contact_name: None,
            email: "teszt.elek@example.com".parse().unwrap(),
            phone_number: "+36301234567".parse().unwrap(),
            owner_profile_type: "natural".parse().unwrap(),
            billing_address: Some(test_address_user_input_builder().build().unwrap()),
            mailing_address: None,
        };

        assert!(owner_profile_user_input.is_ok());
        assert_eq!(
            expected_owner_profile_user_input,
            owner_profile_user_input.unwrap()
        );
    }

    #[test]
    fn valid_owner_profile_user_input_legal() {
        let owner_profile_user_input =
            OwnerProfileUserInput::try_from(OwnerProfileUserInputHelper {
                id: None,
                name: String::from("Teszt Kft."),
                contact_name: String::from("Teszt Elek"),
                email: String::from("teszt.elek@example.com"),
                phone_number: String::from("+36301234567"),
                owner_profile_type: String::from("legal"),
                billing_address: Some(test_address_user_input_helper_builder().build().unwrap()),
                mailing_address: None,
            });
        let expected_owner_profile_user_input = OwnerProfileUserInput {
            id: "".parse().unwrap(),
            name: "Teszt Kft.".parse().unwrap(),
            contact_name: Some("Teszt Elek".parse().unwrap()),
            email: "teszt.elek@example.com".parse().unwrap(),
            phone_number: "+36301234567".parse().unwrap(),
            owner_profile_type: "legal".parse().unwrap(),
            billing_address: Some(test_address_user_input_builder().build().unwrap()),
            mailing_address: None,
        };
        assert!(owner_profile_user_input.is_ok());
        assert_eq!(
            expected_owner_profile_user_input,
            owner_profile_user_input.unwrap()
        );
    }
    #[test]
    fn invalid_owner_profile_user_input_natural() {
        let owner_profile_user_input =
            OwnerProfileUserInput::try_from(OwnerProfileUserInputHelper {
                id: Some(String::from("asd")),
                name: String::from(""),
                contact_name: String::from(""),
                email: String::from("teszt.elekexample.com"),
                phone_number: String::from("+36@301234567"),
                owner_profile_type: String::from("natural"),
                billing_address: Some(
                    test_address_user_input_helper_builder()
                        .country_code("HUN".to_string())
                        .build()
                        .unwrap(),
                ),
                mailing_address: None,
            });

        let expected_owner_profile_user_input_error = OwnerProfileUserInputError {
            id: Some(UuidVO::PARSE_ERROR.to_string()),
            name: Some(ValueObjectError::REQUIRED.to_string()),
            contact_name: None,
            email: Some(Email::VALIDATION_ERROR.to_string()),
            phone_number: Some(OwnerProfilePhoneNumber::VALIDATION_ERROR.to_string()),
            owner_profile_type: None,
            billing_address: AddressUserInputError {
                id: None,
                address_type: None,
                country_code: Some(CountryCode::VALIDATION_ERROR.to_string()),
                postal_code: None,
                settlement: None,
                mailbox: None,
                topographic_number: None,
                name_of_public_space: None,
                type_of_public_space: None,
                house_number: None,
                building: None,
                stairway: None,
                floor: None,
                door: None,
            },
            mailing_address: AddressUserInputError {
                id: None,
                address_type: None,
                country_code: None,
                postal_code: None,
                settlement: None,
                mailbox: None,
                topographic_number: None,
                name_of_public_space: None,
                type_of_public_space: None,
                house_number: None,
                building: None,
                stairway: None,
                floor: None,
                door: None,
            },
        };
        assert!(owner_profile_user_input.is_err());
        assert_eq!(
            expected_owner_profile_user_input_error,
            owner_profile_user_input.unwrap_err()
        );
    }

    #[test]
    fn invalid_owner_profile_user_input_legal() {
        let owner_profile_user_input =
            OwnerProfileUserInput::try_from(OwnerProfileUserInputHelper {
                id: None,
                name: String::from(""),
                contact_name: String::from(""),
                email: String::from(""),
                phone_number: String::from("+3630a234567"),
                owner_profile_type: String::from("legal"),
                billing_address: Some(
                    test_address_user_input_helper_builder()
                        .country_code("HUN".to_string())
                        .build()
                        .unwrap(),
                ),
                mailing_address: None,
            });

        let expected_owner_profile_user_input_error = OwnerProfileUserInputError {
            id: None,
            name: Some(ValueObjectError::REQUIRED.to_string()),
            contact_name: Some(ValueObjectError::REQUIRED.to_string()),
            email: Some(ValueObjectError::REQUIRED.to_string()),
            phone_number: Some(OwnerProfilePhoneNumber::VALIDATION_ERROR.to_string()),
            owner_profile_type: None,
            billing_address: AddressUserInputError {
                id: None,
                address_type: None,
                country_code: Some(CountryCode::VALIDATION_ERROR.to_string()),
                postal_code: None,
                settlement: None,
                mailbox: None,
                topographic_number: None,
                name_of_public_space: None,
                type_of_public_space: None,
                house_number: None,
                building: None,
                stairway: None,
                floor: None,
                door: None,
            },
            mailing_address: AddressUserInputError {
                id: None,
                address_type: None,
                country_code: None,
                postal_code: None,
                settlement: None,
                mailbox: None,
                topographic_number: None,
                name_of_public_space: None,
                type_of_public_space: None,
                house_number: None,
                building: None,
                stairway: None,
                floor: None,
                door: None,
            },
        };

        assert!(owner_profile_user_input.is_err());
        assert_eq!(
            expected_owner_profile_user_input_error,
            owner_profile_user_input.unwrap_err()
        );
    }
}
