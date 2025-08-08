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

use crate::app::config::{BasicDatabaseConfig, TenantDatabaseConfig};
use crate::app::database::PgConnectionTester;
use crate::app::services::migrate_tenant_db;
use crate::auth::dto::claims::Claims;
use crate::common::dto::{OkResponse, SimpleMessageResponse};
use crate::common::error::FriendlyError;
use crate::common::services::generate_string_csprng;
use crate::common::types::value_object::ValueObjectable;
use crate::organizational_units::OrganizationalUnitsModule;
use crate::organizational_units::dto::CreateRequest;
use crate::organizational_units::repository::OrganizationalUnitsRepository;
use axum::http::StatusCode;
use sqlx::postgres::PgSslMode;
use std::sync::Arc;
use tracing::Level;
use uuid::Uuid;

/// Handles the process of setting up a self-hosted organizational unit in the system.
///
/// # Workflow
///
/// 1. Converts the provided payload into a `TenantDatabaseConfig`.
/// 2. Tests the connectivity to the provided PostgreSQL database with SSL mode verification.
/// 3. Checks if the targeted database is empty.
/// 4. Creates a new organizational unit in the repository.
/// 5. Adds the organizational unit's database connection pool to the pool manager.
/// 6. Initializes the tenant's database schema by running migrations.
/// 7. Returns a success message upon the successful completion of all the steps.
///
/// # Parameters
/// - `repo`: A mutable reference to a dynamic implementation of the `OrganizationalUnitsRepository` trait. Used to manage organizational units in the system.
/// - `claims`: The JWT claims of the requester, typically containing user identity and access details.
/// - `payload`: The data required for creating a self-hosted organizational unit, like database configurations.
/// - `organizational_units_module`: A shared reference (`Arc`) to the `OrganizationalUnitsModule` which holds relevant configurations and services.
///
/// # Returns
/// - `Ok(OkResponse<SimpleMessageResponse>)`: If the self-hosted organizational unit setup is successful.
/// - `Err(FriendlyError)`: If any step fails due to a validation, database connectivity, or internal system error.
///
/// # Errors
/// - `FriendlyError::Internal`: Covers internal system errors, such as configuration conversion failures, tenant pool handling issues, or migration failures.
/// - `FriendlyError::UserFacing`: Covers user-facing errors like the inability to connect to the provided database or validation issues with the provided payload.
///
/// # Notes
/// - The database connectivity is tested with `PgSslMode::VerifyFull` for enhanced security.
/// - If the function fails at any step, an appropriate error message is returned to help diagnose the issue.
/// - All errors are logged using the specified `trace` levels for debugging purposes.
///
/// # Panics
/// This function does not explicitly panic. However, unexpected panics may occur if dependent modules or traits are not correctly implemented.
async fn self_hosted(
    repo: &mut (dyn OrganizationalUnitsRepository + Send + Sync),
    claims: Claims,
    payload: CreateRequest,
    organizational_units_module: Arc<OrganizationalUnitsModule>,
) -> Result<OkResponse<SimpleMessageResponse>, FriendlyError> {
    let config: TenantDatabaseConfig = payload
        .clone()
        .try_into()
        .map_err(|e: String| FriendlyError::Internal(e).trace(Level::ERROR))?;
    match &mut PgConnectionTester::test_connect(&config, PgSslMode::VerifyFull).await {
        Ok(conn) => match PgConnectionTester::is_empty_database(conn).await {
            Ok(_) => match repo
                .setup_self_hosted(payload.name.extract().get_value(), &config.into(), &claims)
                .await
            {
                Ok(organizational_unit) => match organizational_units_module
                    .pool_manager
                    .add_tenant_pool(
                        organizational_unit.id,
                        &TenantDatabaseConfig::try_from(&organizational_unit)
                            .map_err(|e| {
                                FriendlyError::Internal(e.to_string()).trace(Level::ERROR)
                            })?
                            .into(),
                    )
                    .await
                {
                    Ok(_) => {
                        match &organizational_units_module
                            .pool_manager
                            .get_tenant_pool(organizational_unit.id)
                            .map_err(|e| {
                                FriendlyError::Internal(e.to_string()).trace(Level::ERROR)
                            })? {
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
                },
                Err(_) => Err(FriendlyError::UserFacing(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "ORGANIZATIONAL_UNTIS/SERVICE/COULD_NOT_CONNECT_TO_DATABASE".to_string(),
                    "Nem sikerült csatlakozni az adatbázishoz".to_string(),
                )
                .trace(Level::INFO)),
            },
            Err(_) => Err(FriendlyError::UserFacing(
                StatusCode::UNPROCESSABLE_ENTITY,
                "ORGANIZATIONAL_UNTIS/SERVICE/COULD_NOT_CONNECT_TO_SELF_HOSTED_DATABASE"
                    .to_string(),
                "Nem sikerült csatlakozni az adatbázishoz".to_string(),
            )
            .trace(Level::INFO)),
        },
        Err(_) => Err(FriendlyError::UserFacing(
            StatusCode::UNPROCESSABLE_ENTITY,
            "ORGANIZATIONAL_UNTIS/SERVICE/COULD_NOT_CONNECT_TO_SELF_HOSTED_DATABASE".to_string(),
            "Nem sikerült csatlakozni az adatbázishoz".to_string(),
        )
        .trace(Level::INFO)),
    }
}

/// Asynchronously manages the creation and setup of an organizational unit, including
/// the configuration and database initialization needed for a tenant environment.
///
/// # Parameters
/// * `repo` - A mutable reference to an implementation of the `OrganizationalUnitsRepository`
///   trait. This represents the repository for managing organizational units in persistence
///   storage.
/// * `claims` - JWT claims representing the current authenticated user's context. This
///   could include permissions, roles, or other identifying information.
/// * `payload` - The `CreateRequest` object containing the data required to create the
///   organizational unit.
/// * `organizational_units_module` - An `Arc`-wrapped instance of the `OrganizationalUnitsModule`,
///   which provides configurations and pool management utilities for handling tenant databases.
///
/// # Returns
/// * `Result<OkResponse<SimpleMessageResponse>, FriendlyError>` - In the success case, it
///   returns an `OkResponse` containing a `SimpleMessageResponse` with a success message.
///   In the error case, it returns a `FriendlyError` detailing the nature of the failure.
///
/// # Workflow
/// 1. Calls the `setup_managed` method of the repository to initialize and persist the
///    organizational unit using the provided payload, claims, and module configuration.
/// 2. If successful, initializes a tenant database pool using the organizational unit's ID and
///    configuration derived from `TenantDatabaseConfig`.
/// 3. Attempts to retrieve the tenant database pool, and if found, runs database migration via
///    `migrate_tenant_db`.
/// 4. If all steps complete successfully, confirms the creation of the organizational unit with
///    a success message.
/// 5. Any failure at any step is captured and returned as a `FriendlyError`, including
///    context for easier debugging.
///
/// # Errors
/// The function can return a `FriendlyError` in one of the following cases:
/// * Failure to setup the organizational unit in the repository.
/// * Failure to add a tenant pool to the pool manager.
/// * Failure to retrieve the tenant pool from the pool manager.
/// * Failure to perform database migrations for the tenant.
///
/// # Localization
/// * The success message, "Szervezeti egység létrehozása sikeresen megtörtént!", is hardcoded
///   in Hungarian. Modify it if localization is necessary for other languages.
async fn managed(
    repo: &mut (dyn OrganizationalUnitsRepository + Send + Sync),
    claims: Claims,
    payload: CreateRequest,
    organizational_units_module: Arc<OrganizationalUnitsModule>,
) -> Result<OkResponse<SimpleMessageResponse>, FriendlyError> {
    let uuid = Uuid::new_v4();
    let db_config = BasicDatabaseConfig {
        host: organizational_units_module
            .config
            .default_tenant_database()
            .host
            .clone(),
        port: organizational_units_module
            .config
            .default_tenant_database()
            .port,
        username: format!("tenant_{}", uuid.to_string().replace("-", "")),
        password: generate_string_csprng(40),
        database: format!("tenant_{}", uuid.to_string().replace("-", "")),
        max_pool_size: None,
        ssl_mode: Some(String::from("disable")),
    };
    match repo
        .setup_managed(
            uuid,
            payload.name.extract().get_value(),
            &db_config,
            &claims,
            organizational_units_module.config.clone(),
        )
        .await
    {
        Ok(organizational_unit) => {
            match organizational_units_module
                .pool_manager
                .add_tenant_pool(
                    organizational_unit.id,
                    &BasicDatabaseConfig::try_from(&organizational_unit)
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

/// Attempts to create an organizational unit based on the provided payload, handling both self-hosted
/// and managed scenarios asynchronously.
///
/// # Arguments
/// * `repo` - A mutable reference to an object implementing the `OrganizationalUnitsRepository` trait,
///   which allows for interaction with the underlying organizational units data store. Must be `Send` and `Sync`.
///
/// * `claims` - The authentication and authorization claims for the current user or process,
///   used to validate permissions for the requested operation.
///
/// * `payload` - The `CreateRequest` object containing the necessary data to create the organizational
///   unit. The `payload` determines whether the creation is for a self-hosted or managed unit.
///
/// * `organizational_units_module` - An `Arc` reference to the `OrganizationalUnitsModule`,
///   which encapsulates logic and dependencies for organizational units functionality.
///
/// # Returns
/// Returns a `Result`:
/// * `Ok(OkResponse<SimpleMessageResponse>)` - Indicates a successful creation operation, with a response
///   message encapsulated in a `SimpleMessageResponse`.
/// * `Err(FriendlyError)` - Indicates a failure during the creation process, returning a user-friendly error.
///
/// # Behavior
/// This function evaluates whether the `payload` specifies a self-hosted or managed organizational unit:
/// * If `payload.is_self_hosted()` evaluates to true, the `self_hosted` function is invoked.
/// * If false, the `managed` function is invoked.
///
/// Both `self_hosted` and `managed` perform the creation logic for their respective scenarios asynchronously.
///
/// # Errors
/// If the creation fails, a `FriendlyError` is returned, which provides a user-comprehensible description
/// of the error for better clarity and user experience.
pub async fn try_create(
    repo: &mut (dyn OrganizationalUnitsRepository + Send + Sync),
    claims: Claims,
    payload: CreateRequest,
    organizational_units_module: Arc<OrganizationalUnitsModule>,
) -> Result<OkResponse<SimpleMessageResponse>, FriendlyError> {
    if payload.is_self_hosted() {
        self_hosted(repo, claims, payload, organizational_units_module).await
    } else {
        managed(repo, claims, payload, organizational_units_module).await
    }
}
