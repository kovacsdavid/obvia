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

use crate::common::config::database_config::BasicDatabaseConfig;
use crate::common::database::{DatabaseMigrator, PoolManager};
use crate::common::dto::PaginatorMeta;
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError, ServiceResult};
use crate::common::utils::generate_string_csprng;
use crate::manager::tenants::TenantsModuleInterface;
use crate::manager::tenants::dto::{CreateTenant, NewTokenResponse, PublicTenant, TenantIdRequest};
use crate::manager::tenants::model::Tenant;
use crate::manager::tenants::types::{TenantFilterBy, TenantOrderBy};
use uuid::Uuid;

pub trait TenantService {
    fn create_managed(
        &self,
        payload: &CreateTenant,
    ) -> impl Future<Output = ServiceResult<Tenant>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<TenantOrderBy, TenantFilterBy>,
    ) -> impl Future<Output = ServiceResult<(PaginatorMeta, Vec<PublicTenant>)>> + Send;
    fn activate(
        &self,
        payload: &TenantIdRequest,
    ) -> impl Future<Output = ServiceResult<NewTokenResponse>> + Send;
    fn delete(&self, uuid: Uuid) -> impl Future<Output = ServiceResult<NewTokenResponse>> + Send;
}

impl<'a, T> TenantService for Service<'a, T>
where
    T: TenantsModuleInterface,
{
    async fn create_managed(&self, payload: &CreateTenant) -> ServiceResult<Tenant> {
        let config = self.module().config();
        let uuid = Uuid::new_v4();
        let db_config = BasicDatabaseConfig {
            host: config.main_database().host.clone(),
            port: config.main_database().port,
            username: format!("tenant_{}", uuid.to_string().replace("-", "")),
            password: generate_string_csprng(40).map_err(|_| ServiceError::RngError)?,
            database: format!("tenant_{}", uuid.to_string().replace("-", "")),
            max_pool_size: None,
            ssl_mode: Some(String::from("disable")),
        };

        let tenant = self
            .module()
            .tenants_repo()
            .setup_managed(
                uuid,
                payload.name.as_str()?,
                &db_config,
                self.claims()?,
                config,
            )
            .await?;

        PoolManager::add_tenant_pool(
            self.module(),
            tenant.id,
            &BasicDatabaseConfig::try_from(&tenant).map_err(ServiceError::Config)?,
        )
        .await?;

        DatabaseMigrator::migrate_tenant_db(self.module(), tenant.id).await?;

        let manager_user = self
            .module()
            .manager_user_repo()
            .get_by_uuid(self.claims()?.sub())
            .await?;

        self.module()
            .tenant_user_repo(tenant.id)?
            .insert_from_manager(manager_user.into())
            .await?;

        Ok(tenant)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<TenantOrderBy, TenantFilterBy>,
    ) -> ServiceResult<(PaginatorMeta, Vec<PublicTenant>)> {
        let (meta, data) = self
            .module()
            .tenants_repo()
            .get_all_by_user_id(self.claims()?.sub(), get_query)
            .await?;
        let mut public_tenants = vec![];
        for tenant in data {
            public_tenants.push(PublicTenant::from(tenant))
        }
        Ok((meta, public_tenants))
    }

    async fn activate(&self, payload: &TenantIdRequest) -> ServiceResult<NewTokenResponse> {
        let user_tenant = self
            .module()
            .tenants_repo()
            .get_user_active_tenant_by_id(self.claims()?.sub(), payload.uuid)
            .await?
            .ok_or(ServiceError::Unauthorized)?;
        let claims = self
            .claims()?
            .clone()
            .set_active_tenant(Some(user_tenant.tenant_id));

        Ok(NewTokenResponse {
            token: claims
                .to_token(self.module().config().auth().jwt_secret().as_bytes())
                .map_err(ServiceError::Token)?,
            claims,
        })
    }

    async fn delete(&self, uuid: Uuid) -> ServiceResult<NewTokenResponse> {
        let claims = self.claims()?.clone();
        self.module()
            .tenants_repo()
            .delete(uuid, claims.sub())
            .await?;
        let claims = if let Some(active_tenant) = claims.active_tenant()
            && active_tenant == uuid
        {
            claims.set_active_tenant(None)
        } else {
            claims
        };

        self.module().delete_tenant_pool(uuid).await?;

        Ok(NewTokenResponse {
            token: claims
                .to_token(self.module().config().auth().jwt_secret().as_bytes())
                .map_err(ServiceError::Token)?,
            claims,
        })
    }
}
