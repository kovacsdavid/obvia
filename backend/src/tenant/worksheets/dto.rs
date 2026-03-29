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
use crate::common::types::{UuidVO, ValueObject};
use crate::tenant::worksheets::types::worksheet::{
    WorksheetDescription, WorksheetName, WorksheetStatus,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct WorksheetUserInputHelper {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub customer_id: String,
    pub project_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Default)]
pub struct WorksheetUserInputError {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub customer_id: Option<String>,
    pub project_id: Option<String>,
    pub status: Option<String>,
}

impl WorksheetUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.name.is_none()
            && self.description.is_none()
            && self.customer_id.is_none()
            && self.project_id.is_none()
            && self.status.is_none()
    }
}

impl Display for WorksheetUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateWorksheetError: {}", json),
            Err(e) => write!(f, "CreateWorksheetError: {}", e),
        }
    }
}

impl FormErrorResponse for WorksheetUserInputError {}

impl IntoResponse for WorksheetUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorksheetUserInput {
    pub id: Option<ValueObject<UuidVO>>,
    pub name: ValueObject<WorksheetName>,
    pub description: Option<ValueObject<WorksheetDescription>>,
    pub customer_id: ValueObject<UuidVO>,
    pub project_id: Option<ValueObject<UuidVO>>,
    pub status: ValueObject<WorksheetStatus>,
}

impl TryFrom<WorksheetUserInputHelper> for WorksheetUserInput {
    type Error = WorksheetUserInputError;
    fn try_from(value: WorksheetUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = WorksheetUserInputError::default();

        let id = if let Some(id) = value.id {
            ValueObject::new_optional(UuidVO(id)).inspect_err(|e| {
                error.id = Some(e.to_string());
            })
        } else {
            Ok(None)
        };

        let name = ValueObject::new_required(WorksheetName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });

        let status = ValueObject::new_required(WorksheetStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        let customer_id = ValueObject::new_required(UuidVO(value.customer_id)).inspect_err(|_| {
            error.customer_id = Some("A mező kitöltése kötelező!".to_string());
        });

        let project_id = ValueObject::new_optional(UuidVO(value.project_id)).inspect_err(|_| {
            error.project_id = Some("A mező kitöltése kötelező!".to_string());
        });

        let description = ValueObject::new_optional(WorksheetDescription(value.description))
            .inspect_err(|e| {
                error.description = Some(e.to_string());
            });

        if error.is_empty() {
            Ok(WorksheetUserInput {
                id: id.map_err(|_| WorksheetUserInputError::default())?,
                name: name.map_err(|_| WorksheetUserInputError::default())?,
                description: description.map_err(|_| WorksheetUserInputError::default())?,
                customer_id: customer_id.map_err(|_| WorksheetUserInputError::default())?,
                project_id: project_id.map_err(|_| WorksheetUserInputError::default())?,
                status: status.map_err(|_| WorksheetUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
