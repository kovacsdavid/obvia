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

use crate::common::error::RepositoryError;
use crate::manager::common::repository::PoolManagerWrapper;
use crate::manager::common::types::value_object::ValueObjectable;
use crate::tenant::projects::dto::CreateProject;
use crate::tenant::projects::model::Project;
use async_trait::async_trait;
use chrono::NaiveDateTime;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProjectsRepository: Send + Sync {
    async fn get_all(&self, active_tenant: Uuid) -> Result<Vec<Project>, RepositoryError>;
    async fn insert(
        &self,
        project: CreateProject,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Project, RepositoryError>;
}

#[async_trait]
impl ProjectsRepository for PoolManagerWrapper {
    async fn get_all(&self, active_tenant: Uuid) -> Result<Vec<Project>, RepositoryError> {
        Ok(sqlx::query_as::<_, Project>(
            "SELECT * FROM projects WHERE deleted_at IS NULL ORDER BY name",
        )
        .fetch_all(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }

    async fn insert(
        &self,
        project: CreateProject,
        sub: Uuid,
        active_tenant: Uuid,
    ) -> Result<Project, RepositoryError> {
        let start_date = match project.start_date {
            None => None,
            Some(v) => {
                Some(NaiveDateTime::parse_from_str(
                    &v.extract().get_value(),
                    "%Y-%m-%d %H:%M:%S",
                )
                    .map_err(|e| RepositoryError::Parse(e.to_string()))?)
            }
        };
        let end_date = match project.end_date {
            None => None,
            Some(v) => {
                Some(NaiveDateTime::parse_from_str(
                    &v.extract().get_value(),
                    "%Y-%m-%d %H:%M:%S",
                )
                    .map_err(|e| RepositoryError::Parse(e.to_string()))?)
            }
        };

        Ok(sqlx::query_as::<_, Project>(
            "INSERT INTO projects (name, description, created_by, status, start_date, end_date)
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(project.name.extract().get_value())
        .bind(project.description.map(|v| v.extract().get_value().clone()))
        .bind(sub)
        .bind(project.status.extract().get_value())
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&self.pool_manager.get_tenant_pool(active_tenant)?)
        .await?)
    }
}
