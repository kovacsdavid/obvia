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
use crate::manager::common::dto::{ErrorBody, ErrorResponse};
use crate::manager::common::types::value_object::{ValueObject, ValueObjectable};
use crate::tenant::projects::types::project::{
    ProjectDescription, ProjectEndDate, ProjectName, ProjectStartDate, ProjectStatus,
};
use crate::validate_optional_string;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateProjectHelper {
    pub name: String,
    pub description: String,
    pub status: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateProjectError {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl CreateProjectError {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.status.is_none()
            && self.start_date.is_none()
            && self.end_date.is_none()
    }
}

impl IntoResponse for CreateProjectError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                reference: String::from("PROJECTS/DTO/CREATE"),
                global: String::from("Kérjük, ellenőrizze a hibás mezőket"),
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProject {
    pub name: ValueObject<ProjectName>,
    pub description: Option<ValueObject<ProjectDescription>>,
    pub status: ValueObject<ProjectStatus>,
    pub start_date: Option<ValueObject<ProjectStartDate>>,
    pub end_date: Option<ValueObject<ProjectEndDate>>,
}

impl TryFrom<CreateProjectHelper> for CreateProject {
    type Error = CreateProjectError;
    fn try_from(value: CreateProjectHelper) -> Result<Self, Self::Error> {
        let mut error = CreateProjectError::default();

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
            Ok(CreateProject {
                name: name.map_err(|_| CreateProjectError::default())?,
                description,
                status: status.map_err(|_| CreateProjectError::default())?,
                start_date,
                end_date,
            })
        } else {
            Err(error)
        }
    }
}

pub struct UpdateProjectHelper {
    // TODO: fields
}

pub struct UpdateProjectError {
    // TODO: fields
}

impl UpdateProjectError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateProjectError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProject {
    pub name: ValueObject<ProjectName>,
    pub description: Option<String>,
    pub status: ValueObject<ProjectStatus>,
    pub start_date: Option<DateTime<Local>>,
    pub end_date: Option<DateTime<Local>>,
}

impl TryFrom<UpdateProjectHelper> for UpdateProject {
    type Error = UpdateProjectError;
    fn try_from(value: UpdateProjectHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectAssignment {
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectAssignment {
    pub user_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
}
