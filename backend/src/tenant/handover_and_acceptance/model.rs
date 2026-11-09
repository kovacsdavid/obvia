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
use uuid::Uuid;

use crate::tenant::{
    currencies::model::Currency, customers::model::CustomerFull, users::model::User,
};

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

pub struct ItemIdentifierType {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

pub struct ItemMake {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

pub struct ItemModel {
    pub id: Uuid,
    pub make_id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
pub struct ItemModelFull {
    pub id: Uuid,
    pub make: ItemMake,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
