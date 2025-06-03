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

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginUser {
    pub id: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    user : LoginUser,
    token: String,
}

impl LoginResponse {
    pub fn new(user: LoginUser, token: String) -> Self {
        Self { user, token }
    }
    pub fn token(&self) -> &String {
        &self.token
    }
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    sub: String,         // User's unique ID (e.g., UUID as a string)
    exp: usize,          // Expiration time (as a UNIX timestamp)
    iat: usize,          // Issued at (as a UNIX timestamp)
    nbf: usize,          // Not valid before (as a UNIX timestamp)
    iss: String,         // Issuer (e.g., your service domain)
    aud: String,         // Audience (e.g., your frontend client ID or domain)
    jti: String,         // JWT ID (unique per token, e.g., UUID)
}

impl Claims {
    pub fn new(
        sub: String,
        exp: usize,
        iat: usize,
        nbf: usize,
        iss: String,
        aud: String,
        jti: String,
    ) -> Self {
        Self {
            sub,
            exp,
            iat,
            nbf,
            iss,
            aud,
            jti
        }
    }
    pub fn sub(&self) -> &String {
        &self.sub
    }
    pub fn exp(&self) -> usize {
        self.exp
    }
    pub fn iat(&self) -> usize {
        self.iat
    }
    pub fn nbf(&self) -> usize {
        self.nbf
    }
    pub fn iss(&self) -> &String {
        &self.iss
    }
    pub fn aud(&self) -> &String {
        &self.aud
    }
    pub fn jti(&self) -> &String {
        &self.jti
    }
}



