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
        value_object::{ValueObjectError, ValueObjectOptional, ValueObjectRequired},
    },
    tenant::address::types::{
        AddressType, Building, CountryCode, Door, Floor, HouseNumber, Mailbox, NameOfPublicSpace,
        PostalCode, Settlement, Stairway, TopographicNumber, TypeOfPublicSpace,
    },
};

#[derive(Clone, Debug, Deserialize, Serialize, Builder)]
pub struct AddressUserInputHelper {
    pub id: Option<String>,
    #[serde(rename = "type")]
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

#[derive(Clone, Debug, Serialize, Default, Builder, PartialEq)]
pub struct AddressUserInputError {
    pub id: Option<String>,
    #[serde(rename = "type")]
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

impl From<ValueObjectError> for AddressUserInputError {
    fn from(_: ValueObjectError) -> Self {
        AddressUserInputError::default()
    }
}

#[derive(Debug, Clone, PartialEq, Builder)]
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

impl AddressUserInput {
    pub fn is_full_address(&self) -> bool {
        self.name_of_public_space.is_present()
            || self.type_of_public_space.is_present()
            || self.house_number.is_present()
            || self.building.is_present()
            || self.stairway.is_present()
            || self.floor.is_present()
            || self.door.is_present()
    }
}

impl TryFrom<AddressUserInputHelper> for AddressUserInput {
    type Error = AddressUserInputError;
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

        if error.is_empty() {
            let address_user_input = AddressUserInput {
                id: id?,
                address_type: address_type?,
                country_code: country_code?,
                postal_code: postal_code?,
                settlement: settlement?,
                mailbox: mailbox?,
                topographic_number: topographic_number?,
                name_of_public_space: name_of_public_space?,
                type_of_public_space: type_of_public_space?,
                house_number: house_number?,
                building: building?,
                stairway: stairway?,
                floor: floor?,
                door: door?,
            };

            if address_user_input.mailbox.is_present()
                && address_user_input.topographic_number.is_present()
            {
                let error_msg =
                    "A postafiók és a helyrajzi szám nem adható meg egyszerre".to_string();
                error.mailbox = Some(error_msg.clone());
                error.topographic_number = Some(error_msg);
                return Err(error);
            }

            if address_user_input.mailbox.is_present() && address_user_input.is_full_address() {
                error.mailbox =
                    Some("Postafiók nem adható meg, ha ki van töltve a teljes cím".to_string());
                return Err(error);
            }

            if address_user_input.topographic_number.is_present()
                && address_user_input.is_full_address()
            {
                error.topographic_number = Some(
                    "Helyrajzi szám nem adható meg, ha ki van töltve a teljes cím".to_string(),
                );
                return Err(error);
            }

            if address_user_input.is_full_address()
                && (!address_user_input.name_of_public_space.is_present()
                    || !address_user_input.type_of_public_space.is_present()
                    || !address_user_input.house_number.is_present())
            {
                let error_msg = "A mező kitöltése kötelező".to_string();
                if !address_user_input.name_of_public_space.is_present() {
                    error.name_of_public_space = Some(error_msg.clone());
                }
                if !address_user_input.type_of_public_space.is_present() {
                    error.type_of_public_space = Some(error_msg.clone());
                }
                if !address_user_input.house_number.is_present() {
                    error.house_number = Some(error_msg.clone())
                }
                return Err(error);
            }

            Ok(address_user_input)
        } else {
            Err(error)
        }
    }
}

#[cfg(test)]
pub mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    pub fn test_address_user_input_helper_builder() -> AddressUserInputHelperBuilder {
        let mut builder = AddressUserInputHelperBuilder::default();
        builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("".to_string())
            .topographic_number("".to_string())
            .name_of_public_space("Váci".to_string())
            .type_of_public_space("út".to_string())
            .house_number("1111".to_string())
            .building("A".to_string())
            .stairway("B".to_string())
            .floor("1".to_string())
            .door("2".to_string());

        builder
    }

    pub fn test_address_user_input_builder() -> AddressUserInputBuilder {
        let mut builder = AddressUserInputBuilder::default();
        builder
            .id("".parse().unwrap())
            .address_type("mailing".parse().unwrap())
            .country_code("HU".parse().unwrap())
            .postal_code("1011".parse().unwrap())
            .settlement("Budapest".parse().unwrap())
            .mailbox("".parse().unwrap())
            .topographic_number("".parse().unwrap())
            .name_of_public_space("Váci".parse().unwrap())
            .type_of_public_space("út".parse().unwrap())
            .house_number("1111".parse().unwrap())
            .building("A".parse().unwrap())
            .stairway("B".parse().unwrap())
            .floor("1".parse().unwrap())
            .door("2".parse().unwrap());

        builder
    }

