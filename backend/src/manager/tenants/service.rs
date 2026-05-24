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

use crate::common::MailTransporter;
use crate::common::config::{AppConfig, database_config::BasicDatabaseConfig};
use crate::common::dto::{GeneralError, PaginatorMeta};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::query_parser::ResourceQuery;
use crate::common::utils::generate_string_csprng;
use crate::common::value_object::ValueObjectError;
use crate::manager::auth::dto::claims::Claims;
use crate::manager::tenants::TenantsModule;
use crate::manager::tenants::dto::{CreateTenant, NewTokenResponse, PublicTenant, TenantIdRequest};
use crate::manager::tenants::model::Tenant;
use crate::manager::tenants::repository::TenantsRepository;
use crate::manager::tenants::types::{TenantFilterBy, TenantOrderBy};
use async_trait::async_trait;
use axum::http::StatusCode;
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

    #[error("Hozzáférés megtagadva!")]
    AccessDenied,

    #[error("Token error: {0}")]
    Token(String),

    #[error("Jelenleg nem elérhető")]
    CurrentlyNotAvailable,

    #[error("rng error")]
    RngError,

    #[error("ValueObjectError {0}")]
    ValueObjectError(#[from] ValueObjectError),
}

#[async_trait]
impl IntoFriendlyError<GeneralError> for TenantsServiceError {
    async fn into_friendly_error(
        self,
        module: Arc<dyn MailTransporter>,
    ) -> FriendlyError<GeneralError> {
        match self {
            TenantsServiceError::AccessDenied | TenantsServiceError::CurrentlyNotAvailable => {
                FriendlyError::user_facing(
                    Level::DEBUG,
                    StatusCode::UNAUTHORIZED,
                    file!(),
                    GeneralError {
                        message: TenantsServiceError::AccessDenied.to_string(),
                    },
                )
            }
            e => {
                FriendlyError::internal_with_admin_notify(
                    file!(),
                    GeneralError {
                        message: e.to_string(),
                    },
                    module,
                )
                .await
            }
        }
    }
}

pub async fn create_managed(
    claims: &Claims,
    payload: &CreateTenant,
    tenants_module: Arc<dyn TenantsModule>,
) -> Result<Tenant, TenantsServiceError> {
    let uuid = Uuid::new_v4();
    let db_config = BasicDatabaseConfig {
        host: tenants_module.config().main_database().host.clone(),
        port: tenants_module.config().main_database().port,
        username: format!("tenant_{}", uuid.to_string().replace("-", "")),
        password: generate_string_csprng(40).map_err(|_| TenantsServiceError::RngError)?,
        database: format!("tenant_{}", uuid.to_string().replace("-", "")),
        max_pool_size: None,
        ssl_mode: Some(String::from("disable")),
    };

    let tenant = tenants_module
        .tenants_repo()
        .setup_managed(
            uuid,
            payload.name.as_str()?,
            &db_config,
            claims,
            tenants_module.config().clone(),
        )
        .await?;

    tenants_module
        .add_tenant_pool(
            tenant.id,
            &BasicDatabaseConfig::try_from(&tenant).map_err(TenantsServiceError::Config)?,
        )
        .await?;

    tenants_module
        .migrator()
        .migrate_tenant_db(tenant.id)
        .await?;

    let manager_user = tenants_module
        .manager_user_repo()
        .get_by_uuid(claims.sub())
        .await?;

    tenants_module
        .tenant_user_repo()
        .insert_from_manager(manager_user.into(), tenant.id)
        .await?;

    Ok(tenant)
}

pub async fn get_paged_list(
    get_query: &ResourceQuery<TenantOrderBy, TenantFilterBy>,
    claims: &Claims,
    repo: Arc<dyn TenantsRepository>,
) -> Result<(PaginatorMeta, Vec<PublicTenant>), TenantsServiceError> {
    let (meta, data) = repo.get_all_by_user_id(claims.sub(), get_query).await?;
    let mut public_tenants = vec![];
    for tenant in data {
        public_tenants.push(PublicTenant::from(tenant))
    }
    Ok((meta, public_tenants))
}

pub async fn activate(
    payload: &TenantIdRequest,
    claims: &Claims,
    repo: Arc<dyn TenantsRepository>,
    config: Arc<AppConfig>,
) -> Result<NewTokenResponse, TenantsServiceError> {
    let user_tenant = repo
        .get_user_active_tenant_by_id(claims.sub(), payload.uuid)
        .await?
        .ok_or(TenantsServiceError::AccessDenied)?;
    let claims = claims
        .clone()
        .set_active_tenant(Some(user_tenant.tenant_id));

    Ok(NewTokenResponse {
        token: claims
            .to_token(config.auth().jwt_secret().as_bytes())
            .map_err(TenantsServiceError::Token)?,
        claims,
    })
}

pub async fn delete(
    uuid: Uuid,
    claims: Claims,
    repo: Arc<dyn TenantsRepository>,
    config: Arc<AppConfig>,
) -> Result<NewTokenResponse, TenantsServiceError> {
    repo.delete(uuid, claims.sub()).await?;
    let claims = if let Some(active_tenant) = claims.active_tenant()
        && active_tenant == uuid
    {
        claims.set_active_tenant(None)
    } else {
        claims
    };
    Ok(NewTokenResponse {
        token: claims
            .to_token(config.auth().jwt_secret().as_bytes())
            .map_err(TenantsServiceError::Token)?,
        claims,
    })
}
