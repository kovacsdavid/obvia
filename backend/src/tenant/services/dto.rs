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
use crate::common::types::value_object::{ValueObject, ValueObjectable};
use crate::tenant::currencies::types::CurrencyCode;
use crate::tenant::services::types::service::default_price::DefaultPrice;
use crate::tenant::services::types::service::{
    ServiceDefaultPrice, ServiceDescription, ServiceName, ServiceStatus,
};
use crate::validate_optional_string;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
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

impl FormErrorResponse for ServiceUserInputError {}

impl IntoResponse for ServiceUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUserInput {
    pub id: Option<Uuid>,
    pub name: ValueObject<ServiceName>,
    pub description: Option<ValueObject<ServiceDescription>>,
    pub default_price: Option<ValueObject<DefaultPrice>>,
    pub default_tax_id: Option<Uuid>,
    pub currency_code: Option<ValueObject<CurrencyCode>>,
    pub status: ValueObject<ServiceStatus>,
}

impl TryFrom<ServiceUserInputHelper> for ServiceUserInput {
    type Error = ServiceUserInputError;
    fn try_from(value: ServiceUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = ServiceUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => {
                if id.trim().is_empty() {
                    None
                } else {
                    Uuid::parse_str(&id)
                        .inspect_err(|_| {
                            error.id = Some("Hibás azonosító".to_string());
                        })
                        .ok()
                }
            }
        };

        let name = ValueObject::new(ServiceName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });

        let description =
            validate_optional_string!(ServiceDescription(value.description), error.description);
        let default_price =
            validate_optional_string!(ServiceDefaultPrice(value.default_price), error.description);

        let default_tax_id = if value.default_tax_id.trim().is_empty() {
            None
        } else {
            match Uuid::parse_str(&value.default_tax_id) {
                Ok(uuid) => Some(uuid),
                Err(_) => {
                    error.default_tax_id = Some("Érvénytelen adó azonosító".to_string());
                    None
                }
            }
        };

        let currency_code =
            validate_optional_string!(CurrencyCode(value.currency_code), error.currency_code);

        let status = ValueObject::new(ServiceStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        if error.is_empty() {
            Ok(ServiceUserInput {
                id,
                name: name.map_err(|_| ServiceUserInputError::default())?,
                description,
                default_price,
                default_tax_id,
                currency_code,
                status: status.map_err(|_| ServiceUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
