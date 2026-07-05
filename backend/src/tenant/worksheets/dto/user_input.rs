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
use crate::tenant::worksheets::types::worksheet::{
    WorksheetDescription, WorksheetName, WorksheetStatus,
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use tracing::Level;

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl From<WorksheetUserInputError> for AppError {
    fn from(value: WorksheetUserInputError) -> Self {
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

impl From<ValueObjectError> for WorksheetUserInputError {
    fn from(_: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorksheetUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub name: ValueObjectRequired<WorksheetName>,
    pub description: ValueObjectOptional<WorksheetDescription>,
    pub customer_id: ValueObjectRequired<UuidVO>,
    pub project_id: ValueObjectOptional<UuidVO>,
    pub status: ValueObjectRequired<WorksheetStatus>,
}

impl TryFrom<WorksheetUserInputHelper> for WorksheetUserInput {
    type Error = WorksheetUserInputError;
    fn try_from(value: WorksheetUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = WorksheetUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let name = value
            .name
            .parse::<ValueObjectRequired<WorksheetName>>()
            .inspect_err(|e| {
                error.name = Some(e.to_string());
            });

        let status = value
            .status
            .parse::<ValueObjectRequired<WorksheetStatus>>()
            .inspect_err(|e| {
                error.status = Some(e.to_string());
            });

        let customer_id = value
            .customer_id
            .parse::<ValueObjectRequired<UuidVO>>()
            .inspect_err(|e| {
                error.customer_id = Some(e.to_string());
            });

        let project_id = value
            .project_id
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.project_id = Some(e.to_string());
            });

        let description = value
            .description
            .parse::<ValueObjectOptional<WorksheetDescription>>()
            .inspect_err(|e| {
                error.description = Some(e.to_string());
            });

        if error.is_empty() {
            Ok(WorksheetUserInput {
                id: id?,
                name: name?,
                description: description?,
                customer_id: customer_id?,
                project_id: project_id?,
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
        let customer_id = Uuid::new_v4();
        let user_input = WorksheetUserInput::try_from(WorksheetUserInputHelper {
            id: None,
            name: String::from("Worksheet 1"),
            description: String::from("description"),
            customer_id: customer_id.to_string(),
            project_id: String::from(""),
            status: String::from("active"),
        })
        .unwrap();

        assert!(!user_input.id.is_present());
        assert_eq!(user_input.name.as_str().unwrap(), "Worksheet 1");
        assert_eq!(user_input.description.as_str().unwrap(), "description");
        assert_eq!(user_input.customer_id.as_uuid().unwrap(), customer_id);
        assert!(!user_input.project_id.is_present());
        assert_eq!(user_input.status.as_str().unwrap(), "active");
    }
    #[test]
    fn invalid_user_input() {
        let invalid_description = "a".repeat(3001);
        let user_input = WorksheetUserInput::try_from(WorksheetUserInputHelper {
            id: None,
            name: String::from(""),
            description: invalid_description,
            customer_id: String::from("invalid"),
            project_id: String::from(""),
            status: String::from("invalid"),
        })
        .unwrap_err();

        assert_eq!(user_input.id, None);
        assert_eq!(user_input.name.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(
            user_input.description.unwrap(),
            WorksheetDescription::VALIDATION_ERROR
        );
        assert_eq!(user_input.customer_id.unwrap(), UuidVO::PARSE_ERROR);
        assert_eq!(user_input.project_id, None);
        assert_eq!(
            user_input.status.unwrap(),
            WorksheetStatus::VALIDATION_ERROR
        );
    }
}
