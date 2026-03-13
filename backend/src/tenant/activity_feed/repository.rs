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
use crate::common::error::RepositoryResult;
use crate::common::query_parser::GetQuery;
use crate::common::types::{EmptyFilterBy, EmptyOrderBy};
use crate::manager::app::database::PgPoolManager;
use crate::tenant::activity_feed::model::ActivityFeedResolved;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ActivityFeedRepository: Send + Sync {
    async fn get_all_paged(
        &self,
        query_params: &GetQuery<EmptyOrderBy, EmptyFilterBy>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ActivityFeedResolved>)>;
}

#[async_trait]
impl ActivityFeedRepository for PgPoolManager {
    async fn get_all_paged(
        &self,
        query_params: &GetQuery<EmptyOrderBy, EmptyFilterBy>,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<ActivityFeedResolved>)> {
        unimplemented!()
    }
}
