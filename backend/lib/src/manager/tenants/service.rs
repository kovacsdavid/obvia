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

use crate::common::error::{FriendlyError, RepositoryError};
use crate::manager::app::config::{AppConfig, BasicDatabaseConfig, TenantDatabaseConfig};
use crate::manager::auth::dto::claims::Claims;
use crate::manager::common::dto::{OrderingParams, PagedResult, PaginatorParams};
use crate::manager::common::services::generate_string_csprng;
use crate::manager::common::types::value_object::ValueObjectable;
use crate::manager::tenants::TenantsModule;
use crate::manager::tenants::dto::{
    CreateTenant, FilteringParams, PublicTenant, TenantActivateRequest,
};
use crate::manager::tenants::repository::TenantsRepository;
use axum::http::StatusCode;
use sqlx::postgres::PgSslMode;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum TenantsServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Could not get tenant_pool")]
    AccessTenantPool,
}

pub struct TenantsService;

impl TenantsService {
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
        claims: Claims,
        payload: CreateTenant,
        tenants_module: Arc<TenantsModule>,
    ) -> Result<(), TenantsServiceError> {
        let config: TenantDatabaseConfig = payload
            .clone()
            .try_into()
            .map_err(TenantsServiceError::Config)?;

        let pool = tenants_module
            .connection_tester
            .test_connect(&config.clone().into(), PgSslMode::VerifyFull)
            .await?;

        tenants_module
            .connection_tester
            .is_empty_database(&pool)
            .await?;

        let tenant = tenants_module
            .tenants_repo
            .setup_self_hosted(payload.name.extract().get_value(), &config.into(), &claims)
            .await?;
        tenants_module
            .pool_manager
            .add_tenant_pool(
                tenant.id,
                &TenantDatabaseConfig::try_from(&tenant)
                    .map_err(TenantsServiceError::Config)?
                    .into(),
            )
            .await?;

        let tenant_pool = tenants_module
            .pool_manager
            .get_tenant_pool(tenant.id)?
            .ok_or(TenantsServiceError::AccessTenantPool)?;

        tenants_module
            .migrator
            .migrate_tenant_db(&tenant_pool)
            .await?;

        Ok(())
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
        claims: Claims,
        payload: CreateTenant,
        tenants_module: Arc<TenantsModule>,
    ) -> Result<(), TenantsServiceError> {
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

        let tenant = tenants_module
            .tenants_repo
            .setup_managed(
                uuid,
                payload.name.extract().get_value(),
                &db_config,
                &claims,
                tenants_module.config.clone(),
            )
            .await?;

        tenants_module
            .pool_manager
            .add_tenant_pool(
                tenant.id,
                &BasicDatabaseConfig::try_from(&tenant).map_err(TenantsServiceError::Config)?,
            )
            .await?;

        let tenant_pool = tenants_module
            .pool_manager
            .get_tenant_pool(tenant.id)?
            .ok_or(TenantsServiceError::AccessTenantPool)?;

        tenants_module
            .migrator
            .migrate_tenant_db(&tenant_pool)
            .await?;
        Ok(())
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
        claims: Claims,
        payload: CreateTenant,
        tenants_module: Arc<TenantsModule>,
    ) -> Result<(), TenantsServiceError> {
        if payload.is_self_hosted() {
            Self::self_hosted(claims, payload, tenants_module).await
        } else {
            Self::managed(claims, payload, tenants_module).await
        }
    }
    pub async fn get_paged_list(
        paginator: &PaginatorParams,
        ordering: &OrderingParams,
        filtering: &FilteringParams,
        claims: &Claims,
        repo: Arc<dyn TenantsRepository + Send + Sync>,
    ) -> Result<PagedResult<Vec<PublicTenant>>, FriendlyError> {
        match repo
            .get_all_by_user_id(claims.sub(), paginator, ordering, filtering)
            .await
        {
            Ok(res) => {
                let mut public_tenants = vec![];
                for tenant in res.data {
                    public_tenants.push(PublicTenant::from(tenant))
                }
                Ok(PagedResult {
                    page: res.page,
                    limit: res.limit,
                    total: res.total,
                    data: public_tenants,
                })
            }
            Err(e) => Err(FriendlyError::Internal(e.to_string()).trace(Level::ERROR)),
        }
    }
    pub async fn activate(
        payload: &TenantActivateRequest,
        claims: &Claims,
        repo: Arc<dyn TenantsRepository + Send + Sync>,
        config: Arc<AppConfig>,
    ) -> Result<String, FriendlyError> {
        match repo
            .get_user_active_tenant_by_id(claims.sub(), payload.new_tenant_id)
            .await
        {
            Ok(user_tenant) => match user_tenant {
                None => Err(FriendlyError::UserFacing(
                    StatusCode::UNAUTHORIZED,
                    "ORGANIZATIONAL_UNITS/HANDLER/ACTIVATE".to_string(),
                    "Hozzáférés megtagadva!".to_string(),
                )),
                Some(user_tenant) => {
                    match claims
                        .clone()
                        .set_active_tenant(Some(user_tenant.tenant_id))
                        .to_token(config.auth().jwt_secret().as_bytes())
                    {
                        Ok(new_claims) => Ok(new_claims),
                        Err(e) => Err(FriendlyError::Internal(e.to_string()).trace(Level::ERROR)),
                    }
                }
            },
            Err(e) => Err(FriendlyError::Internal(e.to_string()).trace(Level::ERROR)),
        }
    }
}
