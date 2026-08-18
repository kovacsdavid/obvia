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
use crate::common::query_parser::ResourceQuery;
use crate::common::service::{Service, ServiceError, ServiceResult};
use crate::common::types::Empty;
use crate::common::value_object::ValueObjectRequired;
use crate::tenant::activity_feed::ActivityFeedModuleInterface;
use crate::tenant::activity_feed::model::ActivityFeedResolved;
use crate::tenant::activity_feed::types::ResourceType;
use uuid::Uuid;

pub trait ActivityFeedService {
    fn get_all_paged(
        &self,
        get_query: &ResourceQuery<Empty, Empty>,
        resource_id: Uuid,
        resource_type: &ValueObjectRequired<ResourceType>,
    ) -> impl Future<Output = ServiceResult<(PaginatorMeta, Vec<ActivityFeedResolved>)>> + Send;
}

impl<'a, T> ActivityFeedService for Service<'a, T>
where
    T: ActivityFeedModuleInterface,
{
    async fn get_all_paged(
        &self,
        get_query: &ResourceQuery<Empty, Empty>,
        resource_id: Uuid,
        resource_type: &ValueObjectRequired<ResourceType>,
    ) -> ServiceResult<(PaginatorMeta, Vec<ActivityFeedResolved>)> {
        Ok(self
            .module()
            .activity_feed_repo(
                self.claims()?
                    .active_tenant()
                    .ok_or(ServiceError::Unauthorized)?,
            )?
            .get_paged(get_query, resource_id, resource_type)
            .await?)
    }
}
