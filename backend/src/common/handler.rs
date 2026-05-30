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
use serde::Serialize;
use std::{fmt::Display, sync::Arc};

use crate::{
    common::{MailTransporter, error::IntoFriendlyError, service::Service},
    manager::auth::dto::claims::Claims,
};

pub type HandlerResult = Result<Response, Response>;

pub struct ErrorMapper<T>
where
    T: ?Sized + MailTransporter,
{
    mail_transporter: Arc<T>,
}

impl<T> ErrorMapper<T>
where
    T: ?Sized + MailTransporter,
{
    pub fn new(mail_transporter: Arc<T>) -> Self {
        Self { mail_transporter }
    }
    pub async fn or_handler_error<H, E, I>(&self, result: Result<H, E>) -> Result<H, Response>
    where
        I: Serialize + Display,
        E: IntoFriendlyError<I, T>,
    {
        match result {
            Ok(value) => Ok(value),
            Err(err) => Err(err
                .into_friendly_error(self.mail_transporter.clone())
                .await
                .into_response()),
        }
    }
}

pub fn init_handler<'a, T>(
    claims: Option<&'a Claims>,
    module: Arc<T>,
) -> (Service<'a, T>, ErrorMapper<T>)
where
    T: ?Sized + MailTransporter,
{
    (
        Service::new(claims, module.clone()),
        ErrorMapper::new(module),
    )
}
