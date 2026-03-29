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
use crate::common::types::CurrencyCode;
use crate::common::types::UuidVO;
use crate::common::types::ValueObject;
use crate::common::types::quantity::Quantity;
use crate::tenant::tasks::types::task::{
    TaskDescription, TaskDueDate, TaskPrice, TaskPriority, TaskStatus,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct TaskUserInputHelper {
    pub id: Option<String>,
    pub worksheet_id: String,
    pub service_id: String,
    pub currency_code: String,
    pub quantity: String,
    pub price: String,
    pub tax_id: String,
    pub status: String,
    pub priority: String,
    pub due_date: String,
    pub description: String,
}

#[derive(Debug, Serialize, Default)]
pub struct TaskUserInputError {
    pub id: Option<String>,
    pub worksheet_id: Option<String>,
    pub service_id: Option<String>,
    pub currency_code: Option<String>,
    pub quantity: Option<String>,
    pub price: Option<String>,
    pub tax_id: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
    pub description: Option<String>,
}

impl TaskUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.worksheet_id.is_none()
            && self.service_id.is_none()
            && self.currency_code.is_none()
            && self.quantity.is_none()
            && self.price.is_none()
            && self.tax_id.is_none()
            && self.status.is_none()
            && self.priority.is_none()
            && self.due_date.is_none()
            && self.description.is_none()
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
    pub id: Option<ValueObject<UuidVO>>,
    pub worksheet_id: ValueObject<UuidVO>,
    pub service_id: ValueObject<UuidVO>,
    pub currency_code: ValueObject<CurrencyCode>,
    pub quantity: Option<ValueObject<Quantity>>,
    pub price: Option<ValueObject<TaskPrice>>,
    pub tax_id: ValueObject<UuidVO>,
    pub status: ValueObject<TaskStatus>,
    pub priority: Option<ValueObject<TaskPriority>>,
    pub due_date: Option<ValueObject<TaskDueDate>>,
    pub description: Option<ValueObject<TaskDescription>>,
}

impl TryFrom<TaskUserInputHelper> for TaskUserInput {
    type Error = TaskUserInputError;
    fn try_from(value: TaskUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = TaskUserInputError::default();

        let id = if let Some(id) = value.id {
            ValueObject::new_optional(UuidVO(id)).inspect_err(|e| {
                error.id = Some(e.to_string());
            })
        } else {
            Ok(None)
        };

        let worksheet_id = ValueObject::new_required(UuidVO(value.worksheet_id)).inspect_err(|e| {
            error.worksheet_id = Some(e.to_string());
        });

        let service_id = ValueObject::new_required(UuidVO(value.service_id)).inspect_err(|e| {
            error.service_id = Some(e.to_string());
        });

        let status = ValueObject::new_required(TaskStatus(value.status)).inspect_err(|e| {
            error.status = Some(e.to_string());
        });

        let priority = ValueObject::new_optional(TaskPriority(value.priority)).inspect_err(|e| {
            error.priority = Some(e.to_string());
        });

        let due_date = ValueObject::new_optional(TaskDueDate(value.due_date)).inspect_err(|e| {
            error.due_date = Some(e.to_string());
        });

        let currency_code = ValueObject::new_required(CurrencyCode(value.currency_code))
            .inspect_err(|e| {
                error.currency_code = Some(e.to_string());
            });

        let quantity = ValueObject::new_optional(Quantity(value.quantity)).inspect_err(|e| {
            error.quantity = Some(e.to_string());
        });

        let tax_id = ValueObject::new_required(UuidVO(value.tax_id)).inspect_err(|e| {
            error.tax_id = Some(e.to_string());
        });

        let price = ValueObject::new_optional(TaskPrice(value.price)).inspect_err(|e| {
            error.price = Some(e.to_string());
        });

        let description = ValueObject::new_optional(TaskDescription(value.description))
            .inspect_err(|e| {
                error.description = Some(e.to_string());
            });

        if error.is_empty() {
            Ok(TaskUserInput {
                id: id.map_err(|_| TaskUserInputError::default())?,
                worksheet_id: worksheet_id.map_err(|_| TaskUserInputError::default())?,
                service_id: service_id.map_err(|_| TaskUserInputError::default())?,
                currency_code: currency_code.map_err(|_| TaskUserInputError::default())?,
                quantity: quantity.map_err(|_| TaskUserInputError::default())?,
                price: price.map_err(|_| TaskUserInputError::default())?,
                tax_id: tax_id.map_err(|_| TaskUserInputError::default())?,
                status: status.map_err(|_| TaskUserInputError::default())?,
                priority: priority.map_err(|_| TaskUserInputError::default())?,
                due_date: due_date.map_err(|_| TaskUserInputError::default())?,
                description: description.map_err(|_| TaskUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
