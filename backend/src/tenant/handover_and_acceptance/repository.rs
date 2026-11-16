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
use crate::common::query_parser::ResourceQuery;
use crate::tenant::currencies::repository as currencies_repository;
use crate::tenant::customers::repository as customers_repository;
use crate::tenant::handover_and_acceptance::dto::user_input::HandoverAndAcceptanceUserInput;
use crate::tenant::handover_and_acceptance::model::{
    HandoverAndAcceptance, HandoverAndAcceptanceFull, ItemIdentifierType, ItemMake, ItemModel,
    ItemModelFull,
};
use crate::tenant::handover_and_acceptance::types::handover_and_acceptance;
use crate::tenant::users::repository as users_repository;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::PgPool;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait HandoverAndAcceptancesRepository: Send + Sync {
    async fn get(&self, id: Uuid) -> RepositoryResult<HandoverAndAcceptance>;
    async fn get_full(&self, id: Uuid) -> RepositoryResult<HandoverAndAcceptanceFull>;
    async fn get_paged(
        &self,
        query_params: &ResourceQuery<
            handover_and_acceptance::order_by::OrderBy,
            handover_and_acceptance::filter_by::FilterBy,
        >,
    ) -> RepositoryResult<(PaginatorMeta, Vec<HandoverAndAcceptanceFull>)>;
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
    async fn get(&self, id: Uuid) -> RepositoryResult<HandoverAndAcceptance> {
        get(self, id).await
    }
    async fn get_full(&self, id: Uuid) -> RepositoryResult<HandoverAndAcceptanceFull> {
        get_full(self, id, false).await
    }
    async fn get_paged(
        &self,
        query_params: &ResourceQuery<
            handover_and_acceptance::order_by::OrderBy,
            handover_and_acceptance::filter_by::FilterBy,
        >,
    ) -> RepositoryResult<(PaginatorMeta, Vec<HandoverAndAcceptanceFull>)> {
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

pub async fn get<'e, E>(executor: E, id: Uuid) -> RepositoryResult<HandoverAndAcceptance>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + Clone,
{
    Ok(sqlx::query_as::<_, HandoverAndAcceptance>(
        r#"
            SELECT *
            FROM handover_and_acceptance
            WHERE handover_and_acceptance.deleted_at IS NULL
                AND handover_and_acceptance.id = $1
            "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await?)
}

pub async fn get_item_make<'e, E>(executor: E, id: Uuid) -> RepositoryResult<ItemMake>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + Clone,
{
    Ok(sqlx::query_as::<_, ItemMake>(
        r#"
            SELECT *
            FROM item_makes
            WHERE item_makes.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await?)
}

pub async fn get_item_model<'e, E>(executor: E, id: Uuid) -> RepositoryResult<ItemModel>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + Clone,
{
    Ok(sqlx::query_as::<_, ItemModel>(
        r#"
            SELECT *
            FROM item_models
            WHERE item_models.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await?)
}

async fn get_item_identifier_type<'e, E>(
    executor: E,
    id: Uuid,
) -> RepositoryResult<ItemIdentifierType>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + Clone,
{
    Ok(sqlx::query_as::<_, ItemIdentifierType>(
        r#"
            SELECT *
            FROM item_identifier_types
            WHERE item_identifier_types.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await?)
}

pub async fn get_full<'e, E>(
    executor: E,
    id: Uuid,
    for_update: bool,
) -> RepositoryResult<HandoverAndAcceptanceFull>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + Clone,
{
    let handover_and_acceptance = get(executor.clone(), id).await?;
    let customers_full = customers_repository::get_full(
        executor.clone(),
        handover_and_acceptance.customer_id,
        for_update,
    )
    .await?;
    let item_make = if let Some(item_make_id) = handover_and_acceptance.item_make_id {
        Some(get_item_make(executor.clone(), item_make_id).await?)
    } else {
        None
    };
    let item_model = if let Some(item_model_id) = handover_and_acceptance.item_model_id {
        Some(get_item_model(executor.clone(), item_model_id).await?)
    } else {
        None
    };
    let item_model_full = if let Some(make) = &item_make
        && let Some(model) = item_model
    {
        Some(ItemModelFull::from((model, make.clone())))
    } else {
        None
    };
    let item_identifier_type = get_item_identifier_type(
        executor.clone(),
        handover_and_acceptance.item_identifier_type_id,
    )
    .await?;
    let currency = if let Some(currency_code) = &handover_and_acceptance.currency_code {
        Some(currencies_repository::get(executor.clone(), &currency_code).await?)
    } else {
        None
    };
    let user = users_repository::get(executor.clone(), handover_and_acceptance.accepted_by).await?;

    Ok(HandoverAndAcceptanceFull::from((
        handover_and_acceptance,
        customers_full,
        item_make,
        item_model_full,
        item_identifier_type,
        currency,
        user,
    )))
}
