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

use crate::manager::common::types::value_object::ValueObject;
use crate::tenant::customers::types::customer::contact_name::ContactName;
use crate::tenant::customers::types::customer::{CustomerEmail, CustomerName, CustomerStatus};
use crate::tenant::warehouses::types::warehouse::contact_phone::ContactPhone;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

pub struct CreateCustomerHelper {
    // TODO: fields
}

pub struct CreateCustomerError {
    // TODO: fields
}

impl CreateCustomerError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for CreateCustomerError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub name: ValueObject<CustomerName>,
    pub contact_name: Option<ValueObject<ContactName>>,
    pub email: ValueObject<CustomerEmail>,
    pub phone_number: Option<ValueObject<ContactPhone>>,
    pub status: ValueObject<CustomerStatus>,
}

impl TryFrom<CreateCustomerHelper> for CreateCustomer {
    type Error = CreateCustomerError;
    fn try_from(value: CreateCustomerHelper) -> Result<Self, Self::Error> {
        todo!()
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
    pub contact_name: Option<ValueObject<ContactName>>,
    pub email: ValueObject<CustomerEmail>,
    pub phone_number: Option<ValueObject<ContactPhone>>,
    pub status: ValueObject<CustomerStatus>,
}

impl TryFrom<UpdateCustomerHelper> for UpdateCustomer {
    type Error = UpdateCustomerError;
    fn try_from(value: UpdateCustomerHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}
