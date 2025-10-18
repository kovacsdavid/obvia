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
use crate::common::types::currency_code::CurrencyCode;
use crate::common::types::value_object::{ValueObject, ValueObjectable};
use crate::tenant::tasks::types::task::{TaskDueDate, TaskPrice, TaskPriority, TaskStatus};
use crate::validate_optional_string;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct TaskUserInputHelper {
    pub id: Option<String>,
    pub worksheet_id: Uuid,
    pub service_id: Uuid,
    pub currency_code: String,
    pub price: String,
    pub tax_id: Uuid,
    pub status: String,
    pub priority: String,
    pub due_date: String,
}

#[derive(Debug, Serialize, Default)]
pub struct TaskUserInputError {
    pub id: Option<String>,
    pub worksheet_id: Option<String>,
    pub service_id: Option<String>,
    pub currency_code: Option<String>,
    pub price: Option<String>,
    pub tax_id: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
}

impl TaskUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.worksheet_id.is_none()
            && self.service_id.is_none()
            && self.currency_code.is_none()
            && self.price.is_none()
            && self.tax_id.is_none()
            && self.status.is_none()
            && self.priority.is_none()
            && self.due_date.is_none()
    }
}

impl Display for TaskUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateTaskError: {}", json),
            Err(e) => write!(f, "CreateTaskError: {}", e),
        }
    }
}

impl FormErrorResponse for TaskUserInputError {}

impl IntoResponse for TaskUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskUserInput {
    pub id: Option<Uuid>,
    pub worksheet_id: Uuid,
    pub service_id: Uuid,
    pub currency_code: ValueObject<CurrencyCode>,
    pub price: Option<ValueObject<TaskPrice>>,
    pub tax_id: Uuid,
    pub status: ValueObject<TaskStatus>,
    pub priority: Option<ValueObject<TaskPriority>>,
    pub due_date: Option<ValueObject<TaskDueDate>>,
}

impl TryFrom<TaskUserInputHelper> for TaskUserInput {
    type Error = TaskUserInputError;
    fn try_from(value: TaskUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = TaskUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|e| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let status = ValueObject::new(TaskStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        let priority = validate_optional_string!(TaskPriority(value.priority), error.priority);

        let due_date = validate_optional_string!(TaskDueDate(value.due_date), error.due_date);

        let currency_code = ValueObject::new(CurrencyCode(value.currency_code)).inspect_err(|e| {
            error.currency_code = Some(e.to_string());
        });

        let price = validate_optional_string!(TaskPrice(value.price), error.price);

        if error.is_empty() {
            Ok(TaskUserInput {
                id,
                worksheet_id: value.worksheet_id,
                service_id: value.service_id,
                currency_code: currency_code.map_err(|_| TaskUserInputError::default())?,
                price,
                tax_id: value.tax_id,
                status: status.map_err(|_| TaskUserInputError::default())?,
                priority,
                due_date,
            })
        } else {
            Err(error)
        }
    }
}
