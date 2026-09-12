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

use crate::common::CommonBuilderError;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Builder)]
#[builder(build_fn(error = "CommonBuilderError"))]
pub struct Address {
    pub id: Uuid,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub address_type: String,
    pub country_code: String,
    pub postal_code: String,
    pub settlement: String,
    pub mailbox: Option<String>,
    pub topographic_number: Option<String>,
    pub name_of_public_space: Option<String>,
    pub type_of_public_space: Option<String>,
    pub house_number: Option<String>,
    pub building: Option<String>,
    pub stairway: Option<String>,
    pub floor: Option<String>,
    pub door: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let country_code = &self.country_code;
        let postal_code = &self.postal_code;
        let settlement = &self.settlement;
        let mailbox = &self.mailbox;
        let topographic_number = &self.topographic_number;
        let name_of_public_space = &self.name_of_public_space;
        let type_of_public_space = &self.type_of_public_space;
        let house_number = &self.house_number;
        let building = &self.building;
        let stairway = &self.stairway;
        let floor = &self.floor;
        let door = &self.door;

        let mut address = format!("{country_code}{postal_code} {settlement},");

        if let Some(mailbox) = mailbox {
            address += &format!(" PF. {mailbox}");
            return write!(f, "{}", address);
        }

        if let Some(topographic_number) = topographic_number {
            address += &format!(" HRSZ.: {topographic_number}");
            return write!(f, "{}", address);
        }

        if let Some(name_of_public_space) = name_of_public_space {
            address += &format!(" {name_of_public_space}");
        }

        if let Some(type_of_public_space) = type_of_public_space {
            address += &format!(" {type_of_public_space}");
        }

        if let Some(house_number) = house_number {
            address += &format!(" {house_number}.");
        }

        if let Some(building) = building {
            if stairway.is_none() && floor.is_none() && door.is_none() {
                address += &format!(" {building} épület");
            } else {
                address += &format!(" {building} épület,");
            }
        }

        if let Some(stairway) = stairway {
            if floor.is_none() && door.is_none() {
                address += &format!(" {stairway} lépcsőház");
            } else {
                address += &format!(" {stairway} lépcsőház,");
            }
        }

        if let Some(floor) = floor {
            if door.is_none() {
                address += &format!(" {floor}. emelet");
            } else {
                address += &format!(" {floor}. emelet,");
            }
        }

        if let Some(door) = door {
            address += &format!(" {door}. ajtó");
        }

        write!(f, "{}", address)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Builder)]
#[builder(build_fn(error = "CommonBuilderError"))]
pub struct AddressResolved {
    pub id: Uuid,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub address_type: String,
    pub country_code: String,
    pub country: String,
    pub postal_code: String,
    pub settlement: String,
    pub mailbox: Option<String>,
    pub topographic_number: Option<String>,
    pub name_of_public_space: Option<String>,
    pub type_of_public_space: Option<String>,
    pub house_number: Option<String>,
    pub building: Option<String>,
    pub stairway: Option<String>,
    pub floor: Option<String>,
    pub door: Option<String>,
    pub created_by_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Display for AddressResolved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let country_code = &self.country_code;
        let country = &self.country;
        let postal_code = &self.postal_code;
        let settlement = &self.settlement;
        let mailbox = &self.mailbox;
        let topographic_number = &self.topographic_number;
        let name_of_public_space = &self.name_of_public_space;
        let type_of_public_space = &self.type_of_public_space;
        let house_number = &self.house_number;
        let building = &self.building;
        let stairway = &self.stairway;
        let floor = &self.floor;
        let door = &self.door;

        let mut address = format!("{country} {country_code}{postal_code} {settlement},");

        if let Some(mailbox) = mailbox {
            address += &format!(" PF. {mailbox}");
            return write!(f, "{}", address);
        }

        if let Some(topographic_number) = topographic_number {
            address += &format!(" HRSZ.: {topographic_number}");
            return write!(f, "{}", address);
        }

        if let Some(name_of_public_space) = name_of_public_space {
            address += &format!(" {name_of_public_space}");
        }

        if let Some(type_of_public_space) = type_of_public_space {
            address += &format!(" {type_of_public_space}");
        }

        if let Some(house_number) = house_number {
            address += &format!(" {house_number}.");
        }

        if let Some(building) = building {
            if stairway.is_none() && floor.is_none() && door.is_none() {
                address += &format!(" {building} épület");
            } else {
                address += &format!(" {building} épület,");
            }
        }

        if let Some(stairway) = stairway {
            if floor.is_none() && door.is_none() {
                address += &format!(" {stairway} lépcsőház");
            } else {
                address += &format!(" {stairway} lépcsőház,");
            }
        }

        if let Some(floor) = floor {
            if door.is_none() {
                address += &format!(" {floor}. emelet");
            } else {
                address += &format!(" {floor}. emelet,");
            }
        }

        if let Some(door) = door {
            address += &format!(" {door}. ajtó");
        }

