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

use crate::common::error::{RepositoryError, RepositoryResult};
use crate::common::model::SelectOption;
use crate::tenant::address::dto::user_input::AddressUserInput;
use crate::tenant::address::model::{Address, AddressResolved};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::PgPool;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AddressRepository: Send + Sync {
    async fn get_all_countries_select_list_items(&self) -> RepositoryResult<Vec<SelectOption>>;
    async fn get_resolved_by_id(&self, id: Uuid) -> RepositoryResult<AddressResolved>;
    async fn get_resolved_by_ids(&self, ids: Vec<Uuid>) -> RepositoryResult<Vec<AddressResolved>>;
    async fn insert(&self, address: &AddressUserInput, sub: Uuid) -> RepositoryResult<Address>;
    async fn update(&self, address: &AddressUserInput) -> RepositoryResult<Address>;
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl AddressRepository for PgPool {
    async fn get_all_countries_select_list_items(&self) -> RepositoryResult<Vec<SelectOption>> {
        Ok(sqlx::query_as::<_, SelectOption>(
            r#"SELECT code as value, name as title FROM countries"#,
        )
        .fetch_all(self)
        .await?)
    }
    async fn get_resolved_by_id(&self, id: Uuid) -> RepositoryResult<AddressResolved> {
        Ok(sqlx::query_as::<_, AddressResolved>(
            r#"
                SELECT
                    address.id as id,
                    address.type as type,
                    address.country_code as country_code,
                    countries.name as country,
                    address.postal_code as postal_code,
                    address.settlement as settlement,
                    address.mailbox as mailbox,
                    address.topographic_number as topographic_number,
                    address.name_of_public_space as name_of_public_space,
                    address.type_of_public_space as type_of_public_space,
                    address.house_number as house_number,
                    address.building as building,
                    address.stairway as stairway,
                    address.floor as floor,
                    address.door as door,
                    address.created_by_id as created_by_id,
                    users.last_name || ' ' || users.first_name as created_by,
                    address.created_at as created_at,
                    address.updated_at as updated_at,
                    address.deleted_at as deleted_at
                FROM address
                LEFT JOIN users ON address.created_by_id = users.id
                LEFT JOIN countries ON address.country_code = countries.code
                WHERE address.deleted_at IS NULL
                    AND address.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(self)
        .await?)
    }
    async fn get_resolved_by_ids(&self, ids: Vec<Uuid>) -> RepositoryResult<Vec<AddressResolved>> {
        get_resolved_addresses_by_ids(self, ids).await
    }
    async fn insert(&self, address: &AddressUserInput, sub: Uuid) -> RepositoryResult<Address> {
        insert_address(self, address, sub).await
    }
    async fn update(&self, address: &AddressUserInput) -> RepositoryResult<Address> {
        update_address(self, address).await
    }
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<()> {
        delete_address_by_id(self, id).await
    }
}

pub async fn insert_address<'e, E>(
    executor: E,
    address: &AddressUserInput,
    sub: Uuid,
) -> RepositoryResult<Address>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    Ok(sqlx::query_as::<_, Address>(
        r#"
            INSERT INTO address (
                type,
                country_code,
                postal_code,
                settlement,
                mailbox,
                topographic_number,
                name_of_public_space,
                type_of_public_space,
                house_number,
                building,
                stairway,
                floor,
                door,
                created_by_id
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9,
                $10,
                $11,
                $12,
                $13,
                $14
            )
            RETURNING *
        "#,
    )
    .bind(address.address_type.as_str()?)
    .bind(address.country_code.as_str()?)
    .bind(address.postal_code.as_str()?)
    .bind(address.settlement.as_str()?)
    .bind(address.mailbox.as_str())
    .bind(address.topographic_number.as_str())
    .bind(address.name_of_public_space.as_str())
    .bind(address.type_of_public_space.as_str())
    .bind(address.house_number.as_str())
    .bind(address.building.as_str())
    .bind(address.stairway.as_str())
    .bind(address.floor.as_str())
    .bind(address.door.as_str())
    .bind(sub)
    .fetch_one(executor)
    .await?)
}

pub async fn update_address<'e, E>(
    executor: E,
    address: &AddressUserInput,
) -> RepositoryResult<Address>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let address_id = address
        .id
        .as_uuid()
        .ok_or_else(|| RepositoryError::InvalidInput("address_id".to_string()))?;
    Ok(sqlx::query_as::<_, Address>(
        r#"
            UPDATE address
            SET type = $1,
                country_code = $2,
                postal_code = $3,
                settlement = $4,
                mailbox = $5,
                topographic_number = $6,
                name_of_public_space = $7,
                type_of_public_space = $8,
                house_number = $9,
                building = $10,
                stairway = $11,
                floor = $12,
                door = $13
            WHERE id = $14
                AND deleted_at IS NULL
            RETURNING *
        "#,
    )
    .bind(address.address_type.as_str()?)
    .bind(address.country_code.as_str()?)
    .bind(address.postal_code.as_str()?)
    .bind(address.settlement.as_str()?)
    .bind(address.mailbox.as_str())
    .bind(address.topographic_number.as_str())
    .bind(address.name_of_public_space.as_str())
    .bind(address.type_of_public_space.as_str())
    .bind(address.house_number.as_str())
    .bind(address.building.as_str())
    .bind(address.stairway.as_str())
    .bind(address.floor.as_str())
    .bind(address.door.as_str())
    .bind(address_id)
    .fetch_one(executor)
    .await?)
}

pub async fn delete_address_by_id<'e, E>(executor: E, id: Uuid) -> RepositoryResult<()>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    sqlx::query(
        r#"
        UPDATE address
        SET deleted_at = NOW()
        WHERE id = $1
            AND deleted_at IS NULL
        "#,
    )
    .bind(id)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_resolved_addresses_by_ids<'e, E>(
    executor: E,
    ids: Vec<Uuid>,
) -> RepositoryResult<Vec<AddressResolved>>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    Ok(sqlx::query_as::<_, AddressResolved>(
        r#"
        SELECT
            address.id as id,
            address.type as type,
            address.country_code as country_code,
            countries.name as country,
            address.postal_code as postal_code,
            address.settlement as settlement,
            address.mailbox as mailbox,
            address.topographic_number as topographic_number,
            address.name_of_public_space as name_of_public_space,
            address.type_of_public_space as type_of_public_space,
            address.house_number as house_number,
            address.building as building,
            address.stairway as stairway,
            address.floor as floor,
            address.door as door,
            address.created_by_id as created_by_id,
            users.last_name || ' ' || users.first_name as created_by,
            address.created_at as created_at,
            address.updated_at as updated_at,
            address.deleted_at as deleted_at
        FROM address
        LEFT JOIN users ON address.created_by_id = users.id
        LEFT JOIN countries ON address.country_code = countries.code
        WHERE address.deleted_at IS NULL
            AND address.id = ANY($1)
    "#,
    )
    .bind(ids)
    .fetch_all(executor)
    .await?)
}
