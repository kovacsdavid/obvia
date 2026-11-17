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

use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use crate::manager::auth::dto::claims::Claims;

#[derive(Debug, Error)]
pub enum BaseServiceError {
    #[error("Hozzáférés megtagadva!")]
    Unauthorized,
}

type BaseServiceResult<T> = Result<T, BaseServiceError>;

pub struct BaseService<'a, T>
where
    T: Send + Sync,
{
    claims: Option<&'a Claims>,
    module: Arc<T>,
}

impl<'a, T> BaseService<'a, T>
where
    T: Send + Sync,
{
    pub fn new(claims: Option<&'a Claims>, module: Arc<T>) -> Self {
        BaseService { claims, module }
    }
    pub fn claims(&self) -> BaseServiceResult<&Claims> {
        self.claims.ok_or(BaseServiceError::Unauthorized)
    }
    pub fn module(&self) -> &T {
        &self.module
    }
    pub fn active_tenant(&self) -> BaseServiceResult<Uuid> {
        self.claims()?
            .active_tenant()
            .ok_or(BaseServiceError::Unauthorized)
    }
}
