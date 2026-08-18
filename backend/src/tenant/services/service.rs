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

use crate::common::dto::PaginatorMeta;
use crate::common::model::SelectOption;
#[double]
use crate::common::pdf::PdfGenerator;
use crate::common::pdf::PdfTemplates;
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError, ServiceResult};
use crate::tenant::services::ServicesModule;
use crate::tenant::services::dto::print::ServicesResolvedPrint;
use crate::tenant::services::dto::user_input::ServiceUserInput;
use crate::tenant::services::model::{Service as ServiceModel, ServiceResolved};
use crate::tenant::services::types::service::{ServiceFilterBy, ServiceOrderBy};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use mockall_double::double;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

pub enum ServicesSelectLists {
    Currencies,
    Taxes,
}

impl FromStr for ServicesSelectLists {
    type Err = ServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "currencies" => Ok(Self::Currencies),
            "taxes" => Ok(Self::Taxes),
            _ => Err(ServiceError::InvalidSelectList),
        }
    }
}

pub trait ServiceService {
    fn insert(
        &self,
        payload: &ServiceUserInput,
    ) -> impl Future<Output = ServiceResult<ServiceModel>> + Send;
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = ServiceResult<Vec<SelectOption>>> + Send;
    fn get_resolved(
        &self,
        payload: Uuid,
    ) -> impl Future<Output = ServiceResult<ServiceResolved>> + Send;
    fn get(&self, payload: Uuid) -> impl Future<Output = ServiceResult<ServiceModel>> + Send;
    fn update(
        &self,
        payload: &ServiceUserInput,
    ) -> impl Future<Output = ServiceResult<ServiceModel>> + Send;
    fn delete(&self, payload: Uuid) -> impl Future<Output = ServiceResult<()>> + Send;
    fn get_paged(
        &self,
        get_query: &ResourceQuery<ServiceOrderBy, ServiceFilterBy>,
    ) -> impl Future<Output = ServiceResult<(PaginatorMeta, Vec<ServiceResolved>)>> + Send;
    fn print(
        &self,
        payload: &[ServicesResolvedPrint],
    ) -> impl Future<Output = ServiceResult<Vec<u8>>> + Send;
    fn print_snapshot(&self, path: &Path) -> impl Future<Output = ServiceResult<()>> + Sync;
}

impl<'a, T> ServiceService for Service<'a, T>
where
    T: ServicesModule,
{
    async fn insert(&self, payload: &ServiceUserInput) -> ServiceResult<ServiceModel> {
        self.module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .insert(payload, self.claims()?.sub())
            .await
            .map_err(|e| {
                if e.is_unique_violation() {
                    ServiceError::Conflict(
                        "A megadott névvel már létezik szolgáltatás a rendszerben!",
                    )
                } else {
                    e.into()
                }
            })
    }

    async fn get_resolved(&self, payload: Uuid) -> ServiceResult<ServiceResolved> {
        Ok(self
            .module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .get_resolved_by_id(payload)
            .await?)
    }

    async fn get(&self, payload: Uuid) -> ServiceResult<ServiceModel> {
        Ok(self
            .module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .get_by_id(payload)
            .await?)
    }

    async fn update(&self, payload: &ServiceUserInput) -> ServiceResult<ServiceModel> {
        if !payload.id.is_present() {
            return Err(ServiceError::UnprocessableEntry(
                "Az azonosító megadása kötelező!",
            ));
        }
        Ok(self
            .module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .update(payload)
            .await?)
    }
    async fn delete(&self, payload: Uuid) -> ServiceResult<()> {
        Ok(self
            .module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .delete_by_id(payload)
            .await?)
    }

    async fn get_paged(
        &self,
        get_query: &ResourceQuery<ServiceOrderBy, ServiceFilterBy>,
    ) -> ServiceResult<(PaginatorMeta, Vec<ServiceResolved>)> {
        Ok(self
            .module()
            .services_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .get_paged(get_query)
            .await?)
    }

    async fn get_select_list_items(&self, select_list: &str) -> ServiceResult<Vec<SelectOption>> {
        let active_tenant = self
            .claims()?
            .active_tenant()
            .ok_or(ServiceError::Unauthorized)?;
        match ServicesSelectLists::from_str(select_list)? {
            ServicesSelectLists::Currencies => Ok(self
                .module()
                .currencies_repo(active_tenant)?
                .get_all_countries_select_list_items()
                .await?),
            ServicesSelectLists::Taxes => Ok(self
                .module()
                .taxes_repo(active_tenant)?
                .get_select_list_items()
                .await?),
        }
    }

    async fn print(&self, payload: &[ServicesResolvedPrint]) -> ServiceResult<Vec<u8>> {
        Ok(PdfGenerator::gen_pdf_temporary(
            &PdfTemplates::ServiceView,
            payload.to_vec(),
        )?)
    }
    async fn print_snapshot(&self, path: &Path) -> ServiceResult<()> {
        let test_time: DateTime<Utc> = "2026-01-02T11:11:11Z"
            .parse()
            .map_err(|e: chrono::ParseError| ServiceError::ParseError(e.to_string()))?;
        let tz: Tz = "Europe/Budapest"
            .parse()
            .map_err(|e: chrono_tz::ParseError| ServiceError::ParseError(e.to_string()))?;
        let service_id = "4f321721-37c6-4e91-8e42-6281c36937bc"
            .parse()
            .map_err(|e: uuid::Error| ServiceError::ParseError(e.to_string()))?;
        let created_by_id = "97054cdb-781c-4f40-a489-b43373d75bf0"
            .parse()
            .map_err(|e: uuid::Error| ServiceError::ParseError(e.to_string()))?;
        let service_resolved = ServiceResolved {
            id: service_id,
            name: "Test Service".to_string(),
            description: Some("Test description".to_string()),
            default_price: None,
            default_tax_id: None,
            default_tax: None,
            currency_code: Some("HUF".to_string()),
            status: "active".to_string(),
            created_by_id,
            created_by: "Test User".to_string(),
            created_at: test_time,
            updated_at: test_time,
            deleted_at: None,
        };
        let service_resolved_print =
            ServicesResolvedPrint::from_service_resolved(service_resolved, tz);
        let pdf = self.print(&[service_resolved_print]).await?;
        let mut file = File::create(path)?;
        file.write_all(&pdf)?;
        Ok(())
    }
}
