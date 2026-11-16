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

// NOTE: The currently used resolved version of models should be deprecated.
// To avoid confusion use HandoverAndAcceptance and HandoverAndAcceptanceFull
// HandoverAndAcceptance - row data available within the handover_and_acceptance table
// HandoverAndAcceptanceFull - row data from handover_and_acceptance table and fully loaded data
// from every referenced tables

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::tenant::{
    currencies::model::Currency, customers::model::CustomerFull, users::model::User,
};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct HandoverAndAcceptance {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub item_make_id: Option<Uuid>,
    pub item_model_id: Option<Uuid>,
    pub item_identifier_type_id: Uuid,
    pub item_identifier_value: String,
    pub reported_issue: Option<String>,
    pub condition_at_receipt: Option<String>,
    pub included_accessories: Option<Vec<String>>,
    pub authorized_work: Option<String>,
    pub repair_limit: Option<BigDecimal>,
    pub currency_code: Option<String>,
    pub terms_version: Option<String>,
    pub accepted_by: Uuid,
    pub received_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct HandoverAndAcceptanceFull {
    pub id: Uuid,
    pub customer: CustomerFull,
    pub item_make: Option<ItemMake>,
    pub item_model: Option<ItemModelFull>,
    pub item_identifier_type: ItemIdentifierType,
    pub item_identifier_value: String,
    pub reported_issue: Option<String>,
    pub condition_at_receipt: Option<String>,
    pub included_accessories: Option<Vec<String>>,
    pub authorized_work: Option<String>,
    pub repair_limit: Option<BigDecimal>,
    pub currency_code: Option<Currency>,
    pub terms_version: Option<String>,
    pub accepted_by: User,
    pub received_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl
    From<(
        HandoverAndAcceptance,
        CustomerFull,
        Option<ItemMake>,
        Option<ItemModelFull>,
        ItemIdentifierType,
        Option<Currency>,
        User,
    )> for HandoverAndAcceptanceFull
{
    fn from(
        value: (
            HandoverAndAcceptance,
            CustomerFull,
            Option<ItemMake>,
            Option<ItemModelFull>,
            ItemIdentifierType,
            Option<Currency>,
            User,
        ),
    ) -> Self {
        Self {
            id: value.0.id,
            customer: value.1,
            item_make: value.2,
            item_model: value.3,
            item_identifier_type: value.4,
            item_identifier_value: value.0.item_identifier_value,
            reported_issue: value.0.reported_issue,
            condition_at_receipt: value.0.condition_at_receipt,
            included_accessories: value.0.included_accessories,
            authorized_work: value.0.authorized_work,
            repair_limit: value.0.repair_limit,
            currency_code: value.5,
            terms_version: value.0.terms_version,
            accepted_by: value.6,
            received_at: value.0.received_at,
            created_at: value.0.created_at,
            updated_at: value.0.updated_at,
            deleted_at: value.0.deleted_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder, PartialEq)]
pub struct ItemIdentifierType {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder, PartialEq)]
pub struct ItemMake {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct ItemModel {
    pub id: Uuid,
    pub make_id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder, PartialEq)]
pub struct ItemModelFull {
    pub id: Uuid,
    pub make: ItemMake,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<(ItemModel, ItemMake)> for ItemModelFull {
    fn from(value: (ItemModel, ItemMake)) -> Self {
        Self {
            id: value.0.id,
            make: value.1,
            name: value.0.name,
            is_active: value.0.is_active,
            created_at: value.0.created_at,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        common::TEST_TIME,
        tenant::{
            customers::model::tests::test_customer_full_builder,
            users::model::tests::test_user_builder,
        },
    };

    use super::*;

    pub fn test_item_make_builder() -> ItemMakeBuilder {
        let mut builder = ItemMakeBuilder::default();

        builder
            .id(Uuid::now_v7())
            .name("TestMake".to_string())
            .is_active(true)
            .created_at(*TEST_TIME);

        builder
    }

    pub fn test_item_identifier_type_builder() -> ItemIdentifierTypeBuilder {
        let mut builder = ItemIdentifierTypeBuilder::default();

        builder
            .id(Uuid::now_v7())
            .name("serial number".to_string())
            .is_active(true)
            .created_at(*TEST_TIME);

        builder
    }

    pub fn test_item_model_full_builder(
        item_make_builder: ItemMakeBuilder,
    ) -> ItemModelFullBuilder {
        let item_make = item_make_builder.build().unwrap();
        let mut builder = ItemModelFullBuilder::default();

        builder
            .id(Uuid::now_v7())
            .make(item_make)
            .name("TestModel".to_string())
            .is_active(true)
            .created_at(*TEST_TIME);

        builder
    }

    pub fn test_handover_and_acceptande_full_builder() -> HandoverAndAcceptanceFullBuilder {
        let mut builder = HandoverAndAcceptanceFullBuilder::default();
        let item_make = test_item_make_builder();

        builder
            .id(Uuid::now_v7())
            .customer(test_customer_full_builder().build().unwrap())
            .item_make(Some(item_make.build().unwrap()))
            .item_model(Some(
                test_item_model_full_builder(item_make).build().unwrap(),
            ))
            .item_identifier_type(test_item_identifier_type_builder().build().unwrap())
            .item_identifier_value("ASD123456789".to_string())
            .reported_issue(Some("Some reported issue".to_string()))
            .condition_at_receipt(Some("Condition at receipt".to_string()))
            .included_accessories(Some(vec![
                "chargeing cable".to_string(),
                "laptop bag".to_string(),
            ]))
            .authorized_work(Some("Some work to do.".to_string()))
            .repair_limit(Some("1000".parse().unwrap()))
            .terms_version(Some("v1".to_string()))
            .accepted_by(test_user_builder().build().unwrap())
            .received_at(*TEST_TIME)
            .created_at(*TEST_TIME)
            .updated_at(*TEST_TIME)
            .deleted_at(None);

        builder
    }
}
