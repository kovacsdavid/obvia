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
use crate::common::types::UuidVO;
use crate::common::value_object::{ValueObjectError, ValueObjectOptional, ValueObjectRequired};
use crate::tenant::address::types::country::CountryCode;
use crate::tenant::taxes::types::legal_text::LegalText;
use crate::tenant::taxes::types::reporting_code::ReportingCode;
use crate::tenant::taxes::types::{
    TaxCategory, TaxDescription, TaxLegalText, TaxRate, TaxReportingCode, TaxStatus,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct TaxUserInputHelper {
    pub id: Option<String>,
    pub rate: String,
    pub description: String,
    pub country_code: String,
    pub tax_category: String,
    pub is_rate_applicable: Option<bool>,
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
    pub country_code: Option<String>,
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
            && self.country_code.is_none()
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

impl From<ValueObjectError> for TaxUserInputError {
    fn from(_: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct TaxUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub rate: Option<ValueObjectRequired<TaxRate>>,
    pub description: ValueObjectRequired<TaxDescription>,
    pub country_code: ValueObjectRequired<CountryCode>,
    pub tax_category: ValueObjectRequired<TaxCategory>,
    pub is_rate_applicable: bool,
    pub legal_text: ValueObjectOptional<TaxLegalText>,
    pub reporting_code: ValueObjectOptional<TaxReportingCode>,
    pub is_default: bool,
    pub status: ValueObjectRequired<TaxStatus>,
}

impl TryFrom<TaxUserInputHelper> for TaxUserInput {
    type Error = TaxUserInputError;
    fn try_from(value: TaxUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = TaxUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let is_rate_applicable = if let Some(v) = value.is_rate_applicable {
            v
        } else {
            error.is_rate_applicable = Some("A mező kitöltése kötelező!".to_string());
            false
        };

        let rate = if let Some(is_rate_applicable) = value.is_rate_applicable
            && is_rate_applicable
        {
            value
                .rate
                .parse::<ValueObjectRequired<TaxRate>>()
                .inspect_err(|e| error.rate = Some(e.to_string()))
                .map(Some)
        } else {
            Ok(None)
        };

        let description = value
            .description
            .parse::<ValueObjectRequired<TaxDescription>>()
            .inspect_err(|e| {
                error.description = Some(e.to_string());
            });

        let country_code = value
            .country_code
            .parse::<ValueObjectRequired<CountryCode>>()
            .inspect_err(|e| error.country_code = Some(e.to_string()));

        let tax_category = value
            .tax_category
            .parse::<ValueObjectRequired<TaxCategory>>()
            .inspect_err(|e| {
                error.tax_category = Some(e.to_string());
            });

        let legal_text = value
            .legal_text
            .parse::<ValueObjectOptional<LegalText>>()
            .inspect_err(|e| {
                error.legal_text = Some(e.to_string());
            });

        let reporting_code = value
            .reporting_code
            .parse::<ValueObjectOptional<ReportingCode>>()
            .inspect_err(|e| {
                error.reporting_code = Some(e.to_string());
            });

        let status = value
            .status
            .parse::<ValueObjectRequired<TaxStatus>>()
            .inspect_err(|e| {
                error.status = Some(e.to_string());
            });

        if error.is_empty() {
            Ok(TaxUserInput {
                id: id?,
                rate: rate?,
                description: description?,
                country_code: country_code?,
                tax_category: tax_category?,
                is_rate_applicable,
                legal_text: legal_text?,
                reporting_code: reporting_code?,
                is_default: value.is_default,
                status: status?,
            })
        } else {
            Err(error)
        }
    }
}
