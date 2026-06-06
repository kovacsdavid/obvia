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

use crate::common::BaseModule;
use crate::common::config::database_config::BasicDatabaseConfig;
use crate::common::database::{DatabaseMigrator, PoolManager};
use crate::common::dto::{GeneralError, PaginatorMeta};
use crate::common::error::{FriendlyError, IntoFriendlyError, RepositoryError};
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError};
use crate::common::utils::generate_string_csprng;
use crate::common::value_object::ValueObjectError;
use crate::manager::tenants::TenantsModule;
use crate::manager::tenants::dto::{CreateTenant, NewTokenResponse, PublicTenant, TenantIdRequest};
use crate::manager::tenants::model::Tenant;
use crate::manager::tenants::repository::TenantsRepository;
use crate::manager::tenants::types::{TenantFilterBy, TenantOrderBy};
use crate::manager::users::repository::UsersRepository as ManagerUserRepository;
use crate::tenant::users::repository::UsersRepository as TenantUserRepository;
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
    Unauthorized,

    #[error("Token error: {0}")]
    Token(String),

    #[error("rng error")]
    RngError,

    #[error("ValueObjectError {0}")]
    ValueObjectError(#[from] ValueObjectError),
}

impl From<ServiceError> for TenantsServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => TenantsServiceError::Unauthorized,
        }
    }
}

impl IntoFriendlyError for TenantsServiceError {
    async fn into_friendly_error<M>(self, mailer: Arc<M>) -> FriendlyError
    where
        M: BaseModule,
    {
        match self {
            TenantsServiceError::Unauthorized => FriendlyError::user_facing(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                file!(),
                GeneralError {
                    message: TenantsServiceError::Unauthorized.to_string(),
                }
                .to_string(),
            ),
            e => {
                FriendlyError::internal_with_admin_notify(
                    file!(),
                    GeneralError {
                        message: e.to_string(),
                    }
                    .to_string(),
                    mailer,
                )
                .await
            }
        }
    }
}

type TenantsServiceResult<T> = Result<T, TenantsServiceError>;

pub trait TenantService {
    fn create_managed(
        &self,
        payload: &CreateTenant,
    ) -> impl Future<Output = TenantsServiceResult<Tenant>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<TenantOrderBy, TenantFilterBy>,
    ) -> impl Future<Output = TenantsServiceResult<(PaginatorMeta, Vec<PublicTenant>)>> + Send;
    fn activate(
        &self,
        payload: &TenantIdRequest,
    ) -> impl Future<Output = TenantsServiceResult<NewTokenResponse>> + Send;
    fn delete(
        &self,
        uuid: Uuid,
    ) -> impl Future<Output = TenantsServiceResult<NewTokenResponse>> + Send;
}

impl<'a, T> TenantService for Service<'a, T>
where
    T: TenantsModule,
{
    async fn create_managed(&self, payload: &CreateTenant) -> TenantsServiceResult<Tenant> {
        let uuid = Uuid::new_v4();
        let db_config = BasicDatabaseConfig {
            host: self.module().config().main_database().host.clone(),
            port: self.module().config().main_database().port,
            username: format!("tenant_{}", uuid.to_string().replace("-", "")),
            password: generate_string_csprng(40).map_err(|_| TenantsServiceError::RngError)?,
            database: format!("tenant_{}", uuid.to_string().replace("-", "")),
            max_pool_size: None,
            ssl_mode: Some(String::from("disable")),
        };

        let tenant = TenantsRepository::setup_managed(
            self.module(),
            uuid,
            payload.name.as_str()?,
            &db_config,
            self.claims()?,
            self.module().config(),
        )
        .await?;

        PoolManager::add_tenant_pool(
            self.module(),
            tenant.id,
            &BasicDatabaseConfig::try_from(&tenant).map_err(TenantsServiceError::Config)?,
        )
        .await?;

        DatabaseMigrator::migrate_tenant_db(self.module(), tenant.id).await?;

        let manager_user =
            ManagerUserRepository::get_by_uuid(self.module(), self.claims()?.sub()).await?;

        TenantUserRepository::insert_from_manager(self.module(), manager_user.into(), tenant.id)
            .await?;

        Ok(tenant)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<TenantOrderBy, TenantFilterBy>,
    ) -> TenantsServiceResult<(PaginatorMeta, Vec<PublicTenant>)> {
        let (meta, data) =
            TenantsRepository::get_all_by_user_id(self.module(), self.claims()?.sub(), get_query)
                .await?;
        let mut public_tenants = vec![];
        for tenant in data {
            public_tenants.push(PublicTenant::from(tenant))
        }
        Ok((meta, public_tenants))
    }

    async fn activate(&self, payload: &TenantIdRequest) -> TenantsServiceResult<NewTokenResponse> {
        let user_tenant = TenantsRepository::get_user_active_tenant_by_id(
            self.module(),
            self.claims()?.sub(),
            payload.uuid,
        )
        .await?
        .ok_or(TenantsServiceError::Unauthorized)?;
        let claims = self
            .claims()?
            .clone()
            .set_active_tenant(Some(user_tenant.tenant_id));

        Ok(NewTokenResponse {
            token: claims
                .to_token(self.module().config().auth().jwt_secret().as_bytes())
                .map_err(TenantsServiceError::Token)?,
            claims,
        })
    }

    async fn delete(&self, uuid: Uuid) -> TenantsServiceResult<NewTokenResponse> {
        let claims = self.claims()?.clone();
        TenantsRepository::delete(self.module(), uuid, claims.sub()).await?;
        let claims = if let Some(active_tenant) = claims.active_tenant()
            && active_tenant == uuid
        {
            claims.set_active_tenant(None)
        } else {
            claims
        };
        Ok(NewTokenResponse {
            token: claims
                .to_token(self.module().config().auth().jwt_secret().as_bytes())
                .map_err(TenantsServiceError::Token)?,
            claims,
        })
    }
}
