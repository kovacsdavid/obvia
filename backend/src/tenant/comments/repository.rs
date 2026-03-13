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
use crate::manager::app::database::PgPoolManager;
use crate::tenant::comments::dto::CommentUserInput;
use crate::tenant::comments::model::CommentsResolved;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CommentsRepository: Send + Sync {
    async fn post(
        &self,
        payload: &CommentUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<CommentsResolved>)>;
}

#[async_trait]
impl CommentsRepository for PgPoolManager {
    async fn post(
        &self,
        payload: &CommentUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<(PaginatorMeta, Vec<CommentsResolved>)> {
        unimplemented!()
    }
}
