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

use crate::common::error::RepositoryError;
use crate::manager::auth::dto::claims::Claims;
use crate::tenant::warehouses::WarehousesModule;
use crate::tenant::warehouses::dto::CreateWarehouse;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WarehousesServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Unauthorized")]
    Unauthorized,
}

pub struct WarehousesService;

impl WarehousesService {
    pub async fn try_create(
        claims: &Claims,
        payload: &CreateWarehouse,
        warehouses_module: Arc<WarehousesModule>,
    ) -> Result<(), WarehousesServiceError> {
        warehouses_module
            .warehouses_repo
            .insert(
                payload.clone(),
                claims.sub(),
                claims
                    .active_tenant()
                    .ok_or(WarehousesServiceError::Unauthorized)?,
            )
            .await?;
        Ok(())
    }
}
