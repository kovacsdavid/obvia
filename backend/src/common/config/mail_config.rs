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

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MailConfig {
    smtp_host: String,
    smtp_user: String,
    smtp_passwd: String,
    default_from: String,
    default_from_name: String,
    default_notification_email: String,
}

impl MailConfig {
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

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    pub struct MailConfigBuilder {
        smtp_host: Option<String>,
        smtp_user: Option<String>,
        smtp_passwd: Option<String>,
        default_from: Option<String>,
        default_from_name: Option<String>,
        default_notification_email: Option<String>,
    }

    impl MailConfigBuilder {
        pub fn new() -> Self {
            MailConfigBuilder {
                smtp_host: None,
                smtp_user: None,
                smtp_passwd: None,
                default_from: None,
                default_from_name: None,
                default_notification_email: None,
            }
        }
        pub fn smtp_host(mut self, smtp_host: &str) -> Self {
            self.smtp_host = Some(smtp_host.to_owned());
            self
        }
        pub fn smtp_user(mut self, smtp_user: &str) -> Self {
            self.smtp_user = Some(smtp_user.to_owned());
            self
        }
        pub fn smtp_passwd(mut self, smtp_passwd: &str) -> Self {
            self.smtp_passwd = Some(smtp_passwd.to_owned());
            self
        }
        pub fn default_from(mut self, default_from: &str) -> Self {
            self.default_from = Some(default_from.to_owned());
            self
        }
        pub fn default_from_name(mut self, default_from_name: &str) -> Self {
            self.default_from_name = Some(default_from_name.to_owned());
            self
        }
        pub fn default_notification_email(mut self, default_notification_email: &str) -> Self {
            self.default_notification_email = Some(default_notification_email.to_owned());
            self
        }
        pub fn build(self) -> Result<MailConfig, String> {
            Ok(MailConfig {
                smtp_host: self.smtp_host.ok_or("smtp_host is required")?,
                smtp_user: self.smtp_user.ok_or("smtp_user is required")?,
                smtp_passwd: self.smtp_passwd.ok_or("smtp_passwd is required")?,
                default_from: self.default_from.ok_or("default_from is required")?,
                default_from_name: self
                    .default_from_name
                    .ok_or("default_from_name is required")?,
                default_notification_email: self
                    .default_notification_email
                    .ok_or("default_notification_email is required")?,
            })
        }
    }

    impl Default for MailConfigBuilder {
        fn default() -> Self {
            MailConfigBuilder::new()
                .smtp_host("localhost")
                .smtp_user("noreply@example.com")
                .smtp_passwd("secret")
                .default_from("noreply@example.com")
                .default_from_name("Example")
                .default_notification_email("admin@example.com")
        }
    }
}
