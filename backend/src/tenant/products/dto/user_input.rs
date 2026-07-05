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

use crate::common::error::v2::{AppError, AppErrorVisibility};
use crate::common::types::UuidVO;
use crate::common::value_object::{ValueObjectError, ValueObjectOptional, ValueObjectRequired};
use crate::tenant::products::types::product::{ProductDescription, ProductName, ProductStatus};
use crate::tenant::products::types::unit_of_measure::UnitsOfMeasure;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use tracing::Level;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductUserInputHelper {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub unit_of_measure_id: String,
    pub new_unit_of_measure: String,
    pub status: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ProductUserInputError {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub unit_of_measure_id: Option<String>,
    pub new_unit_of_measure: Option<String>,
    pub status: Option<String>,
}

impl ProductUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.name.is_none()
            && self.description.is_none()
            && self.unit_of_measure_id.is_none()
            && self.status.is_none()
    }
}

impl Display for ProductUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateInventoryError: {}", json),
            Err(e) => write!(f, "CreateInventoryError: {}", e),
        }
    }
}

impl From<ProductUserInputError> for AppError {
    fn from(value: ProductUserInputError) -> Self {
        Self::new(
            Level::DEBUG,
            StatusCode::UNPROCESSABLE_ENTITY,
            file!(),
            AppErrorVisibility::UserFacing,
            json!({
                "message": "Kérjük ellenőrizze a hibás mezőket!",
                "fields": value
            }),
        )
    }
}

impl From<ValueObjectError> for ProductUserInputError {
    fn from(_: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub name: ValueObjectRequired<ProductName>,
    pub description: ValueObjectOptional<ProductDescription>,
    pub unit_of_measure_id: Option<ValueObjectRequired<UuidVO>>,
    pub new_unit_of_measure: Option<ValueObjectRequired<UnitsOfMeasure>>,
    pub status: ValueObjectRequired<ProductStatus>,
}

impl TryFrom<ProductUserInputHelper> for ProductUserInput {
    type Error = ProductUserInputError;
    fn try_from(value: ProductUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = ProductUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let name = value
            .name
            .parse::<ValueObjectRequired<ProductName>>()
            .inspect_err(|e| {
                error.name = Some(e.to_string());
            });

        let status = value
            .status
            .parse::<ValueObjectRequired<ProductStatus>>()
            .inspect_err(|e| {
                error.status = Some(e.to_string());
            });

        let description = value
            .description
            .parse::<ValueObjectOptional<ProductDescription>>()
            .inspect_err(|e| {
                error.description = Some(e.to_string());
            });

        let unit_of_measure_id = if value.unit_of_measure_id.as_str() != "other" {
            value
                .unit_of_measure_id
                .parse::<ValueObjectRequired<UuidVO>>()
                .inspect_err(|e| {
                    error.unit_of_measure_id = Some(e.to_string());
                })
                .map(Some)
        } else {
            Ok(None)
        };

        let new_unit_of_measure = if let Ok(result) = &unit_of_measure_id
            && result.is_some()
        {
            Ok(None)
        } else {
            value
                .new_unit_of_measure
                .parse::<ValueObjectRequired<UnitsOfMeasure>>()
                .inspect_err(|e| error.new_unit_of_measure = Some(e.to_string()))
                .map(Some)
        };

        if error.is_empty() {
            Ok(ProductUserInput {
                id: id?,
                name: name?,
                description: description?,
                unit_of_measure_id: unit_of_measure_id?,
                new_unit_of_measure: new_unit_of_measure?,
                status: status?,
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
    fn valid_products_user_input() {
        let user_input = ProductUserInput::try_from(ProductUserInputHelper {
            id: None,
            name: String::from("John Doe"),
            description: String::from("description"),
            unit_of_measure_id: String::from("other"),
            new_unit_of_measure: String::from("cm"),
            status: String::from("active"),
        })
        .unwrap();

        assert_eq!(user_input.id.as_uuid(), None);
        assert_eq!(user_input.name.as_str().unwrap(), "John Doe");
        assert_eq!(user_input.description.as_str().unwrap(), "description");
        assert_eq!(user_input.unit_of_measure_id, None);
        assert_eq!(
            user_input.new_unit_of_measure.unwrap().as_str().unwrap(),
            "cm"
        );
        assert_eq!(user_input.status.as_str().unwrap(), "active");
    }

    #[test]
    fn invalid_products_user_input() {
        let invalid_description = "a".repeat(3001);
        let user_input = ProductUserInput::try_from(ProductUserInputHelper {
            id: None,
            name: String::from(""),
            description: invalid_description,
            unit_of_measure_id: String::from(""),
            new_unit_of_measure: String::from(""),
            status: String::from("activeee"),
        })
        .unwrap_err();

        assert_eq!(user_input.id, None);
        assert_eq!(user_input.name.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(
            user_input.description.unwrap(),
            ProductDescription::VALIDATION_ERROR
        );
        assert_eq!(
            user_input.unit_of_measure_id.unwrap(),
            ValueObjectError::REQUIRED
        );
        assert_eq!(
            user_input.new_unit_of_measure.unwrap(),
            ValueObjectError::REQUIRED
        );
        assert_eq!(user_input.status.unwrap(), ProductStatus::VALIDATION_ERROR);
    }
}