    #[test]
    fn test_valid_full_address() {
        let mut address_user_input_helper_builder = AddressUserInputHelperBuilder::default();
        let address_user_input_helper = address_user_input_helper_builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("".to_string())
            .topographic_number("".to_string())
            .name_of_public_space("Váci".to_string())
            .type_of_public_space("út".to_string())
            .house_number("1111".to_string())
            .building("A".to_string())
            .stairway("B".to_string())
            .floor("1".to_string())
            .door("2".to_string())
            .build()
            .unwrap();

        let mut address_user_input_builder = AddressUserInputBuilder::default();
        let expected_address_user_input = address_user_input_builder
            .id("".parse().unwrap())
            .address_type("mailing".parse().unwrap())
            .country_code("HU".parse().unwrap())
            .postal_code("1011".parse().unwrap())
            .settlement("Budapest".parse().unwrap())
            .mailbox("".parse().unwrap())
            .topographic_number("".parse().unwrap())
            .name_of_public_space("Váci".parse().unwrap())
            .type_of_public_space("út".parse().unwrap())
            .house_number("1111".parse().unwrap())
            .building("A".parse().unwrap())
            .stairway("B".parse().unwrap())
            .floor("1".parse().unwrap())
            .door("2".parse().unwrap())
            .build()
            .unwrap();

        let address_user_input = AddressUserInput::try_from(address_user_input_helper);

        assert!(address_user_input.is_ok());
        assert_eq!(expected_address_user_input, address_user_input.unwrap());
    }

    #[test]
    fn test_valid_topographic_number_address() {
        let mut address_user_input_helper_builder = AddressUserInputHelperBuilder::default();
        let address_user_input_helper = address_user_input_helper_builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("".to_string())
            .topographic_number("111/A".to_string())
            .name_of_public_space("".to_string())
            .type_of_public_space("".to_string())
            .house_number("".to_string())
            .building("".to_string())
            .stairway("".to_string())
            .floor("".to_string())
            .door("".to_string())
            .build()
            .unwrap();

        let mut address_user_input_builder = AddressUserInputBuilder::default();
        let expected_address_user_input = address_user_input_builder
            .id("".parse().unwrap())
            .address_type("mailing".parse().unwrap())
            .country_code("HU".parse().unwrap())
            .postal_code("1011".parse().unwrap())
            .settlement("Budapest".parse().unwrap())
            .mailbox("".parse().unwrap())
            .topographic_number("111/A".parse().unwrap())
            .name_of_public_space("".parse().unwrap())
            .type_of_public_space("".parse().unwrap())
            .house_number("".parse().unwrap())
            .building("".parse().unwrap())
            .stairway("".parse().unwrap())
            .floor("".parse().unwrap())
            .door("".parse().unwrap())
            .build()
            .unwrap();

        let address_user_input = AddressUserInput::try_from(address_user_input_helper);

        assert!(address_user_input.is_ok());
        assert_eq!(expected_address_user_input, address_user_input.unwrap());
    }

    #[test]
    fn test_valid_mailbox_address() {
        let mut address_user_input_helper_builder = AddressUserInputHelperBuilder::default();
        let address_user_input_helper = address_user_input_helper_builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("111".to_string())
            .topographic_number("".to_string())
            .name_of_public_space("".to_string())
            .type_of_public_space("".to_string())
            .house_number("".to_string())
            .building("".to_string())
            .stairway("".to_string())
            .floor("".to_string())
            .door("".to_string())
            .build()
            .unwrap();

        let mut address_user_input_builder = AddressUserInputBuilder::default();
        let expected_address_user_input = address_user_input_builder
            .id("".parse().unwrap())
            .address_type("mailing".parse().unwrap())
            .country_code("HU".parse().unwrap())
            .postal_code("1011".parse().unwrap())
            .settlement("Budapest".parse().unwrap())
            .mailbox("111".parse().unwrap())
            .topographic_number("".parse().unwrap())
            .name_of_public_space("".parse().unwrap())
            .type_of_public_space("".parse().unwrap())
            .house_number("".parse().unwrap())
            .building("".parse().unwrap())
            .stairway("".parse().unwrap())
            .floor("".parse().unwrap())
            .door("".parse().unwrap())
            .build()
            .unwrap();

        let address_user_input = AddressUserInput::try_from(address_user_input_helper);

        assert!(address_user_input.is_ok());
        assert_eq!(expected_address_user_input, address_user_input.unwrap());
    }

