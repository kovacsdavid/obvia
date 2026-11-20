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

use crate::common::dto::PaginatorMeta;
use crate::common::error::{RepositoryError, RepositoryResult};
use crate::common::query_parser::ResourceQuery;
use crate::tenant::currencies::repository as currencies_repository;
use crate::tenant::customers::repository as customers_repository;
use crate::tenant::handover_and_acceptance::{
    self, UserInput,
    model::HandoverAndAcceptance,
    model::{HandoverAndAcceptanceFull, ItemIdentifierType, ItemMake, ItemModel, ItemModelFull},
};
use crate::tenant::users::repository as users_repository;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::{AssertSqlSafe, PgPool};
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Repository: Send + Sync {
    async fn get(&self, id: Uuid) -> RepositoryResult<HandoverAndAcceptance>;
    async fn get_full(&self, id: Uuid) -> RepositoryResult<HandoverAndAcceptanceFull>;
    async fn get_paged(
        &self,
        query_params: &ResourceQuery<
            handover_and_acceptance::types::OrderBy,
            handover_and_acceptance::types::FilterBy,
        >,
    ) -> RepositoryResult<(PaginatorMeta, Vec<HandoverAndAcceptanceFull>)>;
    async fn insert(
        &self,
        user_input: &UserInput,
        sub: Uuid,
    ) -> RepositoryResult<HandoverAndAcceptance>;
    async fn update(
        &self,
        user_input: &UserInput,
        sub: Uuid,
    ) -> RepositoryResult<HandoverAndAcceptance>;
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl Repository for PgPool {
    async fn get(&self, id: Uuid) -> RepositoryResult<HandoverAndAcceptance> {
        get(self, id).await
    }
    async fn get_full(&self, id: Uuid) -> RepositoryResult<HandoverAndAcceptanceFull> {
        get_full(self, id, false).await
    }
    async fn get_paged(
        &self,
        query_params: &ResourceQuery<
            handover_and_acceptance::types::OrderBy,
            handover_and_acceptance::types::FilterBy,
        >,
    ) -> RepositoryResult<(PaginatorMeta, Vec<HandoverAndAcceptanceFull>)> {
        get_paged(self, query_params).await
    }
    async fn insert(
        &self,
        user_input: &UserInput,
        sub: Uuid,
    ) -> RepositoryResult<HandoverAndAcceptance> {
        let mut tx = self.begin().await?;
        let item_make_id = if let Some(new_item_make) = &user_input.new_item_make {
            Some(insert_item_make(&mut tx, new_item_make.as_str()?).await?.id)
        } else {
            user_input.item_make_id.as_uuid()
        };

        let item_model_id = if let Some(new_item_model) = &user_input.new_item_model {
            if let Some(item_make_id) = item_make_id {
                Some(
                    insert_item_model(&mut tx, new_item_model.as_str()?, item_make_id)
                        .await?
                        .id,
                )
            } else {
                return Err(RepositoryError::InvalidState(
                    "item make required if model present",
                ));
            }
        } else {
            if item_make_id.is_some() || !user_input.item_model_id.is_present() {
                user_input.item_model_id.as_uuid()
            } else {
                return Err(RepositoryError::InvalidState(
                    "item make required if model present",
                ));
            }
        };

        let item_identifier_type_id =
            if let Some(new_item_identifier_type) = &user_input.new_item_identifier_type {
                insert_item_identifier_type(&mut tx, new_item_identifier_type.as_str()?)
                    .await?
                    .id
            } else if let Some(item_identifier_type_id) = &user_input.item_identifier_type_id {
                item_identifier_type_id.as_uuid()?
            } else {
                return Err(RepositoryError::InvalidState(
                    "item_identifier_type is required",
                ));
            };

        let handover_and_acceptance = insert(
            &mut tx,
            user_input,
            sub,
            item_make_id,
            item_model_id,
            item_identifier_type_id,
        )
        .await?;

        tx.commit().await?;

        Ok(handover_and_acceptance)
    }
    async fn update(
        &self,
        user_input: &UserInput,
        sub: Uuid,
    ) -> RepositoryResult<HandoverAndAcceptance> {
        let id = user_input
            .id
            .as_uuid()
            .ok_or(RepositoryError::InvalidInput("id".to_string()))?;
        let mut tx = self.begin().await?;
        let item_make_id = if let Some(new_item_make) = &user_input.new_item_make {
            Some(insert_item_make(&mut tx, new_item_make.as_str()?).await?.id)
        } else {
            user_input.item_make_id.as_uuid()
        };

        let item_model_id = if let Some(new_item_model) = &user_input.new_item_model {
            if let Some(item_make_id) = item_make_id {
                Some(
                    insert_item_model(&mut tx, new_item_model.as_str()?, item_make_id)
                        .await?
                        .id,
                )
            } else {
                return Err(RepositoryError::InvalidState(
                    "item make required if model present",
                ));
            }
        } else {
            if item_make_id.is_some() || !user_input.item_model_id.is_present() {
                user_input.item_model_id.as_uuid()
            } else {
                return Err(RepositoryError::InvalidState(
                    "item make required if model present",
                ));
            }
        };

        let item_identifier_type_id =
            if let Some(new_item_identifier_type) = &user_input.new_item_identifier_type {
                insert_item_identifier_type(&mut tx, new_item_identifier_type.as_str()?)
                    .await?
                    .id
            } else if let Some(item_identifier_type_id) = &user_input.item_identifier_type_id {
                item_identifier_type_id.as_uuid()?
            } else {
                return Err(RepositoryError::InvalidState(
                    "item_identifier_type is required",
                ));
            };

        let handover_and_acceptance = update(
            &mut tx,
            id,
            user_input,
            sub,
            item_make_id,
            item_model_id,
            item_identifier_type_id,
        )
        .await?;

        tx.commit().await?;

        Ok(handover_and_acceptance)
    }
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<()> {
        delete(&mut *self.acquire().await?, id).await?;

        Ok(())
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

pub async fn get_item_make_vec<'e, E>(executor: E, ids: &[Uuid]) -> RepositoryResult<Vec<ItemMake>>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + Clone,
{
    if ids.is_empty() {
        return Ok(vec![]);
    }
    Ok(sqlx::query_as::<_, ItemMake>(
        r#"
            SELECT *
            FROM item_makes
            WHERE item_makes.id = ANY($1)
        "#,
    )
    .bind(ids)
    .fetch_all(executor)
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

pub async fn get_item_model_vec<'e, E>(
    executor: E,
    ids: &[Uuid],
) -> RepositoryResult<Vec<ItemModel>>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + Clone,
{
    if ids.is_empty() {
        return Ok(vec![]);
    }
    Ok(sqlx::query_as::<_, ItemModel>(
        r#"
            SELECT *
            FROM item_models
            WHERE item_models.id = ANY($1)
        "#,
    )
    .bind(ids)
    .fetch_all(executor)
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

async fn get_item_identifier_type_vec<'e, E>(
    executor: E,
    ids: &[Uuid],
) -> RepositoryResult<Vec<ItemIdentifierType>>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + Clone,
{
    if ids.is_empty() {
        return Ok(vec![]);
    }
    Ok(sqlx::query_as::<_, ItemIdentifierType>(
        r#"
            SELECT *
            FROM item_identifier_types
            WHERE item_identifier_types.id = ANY($1)
        "#,
    )
    .bind(ids)
    .fetch_all(executor)
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
        Some(currencies_repository::get(executor.clone(), currency_code).await?)
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

pub async fn get_paged<'e, E>(
    executor: E,
    query_params: &ResourceQuery<
        handover_and_acceptance::types::OrderBy,
        handover_and_acceptance::types::FilterBy,
    >,
) -> RepositoryResult<(PaginatorMeta, Vec<HandoverAndAcceptanceFull>)>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + Clone,
{
    let total: (i64,) = match (
        query_params.filtering().filter_by(), // Security: ValueObject
        query_params.filtering().value_unchecked(), // Security: bind
    ) {
        (Some(filter_by), Some(value_unchecked)) => {
            // TODO: check pg_trgm
            sqlx::query_as(AssertSqlSafe(format!(
                    r#"SELECT COUNT(*) FROM handover_and_acceptance
                           WHERE deleted_at IS NULL
                               AND ($1::TEXT IS NULL OR handover_and_acceptance.{filter_by}::TEXT ILIKE '%' || $1 || '%')"#
                )))
                .bind(value_unchecked)
                .fetch_one(executor.clone())
                .await?
        }
        (_, _) => {
            sqlx::query_as("SELECT COUNT(*) FROM handover_and_acceptance WHERE deleted_at IS NULL")
                .fetch_one(executor.clone())
                .await?
        }
    };

    let order_by_clause = match (
        query_params.ordering().order_by(), // Security: ValueObject
        query_params.ordering().order(),    // Security: enum
    ) {
        (Some(order_by), Some(order)) => {
            format!("ORDER BY handover_and_acceptance.{order_by} {order}")
        }
        (_, _) => "".to_string(),
    };

    let limit = i32::try_from(query_params.paging().limit().unwrap_or(25))?;

    let handover_and_acceptance_vec = match (
        query_params.filtering().filter_by(), // Security: ValueObject
        query_params.filtering().value_unchecked(), // Security: bind
    ) {
        (Some(filter_by), Some(value_unchecked)) => {
            let sql = format!(
                r#"
                        SELECT *
                        FROM handover_and_acceptance
                        WHERE handover_and_acceptance.deleted_at IS NULL
                            AND (
                                    $1::TEXT IS NULL 
                                    OR handover_and_acceptance.{filter_by}::TEXT ILIKE '%' || $1 || '%'
                                )
                        {order_by_clause}
                        LIMIT $2
                        OFFSET $3
                    "#
            );

            sqlx::query_as::<_, HandoverAndAcceptance>(AssertSqlSafe(sql))
                .bind(value_unchecked)
                .bind(limit)
                .bind(i32::try_from(query_params.paging().offset().unwrap_or(0))?)
                .fetch_all(executor.clone())
                .await?
        }
        (_, _) => {
            let sql = format!(
                r#"
                    SELECT *
                    FROM handover_and_acceptance
                    WHERE handover_and_acceptance.deleted_at IS NULL
                    {order_by_clause}
                    LIMIT $1
                    OFFSET $2
                "#
            );

            sqlx::query_as::<_, HandoverAndAcceptance>(AssertSqlSafe(sql))
                .bind(limit)
                .bind(i32::try_from(query_params.paging().offset().unwrap_or(0))?)
                .fetch_all(executor.clone())
                .await?
        }
    };

    let customer_full_map: HashMap<_, _> = customers_repository::get_full_by_ids(
        executor.clone(),
        &handover_and_acceptance_vec
            .iter()
            .map(|v| v.customer_id)
            .collect::<Vec<Uuid>>(),
    )
    .await?
    .into_iter()
    .map(|v| (v.id, v))
    .collect();

    let item_make_map: HashMap<_, _> = get_item_make_vec(
        executor.clone(),
        &handover_and_acceptance_vec
            .iter()
            .filter_map(|v| v.item_make_id)
            .collect::<Vec<Uuid>>(),
    )
    .await?
    .into_iter()
    .map(|v| (v.id, v))
    .collect();

    let item_model_map: HashMap<_, _> = get_item_model_vec(
        executor.clone(),
        &handover_and_acceptance_vec
            .iter()
            .filter_map(|v| v.item_model_id)
            .collect::<Vec<Uuid>>(),
    )
    .await?
    .into_iter()
    .map(|v| (v.id, v))
    .collect();

    let item_identifier_type_map: HashMap<_, _> = get_item_identifier_type_vec(
        executor.clone(),
        &handover_and_acceptance_vec
            .iter()
            .map(|v| v.item_identifier_type_id)
            .collect::<Vec<Uuid>>(),
    )
    .await?
    .into_iter()
    .map(|v| (v.id, v))
    .collect();

    let currency_code_map: HashMap<_, _> = currencies_repository::get_vec(
        executor.clone(),
        &handover_and_acceptance_vec
            .iter()
            .filter_map(|v| v.currency_code.as_deref())
            .collect::<Vec<&str>>(),
    )
    .await?
    .into_iter()
    .map(|v| (v.code.clone(), v))
    .collect();

    let accepted_by_map: HashMap<_, _> = users_repository::get_vec(
        executor,
        &handover_and_acceptance_vec
            .iter()
            .map(|v| v.accepted_by)
            .collect::<Vec<Uuid>>(),
    )
    .await?
    .into_iter()
    .map(|v| (v.id, v))
    .collect();

    let mut handover_and_acceptance_full_vec = vec![];

    for handover_and_acceptance in handover_and_acceptance_vec {
        let customer = customer_full_map
            .get(&handover_and_acceptance.customer_id)
            .ok_or(RepositoryError::InvalidState(
                "customer with given key was not loaded",
            ))?
            .clone();
        let item_make = if let Some(item_make_id) = handover_and_acceptance.item_make_id {
            Some(
                item_make_map
                    .get(&item_make_id)
                    .ok_or(RepositoryError::InvalidState(
                        "item_make with given key was not loaded",
                    ))?
                    .clone(),
            )
        } else {
            None
        };
        let item_model_full = if let Some(item_model_id) = handover_and_acceptance.item_model_id
            && let Some(item_make) = &item_make
        {
            Some(ItemModelFull::from((
                item_model_map
                    .get(&item_model_id)
                    .ok_or(RepositoryError::InvalidState(
                        "item_model with given key was not loaded",
                    ))?
                    .clone(),
                item_make.clone(),
            )))
        } else {
            None
        };
        let item_identifier_type = item_identifier_type_map
            .get(&handover_and_acceptance.item_identifier_type_id)
            .ok_or(RepositoryError::InvalidState(
                "item_identifier_type with given key was not loaded",
            ))?
            .clone();
        let currency = if let Some(currency_code) = &handover_and_acceptance.currency_code {
            Some(
                currency_code_map
                    .get(currency_code)
                    .ok_or(RepositoryError::InvalidState(
                        "currency with given key was not loaded",
                    ))?
                    .clone(),
            )
        } else {
            None
        };
        let accepted_by = accepted_by_map
            .get(&handover_and_acceptance.accepted_by)
            .ok_or(RepositoryError::InvalidState(
                "accepted_by with given key was not loaded",
            ))?
            .clone();

        handover_and_acceptance_full_vec.push(HandoverAndAcceptanceFull::from((
            handover_and_acceptance,
            customer,
            item_make,
            item_model_full,
            item_identifier_type,
            currency,
            accepted_by,
        )));
    }

    Ok((
        PaginatorMeta {
            page: query_params.paging().page().unwrap_or(1).try_into()?,
            limit,
            total: total.0,
        },
        handover_and_acceptance_full_vec,
    ))
}

pub async fn insert(
    executor: &mut sqlx::PgConnection,
    user_input: &UserInput,
    sub: Uuid,
    item_make_id: Option<Uuid>,
    item_model_id: Option<Uuid>,
    item_identifier_type_id: Uuid,
) -> RepositoryResult<HandoverAndAcceptance> {
    let prefix: Option<String> = sqlx::query_scalar(
        "SELECT document_prefix FROM owner_profile
                WHERE document_prefix ~ '^[A-Z0-9]{1,10}$'
                LIMIT 1",
    )
    .fetch_optional(&mut *executor)
    .await?;

    let prefix = prefix.ok_or(RepositoryError::DocumentPrefixNotSet)?;

    Ok(sqlx::query_as::<_, HandoverAndAcceptance>(
        r#"
        WITH counter AS (
            INSERT INTO document_counters (document_type, year, last_value)
            VALUES ('worksheet', EXTRACT(YEAR FROM now())::int, 1)
            ON CONFLICT (document_type, year)
            DO UPDATE SET last_value = document_counters.last_value + 1
            RETURNING year, last_value
        )
        INSERT INTO handover_and_acceptance (
            display_id
            display_year
            display_seq
            customer_id
            item_make_id
            item_model_id
            item_identifier_type_id
            item_identifier_value
            reported_issue
            condition_at_receipt
            included_accessories
            authorized_work
            repair_limit
            currency_code
            terms_version
            accepted_by
            received_at
        )
        SELECT $1 || '-AT-' || counter.year || '-' || counter.last_value,
            counter.year,
            counter.last_value,
            $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
        FROM counter
        RETURNING *
    "#,
    )
    .bind(prefix)
    .bind(user_input.customer_id.as_uuid()?)
    .bind(item_make_id)
    .bind(item_model_id)
    .bind(item_identifier_type_id)
    .bind(user_input.item_identifier_value.as_str()?)
    .bind(user_input.reported_issue.as_str())
    .bind(user_input.condition_at_receipt.as_str())
    .bind(
        user_input
            .included_accessories
            .iter()
            .map(|v| v.as_str().unwrap_or("uxpected state"))
            .collect::<Vec<&str>>(),
    )
    .bind(user_input.authorized_work.as_str())
    .bind(user_input.repair_limit.as_f64())
    .bind(user_input.currency_code.as_str())
    .bind(user_input.terms_version.as_str())
    .bind(sub)
    .bind(user_input.received_at.as_date_naive()?)
    .fetch_one(&mut *executor)
    .await?)
}

pub async fn insert_item_identifier_type(
    executor: &mut sqlx::PgConnection,
    name: &str,
) -> RepositoryResult<ItemIdentifierType> {
    Ok(sqlx::query_as::<_, ItemIdentifierType>(
        r#"
            INSERT INTO item_identifier_types (
                name
            ) VALUES (
                $1
            ) RETURNING *
        "#,
    )
    .bind(name)
    .fetch_one(executor)
    .await?)
}

pub async fn insert_item_make(
    executor: &mut sqlx::PgConnection,
    name: &str,
) -> RepositoryResult<ItemMake> {
    Ok(sqlx::query_as::<_, ItemMake>(
        r#"
            INSERT INTO item_makes (
                name
            ) VALUES (
                $1
            ) RETURNING *
        "#,
    )
    .bind(name)
    .fetch_one(executor)
    .await?)
}

pub async fn insert_item_model(
    executor: &mut sqlx::PgConnection,
    name: &str,
    item_make_id: Uuid,
) -> RepositoryResult<ItemModel> {
    Ok(sqlx::query_as::<_, ItemModel>(
        r#"
            INSERT INTO item_models (
                make_id
                name
            ) VALUES (
                $1,
                $2
            ) RETURNING *
        "#,
    )
    .bind(item_make_id)
    .bind(name)
    .fetch_one(executor)
    .await?)
}

pub async fn update(
    executor: &mut sqlx::PgConnection,
    id: Uuid,
    user_input: &UserInput,
    sub: Uuid,
    item_make_id: Option<Uuid>,
    item_model_id: Option<Uuid>,
    item_identifier_type_id: Uuid,
) -> RepositoryResult<HandoverAndAcceptance> {
    Ok(sqlx::query_as::<_, HandoverAndAcceptance>(
        r#"
        UPDATE handover_and_acceptance
        SET customer_id = $1,
            item_make_id = $2,
            item_model_id = $3,
            item_identifier_type_id = $4,
            item_identifier_value = $5,
            reported_issue = $6,
            condition_at_receipt = $7,
            included_accessories = $8,
            authorized_work = $9,
            repair_limit = $10,
            currency_code = $11,
            terms_version = $12,
            accepted_by = $13,
            received_at = $14
        WHERE id = $15
        RETURNING *
    "#,
    )
    .bind(user_input.customer_id.as_uuid()?)
    .bind(item_make_id)
    .bind(item_model_id)
    .bind(item_identifier_type_id)
    .bind(user_input.item_identifier_value.as_str()?)
    .bind(user_input.reported_issue.as_str())
    .bind(user_input.condition_at_receipt.as_str())
    .bind(
        user_input
            .included_accessories
            .iter()
            .map(|v| v.as_str().unwrap_or("uxpected state"))
            .collect::<Vec<&str>>(),
    )
    .bind(user_input.authorized_work.as_str())
    .bind(user_input.repair_limit.as_f64())
    .bind(user_input.currency_code.as_str())
    .bind(user_input.terms_version.as_str())
    .bind(sub)
    .bind(user_input.received_at.as_date_naive()?)
    .bind(id)
    .fetch_one(executor)
    .await?)
}

pub async fn delete(executor: &mut sqlx::PgConnection, id: Uuid) -> RepositoryResult<()> {
    sqlx::query(
        r#"
            UPDATE handover_and_acceptance
            SET deleted_at = NOW()
            WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(executor)
    .await?;

    Ok(())
}
