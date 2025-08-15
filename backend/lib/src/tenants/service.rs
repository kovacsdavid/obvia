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
use crate::app::database::DatabaseMigrator;
use crate::auth::dto::claims::Claims;
use crate::common::dto::{OkResponse, SimpleMessageResponse};
use crate::common::error::FriendlyError;
use crate::common::services::generate_string_csprng;
use crate::common::types::value_object::ValueObjectable;
use crate::tenants::TenantsModule;
use crate::tenants::dto::TenantCreateRequest;
use crate::tenants::repository::TenantsRepository;
use axum::http::StatusCode;
use sqlx::postgres::PgSslMode;
use std::sync::Arc;
use tracing::Level;
use uuid::Uuid;

/// Handles the process of setting up a self-hosted tenant in the system.
///
/// # Workflow
///
/// 1. Converts the provided payload into a `TenantDatabaseConfig`.
/// 2. Tests the connectivity to the provided PostgreSQL database with SSL mode verification.
/// 3. Checks if the targeted database is empty.
/// 4. Creates a new tenant in the repository.
/// 5. Adds the tenant's database connection pool to the pool manager.
/// 6. Initializes the tenant's database schema by running migrations.
/// 7. Returns a success message upon the successful completion of all the steps.
///
/// # Parameters
/// - `repo`: A mutable reference to a dynamic implementation of the `TenantsRepository` trait. Used to manage tenants in the system.
/// - `claims`: The JWT claims of the requester, typically containing user identity and access details.
/// - `payload`: The data required for creating a self-hosted tenant, like database configurations.
/// - `tenants_module`: A shared reference (`Arc`) to the `TenantsModule` which holds relevant configurations and services.
///
/// # Returns
/// - `Ok(OkResponse<SimpleMessageResponse>)`: If the self-hosted tenant setup is successful.
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
    repo: &mut (dyn TenantsRepository + Send + Sync),
    migrator: &(dyn DatabaseMigrator + Send + Sync),
    claims: Claims,
    payload: TenantCreateRequest,
    tenants_module: Arc<TenantsModule>,
) -> Result<OkResponse<SimpleMessageResponse>, FriendlyError> {
    let config: TenantDatabaseConfig = payload
        .clone()
        .try_into()
        .map_err(|e: String| FriendlyError::Internal(e).trace(Level::ERROR))?;
    let connection_tester = (tenants_module.connection_tester_factory)();
    match connection_tester
        .test_connect(&config.clone().into(), PgSslMode::VerifyFull)
        .await
    {
        Ok(pool) => {
            match connection_tester.is_empty_database(&pool).await {
                Ok(_) => match repo
                    .setup_self_hosted(payload.name.extract().get_value(), &config.into(), &claims)
                    .await
                {
                    Ok(tenant) => match tenants_module
                        .pool_manager
                        .add_tenant_pool(
                            tenant.id,
                            &TenantDatabaseConfig::try_from(&tenant)
                                .map_err(|e| {
                                    FriendlyError::Internal(e.to_string()).trace(Level::ERROR)
                                })?
                                .into(),
                        )
                        .await
                    {
                        Ok(_) => {
                            match &tenants_module
                                .pool_manager
                                .get_tenant_pool(tenant.id)
                                .map_err(|e| {
                                    FriendlyError::Internal(e.to_string()).trace(Level::ERROR)
                                })? {
                                Some(tenant_pool) => {
                                    match migrator.migrate_tenant_db(tenant_pool).await {
                                        Ok(_) => Ok(OkResponse::new(SimpleMessageResponse {
                                            message: String::from(
                                                "Szervezeti egység létrehozása sikeresen megtörtént!",
                                            ),
                                        })),
                                        Err(e) => Err(FriendlyError::Internal(e.to_string())
                                            .trace(Level::ERROR)),
                                    }
                                }
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
            }
        }
        Err(_) => Err(FriendlyError::UserFacing(
            StatusCode::UNPROCESSABLE_ENTITY,
            "ORGANIZATIONAL_UNTIS/SERVICE/COULD_NOT_CONNECT_TO_SELF_HOSTED_DATABASE".to_string(),
            "Nem sikerült csatlakozni az adatbázishoz".to_string(),
        )
        .trace(Level::INFO)),
    }
}

/// Asynchronously manages the creation and setup of a tenant, including
/// the configuration and database initialization needed for a tenant environment.
///
/// # Parameters
/// * `repo` - A mutable reference to an implementation of the `TenantsRepository`
///   trait. This represents the repository for managing tenants in persistence
///   storage.
/// * `claims` - JWT claims representing the current authenticated user's context. This
///   could include permissions, roles, or other identifying information.
/// * `payload` - The `CreateRequest` object containing the data required to create the
///   tenant.
/// * `tenants_module` - An `Arc`-wrapped instance of the `TenantsModule`,
///   which provides configurations and pool management utilities for handling tenant databases.
///
/// # Returns
/// * `Result<OkResponse<SimpleMessageResponse>, FriendlyError>` - In the success case, it
///   returns an `OkResponse` containing a `SimpleMessageResponse` with a success message.
///   In the error case, it returns a `FriendlyError` detailing the nature of the failure.
///
/// # Workflow
/// 1. Calls the `setup_managed` method of the repository to initialize and persist the
///    tenant using the provided payload, claims, and module configuration.
/// 2. If successful, initializes a tenant database pool using the tenant's ID and
///    configuration derived from `TenantDatabaseConfig`.
/// 3. Attempts to retrieve the tenant database pool, and if found, runs database migration via
///    `migrate_tenant_db`.
/// 4. If all steps complete successfully, confirms the creation of the tenant with
///    a success message.
/// 5. Any failure at any step is captured and returned as a `FriendlyError`, including
///    context for easier debugging.
///
/// # Errors
/// The function can return a `FriendlyError` in one of the following cases:
/// * Failure to setup the tenant in the repository.
/// * Failure to add a tenant pool to the pool manager.
/// * Failure to retrieve the tenant pool from the pool manager.
/// * Failure to perform database migrations for the tenant.
///
/// # Localization
/// * The success message, "Szervezeti egység létrehozása sikeresen megtörtént!", is hardcoded
///   in Hungarian. Modify it if localization is necessary for other languages.
async fn managed(
    repo: &mut (dyn TenantsRepository + Send + Sync),
    migrator: &(dyn DatabaseMigrator + Send + Sync),
    claims: Claims,
    payload: TenantCreateRequest,
    tenants_module: Arc<TenantsModule>,
) -> Result<OkResponse<SimpleMessageResponse>, FriendlyError> {
    let uuid = Uuid::new_v4();
    let db_config = BasicDatabaseConfig {
        host: tenants_module.config.default_tenant_database().host.clone(),
        port: tenants_module.config.default_tenant_database().port,
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
            tenants_module.config.clone(),
        )
        .await
    {
        Ok(tenant) => {
            match tenants_module
                .pool_manager
                .add_tenant_pool(
                    tenant.id,
                    &BasicDatabaseConfig::try_from(&tenant)
                        .map_err(|e| FriendlyError::Internal(e.to_string()).trace(Level::ERROR))?,
                )
                .await
            {
                Ok(_) => {
                    match &tenants_module
                        .pool_manager
                        .get_tenant_pool(tenant.id)
                        .map_err(|e| FriendlyError::Internal(e.to_string()).trace(Level::ERROR))?
                    {
                        Some(tenant_pool) => match migrator.migrate_tenant_db(tenant_pool).await {
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

/// Attempts to create a tenant based on the provided payload, handling both self-hosted
/// and managed scenarios asynchronously.
///
/// # Arguments
/// * `repo` - A mutable reference to an object implementing the `TenantsRepository` trait,
///   which allows for interaction with the underlying tenants data store. Must be `Send` and `Sync`.
///
/// * `claims` - The authentication and authorization claims for the current user or process,
///   used to validate permissions for the requested operation.
///
/// * `payload` - The `CreateRequest` object containing the necessary data to create the tenant. The `payload` determines whether the creation is for a self-hosted or managed unit.
///
/// * `tenants_module` - An `Arc` reference to the `TenantsModule`,
///   which encapsulates logic and dependencies for tenants functionality.
///
/// # Returns
/// Returns a `Result`:
/// * `Ok(OkResponse<SimpleMessageResponse>)` - Indicates a successful creation operation, with a response
///   message encapsulated in a `SimpleMessageResponse`.
/// * `Err(FriendlyError)` - Indicates a failure during the creation process, returning a user-friendly error.
///
/// # Behavior
/// This function evaluates whether the `payload` specifies a self-hosted or managed tenant:
/// * If `payload.is_self_hosted()` evaluates to true, the `self_hosted` function is invoked.
/// * If false, the `managed` function is invoked.
///
/// Both `self_hosted` and `managed` perform the creation logic for their respective scenarios asynchronously.
///
/// # Errors
/// If the creation fails, a `FriendlyError` is returned, which provides a user-comprehensible description
/// of the error for better clarity and user experience.
pub async fn try_create(
    repo: &mut (dyn TenantsRepository + Send + Sync),
    migrator: &(dyn DatabaseMigrator + Send + Sync),
    claims: Claims,
    payload: TenantCreateRequest,
    tenants_module: Arc<TenantsModule>,
) -> Result<OkResponse<SimpleMessageResponse>, FriendlyError> {
    if payload.is_self_hosted() {
        self_hosted(repo, migrator, claims, payload, tenants_module).await
    } else {
        managed(repo, migrator, claims, payload, tenants_module).await
    }
}
