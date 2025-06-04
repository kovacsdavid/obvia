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

use serde::{de, Deserialize, Deserializer};

use crate::common::types::{Email, FirstName, LastName, Password};

// ===== LOGIN =====
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// ===== REGISTER =====
#[derive(Debug, PartialEq, Clone)]
pub struct RegisterRequest {
    pub email: Email,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub password: Password,
    pub password_confirm: String,
    /* NOTE: Should be here on register if i18n
     * pub locale: String,
     */
}

impl<'de> Deserialize<'de> for RegisterRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RegisterRequestHelper {
            email: Email,
            first_name: FirstName,
            last_name: LastName,
            password: Password,
            password_confirm: String,
        }

        let helper = RegisterRequestHelper::deserialize(deserializer)?;

        if helper.password.as_str() != helper.password_confirm {
            return Err(de::Error::custom("A jelszó és a jelszó megerősítés mező nem egyezik"));
        }

        Ok(RegisterRequest {
            email: helper.email,
            first_name: helper.first_name,
            last_name: helper.last_name,
            password: helper.password,
            password_confirm: helper.password_confirm,
        })
    }
}