        write!(f, "{}", address)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::TEST_TIME;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_full_address_display() {
        let expected = "HU1132 Budapest, Váci út 1111. A épület, B lépcsőház, 1. emelet, 2. ajtó";
        let mut address_builder = AddressBuilder::default();
        let address = address_builder
            .id(Uuid::new_v4())
            .address_type("billing".to_string())
            .country_code("HU".to_string())
            .postal_code("1132".to_string())
            .settlement("Budapest".to_string())
            .mailbox(None)
            .topographic_number(None)
            .name_of_public_space(Some("Váci".to_string()))
            .type_of_public_space(Some("út".to_string()))
            .house_number(Some("1111".to_string()))
            .building(Some("A".to_string()))
            .stairway(Some("B".to_string()))
            .floor(Some("1".to_string()))
            .door(Some("2".to_string()))
            .created_by_id(Uuid::new_v4())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None)
            .build()
            .unwrap();

        assert_eq!(expected, address.to_string())
    }

    #[test]
    fn test_mailbox_address_display() {
        let expected = "HU1132 Budapest, PF. 123";
        let mut address_builder = AddressBuilder::default();
        let address = address_builder
            .id(Uuid::new_v4())
            .address_type("billing".to_string())
            .country_code("HU".to_string())
            .postal_code("1132".to_string())
            .settlement("Budapest".to_string())
            .mailbox(Some("123".to_string()))
            .topographic_number(None)
            .name_of_public_space(None)
            .type_of_public_space(None)
            .house_number(None)
            .building(None)
            .stairway(None)
            .floor(None)
            .door(None)
            .created_by_id(Uuid::new_v4())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None)
            .build()
            .unwrap();

        assert_eq!(expected, address.to_string())
    }

    #[test]
    fn test_topographic_number_address_display() {
        let expected = "HU1132 Budapest, HRSZ.: 123";
        let mut address_builder = AddressBuilder::default();
        let address = address_builder
            .id(Uuid::new_v4())
            .address_type("billing".to_string())
            .country_code("HU".to_string())
            .postal_code("1132".to_string())
            .settlement("Budapest".to_string())
            .mailbox(None)
            .topographic_number(Some("123".to_string()))
            .name_of_public_space(None)
            .type_of_public_space(None)
            .house_number(None)
            .building(None)
            .stairway(None)
            .floor(None)
            .door(None)
            .created_by_id(Uuid::new_v4())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None)
            .build()
            .unwrap();

        assert_eq!(expected, address.to_string())
    }

    #[test]
    fn test_full_address_resolved_display() {
        let expected =
            "Magyarország HU1132 Budapest, Váci út 1111. A épület, B lépcsőház, 1. emelet, 2. ajtó";
        let mut address_resolved_builder = AddressResolvedBuilder::default();
        let address_resolved = address_resolved_builder
            .id(Uuid::new_v4())
            .address_type("billing".to_string())
            .country_code("HU".to_string())
            .country("Magyarország".to_string())
            .postal_code("1132".to_string())
            .settlement("Budapest".to_string())
            .mailbox(None)
            .topographic_number(None)
            .name_of_public_space(Some("Váci".to_string()))
            .type_of_public_space(Some("út".to_string()))
            .house_number(Some("1111".to_string()))
            .building(Some("A".to_string()))
            .stairway(Some("B".to_string()))
            .floor(Some("1".to_string()))
            .door(Some("2".to_string()))
            .created_by_id(Uuid::new_v4())
            .created_by("Test User".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None)
            .build()
            .unwrap();

        assert_eq!(expected, address_resolved.to_string())
    }

    #[test]
    fn test_mailbox_address_resolved_display() {
        let expected = "Magyarország HU1132 Budapest, PF. 123";
        let mut address_resolved_builder = AddressResolvedBuilder::default();
        let address_resolved = address_resolved_builder
            .id(Uuid::new_v4())
            .address_type("billing".to_string())
            .country_code("HU".to_string())
            .country("Magyarország".to_string())
            .postal_code("1132".to_string())
            .settlement("Budapest".to_string())
            .mailbox(Some("123".to_string()))
            .topographic_number(None)
            .name_of_public_space(None)
            .type_of_public_space(None)
            .house_number(None)
            .building(None)
            .stairway(None)
            .floor(None)
            .door(None)
            .created_by_id(Uuid::new_v4())
            .created_by("Test User".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None)
            .build()
            .unwrap();

        assert_eq!(expected, address_resolved.to_string())
    }

    #[test]
    fn test_topographic_number_address_resolved_display() {
        let expected = "Magyarország HU1132 Budapest, HRSZ.: 123";
        let mut address_resolved_builder = AddressResolvedBuilder::default();
        let address_resolved = address_resolved_builder
            .id(Uuid::new_v4())
            .address_type("billing".to_string())
            .country_code("HU".to_string())
            .country("Magyarország".to_string())
            .postal_code("1132".to_string())
            .settlement("Budapest".to_string())
            .mailbox(None)
            .topographic_number(Some("123".to_string()))
            .name_of_public_space(None)
            .type_of_public_space(None)
            .house_number(None)
            .building(None)
            .stairway(None)
            .floor(None)
            .door(None)
            .created_by_id(Uuid::new_v4())
            .created_by("Test User".to_string())
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None)
            .build()
            .unwrap();

        assert_eq!(expected, address_resolved.to_string())
    }
}
