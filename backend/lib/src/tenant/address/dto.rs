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
use crate::tenant::address::types::address::street_address::StreetAddress;
use crate::tenant::address::types::city::CityName;
use crate::tenant::address::types::postal_code::postal_code::PostalCode;
use crate::tenant::address::types::state::StateName;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct CreateAddressHelper {
    // TODO: fields
}

pub struct CreateAddressError {
    // TODO: fields
}

impl CreateAddressError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for CreateAddressError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAddress {
    pub street_address: ValueObject<StreetAddress>,
    pub city_id: Uuid,
    pub state_id: Uuid,
    pub country_id: Uuid,
    pub additional_info: Option<String>,
}

impl TryFrom<CreateAddressHelper> for CreateAddress {
    type Error = CreateAddressError;
    fn try_from(value: CreateAddressHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct UpdateAddressHelper {
    // TODO: fields
}

pub struct UpdateAddressError {
    // TODO: fields
}

impl UpdateAddressError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateAddressError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAddress {
    pub street_address: ValueObject<StreetAddress>,
    pub city_id: Uuid,
    pub state_id: Uuid,
    pub country_id: Uuid,
    pub additional_info: Option<String>,
}

impl TryFrom<UpdateAddressHelper> for UpdateAddress {
    type Error = UpdateAddressError;
    fn try_from(value: UpdateAddressHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerAddressConnect {
    pub address_id: Uuid,
    pub addressable_id: Uuid,
    pub addressable_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerAddressConnect {
    pub address_id: Uuid,
    pub addressable_id: Uuid,
    pub addressable_type: String,
}

pub struct CreateCityHelper {
    // TODO: fields
}

pub struct CreateCityError {
    // TODO: fields
}

impl CreateCityError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for CreateCityError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCity {
    pub postal_code: Uuid,
    pub name: ValueObject<CityName>,
}

impl TryFrom<CreateCityHelper> for CreateCity {
    type Error = CreateCityError;
    fn try_from(value: CreateCityHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct UpdateCityHelper {
    // TODO: fields
}

pub struct UpdateCityError {
    // TODO: fields
}

impl UpdateCityError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateCityError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCity {
    pub postal_code: Uuid,
    pub name: ValueObject<CityName>,
}

impl TryFrom<UpdateCityHelper> for UpdateCity {
    type Error = UpdateCityError;
    fn try_from(value: UpdateCityHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct CreatePostalCodeHelper {
    // TODO: fields
}

pub struct CreatePostalCodeError {
    // TODO: fields
}

impl CreatePostalCodeError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for CreatePostalCodeError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostalCode {
    pub postal_code: ValueObject<PostalCode>,
}

impl TryFrom<CreatePostalCodeHelper> for CreatePostalCode {
    type Error = CreatePostalCodeError;
    fn try_from(value: CreatePostalCodeHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct UpdatePostalCodeHelper {
    // TODO: fields
}

pub struct UpdatePostalCodeError {
    // TODO: fields
}

impl UpdatePostalCodeError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdatePostalCodeError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostalCode {
    pub postal_code: ValueObject<PostalCode>,
}

impl TryFrom<UpdatePostalCodeHelper> for UpdatePostalCode {
    type Error = UpdatePostalCodeError;
    fn try_from(value: UpdatePostalCodeHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct CreateStateHelper {
    // TODO: fields
}

pub struct CreateStateError {
    // TODO: fields
}

impl CreateStateError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for CreateStateError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateState {
    pub name: ValueObject<StateName>,
}

impl TryFrom<CreateStateHelper> for CreateState {
    type Error = CreateStateError;
    fn try_from(value: CreateStateHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct UpdateStateHelper {
    // TODO: fields
}

pub struct UpdateStateError {
    // TODO: fields
}

impl UpdateStateError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateStateError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateState {
    pub name: ValueObject<StateName>,
}

impl TryFrom<UpdateStateHelper> for UpdateState {
    type Error = UpdateStateError;
    fn try_from(value: UpdateStateHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}
