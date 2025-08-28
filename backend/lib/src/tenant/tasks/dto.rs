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
use crate::manager::common::types::value_object::ValueObject;
use crate::tenant::tasks::types::task::{TaskPriority, TaskStatus, TaskTitle};
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct CreateTaskHelper {
    // TODO: fields
}

pub struct CreateTaskError {
    // TODO: fields
}

impl CreateTaskError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for CreateTaskError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub id: Uuid,
    pub worksheet_id: Uuid,
    pub title: ValueObject<TaskTitle>,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub status: ValueObject<TaskStatus>,
    pub priority: ValueObject<TaskPriority>,
    pub due_date: Option<DateTime<Local>>,
}

impl TryFrom<CreateTaskHelper> for CreateTask {
    type Error = CreateTaskError;
    fn try_from(value: CreateTaskHelper) -> Result<Self, Self::Error> {
        todo!()
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