    #[test]
    fn test_mailbox_topographic_number_cant_be_present_together() {
        let mut address_user_input_helper_builder = AddressUserInputHelperBuilder::default();
        let address_user_input_helper = address_user_input_helper_builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("111".to_string())
            .topographic_number("111/A".to_string())
            .name_of_public_space("".to_string())
            .type_of_public_space("".to_string())
            .house_number("".to_string())
            .building("".to_string())
            .stairway("".to_string())
            .floor("".to_string())
            .door("".to_string())
            .build()
            .unwrap();

        let error_msg = "A postafiók és a helyrajzi szám nem adható meg egyszerre".to_string();

        let expected_address_user_input_error = AddressUserInputError {
            id: None,
            address_type: None,
            country_code: None,
            postal_code: None,
            settlement: None,
            mailbox: Some(error_msg.clone()),
            topographic_number: Some(error_msg),
            name_of_public_space: None,
            type_of_public_space: None,
            house_number: None,
            building: None,
            stairway: None,
            floor: None,
            door: None,
        };

        let address_user_input = AddressUserInput::try_from(address_user_input_helper);

        assert!(address_user_input.is_err());
        assert_eq!(
            expected_address_user_input_error,
            address_user_input.unwrap_err()
        );
    }

    #[test]
    fn test_mailbox_and_full_address_cant_be_present_together() {
        let mut address_user_input_helper_builder = AddressUserInputHelperBuilder::default();
        let address_user_input_helper = address_user_input_helper_builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("111".to_string())
            .topographic_number("".to_string())
            .name_of_public_space("Váci".to_string())
            .type_of_public_space("út".to_string())
            .house_number("1111".to_string())
            .building("A".to_string())
            .stairway("B".to_string())
            .floor("1".to_string())
            .door("2".to_string())
            .build()
            .unwrap();

        let expected_address_user_input_error = AddressUserInputError {
            id: None,
            address_type: None,
            country_code: None,
            postal_code: None,
            settlement: None,
            mailbox: Some("Postafiók nem adható meg, ha ki van töltve a teljes cím".to_string()),
            topographic_number: None,
            name_of_public_space: None,
            type_of_public_space: None,
            house_number: None,
            building: None,
            stairway: None,
            floor: None,
            door: None,
        };

        let address_user_input = AddressUserInput::try_from(address_user_input_helper);

        assert!(address_user_input.is_err());
        assert_eq!(
            expected_address_user_input_error,
            address_user_input.unwrap_err()
        );
    }

    #[test]
    fn test_topographic_number_and_full_address_cant_be_present_together() {
        let mut address_user_input_helper_builder = AddressUserInputHelperBuilder::default();
        let address_user_input_helper = address_user_input_helper_builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("".to_string())
            .topographic_number("111/A".to_string())
            .name_of_public_space("Váci".to_string())
            .type_of_public_space("út".to_string())
            .house_number("1111".to_string())
            .building("A".to_string())
            .stairway("B".to_string())
            .floor("1".to_string())
            .door("2".to_string())
            .build()
            .unwrap();

        let expected_address_user_input_error = AddressUserInputError {
            id: None,
            address_type: None,
            country_code: None,
            postal_code: None,
            settlement: None,
            mailbox: None,
            topographic_number: Some(
                "Helyrajzi szám nem adható meg, ha ki van töltve a teljes cím".to_string(),
            ),
            name_of_public_space: None,
            type_of_public_space: None,
            house_number: None,
            building: None,
            stairway: None,
            floor: None,
            door: None,
        };

        let address_user_input = AddressUserInput::try_from(address_user_input_helper);

        assert!(address_user_input.is_err());
        assert_eq!(
            expected_address_user_input_error,
            address_user_input.unwrap_err()
        );
    }

