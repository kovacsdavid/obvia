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
use crate::common::types::{ValueObject, ValueObjectable};
use crate::tenant::worksheets::types::worksheet::{
    WorksheetDescription, WorksheetName, WorksheetStatus,
};
use crate::validate_optional_string;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct WorksheetUserInputHelper {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub customer_id: Uuid,
    pub project_id: Uuid,
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
    pub id: Option<Uuid>,
    pub name: ValueObject<WorksheetName>,
    pub description: Option<ValueObject<WorksheetDescription>>,
    pub customer_id: Uuid,
    pub project_id: Uuid,
    pub status: ValueObject<WorksheetStatus>,
}

impl TryFrom<WorksheetUserInputHelper> for WorksheetUserInput {
    type Error = WorksheetUserInputError;
    fn try_from(value: WorksheetUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = WorksheetUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|e| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let name = ValueObject::new(WorksheetName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });
        let status = ValueObject::new(WorksheetStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });
        let description =
            validate_optional_string!(WorksheetDescription(value.description), error.description);

        if error.is_empty() {
            Ok(WorksheetUserInput {
                id,
                name: name.map_err(|_| WorksheetUserInputError::default())?,
                description,
                customer_id: value.customer_id,
                project_id: value.project_id,
                status: status.map_err(|_| WorksheetUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
