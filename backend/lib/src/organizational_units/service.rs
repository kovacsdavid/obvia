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

use crate::app::database::PgPoolManagerTrait;
use crate::auth::dto::claims::Claims;
use crate::common::dto::{OkResponse, SimpleMessageResponse};
use crate::common::error::FriendlyError;
use crate::organizational_units::OrganizationalUnitsModule;
use crate::organizational_units::dto::CreateRequest;
use crate::organizational_units::repository::OrganizationalUnitsRepository;
use std::sync::Arc;
use tracing::Level;

pub async fn try_create(
    repo: &mut (dyn OrganizationalUnitsRepository + Send + Sync),
    claims: Claims,
    payload: CreateRequest,
    organizational_units_module: Arc<OrganizationalUnitsModule>,
) -> Result<OkResponse<SimpleMessageResponse>, FriendlyError> {
    match repo
        .insert_and_connect(payload, claims, organizational_units_module.config.clone())
        .await
    {
        Ok(organizational_unit) => {
            match organizational_units_module
                .db_pools
                .add_tenant_pool(
                    organizational_unit.id.to_string(),
                    &organizational_unit.into(),
                )
                .await
            {
                Ok(_) => Ok(OkResponse::new(SimpleMessageResponse {
                    message: String::from("Szervezeti egység létrehozása sikeresen megtörtént!"),
                })),
                Err(e) => Err(FriendlyError::Internal(e.to_string()).trace(Level::ERROR)),
            }
        }
        Err(e) => Err(FriendlyError::Internal(e.to_string()).trace(Level::ERROR)),
    }
}
