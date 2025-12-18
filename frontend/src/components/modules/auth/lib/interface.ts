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

import {
  type CommonResponse,
  type FormError,
  type SimpleError,
  type SimpleMessageData,
} from "@/lib/interfaces/common.ts";

export interface RegisterRequest {
  firstName: string;
  lastName: string;
  email: string;
  password: string;
  passwordConfirm: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginData {
  claims: Claims;
  user: LoginUser;
  token: string;
}

export interface LoginUser {
  id: string;
  email: string;
  first_name: string | null;
  last_name: string | null;
  status: string;
  profile_picture_url: string | null;
}

export interface ForgottenPasswordRequest {
  email: string;
}

export interface NewPasswordRequest {
  token: string;
  password: string;
  password_confirm: string;
}

export interface Claims {
  /** The subject of the token, which represents the user's unique identifier */
  sub: string;

  /** The expiration timestamp of the token in UNIX time */
  exp: number;

  /** The issued-at timestamp of the token in UNIX time */
  iat: number;

  /** The "not valid before" timestamp in UNIX time */
  nbf: number;

  /** The issuer of the token, typically representing the domain or service name */
  iss: string;

  /** The audience for the token, identifying the intended recipient(s) */
  aud: string;

  /** A unique identifier for the token, typically a UUID */
  jti: string;

  family_id: string | null;

  /** The UUID of the active tenant associated with the current context */
  active_tenant: string | null;
}

export type RegisterResponse = CommonResponse<SimpleMessageData, FormError>;
export type LoginResponse = CommonResponse<LoginData, FormError>;
export type ClaimsResponse = CommonResponse<Claims, SimpleError>;
export type VerifyEmailResponse = CommonResponse<
  SimpleMessageData,
  SimpleError
>;
export type ForgottenPasswordResponse = CommonResponse<
  SimpleMessageData,
  FormError
>;
export type NewPasswordResponse = CommonResponse<SimpleMessageData, FormError>;
