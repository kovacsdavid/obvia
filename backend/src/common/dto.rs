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

use crate::common::error::{BuilderError, BuilderResult};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Serialize)]
pub struct SuccessResponse<M, D>
where
    M: Serialize,
    D: Serialize,
{
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub meta: Option<M>,
    pub data: Option<D>,
}

impl<M, D> IntoResponse for SuccessResponse<M, D>
where
    M: Serialize,
    D: Serialize,
{
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}

pub struct SuccessResponseBuilder<M, D>
where
    M: Serialize,
    D: Serialize,
{
    pub status_code: Option<StatusCode>,
    pub meta: Option<M>,
    pub data: Option<D>,
}

impl<M, D> SuccessResponseBuilder<M, D>
where
    M: Serialize,
    D: Serialize,
{
    pub fn new() -> Self {
        Self {
            status_code: None,
            meta: None,
            data: None,
        }
    }
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }
    pub fn meta(mut self, meta: M) -> Self {
        self.meta = Some(meta);
        self
    }
    pub fn data(mut self, data: D) -> Self {
        self.data = Some(data);
        self
    }
    pub fn build(self) -> BuilderResult<SuccessResponse<M, D>> {
        Ok(SuccessResponse {
            status_code: self
                .status_code
                .ok_or(BuilderError::MissingRequired("status_code"))?,
            meta: self.meta,
            data: self.data,
        })
    }
}

#[derive(Serialize)]
pub struct EmptyType;

impl Display for EmptyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmptyType")
    }
}

#[derive(Serialize)]
pub struct ErrorResponse<E>
where
    E: Serialize,
{
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub error: E,
}

impl<E> IntoResponse for ErrorResponse<E>
where
    E: Serialize,
{
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}

pub type HandlerResult = Result<Response, Response>;

#[derive(Serialize)]
pub struct FormError<T>
where
    T: Serialize + Display,
{
    pub message: String,
    pub fields: T,
}

impl<T> Display for FormError<T>
where
    T: Serialize + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FormError: message: {}, fields: {}",
            self.message, self.fields
        )
    }
}

#[derive(Serialize)]
pub struct GeneralError {
    pub message: String,
}

impl Display for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FormError: message: {}", self.message)
    }
}

#[derive(Serialize)]
pub struct SimpleMessageResponse {
    pub message: String,
}

impl SimpleMessageResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
}

#[derive(Serialize)]
pub struct PaginatorMeta {
    pub page: i32,
    pub limit: i32,
    pub total: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UuidParam {
    pub uuid: Uuid,
}

#[cfg(test)]
mod tests {}
