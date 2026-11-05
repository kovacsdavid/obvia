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

use crate::common::dto::PaginatorMeta;
use crate::common::error::{RepositoryError, RepositoryResult};
use crate::common::query_parser::ResourceQuery;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::PgPool;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait HandoverAndAcceptancesRepository: Send + Sync {
    async fn get_full(&self, id: Uuid) -> RepositoryResult<HandoverAndAcceptanceFull>;
    async fn get_paged(
        &self,
        query_params: &ResourceQuery<HandoverAndAcceptanceOrderBy, HandoverAndAcceptanceFilterBy>,
    ) -> RepositoryResult<(PaginatorMeta, Vec<HandoverAndAcceptanceResolved>)>;
    async fn insert(
        &self,
        handover_and_acceptance_user_input: &HandoverAndAcceptanceUserInput,
        sub: Uuid,
    ) -> RepositoryResult<HandoverAndAcceptance>;
    async fn update(
        &self,
        handover_and_acceptance_user_input: &HandoverAndAcceptanceUserInput,
        sub: Uuid,
    ) -> RepositoryResult<HandoverAndAcceptance>;
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl HandoverAndAcceptancesRepository for PgPool {
    async fn get_full(&self, id: Uuid) -> RepositoryResult<HandoverAndAcceptanceFull> {
        todo!();
    }
    async fn get_paged(
        &self,
        query_params: &ResourceQuery<HandoverAndAcceptanceOrderBy, HandoverAndAcceptanceFilterBy>,
    ) -> RepositoryResult<(PaginatorMeta, Vec<HandoverAndAcceptanceResolved>)> {
        todo!();
    }
    async fn insert(
        &self,
        handover_and_acceptance_user_input: &HandoverAndAcceptanceUserInput,
        sub: Uuid,
    ) -> RepositoryResult<HandoverAndAcceptance> {
        todo!();
    }
    async fn update(
        &self,
        handover_and_acceptance_user_input: &HandoverAndAcceptanceUserInput,
        sub: Uuid,
    ) -> RepositoryResult<HandoverAndAcceptance> {
        todo!();
    }
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<()> {
        todo!();
    }
}
