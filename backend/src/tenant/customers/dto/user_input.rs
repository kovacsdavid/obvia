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

use crate::common::error::FormErrorResponse;
use crate::common::types::{Email, UuidVO};
use crate::common::value_object::*;
use crate::tenant::customers::types::customer::*;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerUserInputHelper {
    pub id: Option<String>,
    pub name: String,
    pub contact_name: String,
    pub email: String,
    pub phone_number: String,
    pub status: String,
    pub customer_type: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CustomerUserInputError {
    pub id: Option<String>,
    pub name: Option<String>,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub status: Option<String>,
    pub customer_type: Option<String>,
}

impl CustomerUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.name.is_none()
            && self.contact_name.is_none()
            && self.email.is_none()
            && self.phone_number.is_none()
            && self.status.is_none()
            && self.customer_type.is_none()
    }
}

impl Display for CustomerUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateCustomerError: {}", json),
            Err(e) => write!(f, "CreateCustomerError: {}", e),
        }
    }
}

impl FormErrorResponse for CustomerUserInputError {}

impl IntoResponse for CustomerUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

impl From<ValueObjectError> for CustomerUserInputError {
    fn from(_: ValueObjectError) -> Self {
        CustomerUserInputError::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomerUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub name: ValueObjectRequired<CustomerName>,
    pub contact_name: Option<ValueObjectRequired<CustomerContactName>>,
    pub email: ValueObjectRequired<Email>,
    pub phone_number: ValueObjectOptional<CustomerPhoneNumber>,
    pub status: ValueObjectRequired<CustomerStatus>,
    pub customer_type: ValueObjectRequired<CustomerType>,
}

impl TryFrom<CustomerUserInputHelper> for CustomerUserInput {
    type Error = CustomerUserInputError;
    fn try_from(value: CustomerUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = CustomerUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let name = value
            .name
            .parse::<ValueObjectRequired<CustomerName>>()
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
            .parse::<ValueObjectOptional<CustomerPhoneNumber>>()
            .inspect_err(|e| {
                error.phone_number = Some(e.to_string());
            });
        let status = value
            .status
            .parse::<ValueObjectRequired<CustomerStatus>>()
            .inspect_err(|e| {
                error.status = Some(e.to_string());
            });

        let customer_type = value
            .customer_type
            .parse::<ValueObjectRequired<CustomerType>>()
            .inspect_err(|e| {
                error.customer_type = Some(e.to_string());
            });

        let contact_name = if let Ok(customer_type) = &customer_type
            && let Ok(customer_type) = customer_type.as_str()
            && customer_type == "legal"
        {
            value
                .contact_name
                .parse::<ValueObjectRequired<CustomerContactName>>()
                .inspect_err(|e| {
                    error.contact_name = Some(e.to_string());
                })
                .map(Some)
        } else {
            Ok(None)
        };

        if error.is_empty() {
            Ok(CustomerUserInput {
                id: id?,
                name: name?,
                contact_name: contact_name?,
                email: email?,
                phone_number: phone_number?,
                status: status?,
                customer_type: customer_type?,
            })
        } else {
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_customer_user_input_natural() {
        let cui = CustomerUserInput::try_from(CustomerUserInputHelper {
            id: None,
            name: String::from("Teszt Elek"),
            contact_name: String::from(""),
            email: String::from("teszt.elek@example.com"),
            phone_number: String::from("+36301234567"),
            status: String::from("active"),
            customer_type: String::from("natural"),
        })
        .unwrap();
        assert_eq!(cui.id.as_uuid(), None);
        assert_eq!(cui.name.as_str().unwrap(), "Teszt Elek");
        assert_eq!(cui.contact_name, None);
        assert_eq!(cui.email.as_str().unwrap(), "teszt.elek@example.com");
        assert_eq!(cui.phone_number.as_str(), Some("+36301234567"));
        assert_eq!(cui.status.as_str().unwrap(), "active");
        assert_eq!(cui.customer_type.as_str().unwrap(), "natural");
    }

    #[test]
    fn valid_customer_user_input_legal() {
        let cui = CustomerUserInput::try_from(CustomerUserInputHelper {
            id: None,
            name: String::from("Teszt Kft."),
            contact_name: String::from("Teszt Elek"),
            email: String::from("teszt.elek@example.com"),
            phone_number: String::from("+36301234567"),
            status: String::from("active"),
            customer_type: String::from("legal"),
        })
        .unwrap();
        assert_eq!(cui.id.as_uuid(), None);
        assert_eq!(cui.name.as_str().unwrap(), "Teszt Kft.");
        assert_eq!(cui.contact_name.unwrap().as_str().unwrap(), "Teszt Elek");
        assert_eq!(cui.email.as_str().unwrap(), "teszt.elek@example.com");
        assert_eq!(cui.phone_number.as_str(), Some("+36301234567"));
        assert_eq!(cui.status.as_str().unwrap(), "active");
        assert_eq!(cui.customer_type.as_str().unwrap(), "legal");
    }
    #[test]
    fn invalid_customer_user_input_natural() {
        let cuie = CustomerUserInput::try_from(CustomerUserInputHelper {
            id: Some(String::from("asd")),
            name: String::from(""),
            contact_name: String::from(""),
            email: String::from("teszt.elekexample.com"),
            phone_number: String::from("+36@301234567"),
            status: String::from("activee"),
            customer_type: String::from("natural"),
        })
        .unwrap_err();
        assert_eq!(cuie.id.unwrap(), UuidVO::PARSE_ERROR);
        assert_eq!(cuie.name.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(cuie.contact_name, None);
        assert_eq!(cuie.email.unwrap(), Email::VALIDATION_ERROR);
        assert_eq!(
            cuie.phone_number.unwrap(),
            CustomerPhoneNumber::VALIDATION_ERROR
        );
        assert_eq!(cuie.status.unwrap(), CustomerStatus::VALIDATION_ERROR);
        assert_eq!(cuie.customer_type, None);
    }

    #[test]
    fn invalid_customer_user_input_legal() {
        let cuie = CustomerUserInput::try_from(CustomerUserInputHelper {
            id: None,
            name: String::from(""),
            contact_name: String::from(""),
            email: String::from(""),
            phone_number: String::from("+3630a234567"),
            status: String::from(""),
            customer_type: String::from("legal"),
        })
        .unwrap_err();
        assert_eq!(cuie.id, None);
        assert_eq!(cuie.name.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(cuie.contact_name.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(cuie.email.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(
            cuie.phone_number.unwrap(),
            CustomerPhoneNumber::VALIDATION_ERROR
        );
        assert_eq!(cuie.status.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(cuie.customer_type, None);
    }
}
