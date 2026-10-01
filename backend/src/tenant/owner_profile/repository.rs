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

use std::collections::HashMap;

use crate::common::error::{RepositoryError, RepositoryResult};
use crate::tenant::address::repository::{
    delete_address_by_id, get_resolved_addresses_by_ids, insert_address, update_address,
};
use crate::tenant::owner_profile::dto::user_input::OwnerProfileUserInput;
use crate::tenant::owner_profile::model::{OwnerProfile, OwnerProfileFull, OwnerProfileResolved};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::{AssertSqlSafe, PgPool};
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait OwnerProfileRepository: Send + Sync {
    async fn get_resolved(&self) -> RepositoryResult<OwnerProfileResolved>;
    async fn get_full(&self) -> RepositoryResult<OwnerProfileFull>;
    async fn update(
        &self,
        owner_profile_user_input: &OwnerProfileUserInput,
        sub: Uuid,
    ) -> RepositoryResult<OwnerProfile>;
}

#[async_trait]
impl OwnerProfileRepository for PgPool {
    async fn get_resolved(&self) -> RepositoryResult<OwnerProfileResolved> {
        get_resolved_owner_profile(self, false).await
    }
    async fn get_full(&self) -> RepositoryResult<OwnerProfileFull> {
        let owner_profile_resolved = self.get_resolved().await?;

        let mut address_ids = vec![];
        if let Some(billing_address) = owner_profile_resolved.billing_address {
            address_ids.push(billing_address);
        }
        if let Some(mailing_address) = owner_profile_resolved.mailing_address {
            address_ids.push(mailing_address);
        }

        let mut addresses = if !address_ids.is_empty() {
            let mut map = HashMap::new();
            for address in get_resolved_addresses_by_ids(self, address_ids).await? {
                map.insert(address.id, address);
            }
            map
        } else {
            HashMap::new()
        };

        Ok(
            match (
                owner_profile_resolved.billing_address,
                owner_profile_resolved.mailing_address,
            ) {
                (None, None) => OwnerProfileFull::from((owner_profile_resolved, None, None)),
                (None, Some(mailing_address)) => OwnerProfileFull::from((
                    owner_profile_resolved,
                    None,
                    addresses.remove(&mailing_address),
                )),
                (Some(billing_address), None) => OwnerProfileFull::from((
                    owner_profile_resolved,
                    addresses.remove(&billing_address),
                    None,
                )),
                (Some(billing_address), Some(mailing_address)) => OwnerProfileFull::from((
                    owner_profile_resolved,
                    addresses.remove(&billing_address),
                    addresses.remove(&mailing_address),
                )),
            },
        )
    }
    async fn update(
        &self,
        owner_profile_user_input: &OwnerProfileUserInput,
        sub: Uuid,
    ) -> RepositoryResult<OwnerProfile> {
        let contact_name = match &owner_profile_user_input.contact_name {
            Some(v) => Some(v.as_str()?),
            None => None,
        };
        let id = owner_profile_user_input
            .id
            .as_uuid()
            .ok_or_else(|| RepositoryError::InvalidInput("id".to_string()))?;

        let mut tx = self.begin().await?;

        let owner_profile_resolved = get_resolved_owner_profile(&mut *tx, true).await?;

        let billing_address = match (
            owner_profile_resolved.billing_address,
            &owner_profile_user_input.billing_address,
        ) {
            (None, None) => None,
            (None, Some(address_user_input)) => {
                insert_address(&mut *tx, address_user_input, sub).await.ok()
            }
            (Some(current_address_id), None) => {
                delete_address_by_id(&mut *tx, current_address_id).await?;
                None
            }
            (Some(current_address_id), Some(address_user_input)) => {
                if let Some(user_address_id) = address_user_input.id.as_uuid()
                    && user_address_id == current_address_id
                {
                    update_address(&mut *tx, address_user_input).await.ok()
                } else {
                    return Err(RepositoryError::InvalidState(
                        "unexpected billing_address id missmatch",
                    ));
                }
            }
        };

        let mailing_address = match (
            owner_profile_resolved.mailing_address,
            &owner_profile_user_input.mailing_address,
        ) {
            (None, None) => None,
            (None, Some(address_user_input)) => {
                insert_address(&mut *tx, address_user_input, sub).await.ok()
            }
            (Some(current_address_id), None) => {
                delete_address_by_id(&mut *tx, current_address_id).await?;
                None
            }
            (Some(current_address_id), Some(address_user_input)) => {
                if let Some(user_address_id) = address_user_input.id.as_uuid()
                    && user_address_id == current_address_id
                {
                    update_address(&mut *tx, address_user_input).await.ok()
                } else {
                    return Err(RepositoryError::InvalidState(
                        "unexpected mailing_address id missmatch",
                    ));
                }
            }
        };

        let owner_profile_updated = sqlx::query_as::<_, OwnerProfile>(
            r#"
            UPDATE owner_profile
            SET name = $1,
                contact_name = $2,
                email = $3,
                website = $4,
                phone_number = $5,
                owner_profile_type = $6,
                billing_address = $7,
                mailing_address = $8
            WHERE id = $9
            RETURNING *
            "#,
        )
        .bind(owner_profile_user_input.name.as_str()?)
        .bind(contact_name)
        .bind(owner_profile_user_input.email.as_str()?)
        .bind(owner_profile_user_input.website.as_str())
        .bind(owner_profile_user_input.phone_number.as_str())
        .bind(owner_profile_user_input.owner_profile_type.as_str()?)
        .bind(billing_address.map(|v| v.id))
        .bind(mailing_address.map(|v| v.id))
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(owner_profile_updated)
    }
}

pub async fn get_resolved_owner_profile<'e, E>(
    executor: E,
    for_update: bool,
) -> RepositoryResult<OwnerProfileResolved>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let for_update = match for_update {
        true => "FOR UPDATE OF owner_profile",
        false => "",
    }; // Safety: generated value

    Ok(
        sqlx::query_as::<_, OwnerProfileResolved>(AssertSqlSafe(format!(
            r#"
        SELECT
            owner_profile.id as id,
            owner_profile.name as name,
            owner_profile.contact_name as contact_name,
            owner_profile.email as email,
            owner_profile.website as website,
            owner_profile.phone_number as phone_number,
            owner_profile.owner_profile_type as owner_profile_type,
            owner_profile.created_at as created_at,
            owner_profile.updated_at as updated_at,
            owner_profile.billing_address as billing_address,
            owner_profile.mailing_address as mailing_address
        FROM owner_profile
        {for_update}
        LIMIT 1
        "#
        )))
        .fetch_one(executor)
        .await?,
    )
}
