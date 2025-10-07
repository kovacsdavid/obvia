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
use crate::tenant::projects::types::project::{
    ProjectDescription, ProjectEndDate, ProjectName, ProjectStartDate, ProjectStatus,
};
use crate::validate_optional_string;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ProjectUserInputHelper {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub status: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ProjectUserInputError {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl ProjectUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.name.is_none()
            && self.description.is_none()
            && self.status.is_none()
            && self.start_date.is_none()
            && self.end_date.is_none()
    }
}

impl Display for ProjectUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateProjectError: {}", json),
            Err(e) => write!(f, "CreateProjectError: {}", e),
        }
    }
}

impl FormErrorResponse for ProjectUserInputError {}

impl IntoResponse for ProjectUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUserInput {
    pub id: Option<Uuid>,
    pub name: ValueObject<ProjectName>,
    pub description: Option<ValueObject<ProjectDescription>>,
    pub status: ValueObject<ProjectStatus>,
    pub start_date: Option<ValueObject<ProjectStartDate>>,
    pub end_date: Option<ValueObject<ProjectEndDate>>,
}

impl TryFrom<ProjectUserInputHelper> for ProjectUserInput {
    type Error = ProjectUserInputError;
    fn try_from(value: ProjectUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = ProjectUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|e| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let name = ValueObject::new(ProjectName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });
        let status = ValueObject::new(ProjectStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });
        let description =
            validate_optional_string!(ProjectDescription(value.description), error.description);
        let start_date =
            validate_optional_string!(ProjectStartDate(value.start_date), error.start_date);
        let end_date = validate_optional_string!(ProjectEndDate(value.end_date), error.end_date);

        if error.is_empty() {
            Ok(ProjectUserInput {
                id,
                name: name.map_err(|_| ProjectUserInputError::default())?,
                description,
                status: status.map_err(|_| ProjectUserInputError::default())?,
                start_date,
                end_date,
            })
        } else {
            Err(error)
        }
    }
}
