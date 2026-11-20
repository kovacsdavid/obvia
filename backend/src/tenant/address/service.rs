/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

use std::str::FromStr;

use crate::{
    common::{
        error::{
            RepositoryError,
            v2::{AppError, AppErrorVisibility},
        },
        model::SelectOption,
        service::{Service, ServiceError},
    },
    tenant::address::AddressModuleInterface,
};
use axum::http::StatusCode;
use serde_json::json;
use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum AddressServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Hozzáférés megtagadva!")]
    Unauthorized,

    #[error("A lista nem létezik")]
    InvalidSelectList,
}

impl From<ServiceError> for AddressServiceError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Unauthorized => AddressServiceError::Unauthorized,
        }
    }
}

impl From<AddressServiceError> for AppError {
    fn from(value: AddressServiceError) -> Self {
        match value {
            AddressServiceError::Unauthorized => Self::new(
                Level::DEBUG,
                StatusCode::UNAUTHORIZED,
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            AddressServiceError::InvalidSelectList => Self::new(
                Level::DEBUG,
                StatusCode::UNPROCESSABLE_ENTITY,
                AppErrorVisibility::UserFacing,
                json!({"message": value.to_string()}),
            ),
            _ => Self::new(
                Level::ERROR,
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorVisibility::Internal,
                json!({"message": value.to_string()}),
            ),
        }
    }
}

pub enum AddressSelectLists {
    Countries,
}

impl FromStr for AddressSelectLists {
    type Err = AddressServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "countries" => Ok(Self::Countries),
            _ => Err(AddressServiceError::InvalidSelectList),
        }
    }
}

type AddressServiceResult<T> = Result<T, AddressServiceError>;

pub trait AddressService {
    fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> impl Future<Output = AddressServiceResult<Vec<SelectOption>>> + Send;
}

impl<'a, T> AddressService for Service<'a, T>
where
    T: AddressModuleInterface,
{
    async fn get_select_list_items(
        &self,
        select_list: &str,
    ) -> AddressServiceResult<Vec<SelectOption>> {
        match AddressSelectLists::from_str(select_list)? {
            AddressSelectLists::Countries => Ok(self
                .module()
                .address_repo(self.active_tenant()?)?
                .get_all_countries_select_list_items()
                .await?),
        }
    }
}
