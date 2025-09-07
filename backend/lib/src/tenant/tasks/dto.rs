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
use crate::manager::common::types::value_object::ValueObject;
use crate::tenant::tasks::types::task::{TaskPriority, TaskStatus, TaskTitle};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateTaskHelper {
    pub worksheet_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub due_date: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTaskError {
    pub worksheet_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
}

impl CreateTaskError {
    pub fn is_empty(&self) -> bool {
        self.worksheet_id.is_none()
            && self.title.is_none()
            && self.description.is_none()
            && self.status.is_none()
            && self.priority.is_none()
            && self.due_date.is_none()
    }
}

impl IntoResponse for CreateTaskError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                reference: String::from("TASKS/DTO/CREATE"),
                global: String::from("Kérjük, ellenőrizze a hibás mezőket"),
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub worksheet_id: Uuid,
    pub title: ValueObject<TaskTitle>,
    pub description: Option<String>,
    pub status: ValueObject<TaskStatus>,
    pub priority: ValueObject<TaskPriority>,
    pub due_date: Option<DateTime<Local>>,
}

impl TryFrom<CreateTaskHelper> for CreateTask {
    type Error = CreateTaskError;
    fn try_from(value: CreateTaskHelper) -> Result<Self, Self::Error> {
        let mut error = CreateTaskError {
            worksheet_id: None,
            title: None,
            description: None,
            status: None,
            priority: None,
            due_date: None,
        };

        let title = ValueObject::new(TaskTitle(value.title));
        let status = ValueObject::new(TaskStatus(value.status));
        let priority = ValueObject::new(TaskPriority(value.priority));

        if let Err(e) = &title {
            error.title = Some(e.to_string());
        }

        if let Err(e) = &status {
            error.status = Some(e.to_string());
        }

        if let Err(e) = &priority {
            error.priority = Some(e.to_string());
        }

        if error.is_empty() {
            Ok(CreateTask {
                worksheet_id: value.worksheet_id,
                title: title.unwrap(),
                description: Some(value.description),
                status: status.unwrap(),
                priority: priority.unwrap(),
                due_date: Some(Local::now()), // TODO: date handling
            })
        } else {
            Err(error)
        }
    }
}

pub struct UpdateTaskHelper {
    // TODO: fields
}

pub struct UpdateTaskError {
    // TODO: fields
}

impl UpdateTaskError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateTaskError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTask {
    pub worksheet_id: Uuid,
    pub title: ValueObject<TaskTitle>,
    pub description: Option<String>,
    pub status: ValueObject<TaskStatus>,
    pub priority: ValueObject<TaskPriority>,
    pub due_date: Option<DateTime<Local>>,
}

impl TryFrom<UpdateTaskHelper> for UpdateTask {
    type Error = UpdateTaskError;
    fn try_from(value: UpdateTaskHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskAssignment {
    pub user_id: Uuid,
    pub task_id: Uuid,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskAssignment {
    pub user_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
}
