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
use crate::tenant::taxes::types::legal_text::LegalText;
use crate::tenant::taxes::types::reporting_code::ReportingCode;
use crate::tenant::taxes::types::{
    TaxCategory, TaxDescription, TaxLegalText, TaxRate, TaxReportingCode, TaxStatus,
};
use crate::validate_optional_string;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct TaxUserInputHelper {
    pub id: Option<String>,
    pub rate: String,
    pub description: String,
    pub country_id: String,
    pub tax_category: String,
    pub is_rate_applicable: bool,
    pub legal_text: String,
    pub reporting_code: String,
    pub is_default: bool,
    pub status: String,
}

#[derive(Debug, Serialize, Default)]
pub struct TaxUserInputError {
    pub id: Option<String>,
    pub rate: Option<String>,
    pub description: Option<String>,
    pub country_id: Option<String>,
    pub tax_category: Option<String>,
    pub is_rate_applicable: Option<String>,
    pub legal_text: Option<String>,
    pub reporting_code: Option<String>,
    pub is_default: Option<String>,
    pub status: Option<String>,
}

impl TaxUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.rate.is_none()
            && self.description.is_none()
            && self.country_id.is_none()
            && self.tax_category.is_none()
            && self.is_rate_applicable.is_none()
            && self.legal_text.is_none()
            && self.reporting_code.is_none()
            && self.is_default.is_none()
            && self.status.is_none()
    }
}

impl Display for TaxUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateTaxError: {}", json),
            Err(e) => write!(f, "CreateTaxError: {}", e),
        }
    }
}

impl FormErrorResponse for TaxUserInputError {}

impl IntoResponse for TaxUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxUserInput {
    pub id: Option<Uuid>,
    pub rate: Option<ValueObject<TaxRate>>,
    pub description: ValueObject<TaxDescription>,
    pub country_id: Uuid,
    pub tax_category: ValueObject<TaxCategory>,
    pub is_rate_applicable: bool,
    pub legal_text: Option<ValueObject<TaxLegalText>>,
    pub reporting_code: Option<ValueObject<TaxReportingCode>>,
    pub is_default: bool,
    pub status: ValueObject<TaxStatus>,
}

impl TryFrom<TaxUserInputHelper> for TaxUserInput {
    type Error = TaxUserInputError;
    fn try_from(value: TaxUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = TaxUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|e| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let rate = ValueObject::new(TaxRate(value.rate))
            .inspect_err(|e| {
                if value.is_rate_applicable {
                    error.rate = Some(e.to_string())
                }
            })
            .ok();

        let description = ValueObject::new(TaxDescription(value.description)).inspect_err(|e| {
            error.description = Some(e.to_string());
        });

        let country_id = Uuid::parse_str(value.country_id.as_str())
            .inspect_err(|_| error.country_id = Some("Hibás ország azonosító!".to_string()));

        let tax_category = ValueObject::new(TaxCategory(value.tax_category)).inspect_err(|e| {
            error.tax_category = Some(e.to_string());
        });

        let legal_text = validate_optional_string!(LegalText(value.legal_text), error.legal_text);
        let reporting_code =
            validate_optional_string!(ReportingCode(value.reporting_code), error.reporting_code);

        let status = ValueObject::new(TaxStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        if error.is_empty() {
            Ok(TaxUserInput {
                id,
                rate,
                description: description.map_err(|_| TaxUserInputError::default())?,
                country_id: country_id.map_err(|_| TaxUserInputError::default())?,
                tax_category: tax_category.map_err(|_| TaxUserInputError::default())?,
                is_rate_applicable: value.is_rate_applicable,
                legal_text,
                reporting_code,
                is_default: value.is_default,
                status: status.map_err(|_| TaxUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
