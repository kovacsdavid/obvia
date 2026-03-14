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

use crate::common::error::RepositoryResult;
use crate::manager::app::database::PgPoolManager;
use crate::tenant::comments::dto::CommentUserInput;
use crate::tenant::comments::model::CommentsResolved;
use async_trait::async_trait;
use chrono::Local;
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
    ) -> RepositoryResult<CommentsResolved>;
}

#[async_trait]
impl CommentsRepository for PgPoolManager {
    async fn post(
        &self,
        payload: &CommentUserInput,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> RepositoryResult<CommentsResolved> {
        Ok(CommentsResolved {
            id: Uuid::new_v4(),
            commentable_type: String::from("customer"),
            commentable_id: Uuid::new_v4(),
            comment: String::from(
                r#"
                          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
                          eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
                          enim ad minim veniam, quis nostrud exercitation ullamco laboris
                          nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
                          reprehenderit in voluptate velit esse cillum dolore eu fugiat
                          nulla pariatur. Excepteur sint occaecat cupidatat non proident,
                          sunt in culpa qui officia deserunt mollit anim id est laborum.
                        "#,
            ),
            created_by_id: Uuid::new_v4(),
            created_by: String::from("Kovács Dávid <kapcsolat@kovacsdavid.dev"),
            created_at: Local::now(),
            updated_at: Local::now(),
            deleted_at: None,
        })
    }
}
