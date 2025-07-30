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
use crate::app::config::AppConfig;
use crate::auth::dto::claims::Claims;
use crate::common::error::DatabaseError;
use crate::common::repository::PoolWrapper;
use crate::common::services::generate_string_csprng;
use crate::common::types::DdlParameter;
use crate::organizational_units::dto::CreateRequest;
use crate::organizational_units::model::{OrganizationalUnit, UserOrganizationalUnit};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait OrganizationalUnitsRepository: Send + Sync + 'static {
    #[allow(dead_code)]
    async fn get_by_uuid(&mut self, uuid: &str) -> Result<OrganizationalUnit, DatabaseError>;
    async fn insert_and_connect(
        &mut self,
        payload: CreateRequest,
        claims: Claims,
        app_config: Arc<AppConfig>,
    ) -> Result<(), DatabaseError>;
    #[allow(dead_code)]
    async fn get_all_by_user_uuid(
        &mut self,
        user_uuid: &str,
    ) -> Result<Vec<OrganizationalUnit>, DatabaseError>;
}

#[async_trait]
impl OrganizationalUnitsRepository for PoolWrapper {
    async fn get_by_uuid(&mut self, uuid: &str) -> Result<OrganizationalUnit, DatabaseError> {
        Ok(sqlx::query_as::<_, OrganizationalUnit>(
            "SELECT * FROM organizational_units WHERE uuid = $1",
        )
        .bind(uuid)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?)
    }
    async fn insert_and_connect(
        &mut self,
        payload: CreateRequest,
        claims: Claims,
        app_config: Arc<AppConfig>,
    ) -> Result<(), DatabaseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let organizational_unit_id = Uuid::new_v4();
        let db_password = payload
            .db_password
            .unwrap_or_else(|| generate_string_csprng(40));
        let _organizational_unit = sqlx::query_as::<_, OrganizationalUnit>(
            "INSERT INTO organizational_units (
            id, name, db_host, db_port, db_name, db_user, db_password
            ) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        )
        .bind(organizational_unit_id)
        .bind(payload.name)
        .bind(
            payload
                .db_host
                .unwrap_or(app_config.tenant_database().host.clone()),
        )
        .bind(
            payload
                .db_port
                .unwrap_or(app_config.tenant_database().port as i32),
        )
        .bind(
            payload
                .db_name
                .unwrap_or(organizational_unit_id.to_string()),
        )
        .bind(
            payload
                .db_user
                .unwrap_or(organizational_unit_id.to_string()),
        )
        .bind(db_password.clone())
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let _connect = sqlx::query_as::<_, UserOrganizationalUnit>(
            "INSERT INTO user_organizational_units (
            user_id, organizational_unit_id, role 
            ) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(
            Uuid::parse_str(claims.sub())
                .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
        )
        .bind(organizational_unit_id)
        .bind("owner")
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let create_user_sql = format!(
            "CREATE USER tenant_{} WITH PASSWORD '{}'",
            DdlParameter::from_str(&organizational_unit_id.to_string().replace("-", ""))
                .map_err(DatabaseError::DatabaseError)?,
            DdlParameter::from_str(&db_password).map_err(DatabaseError::DatabaseError)?
        );

        let _create_user = sqlx::query(&create_user_sql)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let grant_sql = format!(
            "GRANT tenant_{} to {};",
            DdlParameter::from_str(&organizational_unit_id.to_string().replace("-", ""))
                .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
            app_config.tenant_database().username // safety: not user input
        );

        let _grant = sqlx::query(&grant_sql)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        let create_db_sql = format!(
            "CREATE DATABASE tenant_{} WITH OWNER = 'tenant_{}'",
            DdlParameter::from_str(&organizational_unit_id.to_string().replace("-", ""))
                .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
            DdlParameter::from_str(&organizational_unit_id.to_string().replace("-", ""))
                .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?,
        );

        let _create_db = sqlx::query(&create_db_sql)
            .bind(organizational_unit_id)
            .bind(organizational_unit_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_all_by_user_uuid(
        &mut self,
        user_uuid: &str,
    ) -> Result<Vec<OrganizationalUnit>, DatabaseError> {
        sqlx::query_as::<_, OrganizationalUnit>(
            "SELECT * FROM organizational_units WHERE user_uuid = $1",
        )
        .bind(user_uuid)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DatabaseError::DatabaseError(e.to_string()))
    }
}
