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
use crate::tenant::tags::types::tag::TagName;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateTagHelper {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTagError {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl CreateTagError {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.description.is_none()
    }
}

impl IntoResponse for CreateTagError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                reference: String::from("TAGS/DTO/CREATE"),
                global: String::from("Kérjük, ellenőrizze a hibás mezőket"),
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTag {
    pub name: ValueObject<TagName>,
    pub description: Option<String>,
}

impl TryFrom<CreateTagHelper> for CreateTag {
    type Error = CreateTagError;
    fn try_from(value: CreateTagHelper) -> Result<Self, Self::Error> {
        let mut error = CreateTagError {
            name: None,
            description: None,
        };

        let name = ValueObject::new(TagName(value.name));

        if let Err(e) = &name {
            error.name = Some(e.to_string());
        }

        if error.is_empty() {
            Ok(CreateTag {
                name: name.unwrap(),
                description: Some(value.description),
            })
        } else {
            Err(error)
        }
    }
}

pub struct UpdateTagHelper {
    // TODO: fields
}

pub struct UpdateTagError {
    // TODO: fields
}

impl UpdateTagError {
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl IntoResponse for UpdateTagError {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTag {
    pub name: ValueObject<TagName>,
}

impl TryFrom<UpdateTagHelper> for UpdateTag {
    type Error = UpdateTagError;
    fn try_from(value: UpdateTagHelper) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagConnect {
    pub taggable_id: Uuid,
    pub taggable_type: String,
    pub tag_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTagConnect {
    pub taggable_id: Option<Uuid>,
    pub taggable_type: Option<String>,
    pub tag_id: Option<Uuid>,
}
