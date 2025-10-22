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
use crate::common::types::Email;
use crate::common::types::{ValueObject, ValueObjectable};
use crate::tenant::customers::types::customer::customer_type::CustomerType;
use crate::tenant::customers::types::customer::{
    CustomerContactName, CustomerName, CustomerPhoneNumber, CustomerStatus,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerUserInput {
    pub id: Option<Uuid>,
    pub name: ValueObject<CustomerName>,
    pub contact_name: Option<ValueObject<CustomerContactName>>,
    pub email: ValueObject<Email>,
    pub phone_number: Option<ValueObject<CustomerPhoneNumber>>,
    pub status: ValueObject<CustomerStatus>,
    pub customer_type: ValueObject<CustomerType>,
}

impl TryFrom<CustomerUserInputHelper> for CustomerUserInput {
    type Error = CustomerUserInputError;
    fn try_from(value: CustomerUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = CustomerUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|e| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let name = ValueObject::new(CustomerName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });
        let email = ValueObject::new(Email(value.email)).inspect_err(|e| {
            error.email = Some(e.to_string());
        });
        let phone_number =
            ValueObject::new(CustomerPhoneNumber(value.phone_number)).inspect_err(|e| {
                error.phone_number = Some(e.to_string());
            });
        let status = ValueObject::new(CustomerStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });
        let customer_type = ValueObject::new(CustomerType(value.customer_type)).inspect_err(|e| {
            error.customer_type = Some(e.to_string());
        });

        let contact_name = match ValueObject::new(CustomerContactName(value.contact_name)) {
            Ok(val) => {
                if let Ok(customer_type) = &customer_type
                    && customer_type.extract().get_value().as_str() == "legal"
                {
                    Some(val)
                } else {
                    None
                }
            }
            Err(e) => {
                if let Ok(customer_type) = &customer_type
                    && customer_type.extract().get_value().as_str() == "legal"
                {
                    error.contact_name = Some(e.to_string());
                }
                None
            }
        };

        if error.is_empty() {
            Ok(CustomerUserInput {
                id,
                name: name.map_err(|_| CustomerUserInputError::default())?,
                contact_name,
                email: email.map_err(|_| CustomerUserInputError::default())?,
                phone_number: Some(phone_number.map_err(|_| CustomerUserInputError::default())?),
                status: status.map_err(|_| CustomerUserInputError::default())?,
                customer_type: customer_type.map_err(|_| CustomerUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