    #[test]
    fn test_name_of_public_space_required_if_full_address() {
        let mut address_user_input_helper_builder = AddressUserInputHelperBuilder::default();
        let address_user_input_helper = address_user_input_helper_builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("".to_string())
            .topographic_number("".to_string())
            .name_of_public_space("".to_string())
            .type_of_public_space("út".to_string())
            .house_number("1111".to_string())
            .building("A".to_string())
            .stairway("B".to_string())
            .floor("1".to_string())
            .door("2".to_string())
            .build()
            .unwrap();

        let expected_address_user_input_error = AddressUserInputError {
            id: None,
            address_type: None,
            country_code: None,
            postal_code: None,
            settlement: None,
            mailbox: None,
            topographic_number: None,
            name_of_public_space: Some("A mező kitöltése kötelező".to_string()),
            type_of_public_space: None,
            house_number: None,
            building: None,
            stairway: None,
            floor: None,
            door: None,
        };

        let address_user_input = AddressUserInput::try_from(address_user_input_helper);

        assert!(address_user_input.is_err());
        assert_eq!(
            expected_address_user_input_error,
            address_user_input.unwrap_err()
        );
    }

    #[test]
    fn test_type_of_public_space_required_if_full_address() {
        let mut address_user_input_helper_builder = AddressUserInputHelperBuilder::default();
        let address_user_input_helper = address_user_input_helper_builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("".to_string())
            .topographic_number("".to_string())
            .name_of_public_space("Váci".to_string())
            .type_of_public_space("".to_string())
            .house_number("1111".to_string())
            .building("A".to_string())
            .stairway("B".to_string())
            .floor("1".to_string())
            .door("2".to_string())
            .build()
            .unwrap();

        let expected_address_user_input_error = AddressUserInputError {
            id: None,
            address_type: None,
            country_code: None,
            postal_code: None,
            settlement: None,
            mailbox: None,
            topographic_number: None,
            name_of_public_space: None,
            type_of_public_space: Some("A mező kitöltése kötelező".to_string()),
            house_number: None,
            building: None,
            stairway: None,
            floor: None,
            door: None,
        };

        let address_user_input = AddressUserInput::try_from(address_user_input_helper);

        assert!(address_user_input.is_err());
        assert_eq!(
            expected_address_user_input_error,
            address_user_input.unwrap_err()
        );
    }

    #[test]
    fn test_house_number_required_if_full_address() {
        let mut address_user_input_helper_builder = AddressUserInputHelperBuilder::default();
        let address_user_input_helper = address_user_input_helper_builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("".to_string())
            .topographic_number("".to_string())
            .name_of_public_space("Váci".to_string())
            .type_of_public_space("út".to_string())
            .house_number("".to_string())
            .building("A".to_string())
            .stairway("B".to_string())
            .floor("1".to_string())
            .door("2".to_string())
            .build()
            .unwrap();

        let expected_address_user_input_error = AddressUserInputError {
            id: None,
            address_type: None,
            country_code: None,
            postal_code: None,
            settlement: None,
            mailbox: None,
            topographic_number: None,
            name_of_public_space: None,
            type_of_public_space: None,
            house_number: Some("A mező kitöltése kötelező".to_string()),
            building: None,
            stairway: None,
            floor: None,
            door: None,
        };

        let address_user_input = AddressUserInput::try_from(address_user_input_helper);

        assert!(address_user_input.is_err());
        assert_eq!(
            expected_address_user_input_error,
            address_user_input.unwrap_err()
        );
    }

    #[test]
    fn test_all_required_if_full_address() {
        let mut address_user_input_helper_builder = AddressUserInputHelperBuilder::default();
        let address_user_input_helper = address_user_input_helper_builder
            .id(None)
            .address_type("mailing".to_string())
            .country_code("HU".to_string())
            .postal_code("1011".to_string())
            .settlement("Budapest".to_string())
            .mailbox("".to_string())
            .topographic_number("".to_string())
            .name_of_public_space("".to_string())
            .type_of_public_space("".to_string())
            .house_number("".to_string())
            .building("A".to_string())
            .stairway("B".to_string())
            .floor("1".to_string())
            .door("2".to_string())
            .build()
            .unwrap();

        let expected_address_user_input_error = AddressUserInputError {
            id: None,
            address_type: None,
            country_code: None,
            postal_code: None,
            settlement: None,
            mailbox: None,
            topographic_number: None,
            name_of_public_space: Some("A mező kitöltése kötelező".to_string()),
            type_of_public_space: Some("A mező kitöltése kötelező".to_string()),
            house_number: Some("A mező kitöltése kötelező".to_string()),
            building: None,
            stairway: None,
            floor: None,
            door: None,
        };

        let address_user_input = AddressUserInput::try_from(address_user_input_helper);

        assert!(address_user_input.is_err());
        assert_eq!(
            expected_address_user_input_error,
            address_user_input.unwrap_err()
        );
    }
}
