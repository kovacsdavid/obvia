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

use derive_builder::Builder;
use serde::Deserialize;
use std::fmt::Debug;

use crate::common::types::Password;

#[derive(Clone, Deserialize, Builder)]
pub struct MailConfig {
    mail_enabled: bool,
    smtp_host: String,
    smtp_user: String,
    smtp_passwd: String,
    default_from: String,
    default_from_name: String,
    default_notification_email: String,
}

impl MailConfig {
    pub fn mail_enabled(&self) -> bool {
        self.mail_enabled
    }
    pub fn smtp_host(&self) -> &str {
        &self.smtp_host
    }
    pub fn smtp_user(&self) -> &str {
        &self.smtp_user
    }
    pub fn smtp_passwd(&self) -> &str {
        &self.smtp_passwd
    }
    pub fn default_from(&self) -> &str {
        &self.default_from
    }
    pub fn default_from_name(&self) -> &str {
        &self.default_from_name
    }
    pub fn default_notification_email(&self) -> &str {
        &self.default_notification_email
    }
}

impl Debug for MailConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MailConfig")
            .field("mail_enabled", &self.mail_enabled)
            .field("smtp_host", &self.smtp_host)
            .field("smtp_user", &self.smtp_user)
            .field("smtp_passwd", &Password::HIDDEN_PASSWORD)
            .field("default_from", &self.default_from)
            .field("default_from_name", &self.default_from_name)
            .field(
                "default_notification_email",
                &self.default_notification_email,
            )
            .finish()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_mail_config_builder() -> MailConfigBuilder {
        let mut builder = MailConfigBuilder::default();
        builder
            .mail_enabled(false)
            .smtp_host("localhost".to_string())
            .smtp_user("noreply@example.com".to_string())
            .smtp_passwd("secret".to_string())
            .default_from("noreply@example.com".to_string())
            .default_from_name("Example".to_string())
            .default_notification_email("admin@example.com".to_string());
        builder
    }
}
