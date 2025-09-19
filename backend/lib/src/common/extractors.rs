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
use crate::common::error::FriendlyError;
use axum::Json;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use tracing::Level;

pub struct ValidJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, state).await.map_err(|_| {
            FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::BAD_REQUEST,
                file!(),
                "Invalid JSON",
            )
            .into_response()
        })?;

        Ok(ValidJson(payload))
    }
}

pub struct UserInput<T, H>(pub T, pub PhantomData<H>);

impl<T, H, S> FromRequest<S> for UserInput<T, H>
where
    T: TryFrom<H>,
    T::Error: IntoResponse,
    H: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let user_input = T::try_from(ValidJson::<H>::from_request(req, state).await?.0)
            .map_err(|e| e.into_response())?;

        Ok(UserInput(user_input, PhantomData))
    }
}
