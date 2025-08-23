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

use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a tenant with database configuration and metadata.
///
/// This struct is used to store information about a tenant,
/// including its unique identifier, database connection details, and timestamps
/// for creation, updates, and soft deletion.
///
/// # Attributes
/// - `id` (Uuid): Unique identifier of the tenant.
/// - `name` (String): Name of the tenant.
/// - `db_host` (String): Hostname of the database associated with the unit.
/// - `db_port` (i32): Port number of the database.
/// - `db_name` (String): Name of the database associated with the unit.
/// - `db_user` (String): Username for database authentication.
/// - `db_password` (String): Password for database authentication.
/// - `db_max_pool_size` (i32): Maximum pool size for database connections.
/// - `created_at` (chrono::DateTime<chrono::Local>): Timestamp when the tenant was created.
/// - `updated_at` (chrono::DateTime<chrono::Local>): Timestamp when the tenant was last updated.
/// - `deleted_at` (Option<chrono::DateTime<chrono::Local>>): Optional timestamp indicating when the
///   tenant was soft deleted (if applicable).
#[derive(Serialize, FromRow, Debug, Clone, Default)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub is_self_hosted: bool,
    pub db_host: String,
    pub db_port: i32,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_max_pool_size: i32,
    pub db_ssl_mode: String,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
    pub deleted_at: Option<chrono::DateTime<chrono::Local>>,
}

/// A struct representing the relationship between a user and a tenant in the application.
///
/// This structure is typically used to capture and manage data related to user roles
/// within specific tenants. It also maintains metadata about who invited
/// the user to the tenant, as well as timestamps for creation, updates,
/// and potential soft deletion.
///
/// # Fields
///
/// - `id` (`Uuid`): A unique identifier for the relationship instance.
/// - `user_id` (`Uuid`): The unique identifier of the user associated with this relationship.
/// - `tenant_id` (`Uuid`): The unique identifier of the tenant
///   which the user is a part of.
/// - `role` (`String`): The role assigned to the user within the tenant
///   (e.g., "owner", "member", etc.).
/// - `invited_by` (`Option<Uuid>`): The unique identifier of the user who invited this
///   user to the tenant, if applicable.
/// - `created_at` (`chrono::DateTime<chrono::Local>`): The timestamp indicating when this
///   relationship was created.
/// - `updated_at` (`chrono::DateTime<chrono::Local>`): The timestamp indicating when this
///   relationship was last updated.
/// - `deleted_at` (`Option<chrono::DateTime<chrono::Local>`): The timestamp indicating when
///   this relationship was soft deleted, if applicable.
#[derive(Serialize, FromRow, Debug, Clone, Default)]
pub struct UserTenant {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
    pub invited_by: Option<Uuid>,
    pub last_activated: chrono::DateTime<chrono::Local>,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
    pub deleted_at: Option<chrono::DateTime<chrono::Local>>,
}
