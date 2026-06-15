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

use axum::response::{IntoResponse, Response};
use std::sync::Arc;

use crate::common::{BaseModule, error::IntoFriendlyError};

pub trait ErrorMapperInterface {
    fn or_handler_error<R, E>(
        &self,
        result: Result<R, E>,
    ) -> impl Future<Output = Result<R, Response>> + Send
    where
        R: Send + Sync + 'static,
        E: IntoFriendlyError + Send + Sync + 'static;
}

pub struct ErrorMapper<M>
where
    M: BaseModule,
{
    mailer: Arc<M>,
}

impl<M> ErrorMapper<M>
where
    M: BaseModule,
{
    pub fn new(mailer: Arc<M>) -> Self {
        Self { mailer }
    }
}

impl<M> ErrorMapperInterface for ErrorMapper<M>
where
    M: BaseModule,
{
    async fn or_handler_error<R, E>(&self, result: Result<R, E>) -> Result<R, Response>
    where
        R: Send + Sync + 'static,
        E: IntoFriendlyError + Send + Sync + 'static,
    {
        match result {
            Ok(value) => Ok(value),
            Err(err) => Err(err
                .into_friendly_error(self.mailer.clone())
                .await
                .into_response()),
        }
    }
}

pub type HandlerResult = Result<Response, Response>;
