/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

use std::fmt::Display;

use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::{error::FormErrorResponse, types::ValueObject},
    tenant::comments::types::{Comment, CommentableType},
};

#[derive(Debug, Deserialize)]
pub struct CommentUserInputHelper {
    pub id: Option<String>,
    pub commentable_type: String,
    pub commentable_id: String,
    pub comment: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CommentUserInputError {
    pub id: Option<String>,
    pub commentable_type: Option<String>,
    pub commentable_id: Option<String>,
    pub comment: Option<String>,
}

impl CommentUserInputError {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.commentable_type.is_none()
            && self.commentable_id.is_none()
            && self.comment.is_none()
    }
}

impl Display for CommentUserInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "CommentUserInputError: {}", json),
            Err(e) => write!(f, "CommentUserInputError: {}", e),
        }
    }
}

impl FormErrorResponse for CommentUserInputError {}

impl IntoResponse for CommentUserInputError {
    fn into_response(self) -> axum::response::Response {
        self.get_error_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentUserInput {
    pub id: Option<Uuid>,
    pub commentable_type: ValueObject<CommentableType>,
    pub commentable_id: Uuid,
    pub comment: ValueObject<Comment>,
}

impl TryFrom<CommentUserInputHelper> for CommentUserInput {
    type Error = CommentUserInputError;
    fn try_from(value: CommentUserInputHelper) -> Result<Self, Self::Error> {
        let mut error = CommentUserInputError::default();

        let id = match value.id {
            None => None,
            Some(id) => Uuid::parse_str(&id)
                .inspect_err(|e| {
                    error.id = Some("Hibás azonosító".to_string());
                })
                .ok(),
        };

        let commentable_type = ValueObject::new(CommentableType(value.commentable_type))
            .inspect_err(|e| {
                error.commentable_type = Some(e.to_string());
            });

        let commentable_id = Uuid::parse_str(&value.commentable_id).inspect_err(|e| {
            error.commentable_id = Some("Hibás erőforrás azonosító".to_string());
        });

        let comment = ValueObject::new(Comment(value.comment)).inspect_err(|e| {
            error.comment = Some(e.to_string());
        });

        if error.is_empty() {
            Ok(CommentUserInput {
                id,
                commentable_type: commentable_type.map_err(|_| CommentUserInputError::default())?,
                commentable_id: commentable_id.map_err(|_| CommentUserInputError::default())?,
                comment: comment.map_err(|_| CommentUserInputError::default())?,
            })
        } else {
            Err(error)
        }
    }
}
