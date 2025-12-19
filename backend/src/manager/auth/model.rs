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

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use sqlx::types::JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmailVerification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub valid_until: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForgottenPassword {
    pub id: Uuid,
    pub user_id: Uuid,
    pub valid_until: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub family_id: Uuid,
    pub jti: Uuid,
    pub iat: DateTime<Local>,
    pub exp: DateTime<Local>,
    pub replaced_by: Option<Uuid>,
    pub consumed_at: Option<DateTime<Local>>,
    pub revoked_at: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AccountEventType {
    Login,
    Logout,
    PasswordChange,
    EmailChange,
    MfaEnable,
    MfaDisable,
    PasswordResetRequest,
    AccountLocked,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AccountEventStatus {
    Success,
    Failure,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountEventLogEntry {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub identifier: Option<String>,
    pub event_type: AccountEventType,
    pub status: AccountEventStatus,
    pub user_agent: Option<String>,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Local>
}
