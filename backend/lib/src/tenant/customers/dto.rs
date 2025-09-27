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
use crate::common::types::value_object::{ValueObject, ValueObjectable};
use crate::tenant::customers::types::customer::customer_type::CustomerType;
use crate::tenant::customers::types::customer::{
    CustomerContactName, CustomerName, CustomerPhoneNumber, CustomerStatus,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct CreateCustomerHelper {
    pub name: String,
    pub contact_name: String,
    pub email: String,
    pub phone_number: String,
    pub status: String,
    pub customer_type: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateCustomerError {
    pub name: Option<String>,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub status: Option<String>,
    pub customer_type: Option<String>,
}

impl CreateCustomerError {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.contact_name.is_none()
            && self.email.is_none()
            && self.phone_number.is_none()
            && self.status.is_none()
            && self.customer_type.is_none()
    }
}

impl Display for CreateCustomerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateCustomerError: {}", json),
            Err(e) => write!(f, "CreateCustomerError: {}", e),
        }
    }
}

impl FormErrorResponse for CreateCustomerError {}

impl IntoResponse for CreateCustomerError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub name: ValueObject<CustomerName>,
    pub contact_name: Option<ValueObject<CustomerContactName>>,
    pub email: ValueObject<Email>,
    pub phone_number: Option<ValueObject<CustomerPhoneNumber>>,
    pub status: ValueObject<CustomerStatus>,
    pub customer_type: ValueObject<CustomerType>,
}

impl TryFrom<CreateCustomerHelper> for CreateCustomer {
    type Error = CreateCustomerError;
    fn try_from(value: CreateCustomerHelper) -> Result<Self, Self::Error> {
        let mut error = CreateCustomerError::default();

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
            Ok(CreateCustomer {
                name: name.map_err(|_| CreateCustomerError::default())?,
                contact_name,
                email: email.map_err(|_| CreateCustomerError::default())?,
                phone_number: Some(phone_number.map_err(|_| CreateCustomerError::default())?),
                status: status.map_err(|_| CreateCustomerError::default())?,
                customer_type: customer_type.map_err(|_| CreateCustomerError::default())?,
            })
        } else {
            Err(error)
        }
    }
}

pub struct UpdateCustomerHelper {
    // TODO: fields
}

pub struct UpdateCustomerError {
    // TODO: fields
}

impl UpdateCustomerError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateCustomerError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomer {
    pub name: ValueObject<CustomerName>,
    pub contact_name: Option<ValueObject<CustomerContactName>>,
    pub email: ValueObject<Email>,
    pub phone_number: Option<ValueObject<CustomerPhoneNumber>>,
    pub status: ValueObject<CustomerStatus>,
}

impl TryFrom<UpdateCustomerHelper> for UpdateCustomer {
    type Error = UpdateCustomerError;
    fn try_from(value: UpdateCustomerHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}
