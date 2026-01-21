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

use std::net::IpAddr;

use crate::common::error::{RepositoryError, RepositoryResult};
use crate::common::types::value_object::ValueObjectable;
use crate::manager::app::database::{PgPoolManager, PoolManager};
use crate::manager::auth::dto::claims::Claims;
use crate::manager::auth::dto::register::RegisterRequest;
use crate::manager::auth::model::{
    AccountEventLogEntry, AccountEventStatus, AccountEventType, EmailVerification,
    ForgottenPassword, RefreshToken,
};
use crate::manager::tenants::model::UserTenant;
use crate::manager::users::model::User;
use async_trait::async_trait;
use chrono::{DateTime, Local, TimeZone, Utc};
#[cfg(test)]
use mockall::automock;
use sqlx::Error;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn insert_user(
        &self,
        payload: &RegisterRequest,
        password_hash: &str,
    ) -> RepositoryResult<User>;
    async fn get_user_by_email(&self, email: &str) -> RepositoryResult<User>;

    async fn get_user_by_id(&self, user_id: Uuid) -> RepositoryResult<User>;

    async fn update_user(&self, user: User) -> RepositoryResult<User>;

    async fn update_user_last_login_at(&self, user_id: Uuid) -> RepositoryResult<()>;

    async fn get_user_active_tenant(&self, user_id: Uuid) -> RepositoryResult<Option<UserTenant>>;
    async fn insert_email_verification(&self, user_id: Uuid)
    -> RepositoryResult<EmailVerification>;
    async fn get_email_verification(
        &self,
        email_verification_id: Uuid,
    ) -> RepositoryResult<EmailVerification>;
    async fn invalidate_email_verification(
        &self,
        email_verification_id: Uuid,
    ) -> RepositoryResult<()>;
    async fn insert_forgotten_password(&self, user_id: Uuid)
    -> RepositoryResult<ForgottenPassword>;
    async fn get_forgotten_password(
        &self,
        forgotten_password_id: Uuid,
    ) -> RepositoryResult<ForgottenPassword>;
    async fn invalidate_forgotten_password(
        &self,
        forgotten_password_id: Uuid,
    ) -> RepositoryResult<()>;
    async fn insert_refresh_token(&self, claims: &Claims) -> RepositoryResult<RefreshToken>;
    async fn get_refresh_token(&self, jti: Uuid) -> RepositoryResult<RefreshToken>;
    async fn consume_refresh_token(&self, jti: Uuid, new_jti: Uuid) -> RepositoryResult<()>;
    async fn revoke_refresh_token_by_jti(&self, jti: Uuid) -> RepositoryResult<()>;
    async fn revoke_refresh_tokens_by_user_id(&self, user_id: Uuid) -> RepositoryResult<()>;
    async fn revoke_refresh_tokens_by_family_id(&self, family_id: Uuid) -> RepositoryResult<()>;
    #[allow(clippy::too_many_arguments)]
    async fn insert_account_event_log(
        &self,
        user_id: Option<Uuid>,
        identifier: Option<String>,
        event_type: AccountEventType,
        event_status: AccountEventStatus,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> RepositoryResult<AccountEventLogEntry>;
    async fn account_event_log_ip_and_event_status_count(
        &self,
        ip_address: IpAddr,
        event_status: AccountEventStatus,
        interval_mins: i64,
    ) -> RepositoryResult<i64>;
    async fn account_event_log_by_ip_and_event_type_count(
        &self,
        ip_address: IpAddr,
        event_type: AccountEventType,
        interval_mins: i64,
    ) -> RepositoryResult<i64>;
}

#[async_trait]
impl AuthRepository for PgPoolManager {
    async fn insert_user(
        &self,
        payload: &RegisterRequest,
        password_hash: &str,
    ) -> RepositoryResult<User> {
        Ok(sqlx::query_as::<_, User>(
            "INSERT INTO users (
                    id, email, password_hash, first_name, last_name, status
            ) VALUES ($1, $2, $3, $4, $5, 'unchecked_email') RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(payload.email.extract().get_value())
        .bind(password_hash)
        .bind(payload.first_name.extract().get_value())
        .bind(payload.last_name.extract().get_value())
        .fetch_one(&self.get_main_pool())
        .await?)
    }

