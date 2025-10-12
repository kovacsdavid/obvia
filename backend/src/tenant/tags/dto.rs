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
use crate::tenant::tags::types::tag::{TagDescription, TagName};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct TagUserInputHelper {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Default)]
pub struct TagUserInputError {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl TagUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.name.is_none() && self.description.is_none()
    }
}

impl Display for TagUserInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CreateTagError: {}", json),
            Err(e) => write!(f, "CreateTagError: {}", e),
        }
    }
}

impl FormErrorResponse for TagUserInputError {}

impl IntoResponse for TagUserInputError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagUserInput {
    pub id: Option<Uuid>,
    pub name: ValueObject<TagName>,
    pub description: Option<ValueObject<TagDescription>>,
}

impl TryFrom<TagUserInputHelper> for TagUserInput {
    type Error = TagUserInputError;
    fn try_from(value: TagUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = TagUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|e| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let name = ValueObject::new(TagName(value.name)).inspect_err(|e| {
            error.name = Some(e.to_string());
        });

        let description = match ValueObject::new(TagDescription(value.description))
            .inspect_err(|e| error.description = Some(e.to_string()))
        {
            Ok(val) => {
                if !val.extract().get_value().trim().is_empty() {
                    Some(val)
                } else {
                    None
                }
            }
            Err(_) => None,
        };

        if error.is_empty() {
            Ok(TagUserInput {
                id,
                name: name.map_err(|_| TagUserInputError::default())?,
                description,
            })
        } else {
            Err(error)
        }
    }
}
