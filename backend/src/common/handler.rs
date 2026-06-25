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

use axum::response::{IntoResponse, Response};
use std::sync::Arc;

use crate::common::{BaseModule, error::IntoFriendlyError};

pub trait ErrorMapperInterface {
    fn or_handler_error<R, E>(
        &self,
        result: Result<R, E>,
    ) -> impl Future<Output = Result<R, Response>> + Send
    where
        R: Send + Sync + 'static,
        E: IntoFriendlyError + Send + Sync + 'static;
}

pub struct ErrorMapper<M>
where
    M: BaseModule,
{
    mailer: Arc<M>,
}

impl<M> ErrorMapper<M>
where
    M: BaseModule,
{
    pub fn new(mailer: Arc<M>) -> Self {
        Self { mailer }
    }
}

impl<M> ErrorMapperInterface for ErrorMapper<M>
where
    M: BaseModule,
{
    async fn or_handler_error<R, E>(&self, result: Result<R, E>) -> Result<R, Response>
    where
        R: Send + Sync + 'static,
        E: IntoFriendlyError + Send + Sync + 'static,
    {
        match result {
            Ok(value) => Ok(value),
            Err(err) => Err(err
                .into_friendly_error(self.mailer.clone())
                .await
                .into_response()),
        }
    }
}

pub type HandlerResult = Result<Response, Response>;

#[cfg(test)]
pub mod tests {
    use crate::{common::config::tests::AppConfigBuilder, manager::auth::dto::claims::Claims};
    use chrono::Utc;
    use sqlx::error::{DatabaseError, ErrorKind};
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};
    use std::time::Duration;
    use uuid::Uuid;

    pub struct MockUniqueViolation;

    impl Error for MockUniqueViolation {}
    impl Debug for MockUniqueViolation {
        fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }
    impl Display for MockUniqueViolation {
        fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }
    impl DatabaseError for MockUniqueViolation {
        fn message(&self) -> &str {
            unimplemented!()
        }

        fn as_error(&self) -> &(dyn Error + Send + Sync + 'static) {
            unimplemented!()
        }

        fn as_error_mut(&mut self) -> &mut (dyn Error + Send + Sync + 'static) {
            unimplemented!()
        }

        fn into_error(self: Box<Self>) -> Box<dyn Error + Send + Sync + 'static> {
            unimplemented!()
        }

        fn kind(&self) -> ErrorKind {
            unimplemented!()
        }
        fn is_unique_violation(&self) -> bool {
            true
        }
    }

    pub fn generate_valid_jwt(sub: Option<Uuid>, active_tenant_id: Option<Uuid>) -> String {
        let config = AppConfigBuilder::default().build().unwrap();
        let sub = match sub {
            Some(v) => v,
            None => Uuid::new_v4(),
        };
        let exp = (Utc::now() + Duration::from_secs(100)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        Claims::new(
            sub,
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            format!("{}-api", config.auth().jwt_audience()),
            Uuid::new_v4(),
            "hu-HU".to_string(),
            "Europe/Budapest".parse().unwrap(),
            None,
            active_tenant_id,
        )
        .to_token(config.auth().jwt_secret().as_bytes())
        .unwrap()
    }

    pub fn generate_expired_jwt(active_tenant_id: Option<Uuid>) -> String {
        let config = AppConfigBuilder::default().build().unwrap();
        let exp = (Utc::now() - Duration::from_secs(100)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            format!("{}-api", config.auth().jwt_audience()),
            Uuid::new_v4(),
            "hu-HU".to_string(),
            "Europe/Budapest".parse().unwrap(),
            None,
            active_tenant_id,
        )
        .to_token(config.auth().jwt_secret().as_bytes())
        .unwrap()
    }

    pub fn generate_valid_refresh_token(
        sub: Option<Uuid>,
        active_tenant_id: Option<Uuid>,
        jti: Option<Uuid>,
        family_id: Option<Uuid>,
    ) -> String {
        let config = AppConfigBuilder::default().build().unwrap();
        let sub = match sub {
            Some(v) => v,
            None => Uuid::new_v4(),
        };
        let jti = match jti {
            Some(v) => v,
            None => Uuid::new_v4(),
        };
        let exp = (Utc::now() + Duration::from_secs(100)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        Claims::new(
            sub,
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            format!("{}-auth", config.auth().jwt_audience()),
            jti,
            "hu-HU".to_string(),
            "Europe/Budapest".parse().unwrap(),
            family_id,
            active_tenant_id,
        )
        .to_token(config.auth().jwt_secret().as_bytes())
        .unwrap()
    }

    pub fn generate_expired_refresh_token(
        sub: Option<Uuid>,
        active_tenant_id: Option<Uuid>,
        jti: Option<Uuid>,
        family_id: Option<Uuid>,
    ) -> String {
        let config = AppConfigBuilder::default().build().unwrap();
        let sub = match sub {
            Some(v) => v,
            None => Uuid::new_v4(),
        };
        let jti = match jti {
            Some(v) => v,
            None => Uuid::new_v4(),
        };
        let exp = (Utc::now() - Duration::from_secs(100)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        Claims::new(
            sub,
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            format!("{}-auth", config.auth().jwt_audience()),
            jti,
            "hu-HU".to_string(),
            "Europe/Budapest".parse().unwrap(),
            family_id,
            active_tenant_id,
        )
        .to_token(config.auth().jwt_secret().as_bytes())
        .unwrap()
    }
}
