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
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAddress {
    pub street_address: ValueObject<StreetAddress>,
    pub city_id: Uuid,
    pub state_id: Uuid,
    pub country_id: Uuid,
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAddress {
    pub street_address: ValueObject<StreetAddress>,
    pub city_id: Uuid,
    pub state_id: Uuid,
    pub country_id: Uuid,
    pub additional_info: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCity {
    pub postal_code: Uuid,
    pub name: ValueObject<CityName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCity {
    pub postal_code: Uuid,
    pub name: ValueObject<CityName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostalCode {
    pub postal_code: ValueObject<PostalCode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostalCode {
    pub postal_code: ValueObject<PostalCode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateState {
    pub name: ValueObject<StateName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateState {
    pub name: ValueObject<StateName>,
}
