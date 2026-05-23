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
use crate::common::types::UuidVO;
use crate::common::types::quantity::Quantity;
use crate::common::value_object::ValueObjectError;
use crate::common::value_object::ValueObjectOptional;
use crate::common::value_object::ValueObjectRequired;
use crate::tenant::currencies::types::CurrencyCode;
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

impl From<ValueObjectError> for TaskUserInputError {
    fn from(_: ValueObjectError) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct TaskUserInput {
    pub id: ValueObjectOptional<UuidVO>,
    pub worksheet_id: ValueObjectRequired<UuidVO>,
    pub service_id: ValueObjectRequired<UuidVO>,
    pub currency_code: ValueObjectRequired<CurrencyCode>,
    pub quantity: ValueObjectOptional<Quantity>,
    pub price: ValueObjectOptional<TaskPrice>,
    pub tax_id: ValueObjectRequired<UuidVO>,
    pub status: ValueObjectRequired<TaskStatus>,
    pub priority: ValueObjectOptional<TaskPriority>,
    pub due_date: ValueObjectOptional<TaskDueDate>,
    pub description: ValueObjectOptional<TaskDescription>,
}

impl TryFrom<TaskUserInputHelper> for TaskUserInput {
    type Error = TaskUserInputError;
    fn try_from(value: TaskUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = TaskUserInputError::default();

        let id = value
            .id
            .unwrap_or("".to_owned())
            .parse::<ValueObjectOptional<UuidVO>>()
            .inspect_err(|e| {
                error.id = Some(e.to_string());
            });

        let worksheet_id = value
            .worksheet_id
            .parse::<ValueObjectRequired<UuidVO>>()
            .inspect_err(|e| {
                error.worksheet_id = Some(e.to_string());
            });

        let service_id = value
            .service_id
            .parse::<ValueObjectRequired<UuidVO>>()
            .inspect_err(|e| {
                error.service_id = Some(e.to_string());
            });

        let status = value
            .status
            .parse::<ValueObjectRequired<TaskStatus>>()
            .inspect_err(|e| {
                error.status = Some(e.to_string());
            });

        let priority = value
            .priority
            .parse::<ValueObjectOptional<TaskPriority>>()
            .inspect_err(|e| {
                error.priority = Some(e.to_string());
            });

        let due_date = value
            .due_date
            .parse::<ValueObjectOptional<TaskDueDate>>()
            .inspect_err(|e| {
                error.due_date = Some(e.to_string());
            });

        let currency_code = value
            .currency_code
            .parse::<ValueObjectRequired<CurrencyCode>>()
            .inspect_err(|e| {
                error.currency_code = Some(e.to_string());
            });

        let quantity = value
            .quantity
            .parse::<ValueObjectOptional<Quantity>>()
            .inspect_err(|e| {
                error.quantity = Some(e.to_string());
            });

        let tax_id = value
            .tax_id
            .parse::<ValueObjectRequired<UuidVO>>()
            .inspect_err(|e| {
                error.tax_id = Some(e.to_string());
            });

        let price = value
            .price
            .parse::<ValueObjectOptional<TaskPrice>>()
            .inspect_err(|e| {
                error.price = Some(e.to_string());
            });

        let description = value
            .description
            .parse::<ValueObjectOptional<TaskDescription>>()
            .inspect_err(|e| {
                error.description = Some(e.to_string());
            });

        if error.is_empty() {
            Ok(TaskUserInput {
                id: id?,
                worksheet_id: worksheet_id?,
                service_id: service_id?,
                currency_code: currency_code?,
                quantity: quantity?,
                price: price?,
                tax_id: tax_id?,
                status: status?,
                priority: priority?,
                due_date: due_date?,
                description: description?,
            })
        } else {
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Days, Utc};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn valid_user_input() {
        let worksheet_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let tax_id = Uuid::new_v4();
        let due_date = Utc::now()
            .checked_add_days(Days::new(1))
            .unwrap()
            .date_naive();
        let user_input = TaskUserInput::try_from(TaskUserInputHelper {
            id: None,
            worksheet_id: worksheet_id.to_string(),
            service_id: service_id.to_string(),
            currency_code: String::from("HUF"),
            quantity: String::from("10"),
            price: String::from("1000"),
            tax_id: tax_id.to_string(),
            status: String::from("active"),
            priority: String::from("normal"),
            due_date: due_date.to_string(),
            description: String::from("description"),
        })
        .unwrap();

        assert_eq!(user_input.id.as_uuid(), None);
        assert_eq!(user_input.worksheet_id.as_uuid().unwrap(), worksheet_id);
        assert_eq!(user_input.service_id.as_uuid().unwrap(), service_id);
        assert_eq!(user_input.currency_code.as_str().unwrap(), "HUF");
        assert_eq!(user_input.quantity.as_f64().unwrap(), 10_f64);
        assert_eq!(user_input.price.as_f64().unwrap(), 1000_f64);
        assert_eq!(user_input.tax_id.as_uuid().unwrap(), tax_id);
        assert_eq!(user_input.status.as_str().unwrap(), "active");
        assert_eq!(user_input.priority.as_str().unwrap(), "normal");
        assert_eq!(user_input.due_date.as_date_naive().unwrap(), &due_date);
        assert_eq!(user_input.description.as_str().unwrap(), "description");
    }

    #[test]
    fn invalid_user_input() {
        let invalid_description = "a".repeat(3001);
        let user_input = TaskUserInput::try_from(TaskUserInputHelper {
            id: None,
            worksheet_id: String::from("invalid"),
            service_id: String::from(""),
            currency_code: String::from("HUFF"),
            quantity: String::from("invalid"),
            price: String::from("invalid"),
            tax_id: String::from("invalid"),
            status: String::from("invalid"),
            priority: String::from("invalid"),
            due_date: String::from("invalid"),
            description: invalid_description,
        })
        .unwrap_err();

        assert_eq!(user_input.id, None);
        assert_eq!(user_input.worksheet_id.unwrap(), UuidVO::PARSE_ERROR);
        assert_eq!(user_input.service_id.unwrap(), ValueObjectError::REQUIRED);
        assert_eq!(
            user_input.currency_code.unwrap(),
            CurrencyCode::VALIDATION_ERROR
        );
        assert_eq!(user_input.quantity.unwrap(), Quantity::PARSE_ERROR);
        assert_eq!(user_input.price.unwrap(), TaskPrice::PARSE_ERROR);
        assert_eq!(user_input.tax_id.unwrap(), UuidVO::PARSE_ERROR);
        assert_eq!(user_input.status.unwrap(), TaskStatus::VALIDATION_ERROR);
        assert_eq!(user_input.priority.unwrap(), TaskPriority::VALIDATION_ERROR);
        assert_eq!(user_input.due_date.unwrap(), TaskDueDate::PARSE_ERROR);
        assert_eq!(
            user_input.description.unwrap(),
            TaskDescription::VALIDATION_ERROR
        );
    }
}
