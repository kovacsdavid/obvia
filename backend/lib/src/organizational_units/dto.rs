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
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CreateRequestHelper {
    pub name: String,
    pub db_host: Option<String>,
    pub db_port: Option<i32>,
    pub db_name: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateRequestError {
    pub name: Option<String>,
    pub db_host: Option<String>,
    pub db_port: Option<String>,
    pub db_name: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
}

impl IntoResponse for CreateRequestError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse::new(ErrorBody {
                reference: String::from("ORGANIZATIONAL_UNITS/DTO/CREATE"),
                global: String::from("Kérjük, ellenőrizze a hibás mezőket"),
                fields: Some(self),
            })),
        )
            .into_response()
    }
}

#[allow(dead_code)]
pub struct CreateRequest {
    pub name: String,
    pub db_host: Option<String>,
    pub db_port: Option<i32>,
    pub db_name: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
}

impl TryFrom<CreateRequestHelper> for CreateRequest {
    type Error = CreateRequestError;
    fn try_from(value: CreateRequestHelper) -> Result<Self, Self::Error> {
        Ok(CreateRequest {
            name: value.name,
            db_host: value.db_host,
            db_port: value.db_port,
            db_name: value.db_name,
            db_user: value.db_user,
            db_password: value.db_password,
        })
    }
}

#[allow(dead_code)]
pub struct UserOrganizationalUnitConnect {
    pub user_id: Uuid,
    pub organizational_unit_id: Uuid,
    pub role: String,
    pub invited_by: Option<Uuid>,
}
