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

use crate::common::dto::{ErrorBody, ErrorResponse};
use crate::common::types::value_object::{ValueObject, ValueObjectable};
use crate::tenant::worksheets::types::worksheet::{
    WorksheetDescription, WorksheetName, WorksheetStatus,
};
use crate::validate_optional_string;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateWorksheetHelper {
    pub name: String,
    pub description: String,
    pub project_id: Uuid,
    pub status: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateWorksheetError {
    pub name: Option<String>,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub status: Option<String>,
}

impl CreateWorksheetError {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.project_id.is_none()
            && self.status.is_none()
    }
}

impl IntoResponse for CreateWorksheetError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                global: String::from("Kérjük, ellenőrizze a hibás mezőket"),
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorksheet {
    pub name: ValueObject<WorksheetName>,
    pub description: Option<ValueObject<WorksheetDescription>>,
    pub project_id: Uuid,
    pub status: ValueObject<WorksheetStatus>,
}

impl TryFrom<CreateWorksheetHelper> for CreateWorksheet {
    type Error = CreateWorksheetError;
    fn try_from(value: CreateWorksheetHelper) -> Result<Self, Self::Error> {
        let mut error = CreateWorksheetError::default();

        let name = ValueObject::new(WorksheetName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });
        let status = ValueObject::new(WorksheetStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });
        let description =
            validate_optional_string!(WorksheetDescription(value.description), error.description);

        if error.is_empty() {
            Ok(CreateWorksheet {
                name: name.map_err(|_| CreateWorksheetError::default())?,
                description,
                project_id: value.project_id,
                status: status.map_err(|_| CreateWorksheetError::default())?,
            })
        } else {
            Err(error)
        }
    }
}

pub struct UpdateWorksheetHelper {
    // TODO: fields
}

pub struct UpdateWorksheetError {
    // TODO: fields
}

impl UpdateWorksheetError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateWorksheetError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorksheet {
    pub name: ValueObject<WorksheetName>,
    pub description: Option<String>,
    pub project_id: Option<Uuid>,
    pub status: Option<ValueObject<WorksheetStatus>>,
}

impl TryFrom<UpdateWorksheetHelper> for UpdateWorksheet {
    type Error = UpdateWorksheetError;
    fn try_from(value: UpdateWorksheetHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}
