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
use crate::tenant::currencies::types::CurrencyCode;
use crate::tenant::products::types::unit_of_measure::UnitsOfMeasure;
use crate::tenant::services::types::service::default_price::DefaultPrice;
use crate::tenant::services::types::service::{
    ServiceDefaultPrice, ServiceDescription, ServiceName, ServiceStatus,
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use tracing::Level;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceUserInputHelper {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub default_price: String,
    pub default_tax_id: String,
    pub currency_code: String,
    pub status: String,
    pub unit_of_measure_id: String,
    pub new_unit_of_measure: String,
}

#[derive(Debug, Serialize, Default, PartialEq)]
pub struct ServiceUserInputError {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub default_price: Option<String>,
    pub default_tax_id: Option<String>,
    pub currency_code: Option<String>,
    pub status: Option<String>,
    pub unit_of_measure_id: Option<String>,
    pub new_unit_of_measure: Option<String>,
}

impl ServiceUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.name.is_none()
            && self.description.is_none()
            && self.default_price.is_none()
            && self.default_tax_id.is_none()
            && self.currency_code.is_none()
            && self.status.is_none()
            && self.unit_of_measure_id.is_none()
            && self.new_unit_of_measure.is_none()
    }
}

impl Display for ServiceUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "ServiceUserInputError: {}", json),
            Err(e) => write!(f, "ServiceUserInputError: {}", e),
        }
    }
}

impl From<ServiceUserInputError> for AppError {
    fn from(value: ServiceUserInputError) -> Self {
        Self::new(
            Level::DEBUG,
            StatusCode::UNPROCESSABLE_ENTITY,
            AppErrorVisibility::UserFacing,
            json!({
                "message": "Kérjük ellenőrizze a hibás mezőket!",
                "fields": value
            }),
        )
    }
}

impl From<ValueObjectError> for ServiceUserInputError {
    fn from(_: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub name: ValueObjectRequired<ServiceName>,
    pub description: ValueObjectOptional<ServiceDescription>,
    pub default_price: ValueObjectOptional<DefaultPrice>,
    pub default_tax_id: ValueObjectOptional<UuidVO>,
    pub currency_code: ValueObjectOptional<CurrencyCode>,
    pub status: ValueObjectRequired<ServiceStatus>,
    pub unit_of_measure_id: Option<ValueObjectRequired<UuidVO>>,
    pub new_unit_of_measure: Option<ValueObjectRequired<UnitsOfMeasure>>,
}

impl TryFrom<ServiceUserInputHelper> for ServiceUserInput {
    type Error = ServiceUserInputError;
    fn try_from(value: ServiceUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = ServiceUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let name = value
            .name
            .parse::<ValueObjectRequired<ServiceName>>()
            .inspect_err(|e| {
                error.name = Some(e.to_string());
            });

        let description = value
            .description
            .parse::<ValueObjectOptional<ServiceDescription>>()
            .inspect_err(|e| {
                error.description = Some(e.to_string());
            });

        let default_price = value
            .default_price
            .parse::<ValueObjectOptional<ServiceDefaultPrice>>()
            .inspect_err(|e| {
                error.default_price = Some(e.to_string());
            });

        let default_tax_id = value
            .default_tax_id
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.default_tax_id = Some(e.to_string());
            });

        let currency_code = value
            .currency_code
            .parse::<ValueObjectOptional<CurrencyCode>>()
            .inspect_err(|e| {
                error.currency_code = Some(e.to_string());
            });

        let status = value
            .status
            .parse::<ValueObjectRequired<ServiceStatus>>()
            .inspect_err(|e| {
                error.status = Some(e.to_string());
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

        let new_unit_of_measure = if let Ok(unit_of_measure_id) = &unit_of_measure_id
            && unit_of_measure_id.is_none()
        {
            value
                .new_unit_of_measure
                .parse::<ValueObjectRequired<UnitsOfMeasure>>()
                .inspect_err(|e| error.new_unit_of_measure = Some(e.to_string()))
                .map(Some)
        } else {
            Ok(None)
        };

        if error.is_empty() {
            Ok(ServiceUserInput {
                id: id?,
                name: name?,
                description: description?,
                default_price: default_price?,
                default_tax_id: default_tax_id?,
                currency_code: currency_code?,
                status: status?,
                unit_of_measure_id: unit_of_measure_id?,
                new_unit_of_measure: new_unit_of_measure?,
            })
        } else {
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn valid_user_input() {
        let default_tax_id = Uuid::now_v7();
        let user_input = ServiceUserInput::try_from(ServiceUserInputHelper {
            id: None,
            name: String::from("Service Name"),
            description: String::from("description"),
            default_price: String::from("1000"),
            default_tax_id: default_tax_id.to_string(),
            currency_code: String::from("HUF"),
            status: String::from("active"),
            unit_of_measure_id: String::from("other"),
            new_unit_of_measure: String::from("óra"),
        });

        let expected_user_input = ServiceUserInput {
            id: "".parse().unwrap(),
            name: "Service Name".parse().unwrap(),
            description: "description".parse().unwrap(),
            default_price: "1000".parse().unwrap(),
            default_tax_id: default_tax_id.to_string().parse().unwrap(),
            currency_code: "HUF".parse().unwrap(),
            status: "active".parse().unwrap(),
            unit_of_measure_id: None,
            new_unit_of_measure: Some("óra".parse().unwrap()),
        };

        assert!(user_input.is_ok());
        assert_eq!(user_input.unwrap(), expected_user_input);
    }

    #[test]
    fn invalid_user_input() {
        let invalid_description = "a".repeat(3001);
        let user_input = ServiceUserInput::try_from(ServiceUserInputHelper {
            id: None,
            name: String::from(""),
            description: invalid_description,
            default_price: String::from("asd"),
            default_tax_id: String::from("invalid"),
            currency_code: String::from("HUFF"),
            status: String::from("invalid"),
            unit_of_measure_id: String::from("other"),
            new_unit_of_measure: String::from(""),
        });

        let expected_user_input_error = ServiceUserInputError {
            id: None,
            name: Some(ValueObjectError::REQUIRED.to_string()),
            description: Some(ServiceDescription::VALIDATION_ERROR.to_string()),
            default_price: Some(DefaultPrice::PARSE_ERROR.to_string()),
            default_tax_id: Some(UuidVO::PARSE_ERROR.to_string()),
            currency_code: Some(CurrencyCode::VALIDATION_ERROR.to_string()),
            status: Some(ServiceStatus::VALIDATION_ERROR.to_string()),
            unit_of_measure_id: None,
            new_unit_of_measure: Some(ValueObjectError::REQUIRED.to_string()),
        };

        assert!(user_input.is_err());
        assert_eq!(user_input.unwrap_err(), expected_user_input_error);
    }
}
