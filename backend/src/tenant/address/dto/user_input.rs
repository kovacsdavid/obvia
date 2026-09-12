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

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        types::UuidVO,
        value_object::{ValueObjectOptional, ValueObjectRequired},
    },
    tenant::address::types::{
        AddressType, Building, CountryCode, Door, Floor, HouseNumber, Mailbox, NameOfPublicSpace,
        PostalCode, Settlement, Stairway, TopographicNumber, TypeOfPublicSpace,
    },
};

#[derive(Clone, Debug, Deserialize, Serialize, Builder)]
pub struct AddressUserInputHelper {
    pub id: Option<String>,
    pub address_type: String,
    pub country_code: String,
    pub postal_code: String,
    pub settlement: String,
    pub mailbox: String,
    pub topographic_number: String,
    pub name_of_public_space: String,
    pub type_of_public_space: String,
    pub house_number: String,
    pub building: String,
    pub stairway: String,
    pub floor: String,
    pub door: String,
}

#[derive(Debug, Serialize, Default, Builder)]
pub struct AddressUserInputError {
    pub id: Option<String>,
    pub address_type: Option<String>,
    pub country_code: Option<String>,
    pub postal_code: Option<String>,
    pub settlement: Option<String>,
    pub mailbox: Option<String>,
    pub topographic_number: Option<String>,
    pub name_of_public_space: Option<String>,
    pub type_of_public_space: Option<String>,
    pub house_number: Option<String>,
    pub building: Option<String>,
    pub stairway: Option<String>,
    pub floor: Option<String>,
    pub door: Option<String>,
}

impl AddressUserInputError {
    #[expect(unused)]
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.address_type.is_none()
            && self.country_code.is_none()
            && self.postal_code.is_none()
            && self.settlement.is_none()
            && self.mailbox.is_none()
            && self.topographic_number.is_none()
            && self.name_of_public_space.is_none()
            && self.type_of_public_space.is_none()
            && self.house_number.is_none()
            && self.building.is_none()
            && self.stairway.is_none()
            && self.floor.is_none()
            && self.door.is_none()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[expect(unused)]
pub struct AddressUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub address_type: ValueObjectRequired<AddressType>,
    pub country_code: ValueObjectRequired<CountryCode>,
    pub postal_code: ValueObjectRequired<PostalCode>,
    pub settlement: ValueObjectRequired<Settlement>,
    pub mailbox: ValueObjectOptional<Mailbox>,
    pub topographic_number: ValueObjectOptional<TopographicNumber>,
    pub name_of_public_space: ValueObjectOptional<NameOfPublicSpace>,
    pub type_of_public_space: ValueObjectOptional<TypeOfPublicSpace>,
    pub house_number: ValueObjectOptional<HouseNumber>,
    pub building: ValueObjectOptional<Building>,
    pub stairway: ValueObjectOptional<Stairway>,
    pub floor: ValueObjectOptional<Floor>,
    pub door: ValueObjectOptional<Door>,
}

impl TryFrom<AddressUserInputHelper> for AddressUserInput {
    type Error = AddressUserInputError;
    #[expect(unused)]
    fn try_from(value: AddressUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = AddressUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let address_type = value
            .address_type
            .parse::<ValueObjectRequired<AddressType>>()
            .inspect_err(|e| {
                error.address_type = Some(e.to_string());
            });

        let country_code = value
            .country_code
            .parse::<ValueObjectRequired<CountryCode>>()
            .inspect_err(|e| {
                error.country_code = Some(e.to_string());
            });

        let postal_code = value
            .postal_code
            .parse::<ValueObjectRequired<PostalCode>>()
            .inspect_err(|e| {
                error.postal_code = Some(e.to_string());
            });

        let settlement = value
            .settlement
            .parse::<ValueObjectRequired<Settlement>>()
            .inspect_err(|e| {
                error.settlement = Some(e.to_string());
            });

        let mailbox = value
            .mailbox
            .parse::<ValueObjectOptional<Mailbox>>()
            .inspect_err(|e| {
                error.mailbox = Some(e.to_string());
            });

        let topographic_number = value
            .topographic_number
            .parse::<ValueObjectOptional<TopographicNumber>>()
            .inspect_err(|e| {
                error.topographic_number = Some(e.to_string());
            });

        if mailbox.is_ok() && topographic_number.is_ok() {
            let error_msg = "A postafiók és a helyrajzi szám nem adható meg egyszerre".to_string();
            error.mailbox = Some(error_msg.clone());
            error.topographic_number = Some(error_msg);
        }

        let name_of_public_space = value
            .name_of_public_space
            .parse::<ValueObjectOptional<NameOfPublicSpace>>()
            .inspect_err(|e| {
                error.name_of_public_space = Some(e.to_string());
            });

        let type_of_public_space = value
            .type_of_public_space
            .parse::<ValueObjectOptional<TypeOfPublicSpace>>()
            .inspect_err(|e| {
                error.type_of_public_space = Some(e.to_string());
            });

        let house_number = value
            .house_number
            .parse::<ValueObjectOptional<HouseNumber>>()
            .inspect_err(|e| {
                error.house_number = Some(e.to_string());
            });

        let building = value
            .building
            .parse::<ValueObjectOptional<Building>>()
            .inspect_err(|e| {
                error.building = Some(e.to_string());
            });

        let stairway = value
            .stairway
            .parse::<ValueObjectOptional<Stairway>>()
            .inspect_err(|e| {
                error.stairway = Some(e.to_string());
            });

        let floor = value
            .floor
            .parse::<ValueObjectOptional<Floor>>()
            .inspect_err(|e| {
                error.floor = Some(e.to_string());
            });

        let door = value
            .door
            .parse::<ValueObjectOptional<Door>>()
            .inspect_err(|e| {
                error.door = Some(e.to_string());
            });

        todo!()
    }
}
