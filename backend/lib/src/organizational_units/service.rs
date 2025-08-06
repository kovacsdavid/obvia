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

use crate::app::config::TenantDatabaseConfig;
use crate::app::services::migrate_tenant_db;
use crate::auth::dto::claims::Claims;
use crate::common::dto::{OkResponse, SimpleMessageResponse};
use crate::common::error::FriendlyError;
use crate::organizational_units::OrganizationalUnitsModule;
use crate::organizational_units::dto::CreateRequest;
use crate::organizational_units::repository::OrganizationalUnitsRepository;
use std::sync::Arc;
use tracing::Level;

/// Asynchronously creates a new organizational unit and performs necessary configurations.
///
/// This function interacts with the specified repository to insert and connect a new organizational unit
/// based on the provided payload and claims. After a successful insertion, it configures the
/// tenant database for the newly created organizational unit, including adding and setting
/// up the tenant pool and running database migrations.
///
/// # Parameters
/// - `repo`: A mutable reference to a repository implementing the `OrganizationalUnitsRepository` trait, which handles persistence and retrieval of organizational unit data.
/// - `claims`: A `Claims` object containing authorization and authentication details for the current user.
/// - `payload`: A `CreateRequest` object containing the input data required to create the organizational unit.
/// - `organizational_units_module`: An `Arc` reference to the `OrganizationalUnitsModule`, which provides configuration and management capabilities for organizational unit-related tasks.
///
/// # Returns
/// If successful, returns an `OkResponse` object containing a `SimpleMessageResponse` with a success message.
/// If any error occurs during the process, such as failure to insert the organizational unit,
/// add the tenant pool, or complete database migrations, a `FriendlyError` is returned with appropriate error details.
///
/// # Errors
/// - Returns `FriendlyError::Internal` if:
///   - The organizational unit could not be inserted or connected in the repository.
///   - Adding the tenant pool for the newly created organizational unit fails.
///   - Retrieving the tenant pool or executing database migrations encounters an error.
pub async fn try_create(
    repo: &mut (dyn OrganizationalUnitsRepository + Send + Sync),
    claims: Claims,
    payload: CreateRequest,
    organizational_units_module: Arc<OrganizationalUnitsModule>,
) -> Result<OkResponse<SimpleMessageResponse>, FriendlyError> {
    // try_connection
    // insert organizational_unit and connect with user
    // if (managed) { create database user and database }
    // add_tenant_pool

    match repo
        .insert_and_connect(payload, claims, organizational_units_module.config.clone())
        .await
    {
        Ok(organizational_unit) => {
            match organizational_units_module
                .pool_manager
                .add_tenant_pool(
                    organizational_unit.id,
                    &TenantDatabaseConfig::try_from(&organizational_unit)
                        .map_err(|e| FriendlyError::Internal(e.to_string()).trace(Level::ERROR))?,
                )
                .await
            {
                Ok(_) => {
                    match &organizational_units_module
                        .pool_manager
                        .get_tenant_pool(organizational_unit.id)
                        .map_err(|e| FriendlyError::Internal(e.to_string()).trace(Level::ERROR))?
                    {
                        Some(tenant_pool) => match migrate_tenant_db(tenant_pool).await {
                            Ok(_) => Ok(OkResponse::new(SimpleMessageResponse {
                                message: String::from(
                                    "Szervezeti egység létrehozása sikeresen megtörtént!",
                                ),
                            })),
                            Err(e) => {
                                Err(FriendlyError::Internal(e.to_string()).trace(Level::ERROR))
                            }
                        },
                        None => Err(FriendlyError::Internal(
                            "Could not get tenant_pool".to_string(),
                        )
                        .trace(Level::ERROR)),
                    }
                }
                Err(e) => Err(FriendlyError::Internal(e.to_string()).trace(Level::ERROR)),
            }
        }
        Err(e) => Err(FriendlyError::Internal(e.to_string()).trace(Level::ERROR)),
    }
}
