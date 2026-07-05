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
}

#[derive(Debug, Serialize, Default)]
pub struct ServiceUserInputError {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub default_price: Option<String>,
    pub default_tax_id: Option<String>,
    pub currency_code: Option<String>,
    pub status: Option<String>,
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
            file!(),
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

        if error.is_empty() {
            Ok(ServiceUserInput {
                id: id?,
                name: name?,
                description: description?,
                default_price: default_price?,
                default_tax_id: default_tax_id?,
                currency_code: currency_code?,
                status: status?,
            })
        } else {
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn valid_user_input() {
        let default_tax_id = Uuid::new_v4();
        let user_input = ServiceUserInput::try_from(ServiceUserInputHelper {
            id: None,
            name: String::from("Service Name"),
            description: String::from("description"),
            default_price: String::from("1000"),
            default_tax_id: default_tax_id.to_string(),
            currency_code: String::from("HUF"),
            status: String::from("active"),
        })
        .unwrap();

        assert_eq!(user_input.id.as_uuid(), None);
        assert_eq!(user_input.name.as_str().unwrap(), "Service Name");
        assert_eq!(user_input.description.as_str().unwrap(), "description");
        assert_eq!(user_input.default_price.as_f64().unwrap(), 1000_f64);
        assert_eq!(user_input.default_tax_id.as_uuid().unwrap(), default_tax_id);
        assert_eq!(user_input.currency_code.as_str().unwrap(), "HUF");
        assert_eq!(user_input.status.as_str().unwrap(), "active");
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
        })
        .unwrap_err();

        assert_eq!(user_input.id, None);
        assert_eq!(user_input.name.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(
            user_input.description.unwrap(),
            ServiceDescription::VALIDATION_ERROR
        );
        assert_eq!(user_input.default_price.unwrap(), DefaultPrice::PARSE_ERROR);
        assert_eq!(user_input.default_tax_id.unwrap(), UuidVO::PARSE_ERROR);
        assert_eq!(
            user_input.currency_code.unwrap(),
            CurrencyCode::VALIDATION_ERROR
        );
        assert_eq!(user_input.status.unwrap(), ServiceStatus::VALIDATION_ERROR);
    }
}
