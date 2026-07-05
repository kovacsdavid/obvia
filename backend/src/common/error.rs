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

use std::num::TryFromIntError;
use thiserror::Error;

use crate::common::{
    error::v2::{AppError, AppErrorVisibility},
    value_object::ValueObjectError,
};
use axum::http::StatusCode;
use serde::Serialize;
use serde_json::json;
use sqlx::Error;
use sqlx::migrate::MigrateError;
use tracing::Level;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Migrate error: {0}")]
    Migrate(#[from] MigrateError),

    #[error("InvalidInput error: {0}")]
    InvalidInput(String),

    #[error("Custom error: {0}")]
    Custom(String),

    #[error("RwLockReadGuard error: {0}")]
    RwLockReadGuard(String),

    #[error("RwLockWriteGuard error: {0}")]
    RwLockWriteGuard(String),

    #[error("Tenant pool not found")]
    TenantPoolNotFound,
}

impl From<ValueObjectError> for RepositoryError {
    fn from(value: ValueObjectError) -> Self {
        match value {
            ValueObjectError::InvalidInput(e) => Self::InvalidInput(e.to_string()),
            _ => Self::Custom(value.to_string()),
        }
    }
}

impl From<TryFromIntError> for RepositoryError {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidInput("could not parse integer".to_string())
    }
}

impl RepositoryError {
    pub fn is_unique_violation(&self) -> bool {
        if let RepositoryError::Database(sqlxe) = self
            && let Error::Database(database_error) = sqlxe
            && database_error.is_unique_violation()
        {
            return true;
        }
        false
    }
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, Error, Serialize)]
pub enum BuilderError {
    #[error("{0} is required")]
    MissingRequired(&'static str),
}

impl From<BuilderError> for AppError {
    fn from(value: BuilderError) -> Self {
        Self::new(
            Level::ERROR,
            StatusCode::INTERNAL_SERVER_ERROR,
            file!(),
            AppErrorVisibility::Internal,
            json!({
                "message":
                value.to_string(),
            }),
        )
    }
}

pub type BuilderResult<T> = Result<T, BuilderError>;

pub mod v2 {
    use std::sync::Arc;

    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use handlebars::Handlebars;
    use lettre::{
        Message,
        message::{Mailbox, header::ContentType},
    };
    use serde_json::json;
    use tracing::{Level, event};

    use crate::common::{
        BaseModule, ConfigProvider, MailTransporter, config::AppConfig, dto::ErrorResponse,
    };

    #[derive(Debug, PartialEq)]
    pub enum AppErrorVisibility {
        UserFacing,
        Internal,
    }

    #[derive(Debug)]
    pub struct AppError {
        log_level: Level,
        http_status: StatusCode,
        location: &'static str,
        visibility: AppErrorVisibility,
        json: serde_json::Value,
    }

    impl AppError {
        pub fn new(
            log_level: Level,
            http_status: StatusCode,
            location: &'static str,
            visibility: AppErrorVisibility,
            json: serde_json::Value,
        ) -> Self {
            let app_error = Self {
                log_level,
                http_status,
                location,
                visibility,
                json,
            };
            app_error.trace();
            app_error
        }
    }

    impl AppError {
        fn trace(&self) {
            match self.visibility {
                AppErrorVisibility::UserFacing => match self.log_level {
                    Level::ERROR => {
                        event!(Level::ERROR, "{:?}", self,);
                    }
                    Level::WARN => {
                        event!(Level::WARN, "{:?}", self,);
                    }
                    Level::INFO => {
                        event!(Level::INFO, "{:?}", self,);
                    }
                    Level::DEBUG => {
                        event!(Level::DEBUG, "{:?}", self,);
                    }
                    Level::TRACE => {
                        event!(Level::TRACE, "{:?}", self,);
                    }
                },
                AppErrorVisibility::Internal => match self.log_level {
                    Level::ERROR => {
                        event!(Level::ERROR, "{:?}", self,);
                    }
                    Level::WARN => {
                        event!(Level::WARN, "{:?}", self,);
                    }
                    Level::INFO => {
                        event!(Level::INFO, "{:?}", self,);
                    }
                    Level::DEBUG => {
                        event!(Level::DEBUG, "{:?}", self,);
                    }
                    Level::TRACE => {
                        event!(Level::TRACE, "{:?}", self,);
                    }
                },
            }
        }
        pub async fn notify_admin<M>(&self, mailer: Arc<M>)
        where
            M: MailTransporter + ConfigProvider<Cfg = AppConfig>,
        {
            let config = mailer.config().mail();
            let default_from = config.default_from().parse();
            let default_notification_email = config.default_notification_email().parse();
            let default_from_name = config.default_from_name().to_owned();
            let handlebars = Handlebars::new();
            let template = handlebars.render_template(
                r##"
                    Dear Admin!\n\n
                    Check this error!\n
                    Internal error: location={{loc}} message={{message}}
                    "##,
                &json!({"location": self.location, "message": self.json.to_string()}),
            );

            if let Ok(default_from) = default_from
                && let Ok(default_notification_email) = default_notification_email
                && let Ok(body) = template
                && let Ok(message) = Message::builder()
                    .from(Mailbox::new(Some(default_from_name), default_from))
                    .to(Mailbox::new(None, default_notification_email))
                    .subject("Unexpected error")
                    .header(ContentType::TEXT_PLAIN)
                    .body(body)
            {
                let _ = mailer.send(message).await;
            }
        }
        pub async fn notify_admin_if_internal<M>(&self, mailer: Arc<M>)
        where
            M: BaseModule,
        {
            if self.visibility == AppErrorVisibility::Internal {
                self.notify_admin(mailer).await
            }
        }
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            match self.visibility {
                AppErrorVisibility::UserFacing => ErrorResponse {
                    status_code: self.http_status,
                    error: self.json,
                }
                .into_response(),
                AppErrorVisibility::Internal => ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: json!({
                        "message":
                        "Váratlan hiba történt a feldolgozás során"
                    }),
                }
                .into_response(),
            }
        }
    }
}
