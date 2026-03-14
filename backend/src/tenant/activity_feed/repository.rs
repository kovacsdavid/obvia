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
use chrono::Local;
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
        Ok((
            PaginatorMeta {
                page: 1,
                limit: 25,
                total: 100,
            },
            vec![
                ActivityFeedResolved {
                    id: Uuid::new_v4(),
                    activity_type: String::from("comment"),
                    description: String::from("description1"),
                    created_at: Local::now(),
                    created_by_id: Uuid::new_v4(),
                    created_by: String::from("Kovács Dávid <kapcsolat@kovacsdavid.dev>"),
                },
                ActivityFeedResolved {
                    id: Uuid::new_v4(),
                    activity_type: String::from("activity"),
                    description: String::from("description2"),
                    created_at: Local::now(),
                    created_by_id: Uuid::new_v4(),
                    created_by: String::from("Kovács Dávid <kapcsolat@kovacsdavid.dev>"),
                },
                ActivityFeedResolved {
                    id: Uuid::new_v4(),
                    activity_type: String::from("activity"),
                    description: String::from("description3"),
                    created_at: Local::now(),
                    created_by_id: Uuid::new_v4(),
                    created_by: String::from("Kovács Dávid <kapcsolat@kovacsdavid.dev>"),
                },
                ActivityFeedResolved {
                    id: Uuid::new_v4(),
                    activity_type: String::from("comment"),
                    description: String::from(
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
                    created_at: Local::now(),
                    created_by_id: Uuid::new_v4(),
                    created_by: String::from("Kovács Dávid <kapcsolat@kovacsdavid.dev>"),
                },
            ],
        ))
    }
}
