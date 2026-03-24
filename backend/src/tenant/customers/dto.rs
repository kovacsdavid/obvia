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
use crate::common::types::{Email, UuidVO, ValueObject};
use crate::tenant::customers::types::customer::customer_type::CustomerType;
use crate::tenant::customers::types::customer::{
    CustomerContactName, CustomerName, CustomerPhoneNumber, CustomerStatus,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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
    pub id: Option<ValueObject<UuidVO>>,
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

        let id = if let Some(id) = value.id {
            ValueObject::new_optional(UuidVO(id)).inspect_err(|e| {
                error.id = Some(e.to_string());
            })
        } else {
            Ok(None)
        };

        let name = ValueObject::new_required(CustomerName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });

        let email = ValueObject::new_required(Email(value.email)).inspect_err(|e| {
            error.email = Some(e.to_string());
        });

        let phone_number = ValueObject::new_required(CustomerPhoneNumber(value.phone_number))
            .inspect_err(|e| {
                error.phone_number = Some(e.to_string());
            });

        let status = ValueObject::new_required(CustomerStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        let customer_type = ValueObject::new_required(CustomerType(value.customer_type))
            .inspect_err(|e| {
                error.customer_type = Some(e.to_string());
            });

        let contact_name = if let Ok(customer_type) = &customer_type
            && customer_type.as_str() == "legal"
        {
            ValueObject::new_required(CustomerContactName(value.contact_name))
                .inspect_err(|e| {
                    error.contact_name = Some(e.to_string());
                })
                .map(Some)
        } else {
            Ok(None)
        };

        if error.is_empty() {
            Ok(CustomerUserInput {
                id: id.map_err(|_| CustomerUserInputError::default())?,
                name: name.map_err(|_| CustomerUserInputError::default())?,
                contact_name: contact_name.map_err(|_| CustomerUserInputError::default())?,
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