    async fn get_user_by_email(&self, email: &str) -> RepositoryResult<User> {
        Ok(
            sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL",
            )
            .bind(email)
            .fetch_one(&self.get_main_pool())
            .await?,
        )
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> RepositoryResult<User> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL")
                .bind(user_id)
                .fetch_one(&self.get_main_pool())
                .await?,
        )
    }

    async fn update_user(&self, user: User) -> RepositoryResult<User> {
        Ok(sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET email = $1,
                password_hash = $2,
                first_name = $3,
                last_name = $4,
                phone = $5,
                status = $6,
                last_login_at = $7,
                profile_picture_url = $8,
                locale = $9,
                invited_by = $10,
                email_verified_at = $11,
                is_mfa_enabled = $12,
                mfa_secret = $13
            WHERE id = $14
                AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(user.email)
        .bind(user.password_hash)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.phone)
        .bind(user.status)
        .bind(user.last_login_at)
        .bind(user.profile_picture_url)
        .bind(user.locale)
        .bind(user.invited_by)
        .bind(user.email_verified_at)
        .bind(user.is_mfa_enabled)
        .bind(user.mfa_secret)
        .bind(user.id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }

    async fn update_user_last_login_at(&self, user_id: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET last_login_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.get_main_pool())
        .await?;
        Ok(())
    }

    async fn get_user_active_tenant(&self, user_id: Uuid) -> RepositoryResult<Option<UserTenant>> {
        let user_tenant_result = sqlx::query_as::<_, UserTenant>(
            "SELECT * FROM user_tenants WHERE user_id = $1 AND deleted_at IS NULL ORDER BY last_activated DESC LIMIT 1",
        )
        .bind(user_id)
        .fetch_one(&self.get_main_pool())
        .await;
        let user_tenant_result = match user_tenant_result {
            Ok(user_tenant) => Ok(Some(user_tenant)),
            Err(e) => match e {
                Error::RowNotFound => Ok(None),
                _ => Err(RepositoryError::Database(e)),
            },
        };
        if let Ok(user_tenant_option) = &user_tenant_result
            && let Some(user_tenant) = user_tenant_option
        {
            let _ = sqlx::query("UPDATE user_tenants SET last_activated = NOW() WHERE id = $1 AND deleted_at IS NULL")
                .bind(user_tenant.id)
                .execute(&self.get_main_pool())
                .await?;
        }
        user_tenant_result
    }
    async fn insert_email_verification(
        &self,
        user_id: Uuid,
    ) -> RepositoryResult<EmailVerification> {
        Ok(sqlx::query_as::<_, EmailVerification>(
            "INSERT INTO email_verifications (
                    user_id, valid_until
            ) VALUES ($1, NOW() + '1 day'::interval) RETURNING *",
        )
        .bind(user_id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn get_email_verification(
        &self,
        email_verification_id: Uuid,
    ) -> RepositoryResult<EmailVerification> {
        Ok(sqlx::query_as::<_, EmailVerification>(
            "SELECT * FROM email_verifications WHERE id = $1 AND valid_until > NOW() AND deleted_at IS NULL",
        )
        .bind(email_verification_id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn invalidate_email_verification(
        &self,
        email_verification_id: Uuid,
    ) -> RepositoryResult<()> {
        let _ = sqlx::query("UPDATE email_verifications SET deleted_at = NOW() WHERE id = $1")
            .bind(email_verification_id)
            .execute(&self.get_main_pool())
            .await?;
        Ok(())
    }
    async fn insert_forgotten_password(
        &self,
        user_id: Uuid,
    ) -> RepositoryResult<ForgottenPassword> {
        Ok(sqlx::query_as::<_, ForgottenPassword>(
            "INSERT INTO forgotten_passwords (
                    user_id, valid_until
            ) VALUES ($1, NOW() + '1 hour'::interval) RETURNING *",
        )
        .bind(user_id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn get_forgotten_password(
        &self,
        forgotten_password_id: Uuid,
    ) -> RepositoryResult<ForgottenPassword> {
        Ok(sqlx::query_as::<_, ForgottenPassword>(
            "SELECT * FROM forgotten_passwords WHERE id = $1 AND valid_until > NOW() AND deleted_at IS NULL",
        )
        .bind(forgotten_password_id)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn invalidate_forgotten_password(
        &self,
        forgotten_password_id: Uuid,
    ) -> RepositoryResult<()> {
        let _ = sqlx::query("UPDATE forgotten_passwords SET deleted_at = NOW() WHERE id = $1")
            .bind(forgotten_password_id)
            .execute(&self.get_main_pool())
            .await?;
        Ok(())
    }
    async fn insert_refresh_token(&self, claims: &Claims) -> RepositoryResult<RefreshToken> {
        Ok(sqlx::query_as::<_, RefreshToken>(
            "INSERT INTO refresh_tokens (
                    user_id, family_id, jti, iat, exp
            ) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(claims.sub())
        .bind(claims.family_id())
        .bind(claims.jti())
        .bind(usize_epoch_seconds_to_local(claims.iat())?)
        .bind(usize_epoch_seconds_to_local(claims.exp())?)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn get_refresh_token(&self, jti: Uuid) -> RepositoryResult<RefreshToken> {
        Ok(sqlx::query_as::<_, RefreshToken>(
            "SELECT * FROM refresh_tokens WHERE jti = $1 AND consumed_at IS NULL AND revoked_at IS NULL",
        )
        .bind(jti)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn consume_refresh_token(&self, jti: Uuid, new_jti: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            "UPDATE refresh_tokens SET consumed_at = NOW(), replaced_by = $1 WHERE jti = $2 AND consumed_at IS NULL",
        )
        .bind(new_jti)
        .bind(jti)
        .execute(&self.get_main_pool())
        .await?;
        Ok(())
    }
    async fn revoke_refresh_token_by_jti(&self, jti: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() WHERE jti = $1 AND revoked_at IS NULL",
        )
        .bind(jti)
        .execute(&self.get_main_pool())
        .await?;
        Ok(())
    }
    async fn revoke_refresh_tokens_by_user_id(&self, user_id: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
        )
        .bind(user_id)
        .execute(&self.get_main_pool())
        .await?;
        Ok(())
    }
    async fn revoke_refresh_tokens_by_family_id(&self, family_id: Uuid) -> RepositoryResult<()> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() WHERE family_id = $1 AND revoked_at IS NULL",
        )
        .bind(family_id)
        .execute(&self.get_main_pool())
        .await?;
        Ok(())
    }
    async fn insert_account_event_log(
        &self,
        user_id: Option<Uuid>,
        identifier: Option<String>,
        event_type: AccountEventType,
        event_status: AccountEventStatus,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> RepositoryResult<AccountEventLogEntry> {
        Ok(sqlx::query_as::<_, AccountEventLogEntry>(
            "INSERT INTO account_event_log (
                    user_id, identifier, event_type, status, ip_address, user_agent, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        )
        .bind(user_id)
        .bind(identifier)
        .bind(event_type)
        .bind(event_status)
        .bind(ip_address)
        .bind(user_agent)
        .bind(metadata)
        .fetch_one(&self.get_main_pool())
        .await?)
    }
    async fn account_event_log_ip_and_event_status_count(
        &self,
        ip_address: IpAddr,
        event_status: AccountEventStatus,
        interval_mins: i64,
    ) -> RepositoryResult<i64> {
        Ok(sqlx::query_scalar(
            r#"SELECT count(id)
               FROM account_event_log
               WHERE status = $1
                AND ip_address = $2
                AND created_at > NOW() - $3::interval"#,
        )
        .bind(event_status)
        .bind(ip_address)
        .bind(format!("{interval_mins} minutes"))
        .fetch_optional(&self.get_main_pool())
        .await?
        .ok_or_else(|| {
            RepositoryError::Custom(
                "account_event_log_ip_and_event_status_count: invalid value".to_string(),
            )
        })?)
    }
    async fn account_event_log_by_ip_and_event_type_count(
        &self,
        ip_address: IpAddr,
        event_type: AccountEventType,
        interval_mins: i64,
    ) -> RepositoryResult<i64> {
        Ok(sqlx::query_scalar(
            r#"SELECT count(id)
               FROM account_event_log
               WHERE event_type = $1
                AND ip_address = $2
                AND created_at > NOW() - $3::interval"#,
        )
        .bind(event_type)
        .bind(ip_address)
        .bind(format!("{interval_mins} minutes"))
        .fetch_optional(&self.get_main_pool())
        .await?
        .ok_or_else(|| {
            RepositoryError::Custom(
                "account_event_log_by_ip_and_event_type_count: invalid value".to_string(),
            )
        })?)
    }
}

pub fn usize_epoch_seconds_to_local(secs: usize) -> RepositoryResult<DateTime<Local>> {
    let secs_i64: i64 = secs
        .try_into()
        .map_err(|_| RepositoryError::Custom("timestamp too large for i64".to_string()))?;
    let utc = Utc
        .timestamp_opt(secs_i64, 0)
        .single()
        .ok_or_else(|| RepositoryError::Custom("usize_epoch_seconds_to_local: utc".to_string()))?;
    Ok(utc.with_timezone(&Local))
}
